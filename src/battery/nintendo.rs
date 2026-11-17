use super::{BatteryCollector, BatterySource, DeviceBattery, DeviceType};
use hidapi::HidApi;

pub const NINTENDO_VID: u16 = 0x057E;

pub struct NintendoCollector;

impl NintendoCollector {
    pub fn new() -> Self {
        Self
    }
}

/// Parses battery level and charging state from Nintendo Switch input reports (0x21, 0x30, 0x31, 0x3F).
pub fn parse_nintendo_battery_report(buf: &[u8]) -> Option<(u8, bool)> {
    if buf.is_empty() {
        return None;
    }

    let report_id = buf[0];
    if report_id == 0x21 || report_id == 0x30 || report_id == 0x31 {
        if buf.len() < 3 {
            return None;
        }
        // Byte 2 encodes battery status in high nibble (bits 5-7 level, bit 4 charging)
        let bat_byte = buf[2];
        let level_raw = (bat_byte >> 4) & 0x0E;
        let is_charging = (bat_byte & 0x10) != 0;

        let pct = match level_raw {
            8..=14 => 100,
            6 => 70,
            4 => 40,
            2 => 15,
            _ => 5,
        };
        Some((pct, is_charging))
    } else if report_id == 0x3F {
        // Simple 0x3F HID input report format
        if buf.len() < 2 {
            return None;
        }
        let bat_byte = buf[1];
        let level_raw = (bat_byte >> 4) & 0x0E;
        let is_charging = (bat_byte & 0x10) != 0;

        let pct = match level_raw {
            8..=14 => 100,
            6 => 70,
            4 => 40,
            2 => 15,
            _ => 5,
        };
        Some((pct, is_charging))
    } else {
        None
    }
}

pub fn resolve_nintendo_name(pid: u16, product_string: Option<&str>) -> String {
    if let Some(s) = product_string {
        let trimmed = s.trim();
        if !trimmed.is_empty() && trimmed != "Wireless Gamepad" && trimmed != "Nintendo" {
            return trimmed.to_string();
        }
    }

    match pid {
        0x2009 => "Nintendo Switch Pro Controller".to_string(),
        0x2006 => "Joy-Con (L)".to_string(),
        0x2007 => "Joy-Con (R)".to_string(),
        0x200E => "Joy-Con Charging Grip".to_string(),
        0x2017 => "SNES Controller for Nintendo Switch".to_string(),
        0x2019 => "N64 Controller for Nintendo Switch".to_string(),
        0x201A => "SEGA Genesis Controller for Nintendo Switch".to_string(),
        _ => format!("Nintendo Switch Controller ({:04x})", pid),
    }
}

impl BatteryCollector for NintendoCollector {
    fn poll_devices(&self) -> Vec<DeviceBattery> {
        let mut devices = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();

        let api = match HidApi::new() {
            Ok(api) => api,
            Err(_) => return devices,
        };

        for dev_info in api.device_list() {
            if dev_info.vendor_id() != NINTENDO_VID {
                continue;
            }

            let pid = dev_info.product_id();
            let is_switch = matches!(
                pid,
                0x2009 | 0x2006 | 0x2007 | 0x200E | 0x2017 | 0x2019 | 0x201A
            );

            if !is_switch {
                continue;
            }

            let serial = dev_info
                .serial_number()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .unwrap_or_else(|| dev_info.path().to_string_lossy().to_string());

            let id = format!("nintendo_{:04x}_{}", pid, serial);
            if seen_ids.contains(&id) {
                continue;
            }

            let Ok(device) = dev_info.open_device(&api) else {
                continue;
            };

            // Read live input report stream with 50ms timeout
            let mut buf = [0u8; 64];
            if let Ok(bytes_read) = device.read_timeout(&mut buf, 50) {
                if bytes_read >= 2 {
                    if let Some((percentage, is_charging)) =
                        parse_nintendo_battery_report(&buf[..bytes_read])
                    {
                        let name = resolve_nintendo_name(pid, dev_info.product_string());
                        seen_ids.insert(id.clone());
                        devices.push(DeviceBattery::new(
                            id,
                            name,
                            DeviceType::Gamepad,
                            percentage,
                            is_charging,
                            true,
                            BatterySource::NintendoHid,
                        ));
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
    fn test_parse_nintendo_battery_report_levels() {
        // Full battery (0x80) on report 0x30
        let mut buf = [0u8; 64];
        buf[0] = 0x30;
        buf[1] = 0x01; // Timer
        buf[2] = 0x80; // High nibble 8 = Full, charging = 0
        assert_eq!(parse_nintendo_battery_report(&buf), Some((100, false)));

        // Full battery charging (0x90: high nibble 9 = 8 full | 1 charging)
        buf[2] = 0x90;
        assert_eq!(parse_nintendo_battery_report(&buf), Some((100, true)));

        // Medium battery (0x60)
        buf[2] = 0x60;
        assert_eq!(parse_nintendo_battery_report(&buf), Some((70, false)));

        // Low battery (0x40)
        buf[2] = 0x40;
        assert_eq!(parse_nintendo_battery_report(&buf), Some((40, false)));

        // Critical battery (0x20)
        buf[2] = 0x20;
        assert_eq!(parse_nintendo_battery_report(&buf), Some((15, false)));

        // Empty battery (0x00)
        buf[2] = 0x00;
        assert_eq!(parse_nintendo_battery_report(&buf), Some((5, false)));

        // Simple input report 0x3F format
        let mut simple = [0u8; 12];
        simple[0] = 0x3F;
        simple[1] = 0x60; // Medium
        assert_eq!(parse_nintendo_battery_report(&simple), Some((70, false)));
    }

    #[test]
    fn test_resolve_nintendo_name() {
        assert_eq!(
            resolve_nintendo_name(0x2009, None),
            "Nintendo Switch Pro Controller"
        );
        assert_eq!(resolve_nintendo_name(0x2006, None), "Joy-Con (L)");
        assert_eq!(resolve_nintendo_name(0x2007, None), "Joy-Con (R)");
    }
}
