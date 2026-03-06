mod commands;
mod config;
mod launcher;
mod tray;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // 設定の初期ロード
            let config = config::store::load_or_default(app.handle())?;

            // トレイアイコンの初期化
            tray::setup(app.handle())?;

            // グローバルショートカットの登録
            let trigger_key = config.trigger_key.clone();
            let app_handle = app.handle().clone();
            commands::shortcut::register_shortcut(app_handle, &trigger_key)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::config::get_config,
            commands::config::update_slot,
            commands::config::delete_slot,
            commands::config::update_trigger_key,
            commands::config::open_settings,
            commands::launcher::launch_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
