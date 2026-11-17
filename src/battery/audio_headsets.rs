use super::{BatteryCollector, BatterySource, DeviceBattery, DeviceType};
use hidapi::HidApi;

pub const HYPERX_KINGSTON_VID: u16 = 0x0951;
pub const HYPERX_HP_VID: u16 = 0x03F0;
pub const ASTRO_VID: u16 = 0x9886;

pub struct AudioHeadsetCollector;

impl AudioHeadsetCollector {
    pub fn new() -> Self {
        Self
    }
}

/// Parses battery level and charging state from HyperX wireless headset responses.
pub fn parse_hyperx_battery_response(buf: &[u8]) -> Option<(u8, bool)> {
    if buf.is_empty() {
        return None;
    }
    let slice = if buf[0] == 0x00 { &buf[1..] } else { buf };
    if slice.len() < 3 {
        return None;
    }

    // Search for header token (0xBB, 0x06, 0x0B, or 0x0C) followed by battery level
    for i in 0..slice.len().saturating_sub(1) {
        if slice[i] == 0xBB || slice[i] == 0x06 || slice[i] == 0x0B || slice[i] == 0x0C {
            let candidate_val = slice[i + 1];
            if candidate_val > 0 && candidate_val <= 100 {
                let is_charging = if i + 2 < slice.len() {
                    slice[i + 2] == 0x01 || slice[i + 2] == 0x10
                } else {
                    false
                };
                return Some((candidate_val, is_charging));
            }
        }
    }

    None
}

/// Parses battery level and charging state from Astro Gaming A50 / A20 responses.
pub fn parse_astro_battery_response(buf: &[u8]) -> Option<(u8, bool)> {
    if buf.is_empty() {
        return None;
    }
    let slice = if buf[0] == 0x00 { &buf[1..] } else { buf };
    if slice.len() < 3 {
        return None;
    }

    // Astro A50 Report ID 0x03: [0x03, status, pct, chg, ...]
    let (val, chg) = if slice[0] == 0x03 && slice.len() >= 3 {
        let v = slice[2];
        let c = if slice.len() >= 4 { slice[3] } else { 0 };
        (v, c)
    } else {
        let v = slice[1];
        let c = if slice.len() >= 3 { slice[2] } else { 0 };
        (v, c)
    };

    if val > 0 && val <= 100 {
        let is_charging = chg == 0x01 || chg == 0x02;
        return Some((val, is_charging));
    }

    None
}

pub fn resolve_hyperx_name(pid: u16, product_string: Option<&str>) -> String {
    if let Some(s) = product_string {
        let trimmed = s.trim();
        if !trimmed.is_empty() && trimmed != "HyperX" && trimmed != "Wireless Headset" {
            return trimmed.replace(" Dongle", "").replace(" Transceiver", "");
        }
    }

    match pid {
        0x16C4 | 0x1723 => "HyperX Cloud Flight".to_string(),
        0x16EA | 0x16EB => "HyperX Cloud Flight S".to_string(),
        0x1718 | 0x018B | 0x028B | 0x0696 => "HyperX Cloud II Wireless".to_string(),
        0x098D | 0x048D => "HyperX Cloud Alpha Wireless".to_string(),
        0x068B => "HyperX Cloud Core Wireless".to_string(),
        0x089D | 0x06BE => "HyperX Cloud III Wireless".to_string(),
        _ => format!("HyperX Wireless Headset ({:04x})", pid),
    }
}

pub fn resolve_astro_name(pid: u16, product_string: Option<&str>) -> String {
    if let Some(s) = product_string {
        let trimmed = s.trim();
        if !trimmed.is_empty() && trimmed != "ASTRO" && trimmed != "Astro Gaming" {
            return trimmed.replace(" Base Station", "").replace(" Dongle", "");
        }
    }

    match pid {
        0x0024 | 0x0025 => "Astro A50 Wireless Gen 3".to_string(),
        0x002C | 0x002D => "Astro A50 Wireless Gen 4".to_string(),
        0x0021 | 0x0022 => "Astro A20 Wireless Gen 1".to_string(),
        0x003A | 0x003B => "Astro A20 Wireless Gen 2".to_string(),
        _ => format!("Astro Gaming Headset ({:04x})", pid),
    }
}

