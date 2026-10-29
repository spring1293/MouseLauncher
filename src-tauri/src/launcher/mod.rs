/// アプリを起動またはフォーカスする（OS別実装に委譲）
pub fn launch_or_focus(app_path: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        macos::launch_or_focus(app_path)
    }
    #[cfg(target_os = "windows")]
    {
        windows::launch_or_focus(app_path)
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        let _ = app_path;
        Err("未対応のOSです".to_string())
    }
}

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;
