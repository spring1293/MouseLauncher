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
        let win_w = 300_i32;
        let win_h = 300_i32;

        // スクリーンサイズを取得する（座標系はプラットフォームに依存）
        let screen_size = get_screen_size(app);

        // カーソル位置取得失敗時は画面中央にフォールバックする
        let (cursor_x, cursor_y) = get_cursor_position()
            .unwrap_or((screen_size.0 / 2, screen_size.1 / 2));

        // 画面端対応: ウィンドウが画面外にはみ出さないよう調整する
        let x = (cursor_x - win_w / 2).max(0).min(screen_size.0 - win_w);
        let y = (cursor_y - win_h / 2).max(0).min(screen_size.1 - win_h);

        // macOS: AppleScript は論理座標を返すため LogicalPosition を使用する
        // Windows: GetCursorPos は物理座標を返すため PhysicalPosition を使用する
        #[cfg(target_os = "macos")]
        let _ = window.set_position(tauri::LogicalPosition::new(x, y));
        #[cfg(not(target_os = "macos"))]
        let _ = window.set_position(tauri::PhysicalPosition::new(x, y));

        let _ = window.show();
        let _ = window.set_focus();

        // カーソル座標をフロントエンドに送信する
        let _ = window.emit(
            "launcher-show",
            serde_json::json!({
                "cursorX": cursor_x,
                "cursorY": cursor_y,
                "windowX": x,
                "windowY": y,
            }),
        );
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
    use std::ffi::c_void;

    // CoreGraphics の CGPoint 構造体
    #[repr(C)]
    struct CGPoint {
        x: f64,
        y: f64,
    }

    // CoreGraphics フレームワークを直接呼び出す（アクセシビリティ権限不要）
    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
        fn CGEventCreate(source: *const c_void) -> *mut c_void;
        fn CGEventGetLocation(event: *const c_void) -> CGPoint;
        fn CFRelease(cf: *const c_void);
    }

    unsafe {
        let event = CGEventCreate(std::ptr::null());
        if event.is_null() {
            return Err("マウス座標の取得に失敗しました".to_string());
        }
        let point = CGEventGetLocation(event);
        CFRelease(event);
        // CGEvent は論理座標（ポイント）を返す
        Ok((point.x as i32, point.y as i32))
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
/// macOS: 論理ピクセル（AppleScript の座標系と一致）
/// Windows: 物理ピクセル（GetCursorPos の座標系と一致）
fn get_screen_size(app: &AppHandle) -> (i32, i32) {
    if let Ok(Some(monitor)) = app.primary_monitor() {
        let size = monitor.size();
        #[cfg(target_os = "macos")]
        {
            let scale = monitor.scale_factor();
            return (
                (size.width as f64 / scale) as i32,
                (size.height as f64 / scale) as i32,
            );
        }
        #[cfg(not(target_os = "macos"))]
        return (size.width as i32, size.height as i32);
    }
    (1920, 1080)
}
