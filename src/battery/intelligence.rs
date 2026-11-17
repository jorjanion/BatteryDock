use super::{BatterySource, DeviceBattery, DeviceType};
use crate::config::AppConfig;
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct HistorySample {
    pub timestamp: Instant,
    pub percentage: u8,
    pub is_charging: bool,
}

#[derive(Debug, Clone)]
pub struct DeviceStateTracker {
    pub samples: VecDeque<HistorySample>,
    pub last_active: Instant,
    pub last_percentage: u8,
    pub last_charging: bool,
    pub is_asleep: bool,
}

impl DeviceStateTracker {
    pub fn new(initial_pct: u8, initial_charging: bool) -> Self {
        let now = Instant::now();
        let mut samples = VecDeque::new();
        samples.push_back(HistorySample {
            timestamp: now,
            percentage: initial_pct,
            is_charging: initial_charging,
        });
        Self {
            samples,
            last_active: now,
            last_percentage: initial_pct,
            last_charging: initial_charging,
            is_asleep: false,
        }
    }

    pub fn update(
        &mut self,
        percentage: u8,
        is_charging: bool,
        dev_type: DeviceType,
        _source: BatterySource,
    ) {
        let now = Instant::now();

        // 1. If battery percentage or charging status changed, peripheral is actively reporting
        if percentage != self.last_percentage || is_charging != self.last_charging {
            self.last_active = now;
            self.is_asleep = false;
        }

        // 2. Query Windows system-wide user input idle time
        let system_idle_secs = get_system_idle_seconds().unwrap_or(0);

        // Input peripherals (Mouse, Keyboard, Gamepad)
        match dev_type {
            DeviceType::Mouse | DeviceType::Keyboard | DeviceType::Gamepad => {
                if system_idle_secs < 120 {
                    // User was active in the last 2 minutes -> awake
                    self.last_active = now;
                    self.is_asleep = false;
                } else if !is_charging && system_idle_secs >= 300 {
                    // System has been idle for >= 5 minutes without charging -> sleep mode
                    self.is_asleep = true;
                }
            }
            _ => {
                // Headsets, webcams, VR, etc.
                if !is_charging && now.duration_since(self.last_active) >= Duration::from_secs(1200) {
                    self.is_asleep = true;
                }
            }
        }

        // 3. Record history sample at most once every 30 seconds unless percentage changed
        let should_record = self.samples.back().is_none_or(|last| {
            last.percentage != percentage
                || last.is_charging != is_charging
                || now.duration_since(last.timestamp) >= Duration::from_secs(30)
        });

        if should_record {
            self.samples.push_back(HistorySample {
                timestamp: now,
                percentage,
                is_charging,
            });
        }

        // Keep maximum 60 minutes of history
        while let Some(front) = self.samples.front() {
            if now.duration_since(front.timestamp) > Duration::from_secs(3600) && self.samples.len() > 10 {
                self.samples.pop_front();
            } else {
                break;
            }
        }

        self.last_percentage = percentage;
        self.last_charging = is_charging;
    }

