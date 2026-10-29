# 初回実装 設計

## 実装アプローチ

Tauri v2 を使用し、RustバックエンドでOS操作・設定管理を担い、Vanilla HTML/CSS/JSでUIを描画する。
パイメニューはウィンドウを常時バックグラウンドで保持し、表示/非表示の切り替えのみで100ms以内の表示を実現する。

---

## 実装順序

以下の順序で実装することで、各ステップで動作確認しながら進める。

```
1. プロジェクトセットアップ
2. 設定管理（Config Store）
3. 常駐起動（Tray Icon）
4. グローバルショートカット
5. パイメニューUI
6. アプリ起動・フォーカス
7. 設定画面UI
8. 動作確認・調整
```

---

## 変更するコンポーネント

### 新規作成（すべて）

初回実装のため、すべてのファイルを新規作成する。

#### バックエンド（Rust）

| ファイル | 実装内容 |
|---|---|
| `src-tauri/src/main.rs` | エントリーポイント（最小限） |
| `src-tauri/src/lib.rs` | アプリ初期化、プラグイン登録、ウィンドウ設定 |
| `src-tauri/src/config/mod.rs` | Config・Slot構造体の定義 |
| `src-tauri/src/config/store.rs` | config.jsonの読み込み・保存・デフォルト生成 |
| `src-tauri/src/commands/mod.rs` | コマンドモジュールのエクスポート |
| `src-tauri/src/commands/config.rs` | 設定取得・更新・トリガーキー変更コマンド |
| `src-tauri/src/commands/launcher.rs` | アプリ起動・フォーカスコマンド |
| `src-tauri/src/launcher/mod.rs` | OS分岐ロジック |
| `src-tauri/src/launcher/windows.rs` | Windows向けアプリ起動・フォーカス実装 |
| `src-tauri/src/launcher/macos.rs` | macOS向けアプリ起動・フォーカス実装 |
| `src-tauri/src/tray.rs` | システムトレイ / メニューバー常駐の設定 |

#### フロントエンド（Web）

| ファイル | 実装内容 |
|---|---|
| `ui/launcher/index.html` | パイメニューのHTML構造 |
| `ui/launcher/launcher.css` | パイメニューのスタイル・アニメーション |
| `ui/launcher/launcher.js` | パイメニューの表示制御・イベント処理 |
| `ui/settings/index.html` | 設定画面のHTML構造 |
| `ui/settings/settings.css` | 設定画面のスタイル |
| `ui/settings/settings.js` | 設定画面のイベント処理・Tauri API呼び出し |

#### Tauri設定

| ファイル | 実装内容 |
|---|---|
| `src-tauri/tauri.conf.json` | ウィンドウ設定・アプリ設定 |
| `src-tauri/capabilities/default.json` | パーミッション定義 |
| `src-tauri/Cargo.toml` | 依存クレートの定義 |
| `package.json` | ビルドスクリプト・Node.js依存 |

---

## データ構造

### Config（Rust）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub trigger_key: String,
    pub slots: Vec<Slot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slot {
    pub id: usize,
    pub action_type: String,
    pub app_path: Option<String>,
    pub app_name: Option<String>,
    pub icon_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            trigger_key: "Ctrl+Shift+Space".to_string(),
            slots: (0..6).map(|i| Slot {
                id: i,
                action_type: "app".to_string(),
                app_path: None,
                app_name: None,
                icon_path: None,
            }).collect(),
        }
    }
}
```

---

## Tauri IPCコマンド設計

| コマンド名 | 引数 | 戻り値 | 処理内容 |
|---|---|---|---|
| `get_config` | なし | `Config` | 現在の設定を返す |
| `update_slot` | `slot: Slot` | `Result<()>` | 指定スロットを更新して保存 |
| `delete_slot` | `slot_id: usize` | `Result<()>` | 指定スロットをクリアして保存 |
| `update_trigger_key` | `key: String` | `Result<()>` | トリガーキーを更新・再登録して保存 |
| `launch_app` | `slot_id: usize` | `Result<()>` | アプリ起動またはフォーカス |
| `open_settings` | なし | `()` | 設定ウィンドウを表示 |
| `close_launcher` | なし | `()` | パイメニューを非表示にする |

---

## ウィンドウ設定

### パイメニューウィンドウ

```json
{
  "label": "launcher",
  "transparent": true,
  "decorations": false,
  "alwaysOnTop": true,
  "visible": false,
  "skipTaskbar": true,
  "width": 300,
  "height": 300,
  "resizable": false
}
```

- 起動時は非表示（`visible: false`）
- トリガーキー検知時にカーソル座標へ移動してから表示
- 幅・高さはパイメニューの半径（80px）× 2 + アイテムサイズ + 余白で算出

### 設定ウィンドウ

```json
{
  "label": "settings",
  "decorations": true,
  "visible": false,
  "width": 480,
  "height": 600,
  "resizable": false,
  "center": true
}
```

---

## パイメニューUI設計

### スロット座標計算

中心を原点（0, 0）とし、各スロットの座標を以下の式で算出する。

```
angle = -90° + (slot_id × 60°)   // 12時方向を0°として時計回り
x = radius × cos(angle)
y = radius × sin(angle)
```

| スロット | 角度 | 方向 |
|---|---|---|
| 0 | -90° (270°) | 上（12時） |
| 1 | -30° (330°) | 右上（2時） |
| 2 | 30° | 右下（4時） |
| 3 | 90° | 下（6時） |
| 4 | 150° | 左下（8時） |
| 5 | 210° | 左上（10時） |

### アニメーション

- 表示時：`transform: scale(0.8)` → `scale(1.0)`、`opacity: 0` → `1`、150ms
- 非表示時：即時非表示（`display: none`）

### 画面端対応

トリガーキー検知時に以下を判定してウィンドウ位置を調整する。

```
ウィンドウ左端 = max(0, cursor_x - window_width / 2)
ウィンドウ上端 = max(0, cursor_y - window_height / 2)
ウィンドウ右端 = min(screen_width, left + window_width)  → はみ出す場合は左にずらす
ウィンドウ下端 = min(screen_height, top + window_height) → はみ出す場合は上にずらす
```

---

## アプリ起動・フォーカスの実装方針

### macOS

macOSでは `open` コマンドを使用する。`open` は対象アプリが既に起動中の場合は自動的に既存ウィンドウをフォーカスするため、起動中かどうかの判定が不要でシンプルに実装できる。

```rust
use std::process::Command;

