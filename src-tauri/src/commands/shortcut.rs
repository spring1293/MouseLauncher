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

        let (cursor_x, cursor_y) = get_cursor_position().unwrap_or((960, 540));

        // カーソルが存在するモニター内に収まる位置を計算する
        let (x, y) = monitor_clamped_position(app, cursor_x, cursor_y, win_w, win_h);

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

/// カーソルが存在するモニター内に収まるウィンドウ左上座標を返す
fn monitor_clamped_position(
    app: &AppHandle,
    cursor_x: i32,
    cursor_y: i32,
    win_w: i32,
    win_h: i32,
) -> (i32, i32) {
    if let Some((mon_x, mon_y, mon_w, mon_h)) = find_monitor_for_cursor(app, cursor_x, cursor_y) {
        let x = (cursor_x - win_w / 2).max(mon_x).min(mon_x + mon_w - win_w);
        let y = (cursor_y - win_h / 2).max(mon_y).min(mon_y + mon_h - win_h);
        (x, y)
    } else {
        // モニター特定失敗時: カーソル中心に配置（境界チェックなし）
        (cursor_x - win_w / 2, cursor_y - win_h / 2)
    }
}

/// カーソル座標を含むモニターの境界 (x, y, w, h) を返す
/// 座標系: macOS は論理ピクセル（ポイント）、Windows は物理ピクセル
fn find_monitor_for_cursor(
    app: &AppHandle,
    cursor_x: i32,
    cursor_y: i32,
) -> Option<(i32, i32, i32, i32)> {
    let monitors = app.available_monitors().ok()?;

    for monitor in &monitors {
        let pos = monitor.position();
        let size = monitor.size();

        #[cfg(target_os = "macos")]
        let (mon_x, mon_y, mon_w, mon_h) = {
            let scale = monitor.scale_factor();
            (
                (pos.x as f64 / scale) as i32,
                (pos.y as f64 / scale) as i32,
                (size.width as f64 / scale) as i32,
                (size.height as f64 / scale) as i32,
            )
        };
        #[cfg(not(target_os = "macos"))]
        let (mon_x, mon_y, mon_w, mon_h) =
            (pos.x, pos.y, size.width as i32, size.height as i32);

        if cursor_x >= mon_x
            && cursor_x < mon_x + mon_w
            && cursor_y >= mon_y
            && cursor_y < mon_y + mon_h
        {
            return Some((mon_x, mon_y, mon_w, mon_h));
        }
    }
    None
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
