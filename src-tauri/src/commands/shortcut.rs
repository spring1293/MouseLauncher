use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

/// グローバルショートカットを登録する
pub fn register_shortcut(app: AppHandle, key: &str) -> Result<(), String> {
    app.global_shortcut()
        .on_shortcut(key, move |app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                show_launcher(app);
            }
        })
        .map_err(|e| e.to_string())
}

/// カーソル位置にランチャーウィンドウを表示する
pub fn show_launcher(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("launcher") {
        if let Ok(pos) = get_cursor_position() {
            let screen_size = get_screen_size().unwrap_or((1920, 1080));
            let win_w = 300_i32;
            let win_h = 300_i32;

            // 画面端対応: ウィンドウが画面外にはみ出さないよう調整する
            let x = (pos.0 - win_w / 2).max(0).min(screen_size.0 as i32 - win_w);
            let y = (pos.1 - win_h / 2).max(0).min(screen_size.1 as i32 - win_h);

            let _ = window.set_position(tauri::PhysicalPosition::new(x, y));
            let _ = window.show();
            let _ = window.set_focus();

            // カーソル座標をフロントエンドに送信する
            let _ = window.emit(
                "launcher-show",
                serde_json::json!({
                    "cursorX": pos.0,
                    "cursorY": pos.1,
                    "windowX": x,
                    "windowY": y,
                }),
            );
        }
    }
}

/// グローバルショートカットの登録を解除する
pub fn unregister_shortcut(app: AppHandle, key: &str) -> Result<(), String> {
    app.global_shortcut()
        .unregister(key)
        .map_err(|e| e.to_string())
}

/// マウスカーソルの現在座標を取得する
fn get_cursor_position() -> Result<(i32, i32), String> {
    #[cfg(target_os = "macos")]
    {
        macos_cursor_position()
    }
    #[cfg(target_os = "windows")]
    {
        windows_cursor_position()
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Err("未対応のOSです".to_string())
    }
}

#[cfg(target_os = "macos")]
fn macos_cursor_position() -> Result<(i32, i32), String> {
    use std::process::Command;
    // AppleScriptでカーソル位置を取得する
    let output = Command::new("osascript")
        .arg("-e")
        .arg("tell application \"Finder\" to get the position of the mouse")
        .output()
        .map_err(|e| e.to_string())?;
    let s = String::from_utf8_lossy(&output.stdout);
    // "x, y" 形式でパースする
    let parts: Vec<&str> = s.trim().split(", ").collect();
    if parts.len() == 2 {
        let x = parts[0].parse::<i32>().map_err(|e| e.to_string())?;
        let y = parts[1].parse::<i32>().map_err(|e| e.to_string())?;
        Ok((x, y))
    } else {
        Err(format!("カーソル座標のパースに失敗しました: {}", s))
    }
}

#[cfg(target_os = "windows")]
fn windows_cursor_position() -> Result<(i32, i32), String> {
    use windows::Win32::Foundation::POINT;
    use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
    let mut point = POINT::default();
    unsafe {
        GetCursorPos(&mut point).map_err(|e| e.to_string())?;
    }
    Ok((point.x, point.y))
}

/// スクリーンサイズを取得する
fn get_screen_size() -> Result<(u32, u32), String> {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
        let w = unsafe { GetSystemMetrics(SM_CXSCREEN) };
        let h = unsafe { GetSystemMetrics(SM_CYSCREEN) };
        Ok((w as u32, h as u32))
    }
    #[cfg(target_os = "macos")]
    {
        // macOSではデフォルト値を返す（Tauriのmonitor APIで取得可能だが要AppHandle）
        Ok((1920, 1080))
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Ok((1920, 1080))
    }
}
