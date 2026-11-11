use super::{BatteryCollector, BatterySource, DeviceBattery, DeviceType};
use hidapi::HidApi;

pub struct DualSenseCollector;

impl DualSenseCollector {
    pub fn new() -> Self {
        Self
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
                        if bytes_read > 30 {
                            let (percentage, is_charging) = if is_dualsense {
                                if buf[0] == 0x01 && bytes_read >= 54 {
                                    // USB DualSense report
                                    let bat_byte = buf[53];
                                    let pct = (bat_byte & 0x0F).min(10) * 10;
                                    let charge_state = (bat_byte >> 4) & 0x0F;
                                    let charging = charge_state == 0x01 || charge_state == 0x02;
                                    (pct, charging)
                                } else if buf[0] == 0x31 && bytes_read >= 55 {
                                    // Bluetooth DualSense report
                                    let bat_byte = buf[54];
                                    let pct = (bat_byte & 0x0F).min(10) * 10;
                                    let charge_state = (bat_byte >> 4) & 0x0F;
                                    let charging = charge_state == 0x01 || charge_state == 0x02;
                                    (pct, charging)
                                } else {
                                    (70, false)
                                }
                            } else {
                                // DualShock 4
                                let bat_byte = if bytes_read >= 31 { buf[30] } else { 0 };
                                let pct = (bat_byte & 0x0F).min(10) * 10;
                                let charging = (bat_byte & 0x10) != 0;
                                (pct, charging)
                            };

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
    fn test_dualsense_collector_creation() {
        let collector = DualSenseCollector::new();
        let _ = collector.poll_devices();
    }
}
