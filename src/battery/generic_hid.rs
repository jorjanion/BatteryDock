use super::{BatteryCollector, BatterySource, DeviceBattery, DeviceType};
use hidapi::HidApi;

pub const STEELSERIES_VID: u16 = 0x1038;
pub const CORSAIR_VID: u16 = 0x1B1C;
pub const GLORIOUS_VID_1: u16 = 0x258A;
pub const GLORIOUS_VID_2: u16 = 0x22D4;

pub struct GenericHidCollector;

impl GenericHidCollector {
    pub fn new() -> Self {
        Self
    }
}

/// Parses battery report payload from SteelSeries wireless devices (Aerox / Rival / Arctis).
pub fn parse_steelseries_battery_report(buf: &[u8]) -> Option<(u8, bool)> {
    if buf.is_empty() {
        return None;
    }
    let (header, val, chg_byte) = if buf[0] == 0x00 && buf.len() >= 4 {
        (buf[1], buf[2], buf[3])
    } else if buf.len() >= 3 {
        (buf[0], buf[1], buf[2])
    } else {
        return None;
    };

    if (header == 0x90 || header == 0x92 || header == 0xAA) && val <= 100 {
        let is_charging = chg_byte == 0x01 || chg_byte == 0x10;
        return Some((val, is_charging));
    }
    None
}

/// Parses battery report payload from Corsair Slipstream wireless devices.
pub fn parse_corsair_battery_report(buf: &[u8]) -> Option<(u8, bool)> {
    if buf.len() < 3 {
        return None;
    }
    let (header, val, state) = if buf[0] == 0x00 && buf.len() >= 4 {
        (buf[1], buf[2], buf[3])
    } else {
        (buf[0], buf[1], buf[2])
    };

    if (header == 0x08 || header == 0x0E) && val <= 100 {
        // state: 1 = discharging, 2 = charging, 4 = full/charging
        let is_charging = state == 0x02 || state == 0x04;
        return Some((val, is_charging));
    }
    None
}

/// Parses battery report payload from Glorious wireless devices (Model O / D / I Wireless).
pub fn parse_glorious_battery_report(buf: &[u8]) -> Option<(u8, bool)> {
    if buf.len() < 3 {
        return None;
    }
    let (cmd, val, chg) = if buf[0] == 0x00 && buf.len() >= 4 {
        (buf[1], buf[2], buf[3])
    } else {
        (buf[0], buf[1], buf[2])
    };

    if (cmd == 0x04 || cmd == 0x11) && val <= 100 {
        let is_charging = chg == 0x01;
        return Some((val, is_charging));
    }
    None
}

/// Resolves human-friendly name for SteelSeries devices.
pub fn resolve_steelseries_name(pid: u16, product_string: Option<&str>) -> String {
    if let Some(s) = product_string {
        let trimmed = s.trim();
        if !trimmed.is_empty() && trimmed != "SteelSeries" {
            return trimmed.replace(" Dongle", "").replace(" Transceiver", "");
        }
    }
    match pid {
        0x1830 | 0x1832 => "SteelSeries Rival 3 Wireless".to_string(),
        0x1836 | 0x1838 => "SteelSeries Aerox 3 Wireless".to_string(),
        0x1852 | 0x1854 => "SteelSeries Aerox 5 Wireless".to_string(),
        0x1858 | 0x185A => "SteelSeries Aerox 9 Wireless".to_string(),
        0x1842 | 0x1844 => "SteelSeries Prime Wireless".to_string(),
        0x172B | 0x172E => "SteelSeries Rival 650 Wireless".to_string(),
        0x12AD | 0x12AE => "SteelSeries Arctis 7".to_string(),
        0x12B3 => "SteelSeries Arctis 1 Wireless".to_string(),
        0x12C2 => "SteelSeries Arctis 7+".to_string(),
        0x12C0 => "SteelSeries Arctis 9".to_string(),
        0x12E0 => "SteelSeries Arctis Nova 7".to_string(),
        _ => format!("SteelSeries Wireless Device ({:04x})", pid),
    }
}

/// Resolves human-friendly name for Corsair devices.
pub fn resolve_corsair_name(pid: u16, product_string: Option<&str>) -> String {
    if let Some(s) = product_string {
        let trimmed = s.trim();
        if !trimmed.is_empty() && trimmed != "Corsair" {
            return trimmed.replace(" Wireless Receiver", "").replace(" Dongle", "");
        }
    }
    match pid {
        0x1BA0 => "Corsair Katar Pro Wireless".to_string(),
        0x1B82 => "Corsair Harpoon RGB Wireless".to_string(),
        0x1B27 => "Corsair Dark Core RGB Pro".to_string(),
        0x0A55 | 0x0A56 => "Corsair Virtuoso RGB Wireless".to_string(),
        0x0A4A => "Corsair HS70 Wireless".to_string(),
        0x0A6A => "Corsair HS80 RGB Wireless".to_string(),
        _ => format!("Corsair Wireless Device ({:04x})", pid),
    }
}

/// Resolves human-friendly name for Glorious devices.
pub fn resolve_glorious_name(pid: u16, product_string: Option<&str>) -> String {
    if let Some(s) = product_string {
        let trimmed = s.trim();
        if !trimmed.is_empty() && trimmed != "Glorious" && trimmed != "Gaming Mouse" {
            return trimmed.replace(" Wireless Dongle", "").replace(" Receiver", "");
        }
    }
    match pid {
        0x0029 | 0x002A => "Glorious Model O Wireless".to_string(),
        0x0033 | 0x0034 => "Glorious Model D Wireless".to_string(),
        0x0036 | 0x0037 => "Glorious Model I Wireless".to_string(),
        0x0038 | 0x0039 => "Glorious Model O- Wireless".to_string(),
        0x003A | 0x003B => "Glorious Model D- Wireless".to_string(),
        _ => format!("Glorious Wireless Device ({:04x})", pid),
    }
}

