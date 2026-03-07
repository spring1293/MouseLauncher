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

            // macOS: アクセシビリティ権限を確認する
            #[cfg(target_os = "macos")]
            check_accessibility(app.handle());

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
            commands::config::close_launcher,
            commands::launcher::launch_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// macOS: アクセシビリティ権限を確認し、未付与の場合はシステム設定へ誘導する
#[cfg(target_os = "macos")]
fn check_accessibility(app: &tauri::AppHandle) {
    #[link(name = "ApplicationServices", kind = "framework")]
    extern "C" {
        fn AXIsProcessTrusted() -> bool;
    }

    let trusted = unsafe { AXIsProcessTrusted() };
    if !trusted {
        // システム設定のアクセシビリティページを開く
        let _ = std::process::Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
            .spawn();

        // ダイアログでユーザーに案内する
        tauri_plugin_dialog::DialogExt::dialog(app)
            .message(
                "MouseLauncher がグローバルショートカットを使用するには、\
                アクセシビリティの権限が必要です。\n\n\
                開いたシステム設定の「アクセシビリティ」リストに \
                MouseLauncher を追加してオンにしてください。\n\n\
                権限を付与後、アプリを再起動してください。"
            )
            .title("アクセシビリティ権限が必要です")
            .blocking_show();
    }
}