    pub fn estimate_minutes(&self, percentage: u8, is_charging: bool, dev_type: DeviceType) -> Option<u32> {
        if is_charging {
            if percentage >= 100 {
                return Some(0);
            }
            let remaining_pct = (100 - percentage) as u32;

            // Check if we have empirical charging rate
            let mut empirical_rate = None;
            if let (Some(first), Some(last)) = (self.samples.front(), self.samples.back()) {
                if first.is_charging && last.is_charging && last.percentage > first.percentage {
                    let elapsed_secs = last.timestamp.duration_since(first.timestamp).as_secs();
                    if elapsed_secs >= 60 {
                        let pct_gain = (last.percentage - first.percentage) as f64;
                        let rate_per_sec = pct_gain / elapsed_secs as f64;
                        if rate_per_sec > 0.0 {
                            let secs_left = (remaining_pct as f64) / rate_per_sec;
                            empirical_rate = Some((secs_left / 60.0).round() as u32);
                        }
                    }
                }
            }

            if let Some(mins) = empirical_rate {
                return Some(mins.clamp(2, 360));
            }

            // Baseline charging rate: ~1.4 mins per 1%
            let baseline_mins = (remaining_pct * 7) / 5;
            Some(baseline_mins.clamp(2, 240))
        } else {
            if percentage == 0 {
                return Some(0);
            }

            // Check if we have empirical discharge rate
            let mut empirical_rate = None;
            if let (Some(first), Some(last)) = (self.samples.front(), self.samples.back()) {
                if !first.is_charging && !last.is_charging && first.percentage > last.percentage {
                    let elapsed_secs = last.timestamp.duration_since(first.timestamp).as_secs();
                    if elapsed_secs >= 60 {
                        let pct_drop = (first.percentage - last.percentage) as f64;
                        let rate_per_sec = pct_drop / elapsed_secs as f64;
                        if rate_per_sec > 0.0 {
                            let secs_left = (percentage as f64) / rate_per_sec;
                            empirical_rate = Some((secs_left / 60.0).round() as u32);
                        }
                    }
                }
            }

            // Baseline hours per device category
            let baseline_hours = match dev_type {
                DeviceType::Mouse => 70,
                DeviceType::Keyboard => 150,
                DeviceType::Headset => 30,
                DeviceType::Gamepad => 15,
                DeviceType::VRHeadset => 3,
                DeviceType::Wheel => 20,
                DeviceType::Camera | DeviceType::Unknown => 10,
            };
            let baseline_mins = (percentage as u32) * baseline_hours * 60 / 100;

            if let Some(emp_mins) = empirical_rate {
                // Blend 70% empirical with 30% baseline
                let blended = ((emp_mins as f64 * 0.7) + (baseline_mins as f64 * 0.3)).round() as u32;
                Some(blended.clamp(5, 18_000))
            } else {
                Some(baseline_mins.max(5))
            }
        }
    }
}

#[derive(Default)]
pub struct BatteryIntelligence {
    trackers: HashMap<String, DeviceStateTracker>,
}

impl BatteryIntelligence {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process(&mut self, devices: &mut [DeviceBattery], config: &AppConfig) {
        for dev in devices.iter_mut() {
            let tracker = self
                .trackers
                .entry(dev.id.clone())
                .or_insert_with(|| DeviceStateTracker::new(dev.percentage, dev.is_charging));

            tracker.update(dev.percentage, dev.is_charging, dev.device_type, dev.source);

            dev.is_asleep = tracker.is_asleep;
            dev.estimated_minutes_left = tracker.estimate_minutes(dev.percentage, dev.is_charging, dev.device_type);

            // Apply custom nickname if configured
            if let Some(cfg) = config.device_configs.get(&dev.id) {
                if let Some(ref custom_name) = cfg.custom_name {
                    if !custom_name.trim().is_empty() {
                        dev.name = custom_name.clone();
                    }
                }
            }
        }
    }
}

#[cfg(windows)]
pub fn get_system_idle_seconds() -> Option<u64> {
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
    extern "system" {
        fn GetTickCount() -> u32;
    }
    unsafe {
        let mut lii = LASTINPUTINFO {
            cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
            dwTime: 0,
        };
        if GetLastInputInfo(&mut lii).as_bool() {
            let tick = GetTickCount();
            let elapsed_ms = tick.wrapping_sub(lii.dwTime);
            Some((elapsed_ms / 1000) as u64)
        } else {
            None
        }
    }
}

#[cfg(not(windows))]
pub fn get_system_idle_seconds() -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_baseline_estimation() {
        let tracker = DeviceStateTracker::new(100, false);
        let mins = tracker.estimate_minutes(100, false, DeviceType::Mouse);
        assert_eq!(mins, Some(4200)); // 70 hours = 4200 minutes

        let mins_50 = tracker.estimate_minutes(50, false, DeviceType::Mouse);
        assert_eq!(mins_50, Some(2100)); // 35 hours
    }

    #[test]
    fn test_charging_estimation() {
        let tracker = DeviceStateTracker::new(50, true);
        let mins = tracker.estimate_minutes(50, true, DeviceType::Mouse);
        // (100 - 50) * 7 / 5 = 70 minutes
        assert_eq!(mins, Some(70));

        let full = tracker.estimate_minutes(100, true, DeviceType::Mouse);
        assert_eq!(full, Some(0));
    }
}
