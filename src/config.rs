use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static TMP_SEQ: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DockPositionMode {
    /// Placed on taskbar just to the left of the system tray / clock
    TaskbarLeftOfTray,
    /// Placed on the rightmost edge of the taskbar
    TaskbarRight,
    /// Placed on the left edge of the taskbar
    TaskbarLeft,
    /// Freely placed / dragged by user at specific coordinates
    Custom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub low_battery_threshold: u8,
    pub critical_battery_threshold: u8,
    pub audio_alert_enabled: bool,
    pub visual_alert_enabled: bool,
    pub position_mode: DockPositionMode,
    pub custom_pos_x: f32,
    pub custom_pos_y: f32,
    pub snap_to_taskbar: bool,
    pub start_with_windows: bool,
    pub poll_interval_secs: u64,
    pub show_percentage_text: bool,
    pub compact_mode: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            low_battery_threshold: 20,
            critical_battery_threshold: 10,
            audio_alert_enabled: true,
            visual_alert_enabled: true,
            position_mode: DockPositionMode::TaskbarLeftOfTray,
            custom_pos_x: 0.0,
            custom_pos_y: 0.0,
            snap_to_taskbar: true,
            start_with_windows: false,
            poll_interval_secs: 15,
            show_percentage_text: true,
            compact_mode: false,
        }
    }
}

impl AppConfig {
    pub fn config_path() -> PathBuf {
        if let Some(proj_dirs) = directories::ProjectDirs::from("com", "jorjanion", "BatteryDock") {
            let config_dir = proj_dirs.config_dir();
            let _ = fs::create_dir_all(config_dir);
            config_dir.join("config.toml")
        } else {
            PathBuf::from("config.toml")
        }
    }

    pub fn try_load_from_path(path: &Path) -> Option<Self> {
        let content = fs::read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    }

    pub fn try_load() -> Option<Self> {
        let path = Self::config_path();
        Self::try_load_from_path(&path)
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            if let Some(config) = Self::try_load_from_path(&path) {
                return config;
            }
            // Do not overwrite an existing (possibly transiently unreadable/locked) config file
            return Self::default();
        }
        let config = Self::default();
        config.save_to_path(&path);
        config
    }

    pub fn save_to_path(&self, path: &Path) {
        if let Ok(content) = toml::to_string_pretty(self) {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let seq = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
            let tmp_path = path.with_extension(format!("toml.{}.{}.tmp", std::process::id(), seq));
            if fs::write(&tmp_path, &content).is_ok() {
                if fs::rename(&tmp_path, path).is_ok() {
                    return;
                }
                let _ = fs::remove_file(&tmp_path);
            }
            let _ = fs::write(path, content);
        }
    }

    pub fn save(&self) {
        let path = Self::config_path();
        self.save_to_path(&path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default_and_serialization() {
        let config = AppConfig::default();
        assert_eq!(config.low_battery_threshold, 20);
        assert_eq!(config.critical_battery_threshold, 10);
        assert!(config.audio_alert_enabled);
        assert_eq!(config.position_mode, DockPositionMode::TaskbarLeftOfTray);

        let toml_str = toml::to_string_pretty(&config).expect("Serialization failed");
        let decoded: AppConfig = toml::from_str(&toml_str).expect("Deserialization failed");
        assert_eq!(decoded, config);
    }

    #[test]
    fn test_config_partial_toml_uses_defaults() {
        let partial = "low_battery_threshold = 25\nshow_percentage_text = false\n";
        let decoded: AppConfig = toml::from_str(partial).expect("Partial deserialization failed");
        assert_eq!(decoded.low_battery_threshold, 25);
        assert!(!decoded.show_percentage_text);
        assert_eq!(decoded.position_mode, DockPositionMode::TaskbarLeftOfTray);
        assert_eq!(decoded.critical_battery_threshold, 10);
    }

    #[test]
    fn test_try_load_from_path_handles_corrupt_without_overwriting() {
        let dir = std::env::temp_dir().join(format!("batterydock_cfg_test_{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("config.toml");

        let cfg = AppConfig {
            low_battery_threshold: 33,
            custom_pos_x: -500.0,
            custom_pos_y: 120.0,
            position_mode: DockPositionMode::Custom,
            ..AppConfig::default()
        };
        cfg.save_to_path(&path);

        let loaded = AppConfig::try_load_from_path(&path).expect("Expected saved config");
        assert_eq!(loaded, cfg);

        fs::write(&path, "invalid = [unclosed").unwrap();
        assert!(AppConfig::try_load_from_path(&path).is_none());
        assert_eq!(fs::read_to_string(&path).unwrap(), "invalid = [unclosed");

        let _ = fs::remove_dir_all(&dir);
    }
}
