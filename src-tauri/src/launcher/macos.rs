use std::process::Command;

/// macOS: `open` コマンドでアプリを起動またはフォーカスする
/// `open` は対象アプリが起動中の場合も自動的にフォーカスする
pub fn launch_or_focus(app_path: &str) -> Result<(), String> {
    Command::new("open")
        .arg(app_path)
        .spawn()
        .map_err(|e| format!("アプリの起動に失敗しました: {}", e))?;
    Ok(())
}
