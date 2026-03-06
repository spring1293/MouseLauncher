# リポジトリ構造定義書

## フォルダ・ファイル構成

```
MouseLauncher/
├── src-tauri/                    # Tauriバックエンド（Rust）
│   ├── src/
│   │   ├── main.rs               # エントリーポイント
│   │   ├── lib.rs                # アプリケーション初期化・セットアップ
│   │   ├── commands/             # Tauri IPCコマンド定義
│   │   │   ├── mod.rs
│   │   │   ├── launcher.rs       # アプリ起動・フォーカスコマンド
│   │   │   └── config.rs         # 設定読み書きコマンド
│   │   ├── launcher/             # アプリケーション起動ロジック
│   │   │   ├── mod.rs
│   │   │   ├── windows.rs        # Windows向け実装
│   │   │   └── macos.rs          # macOS向け実装
│   │   ├── config/               # 設定管理
│   │   │   ├── mod.rs
│   │   │   └── store.rs          # 設定の読み込み・保存
│   │   └── tray.rs               # システムトレイ / メニューバー常駐
│   ├── capabilities/             # Tauriパーミッション定義
│   │   └── default.json
│   ├── icons/                    # アプリアイコン（各OS・サイズ）
│   ├── Cargo.toml
│   └── tauri.conf.json           # Tauri設定ファイル
│
├── ui/                           # フロントエンド（HTML/CSS/JS）
│   ├── launcher/                 # パイメニュー画面
│   │   ├── index.html
│   │   ├── launcher.css
│   │   └── launcher.js
│   └── settings/                 # 設定画面
│       ├── index.html
│       ├── settings.css
│       └── settings.js
│
├── docs/                         # 永続的ドキュメント
│   ├── product-requirements.md
│   ├── functional-design.md
│   ├── architecture.md
│   ├── repository-structure.md   # 本ファイル
│   ├── development-guidelines.md
│   └── glossary.md
│
├── .steering/                    # 作業単位のドキュメント
│   └── [YYYYMMDD]-[タイトル]/
│       ├── requirements.md
│       ├── design.md
│       └── tasklist.md
│
├── .github/                      # GitHub設定（将来用）
│   └── workflows/
│
├── package.json                  # Node.js依存・ビルドスクリプト
├── package-lock.json
└── README.md
```

---

## ディレクトリの役割

### `src-tauri/`
Tauriバックエンドのすべてを格納するディレクトリ。Rustのソースコード、Tauri設定、アプリアイコンを含む。

### `src-tauri/src/`
Rustのソースコード。責務ごとにモジュールを分割する。

| ファイル / ディレクトリ | 役割 |
|---|---|
| `main.rs` | プロセスのエントリーポイント。最小限のコードのみ記述 |
| `lib.rs` | Tauriアプリの初期化、プラグイン登録、イベントハンドラの設定 |
| `commands/` | フロントエンドから呼び出せるTauri IPCコマンドを定義 |
| `launcher/` | OSごとのアプリ起動・フォーカスロジック |
| `config/` | config.jsonの読み込み・保存・デフォルト生成 |
| `tray.rs` | システムトレイ（Windows）・メニューバー常駐（macOS）の制御 |

### `src-tauri/capabilities/`
Tauri v2のパーミッションシステム。フロントエンドがアクセスできるAPIを明示的に定義する。

### `ui/`
フロントエンドのHTML/CSS/JSを格納するディレクトリ。画面ごとにサブディレクトリを作成する。

| ディレクトリ | 役割 |
|---|---|
| `ui/launcher/` | パイメニューオーバーレイの画面 |
| `ui/settings/` | 設定画面 |

### `docs/`
アプリケーション全体の永続的ドキュメント。基本設計が変わらない限り更新されない。

### `.steering/`
作業単位のドキュメント。開発タスクごとに新しいディレクトリを作成する。

---

## ファイル配置ルール

### Rustソースコード
- OS固有の実装は `launcher/windows.rs`・`launcher/macos.rs` に分離し、`mod.rs` でプラットフォームに応じて切り替える
- Tauri IPCコマンドはすべて `commands/` 配下に定義し、`lib.rs` で登録する
- 1ファイルの行数が200行を超える場合はモジュール分割を検討する

### フロントエンド
- 画面ごとにディレクトリを分け、HTML・CSS・JSをセットで管理する
- 共通のスタイルが生じた場合は `ui/shared/` を作成して集約する（初期は不要）

### アイコン
- アプリアイコンは `src-tauri/icons/` に配置する
- Tauriが要求する各サイズ（32x32〜1024x1024）を揃える

### ドキュメント
- 永続的ドキュメントは `docs/` に配置する
- 作業単位のドキュメントは `.steering/[YYYYMMDD]-[タイトル]/` に配置する
- 図表はドキュメント内にMermaid記法で直接記述する
