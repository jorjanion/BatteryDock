use super::{BatteryCollector, BatterySource, DeviceBattery, DeviceType};
use windows::Gaming::Input::Gamepad;
use windows::System::Power::BatteryStatus;
use windows::Win32::UI::Input::XboxController::{
    XInputGetBatteryInformation, BATTERY_DEVTYPE_GAMEPAD, BATTERY_LEVEL_EMPTY, BATTERY_LEVEL_FULL,
    BATTERY_LEVEL_LOW, BATTERY_LEVEL_MEDIUM, BATTERY_TYPE_ALKALINE, BATTERY_TYPE_DISCONNECTED,
    BATTERY_TYPE_NIMH, BATTERY_TYPE_WIRED, XINPUT_BATTERY_INFORMATION,
};

pub struct XInputCollector;

impl XInputCollector {
    pub fn new() -> Self {
        Self
    }

    /// Attempts to query Xbox gamepads via Windows.Gaming.Input (provides exact battery percentages
    /// for Xbox One, Xbox Series X/S, and Xbox Elite controllers over Bluetooth & Xbox Wireless Adapter).
    fn poll_winrt_gamepads(&self) -> Vec<DeviceBattery> {
        let mut devices = Vec::new();

        let Ok(gamepads) = Gamepad::Gamepads() else {
            return devices;
        };

        for (idx, gamepad) in gamepads.into_iter().enumerate() {
            let Ok(report) = gamepad.TryGetBatteryReport() else {
                continue;
            };

            let Ok(status) = report.Status() else {
                continue;
            };

            if status == BatteryStatus::NotPresent {
                continue;
            }

            let is_charging = status == BatteryStatus::Charging;

            let remaining = report
                .RemainingCapacityInMilliwattHours()
                .ok()
                .and_then(|r| r.Value().ok());
            let full = report
                .FullChargeCapacityInMilliwattHours()
                .ok()
                .and_then(|f| f.Value().ok());

            let percentage = if let (Some(rem), Some(f)) = (remaining, full) {
                if f > 0 {
                    ((rem as f32 / f as f32) * 100.0).clamp(0.0, 100.0).round() as u8
                } else {
                    100
                }
            } else {
                // If mWh capacity is omitted by firmware, map default charge level
                if is_charging {
                    95
                } else {
                    75
                }
            };

            let name = if idx == 0 {
                "Xbox Wireless Controller".to_string()
            } else {
                format!("Xbox Wireless Controller #{}", idx + 1)
            };

            devices.push(DeviceBattery::new(
                format!("xbox_winrt_{}", idx),
                name,
                DeviceType::Gamepad,
                percentage,
                is_charging,
                true,
                BatterySource::XInput,
            ));
        }

        devices
    }

    /// Fallback query via legacy Win32 XInputGetBatteryInformation (4 discrete levels).
    fn poll_legacy_xinput(&self) -> Vec<DeviceBattery> {
        let mut devices = Vec::new();

        for user_index in 0..4 {
            let mut info = XINPUT_BATTERY_INFORMATION::default();
            let result = unsafe {
                XInputGetBatteryInformation(user_index, BATTERY_DEVTYPE_GAMEPAD, &mut info)
            };

            if result == 0 {
                if info.BatteryType == BATTERY_TYPE_DISCONNECTED {
                    continue;
                }

                let is_wired = info.BatteryType == BATTERY_TYPE_WIRED;
                let (percentage, is_charging) = if is_wired {
                    (100, true)
                } else if info.BatteryType == BATTERY_TYPE_ALKALINE
                    || info.BatteryType == BATTERY_TYPE_NIMH
                    || info.BatteryType.0 == 0x04
                {
                    match info.BatteryLevel {
                        x if x == BATTERY_LEVEL_EMPTY => (10, false),
                        x if x == BATTERY_LEVEL_LOW => (30, false),
                        x if x == BATTERY_LEVEL_MEDIUM => (65, false),
                        x if x == BATTERY_LEVEL_FULL => (95, false),
                        _ => (50, false),
                    }
                } else {
                    continue;
                };

                let name = if user_index == 0 {
                    "Xbox Wireless Controller".to_string()
                } else {
                    format!("Xbox Wireless Controller #{}", user_index + 1)
                };

                devices.push(DeviceBattery::new(
                    format!("xinput_pad_{}", user_index),
                    name,
                    DeviceType::Gamepad,
                    percentage,
                    is_charging,
                    true,
                    BatterySource::XInput,
                ));
            }
        }

        devices
    }
}

impl BatteryCollector for XInputCollector {
    fn poll_devices(&self) -> Vec<DeviceBattery> {
        // Prefer modern Windows.Gaming.Input reports with exact milliwatt-hour precision
        let winrt_devs = self.poll_winrt_gamepads();
        if !winrt_devs.is_empty() {
            return winrt_devs;
        }

        // Fall back to legacy XInput for older controllers
        self.poll_legacy_xinput()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xinput_collector_creation() {
        let collector = XInputCollector::new();
        let _ = collector.poll_devices();
    }
}
