pub mod store;

use serde::{Deserialize, Serialize};

pub const MAX_SLOTS: usize = 6;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub trigger_key: String,
    pub slots: Vec<Slot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slot {
    pub id: usize,
    pub action_type: String,
    pub app_path: Option<String>,
    pub app_name: Option<String>,
    pub icon_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        #[cfg(target_os = "macos")]
        let trigger_key = "Super+Shift+Space".to_string();
        #[cfg(not(target_os = "macos"))]
        let trigger_key = "Ctrl+Shift+Space".to_string();

        Config {
            trigger_key,
            slots: (0..MAX_SLOTS)
                .map(|i| Slot {
                    id: i,
                    action_type: "app".to_string(),
                    app_path: None,
                    app_name: None,
                    icon_path: None,
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_has_six_slots() {
        let config = Config::default();
        assert_eq!(config.slots.len(), MAX_SLOTS);
    }

    #[test]
    fn test_default_trigger_key() {
        let config = Config::default();
        #[cfg(target_os = "macos")]
        assert_eq!(config.trigger_key, "Super+Shift+Space");
        #[cfg(not(target_os = "macos"))]
        assert_eq!(config.trigger_key, "Ctrl+Shift+Space");
    }

    #[test]
    fn test_default_slots_are_unassigned() {
        let config = Config::default();
        for slot in &config.slots {
            assert!(slot.app_path.is_none());
            assert!(slot.app_name.is_none());
        }
    }
}
