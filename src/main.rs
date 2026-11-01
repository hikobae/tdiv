#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use windows::Win32::System::Com::{
    CLSCTX_ALL, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx, CoUninitialize,
    IServiceProvider,
};
use windows::Win32::System::Variant::VARIANT;
use windows::Win32::UI::Shell::{
    CSIDL_DESKTOP, FOLDERVIEWMODE, FVM_ICON, FVM_LIST, IFolderView2, IShellBrowser, IShellWindows,
    SID_STopLevelBrowser, SWC_DESKTOP, SWFO_NEEDDISPATCH, ShellWindows,
};
use windows::core::Interface;
use windows::core::Result as WinResult;
use windows_registry::{CURRENT_USER, Key as RegKey};

macro_rules! dprint {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        { println!($($arg)*); }
    };
}

struct ComInitializer;
impl ComInitializer {
    fn new() -> WinResult<Self> {
        unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()? }
        Ok(Self)
    }
}

impl Drop for ComInitializer {
    fn drop(&mut self) {
        unsafe { CoUninitialize() }
    }
}

macro_rules! reg {
    ($e:expr) => {
        $e.map_err(|e| windows::core::Error::from_hresult(windows::core::HRESULT(e.code().0)))
    };
}

struct Setting {
    key: RegKey,
}

impl Setting {
    const FALLBACK_SETTING: (FOLDERVIEWMODE, i32) = (FVM_ICON, 48); // Windows default for desktop icons
    const REGISTRY_PATH: &str = r"Software\f8d941da-303e-48dc-affb-bb763593bd8d";
    const SUB_KEY_MODE: &str = "Mode";
    const SUB_KEY_ICON_SIZE: &str = "IconSize";

    fn new() -> WinResult<Self> {
        Ok(Self {
            key: reg!(CURRENT_USER.create(Self::REGISTRY_PATH))?,
        })
    }

    fn fallback() -> (FOLDERVIEWMODE, i32) {
        Self::FALLBACK_SETTING
    }

    fn load(&self) -> (FOLDERVIEWMODE, i32) {
        let saved = self
            .key
            .get_u32(Self::SUB_KEY_MODE)
            .ok()
            .zip(self.key.get_u32(Self::SUB_KEY_ICON_SIZE).ok());
        match saved {
            Some((m, s)) => (FOLDERVIEWMODE(m as i32), s as i32),
            None => Self::fallback(),
        }
    }

    fn save(&self, mode: FOLDERVIEWMODE, icon_size: i32) -> WinResult<()> {
        reg!(self.key.set_u32(Self::SUB_KEY_MODE, mode.0 as u32))?;
        reg!(self.key.set_u32(Self::SUB_KEY_ICON_SIZE, icon_size as u32))?;
        Ok(())
    }

    fn clear(&self) {
        let _r = CURRENT_USER.remove_tree(Self::REGISTRY_PATH);
        dprint!("clear: {:?}", _r);
    }
}

fn toggle_desktop_icon_view() -> WinResult<()> {
    unsafe {
        let shell_windows: IShellWindows = CoCreateInstance(&ShellWindows, None, CLSCTX_ALL)?;
        let loc = VARIANT::from(CSIDL_DESKTOP as i32);
        let empty = VARIANT::default();
        let mut hwnd = 0i32;
        let disp =
            shell_windows.FindWindowSW(&loc, &empty, SWC_DESKTOP, &mut hwnd, SWFO_NEEDDISPATCH)?;

        let sp: IServiceProvider = disp.cast()?;
        let browser: IShellBrowser = sp.QueryService(&SID_STopLevelBrowser)?;
        let view = browser.QueryActiveShellView()?;
        let fv: IFolderView2 = view.cast()?;

        let mut mode = Default::default();
        let mut icon_size = 0i32;
        fv.GetViewModeAndIconSize(&mut mode, &mut icon_size)?;
        dprint!("before: mode={} icon_size={}", mode.0, icon_size);

        let setting = Setting::new()?;
        if mode == FVM_LIST {
            let (new_mode, new_icon_size) = setting.load();
            let r = fv
                .SetViewModeAndIconSize(new_mode, new_icon_size)
                .or_else(|_| {
                    let (dm, ds) = Setting::fallback();
                    fv.SetViewModeAndIconSize(dm, ds)
                });
            setting.clear();
            r?;
        } else {
            setting.save(mode, icon_size)?;

            const LIST_ICON_SIZE: i32 = 16; // リスト表示に最適なサイズ
            if let Err(e) = fv.SetViewModeAndIconSize(FVM_LIST, LIST_ICON_SIZE) {
                // 失敗時は元の状態に戻す
                setting.clear();
                return Err(e);
            }
        }

        #[cfg(debug_assertions)]
        {
            fv.GetViewModeAndIconSize(&mut mode, &mut icon_size)?;
            dprint!("after:  mode={} icon_size={}", mode.0, icon_size);
        }
    }
    Ok(())
}

fn main() -> WinResult<()> {
    let _com = ComInitializer::new()?;
    toggle_desktop_icon_view()
}
