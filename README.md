# tdiv

Windows用 デスクトップアイコン表示切り替えツール (Toggle Desktop Icon View)。

`tdiv` は、デスクトップアイコン表示を「リスト」と「小アイコン」の間でトグルする。

## Requirements

- Windows 10 以降

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

- 現在が「リスト」の場合: 「小アイコン」に変更
- それ以外の場合: 「リスト」に変更

## License

MIT License

## Author

TAKAHASHI Satoshi <hikobae@gmail.com>
