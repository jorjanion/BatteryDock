pub mod audio;
pub mod toast;

use crate::battery::DeviceBattery;
use crate::config::AppConfig;
use std::collections::HashMap;

pub struct NotificationManager {
    /// Tracks the last alerted percentage level per device id to prevent repeated alerts
    last_alerted_level: HashMap<String, u8>,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            last_alerted_level: HashMap::new(),
        }
    }

    pub fn check_and_notify(&mut self, devices: &[DeviceBattery], config: &AppConfig) {
        for device in devices {
            // If device is currently charging or disconnected, reset alert state
            if device.is_charging || !device.is_connected {
                self.last_alerted_level.remove(&device.id);
                continue;
            }

            let pct = device.percentage;
            let last_level = self.last_alerted_level.get(&device.id).copied();

            let should_alert_critical = pct <= config.critical_battery_threshold
                && last_level.map_or(true, |l| l > config.critical_battery_threshold);

            let should_alert_low = pct <= config.low_battery_threshold
                && last_level.map_or(true, |l| l > config.low_battery_threshold);

            if should_alert_critical {
                self.last_alerted_level.insert(device.id.clone(), pct);
                let title = format!("⚠️ Critical Battery: {}", device.name);
                let message = format!(
                    "Battery is at {}%! Connect the charger immediately.",
                    pct
                );

                if config.visual_alert_enabled {
                    toast::show_toast(&title, &message);
                }
                if config.audio_alert_enabled {
                    audio::play_gentle_chime();
                }
            } else if should_alert_low {
                self.last_alerted_level.insert(device.id.clone(), pct);
                let title = format!("Low Battery: {}", device.name);
                let message = format!(
                    "Battery has dropped to {}%. Time to charge soon.",
                    pct
                );

                if config.visual_alert_enabled {
                    toast::show_toast(&title, &message);
                }
                if config.audio_alert_enabled {
                    audio::play_gentle_chime();
                }
            } else if pct > config.low_battery_threshold {
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

        let dev = DeviceBattery {
            id: "test_dev_1".to_string(),
            name: "Test Headset".to_string(),
            device_type: DeviceType::Headset,
            percentage: 25,
            is_charging: false,
            is_connected: true,
            source: BatterySource::LogitechHidpp,
        };

        // At 25%, no alert
        mgr.check_and_notify(&[dev.clone()], &config);
        assert!(!mgr.last_alerted_level.contains_key("test_dev_1"));

        // Drops to 19% (low battery) -> should record alert
        let mut dev_low = dev.clone();
        dev_low.percentage = 19;
        mgr.check_and_notify(&[dev_low.clone()], &config);
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
}
