use base64::{engine::general_purpose, Engine as _};
use std::path::Path;

/// アプリのアイコンを data URL (base64 PNG) で返す
#[tauri::command]
pub async fn get_app_icon(app_path: String) -> Option<String> {
    extract_icon(&app_path)
}

#[cfg(target_os = "macos")]
fn extract_icon(app_path: &str) -> Option<String> {
    // Info.plist からアイコンファイル名を取得する
    let plist_path = format!("{}/Contents/Info.plist", app_path);
    if !Path::new(&plist_path).exists() {
        return None;
    }

    let output = std::process::Command::new("defaults")
        .args(["read", &plist_path, "CFBundleIconFile"])
        .output()
        .ok()?;

    let icon_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if icon_name.is_empty() {
        return None;
    }

    // 拡張子がない場合は .icns を追加する
    let icon_file = if icon_name.ends_with(".icns") {
        icon_name
    } else {
        format!("{}.icns", icon_name)
    };

    let icns_path = format!("{}/Contents/Resources/{}", app_path, icon_file);
    if !Path::new(&icns_path).exists() {
        return None;
    }

    // sips で 64x64 PNG に変換する
    let tmp_path = std::env::temp_dir().join(format!(
        "ml_icon_{}.png",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));

    let status = std::process::Command::new("sips")
        .args([
            "-s", "format", "png",
            &icns_path,
            "--out", tmp_path.to_str()?,
            "-z", "64", "64",
        ])
        .status()
        .ok()?;

    if !status.success() {
        return None;
    }

    let data = std::fs::read(&tmp_path).ok()?;
    let _ = std::fs::remove_file(&tmp_path);

    Some(format!(
        "data:image/png;base64,{}",
        general_purpose::STANDARD.encode(&data)
    ))
}

#[cfg(target_os = "windows")]
fn extract_icon(app_path: &str) -> Option<String> {
    // PowerShell で .exe からアイコンを抽出して base64 PNG に変換する
    let escaped = app_path.replace('\'', "''");
    let script = format!(
        r#"Add-Type -AssemblyName System.Drawing
try {{
    $icon = [System.Drawing.Icon]::ExtractAssociatedIcon('{escaped}')
    $bmp = $icon.ToBitmap()
    $ms = New-Object System.IO.MemoryStream
    $bmp.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
    [Convert]::ToBase64String($ms.ToArray())
}} catch {{ '' }}"#
    );

    let output = std::process::Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .output()
        .ok()?;

    let base64 = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if base64.is_empty() {
        return None;
    }

    Some(format!("data:image/png;base64,{}", base64))
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn extract_icon(_app_path: &str) -> Option<String> {
    None
}