pub fn launch_or_focus(app_path: &str) -> Result<(), String> {
    Command::new("open")
        .arg(app_path)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}
```

`open /path/to/App.app` は起動・フォーカスの両方を一つのコマンドで処理できる。

### Windows

Windowsでは `std::process::Command` で起動し、フォーカスには `windows` クレートの `SetForegroundWindow` を使用する。

```rust
use std::process::Command;

pub fn launch_or_focus(app_path: &str) -> Result<(), String> {
    // 起動中のプロセスを検索
    if let Some(hwnd) = find_window_by_exe(app_path) {
        // 起動中の場合はフォーカス
        unsafe {
            windows::Win32::UI::WindowsAndMessaging::ShowWindow(
                hwnd,
                windows::Win32::UI::WindowsAndMessaging::SW_RESTORE,
            );
            windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow(hwnd);
        }
    } else {
        // 未起動の場合は新規起動
        Command::new(app_path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
```

依存クレート：`windows = { version = "0.58", features = ["Win32_UI_WindowsAndMessaging", "Win32_System_Threading"] }`

`find_window_by_exe` の実装方針：`EnumWindows` で全ウィンドウを列挙し、各ウィンドウのプロセスID（`GetWindowThreadProcessId`）からプロセスハンドルを取得（`OpenProcess`）し、`QueryFullProcessImageName` で実行ファイルのフルパスを取得して引数の `app_path` と比較する。

```rust
unsafe fn find_window_by_exe(app_path: &str) -> Option<HWND> {
    // EnumWindows でコールバックを呼び出しながらウィンドウを列挙
    // 各ウィンドウの実行ファイルパスを QueryFullProcessImageName で取得
    // app_path と一致するウィンドウの HWND を返す
}
```

---

## 設定画面のドラッグ&ドロップ実装方針

TauriのWebViewではブラウザの `File` オブジェクトにファイルシステムパスが含まれないため、HTML5の `drop` イベントではパスを取得できない。
代わりに `@tauri-apps/api/webviewWindow` の `onDragDropEvent` を使用してファイルパスを取得する。

`onDragDropEvent` を使用するには以下の設定が必要。

- `tauri.conf.json` のウィンドウ設定に `"dragDropEnabled": true` を追加する
- `capabilities/default.json` に `drag-drop:allow` パーミッションを追加する

```js
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

const appWindow = getCurrentWebviewWindow();

appWindow.onDragDropEvent(async (event) => {
    if (event.payload.type !== 'drop') return;

    const filePath = event.payload.paths[0];
    if (!filePath) return;

    // ドロップ座標からどのスロット枠に落としたかを判定する
    const slotId = getSlotIdFromPosition(event.payload.position);
    if (slotId === null) return;

    try {
        await invoke('update_slot', {
            slot: { id: slotId, actionType: 'app', appPath: filePath, appName: null, iconPath: null }
        });
    } catch (err) {
        console.error('スロット更新に失敗しました', err);
    }
});
```

ドロップ座標からスロットを特定するため、各スロット要素の `getBoundingClientRect()` と `event.payload.position` を比較して対象スロットを判定する。

---

## 影響範囲の分析

初回実装のため既存コードへの影響はなし。
ただし以下の点を設計時点で将来拡張を考慮して実装する。

| 考慮点 | 対応方針 |
|---|---|
| アクション種別の追加 | `action_type` フィールドで分岐する構造にしておく |
| スロット数の変更 | スロット数をハードコードせず定数 `MAX_SLOTS` で管理する |
| UIテーマ | CSS変数でカラーを管理し、将来のダークモード対応に備える |
