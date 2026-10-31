#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use windows::Win32::System::Com::{
    CLSCTX_ALL, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx, CoUninitialize,
    IServiceProvider,
};
use windows::Win32::System::Variant::VARIANT;
use windows::Win32::UI::Shell::{
    CSIDL_DESKTOP, FVM_LIST, FVM_SMALLICON, IFolderView2, IShellBrowser, IShellWindows,
    SID_STopLevelBrowser, SWC_DESKTOP, SWFO_NEEDDISPATCH, ShellWindows,
};
use windows::core::{Interface, Result};

macro_rules! dprint {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        { println!($($arg)*); }
    };
}

struct ComInitializer;
impl ComInitializer {
    fn new() -> Result<Self> {
        unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()? }
        Ok(Self)
    }
}

impl Drop for ComInitializer {
    fn drop(&mut self) {
        unsafe { CoUninitialize() }
    }
}

fn toggle_desktop_icon_view() -> Result<()> {
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

        let (view_mode, new_icon_size) = if mode.0 == FVM_LIST.0 {
            (FVM_SMALLICON, 32)
        } else {
            (FVM_LIST, 16)
        };
        fv.SetViewModeAndIconSize(view_mode, new_icon_size)?;

        fv.GetViewModeAndIconSize(&mut mode, &mut icon_size)?;
        dprint!("after:  mode={} icon_size={}", mode.0, icon_size);
    }
    Ok(())
}

fn main() -> Result<()> {
    let _com = ComInitializer::new()?;
    toggle_desktop_icon_view()
}
