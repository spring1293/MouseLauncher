use std::process::Command;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowThreadProcessId, IsWindowVisible, SetForegroundWindow, ShowWindow,
    SW_RESTORE,
};

/// Windows: アプリを起動またはフォーカスする
pub fn launch_or_focus(app_path: &str) -> Result<(), String> {
    if let Some(hwnd) = find_window_by_exe(app_path) {
        // 起動中の場合は既存ウィンドウをフォーカスする
        unsafe {
            ShowWindow(hwnd, SW_RESTORE);
            SetForegroundWindow(hwnd).map_err(|e| e.to_string())?;
        }
    } else {
        // 未起動の場合は新規起動する
        Command::new(app_path)
            .spawn()
            .map_err(|e| format!("アプリの起動に失敗しました: {}", e))?;
    }
    Ok(())
}

/// 実行ファイルパスで起動中のウィンドウを検索する
fn find_window_by_exe(app_path: &str) -> Option<HWND> {
    let app_path = app_path.to_lowercase();
    let mut result: Option<HWND> = None;

    // EnumWindowsのコールバックに渡すデータ
    struct EnumData {
        app_path: String,
        result: Option<HWND>,
    }

    let mut data = EnumData {
        app_path,
        result: None,
    };

    unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let data = &mut *(lparam.0 as *mut EnumData);

        // 非表示ウィンドウはスキップする
        if !IsWindowVisible(hwnd).as_bool() {
            return BOOL(1);
        }

        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if pid == 0 {
            return BOOL(1);
        }

        // プロセスの実行ファイルパスを取得する
        if let Ok(handle) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            let mut buf = vec![0u16; 1024];
            let mut len = buf.len() as u32;
            if QueryFullProcessImageNameW(
                handle,
                PROCESS_NAME_WIN32,
                windows::core::PWSTR(buf.as_mut_ptr()),
                &mut len,
            )
            .is_ok()
            {
                let exe_path = String::from_utf16_lossy(&buf[..len as usize]).to_lowercase();
                if exe_path == data.app_path {
                    data.result = Some(hwnd);
                    return BOOL(0); // 見つかったら列挙を停止する
                }
            }
        }

        BOOL(1) // 継続する
    }

    unsafe {
        let lparam = LPARAM(&mut data as *mut EnumData as isize);
        let _ = EnumWindows(Some(enum_callback), lparam);
    }

    data.result
}
