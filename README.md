# tdiv

Windows用 デスクトップアイコン表示切り替えツール (Toggle Desktop Icon View)。

`tdiv` は、デスクトップアイコン表示を「リスト」と「元の表示」の間でトグルする。

## Requirements

- Windows 11

## Build

```bash
cargo build --release
```

実行ファイルは `target\release\` 以下に生成される。

## Usage

ビルドしたバイナリを実行する。

```bash
tdiv.exe
```

実行するたびに、デスクトップのアイコン表示を切り替える。

- 現在が「リスト」以外の場合: 現在の表示モードとアイコンサイズをレジストリに保存し、「リスト」(アイコンサイズ 16) に変更
- 現在が「リスト」の場合: 保存した表示に戻し、保存値を削除
  - 保存値がない場合は Windows 既定のアイコン表示 (アイコンサイズ 48) に戻す

## Notes

- 設定は `HKCU\Software\f8d941da-303e-48dc-affb-bb763593bd8d` に保存する。リストから戻すと削除される。
- 元からリスト表示で使っている場合、初回実行でアイコン表示 (48) に切り替わる。
- リリースビルドはコンソールを表示しない。エラー時も通知はない。

## License

MIT License

## Author

TAKAHASHI Satoshi <hikobae@gmail.com>
