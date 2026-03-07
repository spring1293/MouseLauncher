use tauri::{AppHandle, Manager};

use crate::config::{self, store, Slot};

/// 現在の設定を返す
#[tauri::command]
pub fn get_config(app: AppHandle) -> Result<config::Config, String> {
    store::load_or_default(&app).map_err(|e| e.to_string())
}

/// 指定スロットを更新して保存する
#[tauri::command]
pub fn update_slot(app: AppHandle, slot: Slot) -> Result<(), String> {
    let mut config = store::load_or_default(&app).map_err(|e| e.to_string())?;

    if let Some(s) = config.slots.iter_mut().find(|s| s.id == slot.id) {
        *s = slot;
    }

    store::save(&app, &config)
}

/// 指定スロットをクリアして保存する
#[tauri::command]
pub fn delete_slot(app: AppHandle, slot_id: usize) -> Result<(), String> {
    let mut config = store::load_or_default(&app).map_err(|e| e.to_string())?;

    if let Some(s) = config.slots.iter_mut().find(|s| s.id == slot_id) {
        s.app_path = None;
        s.app_name = None;
        s.icon_path = None;
    }

    store::save(&app, &config)
}

/// トリガーキーを更新・再登録して保存する
#[tauri::command]
pub fn update_trigger_key(app: AppHandle, key: String) -> Result<(), String> {
    let mut config = store::load_or_default(&app).map_err(|e| e.to_string())?;

    // 既存のショートカットを解除して新しいキーで再登録する
    super::shortcut::unregister_shortcut(app.clone(), &config.trigger_key)?;
    super::shortcut::register_shortcut(app.clone(), &key)?;

    config.trigger_key = key;
    store::save(&app, &config)
}

/// ランチャーウィンドウを非表示にする
#[tauri::command]
pub fn close_launcher(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("launcher") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 設定ウィンドウを表示する
#[tauri::command]
pub fn open_settings(app: AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("settings")
        .ok_or("settings ウィンドウが見つかりません")?;
    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;
    Ok(())
}
