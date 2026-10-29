use std::os::windows::process::CommandExt;
use std::process::Command;

const CREATE_NO_WINDOW: u32 = 0x08000000;
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
    let ext = std::path::Path::new(app_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "exe" => {
            // .exe: 起動中なら既存ウィンドウをフォーカス、未起動なら新規起動する
            if let Some(hwnd) = find_window_by_exe(app_path) {
                unsafe {
                    ShowWindow(hwnd, SW_RESTORE);
                    SetForegroundWindow(hwnd);
                }
            } else {
                Command::new(app_path)
                    .creation_flags(CREATE_NO_WINDOW)
                    .spawn()
                    .map_err(|e| format!("アプリの起動に失敗しました: {}", e))?;
            }
        }
        // .lnk / .bat などシェル経由での実行が必要なファイル
        _ => shell_open(app_path)?,
    }
    Ok(())
}

/// シェル経由でファイルを開く（.lnk ショートカット・.bat バッチファイルなど）
/// `cmd /c start "" path` を使うことで OS にファイルの種別に応じた処理を委譲する
fn shell_open(path: &str) -> Result<(), String> {
    Command::new("cmd.exe")
        .args(["/c", "start", "", path])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map_err(|e| format!("ファイルの実行に失敗しました: {}", e))?;
    Ok(())
}

/// 実行ファイルパスで起動中のウィンドウを検索する
fn find_window_by_exe(app_path: &str) -> Option<HWND> {
    let app_path = app_path.to_lowercase();

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
