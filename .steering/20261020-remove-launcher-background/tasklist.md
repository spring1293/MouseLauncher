# ランチャー背景の透過対応 タスクリスト

> 変更規模が小さいため、ユーザー承認のもと design.md を省略し一括で実装した。

## タスク

- [x] `src-tauri/tauri.conf.json` の `app` に `"macOSPrivateApi": true` を追加
- [x] `src-tauri/tauri.conf.json` の launcher ウィンドウに `"shadow": false` を追加
- [x] `src-tauri/Cargo.toml` の tauri に `macos-private-api` feature を追加
- [ ] macOS 実機でビルドし、円形メニュー外側に白背景・枠・影が出ないことを確認
- [ ] Windows 実機でビルドし、同様に確認
- [ ] 設定画面の見た目が変わっていないことを確認

## 完了条件

requirements.md の受け入れ条件をすべて満たすこと。
