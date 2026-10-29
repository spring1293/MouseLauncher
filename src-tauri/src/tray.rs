use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};
#[cfg(target_os = "windows")]
use tauri::tray::{MouseButton, MouseButtonState};

/// システムトレイ / メニューバーのアイコンをセットアップする
pub fn setup(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show_launcher = MenuItem::with_id(app, "show_launcher", "ランチャーを表示", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "設定", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "終了", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_launcher, &settings, &quit])?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show_launcher" => {
                crate::commands::shortcut::show_launcher(app);
            }
            "settings" => {
                if let Some(window) = app.get_webview_window("settings") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            // Windows: 左クリックでランチャーを表示する
            // macOS: 左クリックはメニュー表示に使われるため、メニュー項目から起動する
            #[cfg(target_os = "windows")]
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                crate::commands::shortcut::show_launcher(tray.app_handle());
            }
            #[cfg(target_os = "macos")]
            let _ = (tray, event);
        })
        .build(app)?;

    Ok(())
}
