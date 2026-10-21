# Mouse Launcher

マウスカーソルの位置に円形のメニュー（パイメニュー）を出して、登録したアプリをすぐ起動できるデスクトップランチャーです。

ショートカットキーを押すと、カーソルの周りに 6 つのアプリが並んだメニューが表示されます。起動したいアプリをクリックするだけで、タスクバーや Dock までマウスを動かす必要はありません。

- **どのアプリの上からでも呼び出せる**: グローバルショートカットキー（初期値は `Ctrl+Shift+Space`）で起動
- **カーソル位置に表示**: マウスをほとんど動かさずに選べる
- **起動中のアプリは前面に**: すでに起動しているアプリを選ぶと、新しく起動せず既存のウィンドウを前面に出す
- **Windows / macOS 対応**: Windows 10 以降、macOS 12 以降

---

## インストール

[GitHub Actions](../../actions/workflows/build.yml) の最新の成功した実行から、Artifacts をダウンロードします。

| OS | Artifact | 中身 |
|---|---|---|
| Windows | `MouseLauncher-Windows` | `mouse-launcher.exe`（インストール不要のポータブル版）、`nsis/*-setup.exe`（インストーラー版） |
| macOS | `MouseLauncher-macOS` | `.dmg`（開いて Applications フォルダへドラッグ） |

### macOS の初回起動

ショートカットキーを受け取るために **アクセシビリティ権限** が必要です。権限がない状態で起動すると案内のダイアログとシステム設定が開くので、「アクセシビリティ」の一覧で Mouse Launcher をオンにしてからアプリを起動し直してください。

---

## 使い方

### ランチャーを開く・閉じる

1. `Ctrl+Shift+Space` を押すと、カーソルの位置にメニューが表示されます
2. アプリのアイコンをクリックすると起動し、メニューは自動で閉じます
3. 何もせずに閉じたいときは `Esc` キーを押すか、メニューの外側をクリックします

アプリは常駐し、システムトレイ（Windows）またはメニューバー（macOS）にアイコンが表示されます。終了するときはこのアイコンのメニューから「終了」を選びます。

### アプリを登録する

メニュー中央の ⚙ ボタンで設定画面を開きます。

- **スロットへの登録**: 6 つのスロットそれぞれに、次のどちらかの方法でアプリを登録します。
  - 「設定」ボタンを押して、実行ファイル（`.exe` / `.app`）を選ぶ
  - 実行ファイルをスロットにドラッグ&ドロップする
- **変更・削除**: 登録済みのスロットは「変更」で別のアプリに変えたり、「削除」で外したりできます。何も登録していないスロットは、メニュー上でグレー表示になります。
- **ショートカットキーの変更**: 「変更」を押してから、使いたいキーの組み合わせを押します。修飾キーなしの単独キーは登録できません。

スロットはメニューの上（12 時の方向）から時計回りに並びます。

### 設定ファイルの保存場所

| OS | パス |
|---|---|
| Windows | `%APPDATA%\MouseLauncher\config.json` |
| macOS | `~/Library/Application Support/MouseLauncher/config.json` |

---

## 開発

### 必要なもの

- [Rust](https://www.rust-lang.org/tools/install) 1.70 以降
- [Node.js](https://nodejs.org/) 20 以降
- 各 OS の Tauri 前提パッケージ（[Tauri v2 の Prerequisites](https://v2.tauri.app/start/prerequisites/) を参照）

### コマンド

```bash
npm install            # 依存パッケージのインストール
npm run tauri dev      # 開発モードで起動
npm run tauri build    # リリースビルド
```

ビルド結果は `src-tauri/target/release/` に出力されます。ビルドは実行している OS 向けのものしか作れません。Windows 用と macOS 用の両方を作るときは、GitHub Actions を使います（`main` への push、または Actions タブの **Run workflow** で実行）。

コードを変更したら、コミット前に次を実行してください。

```bash
cd src-tauri
cargo fmt
cargo clippy
cargo test
```

### 技術スタック

- [Tauri v2](https://v2.tauri.app/)（バックエンドは Rust）
- フロントエンドは HTML / CSS / JavaScript（フレームワークなし）

### ディレクトリ構成

```
MouseLauncher/
├── src-tauri/          # バックエンド（Rust）・Tauri 設定・アイコン
│   └── src/
│       ├── commands/   # フロントエンドから呼ぶ IPC コマンド
│       ├── launcher/   # OS ごとのアプリ起動・フォーカス処理
│       ├── config/     # 設定ファイルの読み書き
│       └── tray.rs     # トレイ / メニューバー常駐
├── ui/
│   ├── launcher/       # パイメニュー画面
│   └── settings/       # 設定画面
├── docs/               # 設計ドキュメント
└── .steering/          # 作業ごとの要求・設計・タスクリスト
```

設計の詳細は [docs/](docs/) を参照してください。

| ドキュメント | 内容 |
|---|---|
| [product-requirements.md](docs/product-requirements.md) | プロダクト要求定義 |
| [functional-design.md](docs/functional-design.md) | 機能設計 |
| [architecture.md](docs/architecture.md) | 技術仕様 |
| [repository-structure.md](docs/repository-structure.md) | リポジトリ構造 |
| [development-guidelines.md](docs/development-guidelines.md) | 開発ガイドライン |
| [glossary.md](docs/glossary.md) | 用語集 |

---

## ライセンス

[MIT License](LICENSE)
