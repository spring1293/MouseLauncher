# 初回実装 タスクリスト

## 進捗凡例

- `[ ]` 未着手
- `[x]` 完了

---

## Phase 1: プロジェクトセットアップ

- [ ] 1-1. Tauri v2 プロジェクトを初期化する（`npm create tauri-app`）
- [ ] 1-2. `src-tauri/Cargo.toml` に依存クレートを追加する
  - `serde`, `serde_json`
  - `tauri-plugin-global-shortcut`
  - `tauri-plugin-fs`
  - `tauri-plugin-shell`
  - `tauri-plugin-dialog`
  - `windows`（Windows向け、`cfg` で条件付き）
- [ ] 1-3. `package.json` にビルドスクリプトを設定する
- [ ] 1-4. `docs/repository-structure.md` に従いディレクトリ構造を作成する
  - `src-tauri/src/config/`
  - `src-tauri/src/commands/`
  - `src-tauri/src/launcher/`
  - `ui/launcher/`
  - `ui/settings/`
- [ ] 1-5. `src-tauri/tauri.conf.json` の基本設定を記述する（アプリ名・バージョン・ウィンドウ設定）
- [ ] 1-6. `src-tauri/capabilities/default.json` のパーミッションを設定する

---

## Phase 2: 設定管理（Config Store）

- [ ] 2-1. `src-tauri/src/config/mod.rs` に `Config`・`Slot` 構造体を定義する
- [ ] 2-2. `Config::default()` を実装する（トリガーキー: `Ctrl+Shift+Space`、6スロット全て未割り当て）
- [ ] 2-3. `src-tauri/src/config/store.rs` に設定ファイルの読み込み処理を実装する
  - 設定ファイルが存在しない場合はデフォルト設定を生成・保存する
  - 保存先：Windows `%APPDATA%\MouseLauncher\config.json`、macOS `~/Library/Application Support/MouseLauncher/config.json`
- [ ] 2-4. `src-tauri/src/config/store.rs` に設定ファイルの保存処理を実装する
- [ ] 2-5. 設定管理のユニットテストを記述する
  - デフォルト設定のスロット数が6であることを確認
  - 設定の保存・読み込みが正常に動作することを確認

---

## Phase 3: 常駐起動（Tray Icon）

- [ ] 3-1. `src-tauri/src/tray.rs` にシステムトレイ / メニューバーのアイコン設定を実装する
- [ ] 3-2. トレイアイコンのコンテキストメニューに「終了」を追加する
- [ ] 3-3. `src-tauri/src/lib.rs` でアプリ起動時にトレイアイコンを表示する
- [ ] 3-4. macOS でアクセシビリティ権限の確認処理を実装する
- [ ] 3-5. 権限が未付与の場合にシステム環境設定への誘導メッセージを表示する
- [ ] 3-6. ウィンドウを閉じてもアプリが終了しない（常駐し続ける）ことを確認する
- [ ] 3-7. 「終了」からアプリを終了できることを確認する

---

## Phase 4: グローバルショートカット

- [ ] 4-1. `src-tauri/src/lib.rs` で `tauri-plugin-global-shortcut` を登録する
- [ ] 4-2. 起動時に `config.trigger_key` の値でグローバルショートカットを登録する
- [ ] 4-3. ショートカット検知時にマウスカーソルの座標を取得する
- [ ] 4-4. 取得した座標にパイメニューウィンドウを移動して表示するイベントをフロントエンドに送信する
- [ ] 4-5. `src-tauri/src/commands/config.rs` に `update_trigger_key` コマンドを実装する
  - 既存のショートカット登録を解除し、新しいキーで再登録する
  - 設定ファイルに保存する

---

## Phase 5: パイメニューUI

- [ ] 5-1. `ui/launcher/index.html` にパイメニューのHTML構造を実装する
  - 6スロット分の要素
  - 中心の設定ボタン（歯車アイコン）
