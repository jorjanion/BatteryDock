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

fn default_true() -> bool {
    true
}

fn default_hotkey_vk() -> u16 {
    0x42 // 'B'
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverlayHotkey {
    #[serde(default)]
    pub ctrl: bool,
    #[serde(default = "default_true")]
    pub alt: bool,
    #[serde(default)]
    pub shift: bool,
    /// Win32 Virtual-Key code (default 0x42 = 'B')
    #[serde(default = "default_hotkey_vk")]
    pub vk_code: u16,
}

impl Default for OverlayHotkey {
    fn default() -> Self {
        Self {
            ctrl: false,
            alt: true,
            shift: false,
            vk_code: 0x42, // Alt + B
        }
    }
}

impl OverlayHotkey {
    pub fn vk_name(vk: u16) -> Option<String> {
        match vk {
            0x41..=0x5A => Some(((vk as u8) as char).to_string()),
            0x30..=0x39 => Some(((vk as u8) as char).to_string()),
            0x70..=0x7B => Some(format!("F{}", vk - 0x6F)),
            0x2D => Some("Insert".to_string()),
            0x24 => Some("Home".to_string()),
            0x23 => Some("End".to_string()),
            0x21 => Some("PageUp".to_string()),
            0x22 => Some("PageDown".to_string()),
            0xC0 => Some("`".to_string()),
            _ => None,
        }
    }

    pub fn is_standalone_allowed_vk(vk: u16) -> bool {
        matches!(vk, 0x70..=0x7B | 0x2D | 0x24 | 0x23 | 0x21 | 0x22)
    }

    pub fn display_string(&self) -> String {
        let mut parts: Vec<String> = Vec::with_capacity(4);
        if self.ctrl {
            parts.push("Ctrl".to_string());
        }
        if self.alt {
            parts.push("Alt".to_string());
        }
        if self.shift {
            parts.push("Shift".to_string());
        }
        if let Some(k) = Self::vk_name(self.vk_code) {
            parts.push(k);
        } else {
            parts.push(format!("0x{:02X}", self.vk_code));
        }
        parts.join(" + ")
    }
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
    #[serde(default)]
    pub full_charge_alert_enabled: bool,
    pub position_mode: DockPositionMode,
    pub custom_pos_x: f32,
    pub custom_pos_y: f32,
    pub snap_to_taskbar: bool,
    pub start_with_windows: bool,
    pub poll_interval_secs: u64,
    pub show_percentage_text: bool,
    pub compact_mode: bool,
    #[serde(default)]
    pub show_on_all_monitors: bool,
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
    pub game_overlay_hotkey_enabled: bool,
    #[serde(default)]
    pub game_overlay_hotkey: OverlayHotkey,
    #[serde(default)]
    pub theme_mode: ThemeMode,
    #[serde(default)]
    pub device_order: Vec<String>,
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
            full_charge_alert_enabled: false,
            position_mode: DockPositionMode::TaskbarLeftOfTray,
            custom_pos_x: 0.0,
            custom_pos_y: 0.0,
            snap_to_taskbar: true,
            start_with_windows: false,
            poll_interval_secs: 4,
            show_percentage_text: true,
            compact_mode: false,
            show_on_all_monitors: false,
            auto_update: true,
            language: Language::default(),
            game_overlay_enabled: false,
            game_overlay_corner: OverlayCorner::TopRight,
            game_overlay_margin: 16.0,
            game_overlay_custom_x_pct: 95.0,
            game_overlay_custom_y_pct: 4.0,
            game_overlay_click_through: true,
            game_overlay_always_on: false,
            game_overlay_hotkey_enabled: false,
            game_overlay_hotkey: OverlayHotkey::default(),
            theme_mode: ThemeMode::default(),
            device_order: Vec::new(),
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

    pub fn device_sort_key(&self, device_id: &str) -> usize {
        self.device_order
            .iter()
            .position(|id| id == device_id)
            .unwrap_or(usize::MAX)
    }

    /// Moves `device_id` up (`delta = -1`) or down (`delta = 1`) relative to the currently
    /// visible ordered list `current_ids`, while preserving offline devices in `device_order`.
    pub fn move_device_order(&mut self, current_ids: &[String], device_id: &str, delta: isize) -> bool {
        let Some(pos) = current_ids.iter().position(|id| id == device_id) else {
            return false;
        };
        let target_isize = pos as isize + delta;
        if target_isize < 0 || (target_isize as usize) >= current_ids.len() {
            return false;
        }
        let target_pos = target_isize as usize;

        // Ensure all current IDs exist in device_order in their current relative order
        for id in current_ids {
            if !self.device_order.iter().any(|existing| existing == id) {
                self.device_order.push(id.clone());
            }
        }

        let id_a = &current_ids[pos];
        let id_b = &current_ids[target_pos];
        if let (Some(idx_a), Some(idx_b)) = (
            self.device_order.iter().position(|x| x == id_a),
            self.device_order.iter().position(|x| x == id_b),
        ) {
            self.device_order.swap(idx_a, idx_b);
            return true;
        }
        false
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

    #[test]
    fn test_device_order_sorting_and_move() {
        let mut cfg = AppConfig::default();
        let current = vec!["a".to_string(), "b".to_string(), "c".to_string()];

        // Move "c" up -> ["a", "c", "b"]
        assert!(cfg.move_device_order(&current, "c", -1));
        assert_eq!(cfg.device_order, vec!["a", "c", "b"]);
        assert!(cfg.device_sort_key("c") < cfg.device_sort_key("b"));

        // Moving first item up returns false
        let current2 = vec!["a".to_string(), "c".to_string(), "b".to_string()];
        assert!(!cfg.move_device_order(&current2, "a", -1));
        assert!(!cfg.move_device_order(&current2, "b", 1));

        // Move "a" down -> ["c", "a", "b"]
        assert!(cfg.move_device_order(&current2, "a", 1));
        assert_eq!(cfg.device_order, vec!["c", "a", "b"]);
    }

    #[test]
    fn test_overlay_hotkey_formatting_and_roundtrip() {
        let def = OverlayHotkey::default();
        assert_eq!(def.display_string(), "Alt + B");

        let custom = OverlayHotkey {
            ctrl: true,
            alt: false,
            shift: true,
            vk_code: 0x78, // F9
        };
        assert_eq!(custom.display_string(), "Ctrl + Shift + F9");
        assert!(OverlayHotkey::is_standalone_allowed_vk(0x78));
        assert!(!OverlayHotkey::is_standalone_allowed_vk(0x42));

        let cfg = AppConfig {
            game_overlay_hotkey_enabled: true,
            game_overlay_hotkey: custom,
            ..Default::default()
        };
        let toml_str = toml::to_string(&cfg).expect("serialize");
        let decoded: AppConfig = toml::from_str(&toml_str).expect("deserialize");
        assert!(decoded.game_overlay_hotkey_enabled);
        assert_eq!(decoded.game_overlay_hotkey, custom);
    }

    #[test]
    fn test_show_on_all_monitors_defaults_and_roundtrip() {
        let mut cfg = AppConfig::default();
        assert!(!cfg.show_on_all_monitors);

        cfg.show_on_all_monitors = true;
        let toml_str = toml::to_string(&cfg).expect("serialize");
        assert!(toml_str.contains("show_on_all_monitors = true"));

        let decoded: AppConfig = toml::from_str(&toml_str).expect("deserialize");
        assert!(decoded.show_on_all_monitors);
    }
}
