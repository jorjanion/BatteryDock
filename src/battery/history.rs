use crate::battery::DeviceBattery;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_POINTS_PER_DEVICE: usize = 96;
const MAX_AGE_SECS: u64 = 24 * 3600; // Keep last 24h
const PERIODIC_SAMPLE_SECS: u64 = 300; // Record periodic anchor every 5 min

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BatteryPoint {
    pub timestamp_secs: u64,
    pub percentage: u8,
    pub is_charging: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct BatteryHistoryStore {
    #[serde(default)]
    pub devices: HashMap<String, Vec<BatteryPoint>>,
}

impl BatteryHistoryStore {
    pub fn history_path() -> PathBuf {
        if let Some(proj_dirs) = directories::ProjectDirs::from("com", "jorjanion", "BatteryDock") {
            let config_dir = proj_dirs.config_dir();
            let _ = fs::create_dir_all(config_dir);
            config_dir.join("history.toml")
        } else {
            PathBuf::from("history.toml")
        }
    }

    pub fn load_from_path(path: &Path) -> Self {
        fs::read_to_string(path)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn load() -> Self {
        Self::load_from_path(&Self::history_path())
    }

    pub fn save_to_path(&self, path: &Path) {
        if let Ok(content) = toml::to_string(self) {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let tmp = path.with_extension(format!("toml.{}.tmp", std::process::id()));
            if fs::write(&tmp, &content).is_ok() {
                if fs::rename(&tmp, path).is_ok() {
                    return;
                }
                let _ = fs::remove_file(&tmp);
            }
            let _ = fs::write(path, content);
        }
    }

    pub fn save(&self) {
        self.save_to_path(&Self::history_path());
    }

    pub fn now_unix_secs() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }

    /// Records current battery readings if a percentage/charging change occurred or >=5m elapsed.
    /// Returns `true` if any point was added/pruned (meaning disk persistence is warranted).
    pub fn record_snapshot(&mut self, devices: &[DeviceBattery], now_secs: u64) -> bool {
        let mut dirty = false;

        for dev in devices {
            if !dev.is_connected {
                continue;
            }
            let entry = self.devices.entry(dev.id.clone()).or_default();
            let pct = dev.percentage.min(100);

            let should_push = match entry.last() {
                None => true,
                Some(last) => {
                    last.percentage != pct
                        || last.is_charging != dev.is_charging
                        || now_secs.saturating_sub(last.timestamp_secs) >= PERIODIC_SAMPLE_SECS
                }
            };

            if should_push {
                entry.push(BatteryPoint {
                    timestamp_secs: now_secs,
                    percentage: pct,
                    is_charging: dev.is_charging,
                });
                dirty = true;
            }

            // Prune entries older than 24h (always keep at least the latest point)
            let cutoff = now_secs.saturating_sub(MAX_AGE_SECS);
            if entry.len() > 1 {
                let before = entry.len();
                let last_pt = entry.last().cloned();
                entry.retain(|p| p.timestamp_secs >= cutoff);
                if entry.is_empty() {
                    if let Some(lp) = last_pt {
                        entry.push(lp);
                    }
                }
                if entry.len() != before {
                    dirty = true;
                }
            }

            if entry.len() > MAX_POINTS_PER_DEVICE {
                let excess = entry.len() - MAX_POINTS_PER_DEVICE;
                entry.drain(0..excess);
                dirty = true;
            }
        }

        dirty
    }

    pub fn points_for(&self, device_id: &str) -> &[BatteryPoint] {
        self.devices
            .get(device_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battery::{BatterySource, DeviceType};

    #[test]
    fn test_history_record_dedup_pruning_and_roundtrip() {
        let mut store = BatteryHistoryStore::default();
        let dev = DeviceBattery::new(
            "mouse_1",
            "Gaming Mouse",
            DeviceType::Mouse,
            80,
            false,
            true,
            BatterySource::LogitechHidpp,
        );

        // First sample recorded
        assert!(store.record_snapshot(std::slice::from_ref(&dev), 10_000));
        assert_eq!(store.points_for("mouse_1").len(), 1);

        // Same percentage 60s later -> skipped (no disk churn)
        assert!(!store.record_snapshot(std::slice::from_ref(&dev), 10_060));
        assert_eq!(store.points_for("mouse_1").len(), 1);

        // Same percentage 300s later -> periodic anchor recorded
        assert!(store.record_snapshot(std::slice::from_ref(&dev), 10_300));
        assert_eq!(store.points_for("mouse_1").len(), 2);

        // Percentage drops to 79% -> immediately recorded
        let mut dev2 = dev.clone();
        dev2.percentage = 79;
        assert!(store.record_snapshot(&[dev2], 10_310));
        assert_eq!(store.points_for("mouse_1").len(), 3);

        // Roundtrip persistence
        let dir = std::env::temp_dir().join(format!("bd_hist_test_{}", std::process::id()));
        let path = dir.join("history.toml");
        store.save_to_path(&path);
        let loaded = BatteryHistoryStore::load_from_path(&path);
        assert_eq!(loaded, store);
        let _ = fs::remove_dir_all(&dir);
    }
}