- [ ] 5-2. `ui/launcher/launcher.css` にスタイルを実装する
  - 各スロットを60°間隔で放射状に配置するCSS（`transform: rotate + translate` を使用）
  - 未割り当てスロットのグレーアウトスタイル
  - 表示アニメーション（scale + opacity、150ms）
  - 半透明の円形背景
- [ ] 5-3. `ui/launcher/launcher.js` に表示制御を実装する
  - バックエンドからの表示イベントを受信してメニューを表示する
  - `get_config` コマンドで設定を取得してスロット情報を描画する
- [ ] 5-4. スロットクリック時に `launch_app` コマンドを呼び出す処理を実装する
- [ ] 5-5. Escキーでパイメニューを閉じる処理を実装する
- [ ] 5-6. 範囲外クリックでパイメニューを閉じる処理を実装する
- [ ] 5-7. 中心の設定ボタンクリックで `open_settings` コマンドを呼び出す処理を実装する
- [ ] 5-8. 画面端対応のウィンドウ位置調整をバックエンドに実装する

---

## Phase 6: アプリ起動・フォーカス

- [ ] 6-1. `src-tauri/src/launcher/macos.rs` を実装する
  - `open` コマンドでアプリを起動・フォーカスする
- [ ] 6-2. `src-tauri/src/launcher/windows.rs` を実装する
  - `find_window_by_exe`（`EnumWindows` + `QueryFullProcessImageName`）を実装する
  - 起動中なら `SetForegroundWindow`、未起動なら `Command::new` で起動する
- [ ] 6-3. `src-tauri/src/launcher/mod.rs` にOS分岐ロジックを実装する（`#[cfg]` で切り替え）
- [ ] 6-4. `src-tauri/src/commands/launcher.rs` に `launch_app` コマンドを実装する
  - アプリ起動・フォーカス後にバックエンドからパイメニューウィンドウを非表示にする（`window.hide()`）
- [ ] 6-5. Windows・macOS それぞれでアプリ起動・フォーカスを手動テストする

---

## Phase 7: 設定画面UI

- [ ] 7-1. `ui/settings/index.html` に設定画面のHTML構造を実装する
  - トリガーキー表示・変更UI
  - 6スロット分の設定行（アプリ名表示・設定/変更/削除ボタン）
- [ ] 7-2. `ui/settings/settings.css` にスタイルを実装する
- [ ] 7-3. `ui/settings/settings.js` に設定取得・表示処理を実装する
  - 画面表示時に `get_config` で設定を読み込んで描画する
- [ ] 7-4. ファイル選択ダイアログでスロットにアプリを登録する処理を実装する
  - `tauri-plugin-dialog` の `open` でファイル選択ダイアログを表示する
  - 選択されたパスで `update_slot` コマンドを呼び出す
- [ ] 7-5. ドラッグ&ドロップでスロットにアプリを登録する処理を実装する
  - `onDragDropEvent` でファイルパスを取得する
  - ドロップ座標からスロットIDを特定して `update_slot` コマンドを呼び出す
- [ ] 7-6. スロット削除処理を実装する（`delete_slot` コマンドを呼び出す）
- [ ] 7-7. トリガーキー変更UIを実装する
  - 「変更」ボタン押下でキー入力待ち状態にする
  - キー入力を検知して `update_trigger_key` コマンドを呼び出す
  - 単独キーなど無効な組み合わせはエラー表示する
- [ ] 7-8. `src-tauri/src/commands/config.rs` に全設定コマンドを実装する
  - `get_config`
  - `update_slot`
  - `delete_slot`
  - `open_settings`

---

## Phase 8: 動作確認・調整

- [ ] 8-1. Windows で全受け入れ条件を手動テストする
- [ ] 8-2. macOS で全受け入れ条件を手動テストする
- [ ] 8-3. パフォーマンス確認（ランチャー表示が100ms以内か）
- [ ] 8-4. `cargo fmt` でコードフォーマットを整える
- [ ] 8-5. `cargo clippy` で警告をすべて解消する
- [ ] 8-6. `npm run tauri build` でWindows・macOS向けのビルドが成功することを確認する