impl BatteryCollector for AudioHeadsetCollector {
    fn poll_devices(&self) -> Vec<DeviceBattery> {
        let mut devices = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();

        let api = match HidApi::new() {
            Ok(api) => api,
            Err(_) => return devices,
        };

        for dev_info in api.device_list() {
            let vid = dev_info.vendor_id();
            let pid = dev_info.product_id();

            let is_hyperx = vid == HYPERX_KINGSTON_VID || vid == HYPERX_HP_VID;
            let is_astro = vid == ASTRO_VID;

            if !is_hyperx && !is_astro {
                continue;
            }

            let serial = dev_info
                .serial_number()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .unwrap_or_else(|| dev_info.path().to_string_lossy().to_string());

            let id = format!("audio_{:04x}_{:04x}_{}", vid, pid, serial);
            if seen_ids.contains(&id) {
                continue;
            }

            let Ok(device) = dev_info.open_device(&api) else {
                continue;
            };

            let mut battery_opt = None;

            if is_hyperx {
                // Send HyperX battery query packet
                let mut req = [0u8; 64];
                req[0] = 0x00;
                req[1] = 0x06;
                req[2] = 0xFF;
                req[3] = 0xBB;
                req[4] = 0x02;

                if device.send_feature_report(&req).is_ok() {
                    let mut resp = [0u8; 64];
                    resp[0] = 0x00;
                    if device.get_feature_report(&mut resp).is_ok() {
                        battery_opt = parse_hyperx_battery_response(&resp);
                    }
                }
            } else if is_astro {
                // Read Astro A50 / A20 battery feature report (Report ID 0x03 or 0x00)
                let mut resp = [0u8; 64];
                resp[0] = 0x03;
                if device.get_feature_report(&mut resp).is_ok() {
                    battery_opt = parse_astro_battery_response(&resp);
                } else {
                    resp[0] = 0x00;
                    if device.get_feature_report(&mut resp).is_ok() {
                        battery_opt = parse_astro_battery_response(&resp);
                    }
                }
            }

            if let Some((percentage, is_charging)) = battery_opt {
                let name = if is_hyperx {
                    resolve_hyperx_name(pid, dev_info.product_string())
                } else {
                    resolve_astro_name(pid, dev_info.product_string())
                };

                seen_ids.insert(id.clone());
                devices.push(DeviceBattery::new(
                    id,
                    name,
                    DeviceType::Headset,
                    percentage,
                    is_charging,
                    true,
                    BatterySource::AudioHeadsetHid,
                ));
            }
        }

        devices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hyperx_battery_response() {
        let resp = [0x00, 0x06, 0xBB, 82, 0x00];
        assert_eq!(parse_hyperx_battery_response(&resp), Some((82, false)));

        let resp_charging = [0x00, 0x06, 0xBB, 45, 0x01];
        assert_eq!(parse_hyperx_battery_response(&resp_charging), Some((45, true)));

        let invalid = [0x00, 0x01, 0x02, 0x03];
        assert_eq!(parse_hyperx_battery_response(&invalid), None);
    }

    #[test]
    fn test_parse_astro_battery_response() {
        let resp = [0x03, 0x01, 88, 0x00];
        assert_eq!(parse_astro_battery_response(&resp), Some((88, false)));

        let resp_charging = [0x03, 0x01, 60, 0x01];
        assert_eq!(parse_astro_battery_response(&resp_charging), Some((60, true)));
    }

    #[test]
    fn test_resolve_headset_names() {
        assert_eq!(
            resolve_hyperx_name(0x1718, None),
            "HyperX Cloud II Wireless"
        );
        assert_eq!(
            resolve_hyperx_name(0x098D, None),
            "HyperX Cloud Alpha Wireless"
        );
        assert_eq!(
            resolve_astro_name(0x002C, None),
            "Astro A50 Wireless Gen 4"
        );
    }
}
