use std::collections::HashMap;
use std::sync::Mutex;

/// アイコンのインメモリキャッシュ（アプリパス → data URL）
pub struct IconCache(pub Mutex<HashMap<String, String>>);

impl IconCache {
    pub fn new() -> Self {
        IconCache(Mutex::new(HashMap::new()))
    }
}

/// アプリのアイコンを一括で取得して data URL (base64 PNG) のマップで返す
/// キャッシュ済みのアイコンはキャッシュから返し、未取得のみ抽出する
#[tauri::command]
pub async fn get_app_icons(
    app_paths: Vec<String>,
    cache: tauri::State<'_, IconCache>,
) -> Result<HashMap<String, String>, ()> {
    let mut result = HashMap::new();
    let mut uncached: Vec<String> = Vec::new();

    // キャッシュチェック
    {
        let lock = cache.0.lock().unwrap();
        for path in &app_paths {
            if let Some(data_url) = lock.get(path) {
                result.insert(path.clone(), data_url.clone());
            } else {
                uncached.push(path.clone());
            }
        }
    }

    // 未キャッシュのアイコンをまとめて抽出
    if !uncached.is_empty() {
        let extracted = extract_icons_batch(&uncached);
        let mut lock = cache.0.lock().unwrap();
        for (path, data_url) in extracted {
            lock.insert(path.clone(), data_url.clone());
            result.insert(path, data_url);
        }
    }

    Ok(result)
}

// ---- macOS ----------------------------------------------------------------

#[cfg(target_os = "macos")]
fn extract_icons_batch(paths: &[String]) -> Vec<(String, String)> {
    paths
        .iter()
        .filter_map(|path| extract_icon_macos(path).map(|url| (path.clone(), url)))
        .collect()
}

#[cfg(target_os = "macos")]
fn extract_icon_macos(app_path: &str) -> Option<String> {
    use base64::{engine::general_purpose, Engine as _};
    use std::path::Path;

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

    let icon_file = if icon_name.ends_with(".icns") {
        icon_name
    } else {
        format!("{}.icns", icon_name)
    };

    let icns_path = format!("{}/Contents/Resources/{}", app_path, icon_file);
    if !Path::new(&icns_path).exists() {
        return None;
    }

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

// ---- Windows --------------------------------------------------------------

/// Windows: 全パスを1回の PowerShell プロセスで一括抽出する
#[cfg(target_os = "windows")]
fn extract_icons_batch(paths: &[String]) -> Vec<(String, String)> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    if paths.is_empty() {
        return vec![];
    }

    // PowerShell の配列リテラルを構築する（シングルクォートをエスケープ）
    let paths_ps = paths
        .iter()
        .map(|p| format!("'{}'", p.replace('\'', "''")))
        .collect::<Vec<_>>()
        .join(",");

    // 全パスを順番に処理し、成功は base64、失敗は空行を出力する
    let script = format!(
        r#"Add-Type -AssemblyName System.Drawing
$paths = @({paths_ps})
foreach ($p in $paths) {{
    try {{
        $icon = [System.Drawing.Icon]::ExtractAssociatedIcon($p)
        $bmp = $icon.ToBitmap()
        $ms = New-Object System.IO.MemoryStream
        $bmp.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
        [Convert]::ToBase64String($ms.ToArray())
    }} catch {{ Write-Output '' }}
}}"#
    );

    let output = match std::process::Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
    {
        Ok(o) => o,
        Err(_) => return vec![],
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    // 出力行とパスを1対1で対応させる
    paths
        .iter()
        .enumerate()
        .filter_map(|(i, path)| {
            let b64 = lines.get(i).copied().unwrap_or("").trim();
            if b64.is_empty() {
                None
            } else {
                Some((path.clone(), format!("data:image/png;base64,{}", b64)))
            }
        })
        .collect()
}

// ---- Linux (未サポート) ---------------------------------------------------

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn extract_icons_batch(_paths: &[String]) -> Vec<(String, String)> {
    vec![]
}
