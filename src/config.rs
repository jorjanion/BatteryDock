use crate::i18n::Language;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThemeMode {
    #[default]
    System,
    LiquidGlass,
    Dark,
    Light,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OverlayCorner {
    TopLeft,
    #[default]
    TopRight,
    BottomLeft,
    BottomRight,
    /// Custom normalized screen coordinates (0.0..=100.0 % on X and Y)
    Custom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceAlertConfig {
    #[serde(default)]
    pub low_threshold: Option<u8>,
    #[serde(default)]
    pub critical_threshold: Option<u8>,
    #[serde(default)]
    pub mute_alerts: bool,
    #[serde(default)]
    pub custom_name: Option<String>,
    #[serde(default)]
    pub hidden: bool,
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
    pub auto_update: bool,
    pub language: Language,
    #[serde(default)]
    pub game_overlay_enabled: bool,
    #[serde(default)]
    pub game_overlay_corner: OverlayCorner,
    #[serde(default)]
    pub game_overlay_margin: f32,
    #[serde(default)]
    pub game_overlay_custom_x_pct: f32,
    #[serde(default)]
    pub game_overlay_custom_y_pct: f32,
    #[serde(default)]
    pub game_overlay_click_through: bool,
    #[serde(default)]
    pub game_overlay_always_on: bool,
    #[serde(default)]
    pub theme_mode: ThemeMode,
    #[serde(default)]
    pub device_configs: std::collections::HashMap<String, DeviceAlertConfig>,
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
            poll_interval_secs: 4,
            show_percentage_text: true,
            compact_mode: false,
            auto_update: true,
            language: Language::default(),
            game_overlay_enabled: false,
            game_overlay_corner: OverlayCorner::TopRight,
            game_overlay_margin: 16.0,
            game_overlay_custom_x_pct: 95.0,
            game_overlay_custom_y_pct: 4.0,
            game_overlay_click_through: true,
            game_overlay_always_on: false,
            theme_mode: ThemeMode::default(),
            device_configs: std::collections::HashMap::new(),
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

    pub fn get_device_thresholds(&self, device_id: &str) -> (u8, u8) {
        if let Some(cfg) = self.device_configs.get(device_id) {
            let low = cfg.low_threshold.unwrap_or(self.low_battery_threshold);
            let crit = cfg
                .critical_threshold
                .unwrap_or(self.critical_battery_threshold)
                .min(low);
            (low, crit)
        } else {
            (self.low_battery_threshold, self.critical_battery_threshold)
        }
    }

    pub fn is_device_muted(&self, device_id: &str) -> bool {
        self.device_configs
            .get(device_id)
            .map(|c| c.mute_alerts)
            .unwrap_or(false)
    }

    pub fn is_device_hidden(&self, device_id: &str) -> bool {
        self.device_configs
            .get(device_id)
            .map(|c| c.hidden)
            .unwrap_or(false)
    }

    pub fn get_device_display_name<'a>(&'a self, device_id: &str, fallback_name: &'a str) -> &'a str {
        if let Some(cfg) = self.device_configs.get(device_id) {
            if let Some(ref custom) = cfg.custom_name {
                if !custom.trim().is_empty() {
                    return custom.as_str();
                }
            }
        }
        fallback_name
    }

    pub fn is_effective_light_theme(&self) -> bool {
        match self.theme_mode {
            ThemeMode::Light => true,
            ThemeMode::Dark | ThemeMode::LiquidGlass => false,
            ThemeMode::System => detect_windows_system_is_light(),
        }
    }
}

#[cfg(windows)]
pub fn detect_windows_system_is_light() -> bool {
    use std::os::windows::ffi::OsStrExt;
    use windows::core::PCWSTR;
    use windows::Win32::System::Registry::{
        RegCloseKey, RegOpenKeyExW, RegQueryValueExW, HKEY_CURRENT_USER, KEY_READ, REG_DWORD,
    };

    let subkey: Vec<u16> =
        std::ffi::OsStr::new("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize")
            .encode_wide()
            .chain(Some(0))
            .collect();
    let val_name: Vec<u16> = std::ffi::OsStr::new("SystemUsesLightTheme")
        .encode_wide()
        .chain(Some(0))
        .collect();

    unsafe {
        let mut hkey = windows::Win32::System::Registry::HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(subkey.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        )
        .is_ok()
        {
            let mut data: u32 = 0;
            let mut data_len = std::mem::size_of::<u32>() as u32;
            let mut val_type = REG_DWORD;
            let res = RegQueryValueExW(
                hkey,
                PCWSTR(val_name.as_ptr()),
                None,
                Some(&mut val_type),
                Some(&mut data as *mut u32 as *mut u8),
                Some(&mut data_len),
            );
            let _ = RegCloseKey(hkey);
            if res.is_ok() && val_type == REG_DWORD {
                return data != 0;
            }
        }
    }
    false
}

#[cfg(not(windows))]
pub fn detect_windows_system_is_light() -> bool {
    false
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

    #[test]
    fn test_theme_mode_serialization_and_effective_theme() {
        let mut cfg = AppConfig::default();
        assert_eq!(cfg.theme_mode, ThemeMode::System);

        cfg.theme_mode = ThemeMode::LiquidGlass;
        assert!(!cfg.is_effective_light_theme());

        cfg.theme_mode = ThemeMode::Light;
        assert!(cfg.is_effective_light_theme());

        cfg.theme_mode = ThemeMode::Dark;
        assert!(!cfg.is_effective_light_theme());

        let toml_str = toml::to_string(&cfg).expect("serialize");
        assert!(toml_str.contains("theme_mode = \"Dark\""));

        let deserialized: AppConfig = toml::from_str(&toml_str).expect("deserialize");
        assert_eq!(deserialized.theme_mode, ThemeMode::Dark);
    }

    #[test]
    fn test_game_overlay_config_defaults_and_roundtrip() {
        let mut cfg = AppConfig::default();
        assert!(!cfg.game_overlay_enabled);
        assert_eq!(cfg.game_overlay_corner, OverlayCorner::TopRight);
        assert!(cfg.game_overlay_click_through);

        cfg.game_overlay_enabled = true;
        cfg.game_overlay_corner = OverlayCorner::Custom;
        cfg.game_overlay_custom_x_pct = 80.0;
        cfg.game_overlay_custom_y_pct = 15.0;

        let toml_str = toml::to_string(&cfg).expect("serialize");
        assert!(toml_str.contains("game_overlay_enabled = true"));
        assert!(toml_str.contains("game_overlay_corner = \"Custom\""));

        let decoded: AppConfig = toml::from_str(&toml_str).expect("deserialize");
        assert!(decoded.game_overlay_enabled);
        assert_eq!(decoded.game_overlay_corner, OverlayCorner::Custom);
        assert_eq!(decoded.game_overlay_custom_x_pct, 80.0);
    }
}
