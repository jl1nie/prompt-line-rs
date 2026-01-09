# prompt-line-rs

[English](README.md) | 日本語

Windows向けのフローティングテキスト入力ツール。[prompt-line](https://github.com/nkmr-jp/prompt-line)にインスパイアされています。グローバルホットキーで素早く起動し、テキストを入力して、ワンキーで任意のアプリケーションにペーストできます。

## 特徴

- **グローバルホットキー**: どこからでも`Ctrl+Shift+Space`で起動
- **クイックペースト**: `Ctrl+Enter`でテキストをコピーして直前のアプリにペースト
- **履歴ナビゲーション**: `Ctrl+J`/`Ctrl+K`で履歴を移動
- **履歴検索**: `Ctrl+F`で入力履歴を検索
- **システムトレイ**: システムトレイで静かに動作、いつでも準備完了
- **日本語対応**: IMEによる日本語入力を完全サポート
- **カスタマイズ可能**: ショートカットやウィンドウ設定を変更可能

## インストール

### インストーラー（推奨）

[リリースページ](https://github.com/jl1nie/prompt-line-rs/releases)からダウンロード:

- **`prompt-line-rs_x.x.x_x64-setup.exe`** - NSISインストーラー（推奨）
- **`prompt-line-rs_x.x.x_x64_en-US.msi`** - MSIインストーラー

### ポータブル版

`prompt-line-rs.exe`をダウンロードして直接実行。インストール不要。

### ソースからビルド

```bash
# リポジトリをクローン
git clone https://github.com/jl1nie/prompt-line-rs.git
cd prompt-line-rs

# 依存関係をインストール
npm install

# リリースビルド
npm run tauri build

# 出力先:
#   src-tauri/target/release/prompt-line-rs.exe
#   src-tauri/target/release/bundle/nsis/*.exe
#   src-tauri/target/release/bundle/msi/*.msi
```

## 使い方

1. **起動**: `prompt-line-rs.exe`を実行（システムトレイにアイコンが表示）
2. **ウィンドウ表示**: `Ctrl+Shift+Space`を押す
3. **入力**: テキストを入力
4. **ペースト**: `Ctrl+Enter`で直前のアプリケーションにペースト
5. **履歴移動**: `Ctrl+J`（次）/ `Ctrl+K`（前）
6. **履歴検索**: `Ctrl+F`を押して検索文字を入力
7. **閉じる**: `Escape`で非表示（アプリはシステムトレイに残る）

## キーボードショートカット

| ショートカット       | 動作                                   |
|---------------------|----------------------------------------|
| `Ctrl+Shift+Space`  | ウィンドウの表示/非表示（グローバル）    |
| `Ctrl+Enter`        | テキストをコピーして直前のアプリにペースト |
| `Ctrl+J`            | 次の履歴項目                            |
| `Ctrl+K`            | 前の履歴項目                            |
| `Ctrl+F`            | 履歴検索を開く                          |
| `Escape`            | ウィンドウを閉じる / 検索を終了          |

**フォールバックホットキー**: `Ctrl+Shift+Space`が使用できない場合、以下の順で試行:

- `Win+Shift+Space`
- `Alt+Space`
- `Ctrl+Alt+P`

## 設定

設定ファイルの場所:

```text
%APPDATA%\prompt-line\prompt-line-rs\config\config.toml
```

デフォルト設定:

```toml
[shortcuts]
launch = "Ctrl+Shift+Space"
paste = "Ctrl+Enter"
close = "Escape"
history_next = "Ctrl+j"
history_prev = "Ctrl+k"
search = "Ctrl+f"

[history]
max_entries = 1000

[window]
width = 600.0
height = 300.0
font_size = 16.0
```

## 履歴

履歴の保存先:

```text
%APPDATA%\prompt-line\prompt-line-rs\data\history.jsonl
```

## ユースケース

- CLIベースのAIコーディングエージェント（Claude Code、Gemini CLIなど）
- Enterキーで意図せずメッセージを送信してしまうチャットアプリ
- 入力レスポンスが遅いテキストエディタ
- IMEサポートが限定的なアプリケーションでの日本語入力

## 技術詳細

使用技術:

- **Tauri 2.0**: 軽量デスクトップアプリフレームワーク
- **TypeScript/Vite**: モダンなフロントエンドツール
- **Rust**: ネイティブWindows API統合のバックエンド
- **WebView2**: 最小バンドルサイズのためのシステムWebView

## システム要件

- Windows 10/11 (64-bit)
- WebView2 Runtime（Windows 11には含まれています。Windows 10では自動インストール）

## ライセンス

MIT

## 謝辞

nkmr-jpの[prompt-line](https://github.com/nkmr-jp/prompt-line)にインスパイアされました。
