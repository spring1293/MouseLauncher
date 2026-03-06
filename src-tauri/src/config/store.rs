use std::path::PathBuf;
use tauri::{AppHandle, Manager};

use super::Config;

/// 設定ファイルのパスを返す
fn config_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    Ok(dir.join("config.json"))
}

/// 設定を読み込む。ファイルが存在しない場合はデフォルト設定を生成・保存する
pub fn load_or_default(app: &AppHandle) -> Result<Config, Box<dyn std::error::Error>> {
    let path = config_path(app)?;

    if path.exists() {
        let content = std::fs::read_to_string(&path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    } else {
        let config = Config::default();
        save(app, &config)?;
        Ok(config)
    }
}

/// 設定をファイルに保存する
pub fn save(app: &AppHandle, config: &Config) -> Result<(), String> {
    let path = config_path(app)?;

    // 親ディレクトリが存在しない場合は作成する
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let content = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    std::fs::write(&path, content).map_err(|e| e.to_string())?;
    Ok(())
}
