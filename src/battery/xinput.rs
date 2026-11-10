use super::{BatteryCollector, BatterySource, DeviceBattery, DeviceType};
use windows::Win32::UI::Input::XboxController::{
    XInputGetBatteryInformation, BATTERY_DEVTYPE_GAMEPAD, BATTERY_LEVEL_EMPTY,
    BATTERY_LEVEL_FULL, BATTERY_LEVEL_LOW, BATTERY_LEVEL_MEDIUM, BATTERY_TYPE_ALKALINE,
    BATTERY_TYPE_DISCONNECTED, BATTERY_TYPE_NIMH, BATTERY_TYPE_WIRED, XINPUT_BATTERY_INFORMATION,
};

pub struct XInputCollector;

impl XInputCollector {
    pub fn new() -> Self {
        Self
    }
}

impl BatteryCollector for XInputCollector {
    fn poll_devices(&self) -> Vec<DeviceBattery> {
        let mut devices = Vec::new();

        for user_index in 0..4 {
            let mut info = XINPUT_BATTERY_INFORMATION::default();
            let result = unsafe {
                XInputGetBatteryInformation(user_index, BATTERY_DEVTYPE_GAMEPAD, &mut info)
            };

            if result == 0 {
                // Succeeded
                if info.BatteryType == BATTERY_TYPE_DISCONNECTED {
                    continue;
                }

                let is_wired = info.BatteryType == BATTERY_TYPE_WIRED;
                let (percentage, is_charging) = if is_wired {
                    (100, true)
                } else if info.BatteryType == BATTERY_TYPE_ALKALINE
                    || info.BatteryType == BATTERY_TYPE_NIMH
                    || info.BatteryType.0 == 0x04 // Standard wireless pack
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

                devices.push(DeviceBattery {
                    id: format!("xinput_pad_{}", user_index),
                    name,
                    device_type: DeviceType::Gamepad,
                    percentage,
                    is_charging,
                    is_connected: true,
                    source: BatterySource::XInput,
                });
            }
        }

        devices
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
