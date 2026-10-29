use tauri::{AppHandle, Manager};

use crate::config::store;

/// アプリを起動またはフォーカスする
#[tauri::command]
pub fn launch_app(app: AppHandle, slot_id: usize) -> Result<(), String> {
    let config = store::load_or_default(&app).map_err(|e| e.to_string())?;

    let slot = config
        .slots
        .iter()
        .find(|s| s.id == slot_id)
        .ok_or(format!("スロット {} が見つかりません", slot_id))?;

    let app_path = slot
        .app_path
        .as_ref()
        .ok_or("このスロットにはアプリが登録されていません")?;

    // アプリを起動またはフォーカスする
    crate::launcher::launch_or_focus(app_path)?;

    // 実行後にパイメニューを非表示にする
    if let Some(window) = app.get_webview_window("launcher") {
        window.hide().map_err(|e| e.to_string())?;
    }

    Ok(())
}