impl BatteryCollector for GenericHidCollector {
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

            // Handle SteelSeries, Corsair, Glorious, or standard HID battery usage page (0x85 or 0x06)
            let is_steelseries = vid == STEELSERIES_VID;
            let is_corsair = vid == CORSAIR_VID;
            let is_glorious = vid == GLORIOUS_VID_1 || vid == GLORIOUS_VID_2;
            let is_std_battery_usage = dev_info.usage_page() == 0x0085
                || (dev_info.usage_page() == 0x0006 && dev_info.usage() == 0x0020);

            if !is_steelseries && !is_corsair && !is_glorious && !is_std_battery_usage {
                continue;
            }

            let serial = dev_info
                .serial_number()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .unwrap_or_else(|| dev_info.path().to_string_lossy().to_string());

            let id = format!("hid_{:04x}_{:04x}_{}", vid, pid, serial);
            if seen_ids.contains(&id) {
                continue;
            }

            let Ok(device) = dev_info.open_device(&api) else {
                continue;
            };

            let mut battery_opt = None;

            if is_steelseries {
                // SteelSeries: Query feature report 0x90 or 0x00
                let req = [0x00, 0x90];
                if device.send_feature_report(&req).is_ok() {
                    let mut resp = [0u8; 64];
                    resp[0] = 0x00;
                    if device.get_feature_report(&mut resp).is_ok() {
                        battery_opt = parse_steelseries_battery_report(&resp);
                    }
                }
            } else if is_corsair {
                // Corsair: Query feature report 0x08
                let req = [0x00, 0x08, 0x02];
                if device.send_feature_report(&req).is_ok() {
                    let mut resp = [0u8; 64];
                    resp[0] = 0x00;
                    if device.get_feature_report(&mut resp).is_ok() {
                        battery_opt = parse_corsair_battery_report(&resp);
                    }
                }
            } else if is_glorious {
                // Glorious: Query command 0x11
                let req = [0x00, 0x11];
                if device.send_feature_report(&req).is_ok() {
                    let mut resp = [0u8; 64];
                    resp[0] = 0x00;
                    if device.get_feature_report(&mut resp).is_ok() {
                        battery_opt = parse_glorious_battery_report(&resp);
                    }
                }
            } else if is_std_battery_usage {
                // Standard HID Battery usage: read feature report or quick input report
                let mut buf = [0u8; 64];
                if device.get_feature_report(&mut buf).is_ok() {
                    let val = if buf[0] == 0x00 && buf.len() >= 2 {
                        buf[1]
                    } else {
                        buf[0]
                    };
                    if val > 0 && val <= 100 {
                        battery_opt = Some((val, false));
                    }
                }
            }

            if let Some((percentage, is_charging)) = battery_opt {
                let name = if is_steelseries {
                    resolve_steelseries_name(pid, dev_info.product_string())
                } else if is_corsair {
                    resolve_corsair_name(pid, dev_info.product_string())
                } else if is_glorious {
                    resolve_glorious_name(pid, dev_info.product_string())
                } else {
                    dev_info
                        .product_string()
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .unwrap_or_else(|| format!("HID Device ({:04x}:{:04x})", vid, pid))
                };

                let device_type = DeviceType::guess_from_name(&name);
                let source = if is_steelseries {
                    BatterySource::SteelSeriesHid
                } else if is_corsair {
                    BatterySource::CorsairHid
                } else {
                    BatterySource::GenericHid
                };

                seen_ids.insert(id.clone());
                devices.push(DeviceBattery::new(
                    id,
                    name,
                    device_type,
                    percentage,
                    is_charging,
                    true,
                    source,
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
    fn test_parse_steelseries_battery_report() {
        let resp = [0x00, 0x90, 85, 0x00];
        assert_eq!(
            parse_steelseries_battery_report(&resp),
            Some((85, false))
        );

        let resp_charging = [0x90, 60, 0x01];
        assert_eq!(
            parse_steelseries_battery_report(&resp_charging),
            Some((60, true))
        );

        let invalid = [0x12, 50, 0x00];
        assert_eq!(parse_steelseries_battery_report(&invalid), None);
    }

    #[test]
    fn test_parse_corsair_battery_report() {
        let resp = [0x00, 0x08, 92, 0x01];
        assert_eq!(parse_corsair_battery_report(&resp), Some((92, false)));

        let resp_charging = [0x08, 45, 0x02];
        assert_eq!(parse_corsair_battery_report(&resp_charging), Some((45, true)));

        let invalid = [0x01, 50, 0x00];
        assert_eq!(parse_corsair_battery_report(&invalid), None);
    }

    #[test]
    fn test_parse_glorious_battery_report() {
        let resp = [0x00, 0x11, 78, 0x00];
        assert_eq!(parse_glorious_battery_report(&resp), Some((78, false)));

        let resp_charging = [0x04, 30, 0x01];
        assert_eq!(parse_glorious_battery_report(&resp_charging), Some((30, true)));
    }

    #[test]
    fn test_resolve_friendly_names() {
        assert_eq!(
            resolve_steelseries_name(0x1836, None),
            "SteelSeries Aerox 3 Wireless"
        );
        assert_eq!(
            resolve_corsair_name(0x1BA0, None),
            "Corsair Katar Pro Wireless"
        );
        assert_eq!(
            resolve_glorious_name(0x0029, None),
            "Glorious Model O Wireless"
        );
    }
}
