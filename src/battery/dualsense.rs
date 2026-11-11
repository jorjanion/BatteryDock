use super::{BatteryCollector, BatterySource, DeviceBattery, DeviceType};
use hidapi::HidApi;

pub struct DualSenseCollector;

impl DualSenseCollector {
    pub fn new() -> Self {
        Self
    }
}

/// Parses DualSense (USB `0x01` / BT `0x31`) or DualShock 4 (USB `0x01` / BT `0x11`) HID input reports.
pub fn parse_sony_battery_report(buf: &[u8], is_dualsense: bool) -> Option<(u8, bool)> {
    if buf.is_empty() {
        return None;
    }

    if is_dualsense {
        if buf[0] == 0x01 && buf.len() >= 54 {
            // USB DualSense report
            let bat_byte = buf[53];
            let pct = (bat_byte & 0x0F).min(10) * 10;
            let charge_state = (bat_byte >> 4) & 0x0F;
            let charging = charge_state == 0x01 || charge_state == 0x02;
            Some((pct, charging))
        } else if buf[0] == 0x31 && buf.len() >= 55 {
            // Bluetooth DualSense report
            let bat_byte = buf[54];
            let pct = (bat_byte & 0x0F).min(10) * 10;
            let charge_state = (bat_byte >> 4) & 0x0F;
            let charging = charge_state == 0x01 || charge_state == 0x02;
            Some((pct, charging))
        } else {
            None
        }
    } else if buf[0] == 0x01 && buf.len() >= 31 {
        // USB DualShock 4 report
        let bat_byte = buf[30];
        let pct = (bat_byte & 0x0F).min(10) * 10;
        let charging = (bat_byte & 0x10) != 0;
        Some((pct, charging))
    } else if buf[0] == 0x11 && buf.len() >= 33 {
        // Bluetooth DualShock 4 report (2-byte BT header precedes payload)
        let bat_byte = buf[32];
        let pct = (bat_byte & 0x0F).min(10) * 10;
        let charging = (bat_byte & 0x10) != 0;
        Some((pct, charging))
    } else {
        None
    }
}

impl BatteryCollector for DualSenseCollector {
    fn poll_devices(&self) -> Vec<DeviceBattery> {
        let mut devices = Vec::new();

        let api = match HidApi::new() {
            Ok(api) => api,
            Err(_) => return devices,
        };

        for dev_info in api.device_list() {
            // Sony Vendor ID is 0x054C
            if dev_info.vendor_id() == 0x054C {
                let pid = dev_info.product_id();
                let is_dualsense = pid == 0x0CE6 || pid == 0x0DF2;
                let is_dualshock4 = pid == 0x05C4 || pid == 0x09CC;

                if !is_dualsense && !is_dualshock4 {
                    continue;
                }

                if let Ok(device) = dev_info.open_device(&api) {
                    let mut buf = [0u8; 78];
                    // Non-blocking or quick read
                    if let Ok(bytes_read) = device.read_timeout(&mut buf, 50) {
                        if let Some((percentage, is_charging)) =
                            parse_sony_battery_report(&buf[..bytes_read], is_dualsense)
                        {
                            let name = if is_dualsense {
                                "PlayStation DualSense Wireless Controller".to_string()
                            } else {
                                "PlayStation DualShock 4 Controller".to_string()
                            };

                            let serial = dev_info.serial_number().unwrap_or("0");
                            devices.push(DeviceBattery {
                                id: format!("sony_{:04x}_{}", pid, serial),
                                name,
                                device_type: DeviceType::Gamepad,
                                percentage,
                                is_charging,
                                is_connected: true,
                                source: BatterySource::DualSenseHid,
                            });
                            break;
                        }
                    }
                }
            }
        }

        devices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sony_battery_report_usb_and_bluetooth() {
        // DualSense USB charging at 60%
        let mut ds_usb = [0u8; 64];
        ds_usb[0] = 0x01;
        ds_usb[53] = 0x16; // charge_state=1, level=6 -> 60%, charging
        assert_eq!(parse_sony_battery_report(&ds_usb, true), Some((60, true)));

        // DualSense BT discharging at 80%
        let mut ds_bt = [0u8; 78];
        ds_bt[0] = 0x31;
        ds_bt[54] = 0x08;
        assert_eq!(parse_sony_battery_report(&ds_bt, true), Some((80, false)));

        // DualShock 4 BT charging at 90%
        let mut ds4_bt = [0u8; 78];
        ds4_bt[0] = 0x11;
        ds4_bt[32] = 0x19;
        assert_eq!(parse_sony_battery_report(&ds4_bt, false), Some((90, true)));

        // Unknown / short report must return None (never a fake 70%)
        let unknown = [0x99u8; 40];
        assert_eq!(parse_sony_battery_report(&unknown, true), None);
    }

    #[test]
    fn test_dualsense_collector_creation() {
        let collector = DualSenseCollector::new();
        let _ = collector.poll_devices();
    }
}
