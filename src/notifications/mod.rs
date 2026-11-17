pub mod audio;
pub mod toast;

use crate::battery::DeviceBattery;
use crate::config::AppConfig;
use std::collections::{HashMap, HashSet};

pub struct NotificationManager {
    /// Tracks the last alerted percentage level per device id to prevent repeated alerts
    last_alerted_level: HashMap<String, u8>,
    /// Tracks devices seen charging below 100% so we only fire the 100% full-charge alert
    /// on an actual charge completion transition (never spuriously on boot).
    was_charging_below_full: HashSet<String>,
    /// Tracks devices that already received the 100% full-charge alert during the current charge cycle
    alerted_full_charge: HashSet<String>,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            last_alerted_level: HashMap::new(),
            was_charging_below_full: HashSet::new(),
            alerted_full_charge: HashSet::new(),
        }
    }

    pub fn check_and_notify(&mut self, devices: &[DeviceBattery], config: &AppConfig) {
        for device in devices {
            // If device is muted in config, do not alert
            if config.is_device_muted(&device.id) {
                self.last_alerted_level.remove(&device.id);
                self.was_charging_below_full.remove(&device.id);
                self.alerted_full_charge.remove(&device.id);
                continue;
            }

            if !device.is_connected {
                self.last_alerted_level.remove(&device.id);
                self.was_charging_below_full.remove(&device.id);
                self.alerted_full_charge.remove(&device.id);
                continue;
            }

            // Full-charge (100%) notification logic while charging
            if device.is_charging {
                self.last_alerted_level.remove(&device.id);
                if device.percentage < 100 {
                    self.was_charging_below_full.insert(device.id.clone());
                    self.alerted_full_charge.remove(&device.id);
                } else if config.full_charge_alert_enabled
                    && self.was_charging_below_full.contains(&device.id)
                    && !self.alerted_full_charge.contains(&device.id)
                {
                    self.alerted_full_charge.insert(device.id.clone());
                    let display_name = config.get_device_display_name(&device.id, &device.name);
                    let title = config.language.toast_full_charge_title(display_name);
                    let message = config.language.toast_full_charge_body();
                    if config.visual_alert_enabled {
                        toast::show_toast(&title, message);
                    }
                    if config.audio_alert_enabled {
                        audio::play_gentle_chime();
                    }
                }
                continue;
            } else {
                // Disconnected from charger -> reset full-charge cycle tracking
                self.was_charging_below_full.remove(&device.id);
                self.alerted_full_charge.remove(&device.id);
            }

            if device.is_asleep {
                self.last_alerted_level.remove(&device.id);
                continue;
            }

            let pct = device.percentage;
            let last_level = self.last_alerted_level.get(&device.id).copied();
            let (low_threshold, critical_threshold) = config.get_device_thresholds(&device.id);

            let should_alert_critical =
                pct <= critical_threshold && last_level.is_none_or(|l| l > critical_threshold);

            let should_alert_low =
                pct <= low_threshold && last_level.is_none_or(|l| l > low_threshold);

            let display_name = config.get_device_display_name(&device.id, &device.name);
            let time_info = if let Some(mins) = device.estimated_minutes_left {
                let h = mins / 60;
                let m = mins % 60;
                if h > 0 {
                    format!(" (~{}h {}m left)", h, m)
                } else {
                    format!(" (~{}m left)", m)
                }
            } else {
                String::new()
            };

            if should_alert_critical {
                self.last_alerted_level.insert(device.id.clone(), pct);
                let title = format!("⚠️ Critical Battery: {}", display_name);
                let message = format!(
                    "Battery is at {}{}! Connect the charger immediately.",
                    pct, time_info
                );

                if config.visual_alert_enabled {
                    toast::show_toast(&title, &message);
                }
                if config.audio_alert_enabled {
                    audio::play_gentle_chime();
                }
            } else if should_alert_low {
                self.last_alerted_level.insert(device.id.clone(), pct);
                let title = format!("Low Battery: {}", display_name);
                let message = format!(
                    "Battery has dropped to {}{}. Time to charge soon.",
                    pct, time_info
                );

                if config.visual_alert_enabled {
                    toast::show_toast(&title, &message);
                }
                if config.audio_alert_enabled {
                    audio::play_gentle_chime();
                }
            } else if pct > low_threshold {
                // If it recovered above the threshold, clear the alert mark
                self.last_alerted_level.remove(&device.id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battery::{BatterySource, DeviceType};

    #[test]
    fn test_notification_alert_logic() {
        let mut mgr = NotificationManager::new();
        let config = AppConfig {
            low_battery_threshold: 20,
            critical_battery_threshold: 10,
            audio_alert_enabled: false,
            visual_alert_enabled: false,
            ..Default::default()
        };

        let dev = DeviceBattery::new(
            "test_dev_1",
            "Test Headset",
            DeviceType::Headset,
            25,
            false,
            true,
            BatterySource::LogitechHidpp,
        );

        // At 25%, no alert
        mgr.check_and_notify(std::slice::from_ref(&dev), &config);
        assert!(!mgr.last_alerted_level.contains_key("test_dev_1"));

        // Drops to 19% (low battery) -> should record alert
        let mut dev_low = dev.clone();
        dev_low.percentage = 19;
        mgr.check_and_notify(&[dev_low], &config);
        assert_eq!(mgr.last_alerted_level.get("test_dev_1"), Some(&19));

        // Stays at 18% -> shouldn't re-trigger low alert
        let mut dev_low2 = dev.clone();
        dev_low2.percentage = 18;
        mgr.check_and_notify(&[dev_low2], &config);
        assert_eq!(mgr.last_alerted_level.get("test_dev_1"), Some(&19));

        // Drops to 8% (critical battery) -> should update alert level
        let mut dev_crit = dev.clone();
        dev_crit.percentage = 8;
        mgr.check_and_notify(&[dev_crit], &config);
        assert_eq!(mgr.last_alerted_level.get("test_dev_1"), Some(&8));
    }

    #[test]
    fn test_per_device_mute_and_custom_threshold() {
        use crate::config::DeviceAlertConfig;
        let mut mgr = NotificationManager::new();
        let mut config = AppConfig {
            low_battery_threshold: 20,
            critical_battery_threshold: 10,
            audio_alert_enabled: false,
            visual_alert_enabled: false,
            ..Default::default()
        };

        // Device 1: custom higher threshold 30%
        config.device_configs.insert(
            "dev_custom".to_string(),
            DeviceAlertConfig {
                low_threshold: Some(30),
                critical_threshold: Some(15),
                mute_alerts: false,
                custom_name: None,
                hidden: false,
            },
        );

        // Device 2: muted
        config.device_configs.insert(
            "dev_muted".to_string(),
            DeviceAlertConfig {
                low_threshold: None,
                critical_threshold: None,
                mute_alerts: true,
                custom_name: None,
                hidden: false,
            },
        );

        let dev_custom = DeviceBattery::new(
            "dev_custom",
            "Custom Mouse",
            DeviceType::Mouse,
            28,
            false,
            true,
            BatterySource::RazerHid,
        );

        let dev_muted = DeviceBattery::new(
            "dev_muted",
            "Muted Headset",
            DeviceType::Headset,
            5, // below critical!
            false,
            true,
            BatterySource::AudioHeadsetHid,
        );

        mgr.check_and_notify(&[dev_custom, dev_muted], &config);

        // dev_custom at 28% triggered alert because its threshold is 30% (even though global is 20%)
        assert_eq!(mgr.last_alerted_level.get("dev_custom"), Some(&28));
        // dev_muted at 5% was muted so no alert triggered
        assert!(!mgr.last_alerted_level.contains_key("dev_muted"));
    }

    #[test]
    fn test_full_charge_100_alert_transition() {
        let mut mgr = NotificationManager::new();
        let config = AppConfig {
            audio_alert_enabled: false,
            visual_alert_enabled: false,
            full_charge_alert_enabled: true,
            ..Default::default()
        };

        // Already at 100% on startup while plugged in -> must NOT trigger spurious alert
        let dev_100_boot = DeviceBattery::new(
            "dev_chg",
            "Gaming Mouse",
            DeviceType::Mouse,
            100,
            true,
            true,
            BatterySource::LogitechHidpp,
        );
        mgr.check_and_notify(std::slice::from_ref(&dev_100_boot), &config);
        assert!(!mgr.alerted_full_charge.contains("dev_chg"));

        // Seen charging at 95% -> arms the full-charge transition detector
        let mut dev_95 = dev_100_boot.clone();
        dev_95.percentage = 95;
        mgr.check_and_notify(&[dev_95], &config);
        assert!(mgr.was_charging_below_full.contains("dev_chg"));
        assert!(!mgr.alerted_full_charge.contains("dev_chg"));

        // Reaches 100% -> triggers full-charge alert
        mgr.check_and_notify(std::slice::from_ref(&dev_100_boot), &config);
        assert!(mgr.alerted_full_charge.contains("dev_chg"));

        // Unplugged -> resets cycle state
        let mut dev_unplugged = dev_100_boot.clone();
        dev_unplugged.is_charging = false;
        mgr.check_and_notify(&[dev_unplugged], &config);
        assert!(!mgr.was_charging_below_full.contains("dev_chg"));
        assert!(!mgr.alerted_full_charge.contains("dev_chg"));
    }
}
