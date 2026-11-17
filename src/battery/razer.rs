use super::{BatteryCollector, BatterySource, DeviceBattery, DeviceType};
use hidapi::HidApi;

pub const RAZER_VID: u16 = 0x1532;

pub struct RazerCollector;

impl RazerCollector {
    pub fn new() -> Self {
        Self
    }
}

/// Formats a 90-byte Razer report into a buffer (padded with a leading 0x00 Report ID for Windows `hidapi`).
/// Total buffer length is 91 bytes: `[report_id=0, status=0, transaction_id=0x1F, remaining_packets=0x0000,
/// protocol_type=0, data_size, command_class, command_id, arguments..., crc, reserved=0]`.
pub fn create_razer_request(command_class: u8, command_id: u8, data_size: u8, args: &[u8]) -> [u8; 91] {
    let mut buf = [0u8; 91];
    buf[0] = 0x00; // Report ID for Windows hidapi
    buf[1] = 0x00; // Status
    buf[2] = 0x1F; // Transaction ID
    buf[3] = 0x00; // Remaining packets MSB
    buf[4] = 0x00; // Remaining packets LSB
    buf[5] = 0x00; // Protocol type
    buf[6] = data_size;
    buf[7] = command_class;
    buf[8] = command_id;

    for (i, &b) in args.iter().enumerate().take(80) {
        buf[9 + i] = b;
    }

    // CRC is XOR of bytes 3 through 88 (indices 3..89 in 91-byte buffer)
    let mut crc = 0u8;
    for &b in &buf[3..89] {
        crc ^= b;
    }
    buf[89] = crc;
    buf[90] = 0x00; // Reserved

    buf
}

/// Parses the battery level response from a Razer device.
/// Handles both 91-byte (with 0x00 Report ID prefix) and 90-byte raw responses.
pub fn parse_razer_battery_response(buf: &[u8]) -> Option<u8> {
    let (status, class, cmd, val) = if buf.len() >= 91 && buf[0] == 0x00 {
        (buf[1], buf[7], buf[8], buf[10])
    } else if buf.len() >= 90 {
        (buf[0], buf[6], buf[7], buf[9])
    } else {
        return None;
    };

    // 0x02 indicates successful response from Razer MCU
    if status != 0x02 || class != 0x07 || cmd != 0x80 {
        return None;
    }

    if val == 0 {
        return Some(0);
    }

    // OpenRazer standard: if val <= 100, it's already a direct percentage (newer firmware);
    // if val > 100 (up to 255), scale: round(val * 100 / 255).
    let pct = if val <= 100 {
        val
    } else {
        ((val as u32 * 100 + 127) / 255) as u8
    };

    Some(pct.min(100))
}

/// Parses the charging status response from a Razer device.
pub fn parse_razer_charging_response(buf: &[u8]) -> Option<bool> {
    let (status, class, cmd, val) = if buf.len() >= 91 && buf[0] == 0x00 {
        (buf[1], buf[7], buf[8], buf[10])
    } else if buf.len() >= 90 {
        (buf[0], buf[6], buf[7], buf[9])
    } else {
        return None;
    };

    if status != 0x02 || class != 0x07 || cmd != 0x84 {
        return None;
    }

    Some(val == 0x01)
}

/// Resolves a human-friendly name for known Razer wireless devices when product_string is empty.
pub fn resolve_razer_name(pid: u16, product_string: Option<&str>) -> String {
    if let Some(s) = product_string {
        let trimmed = s.trim();
        if !trimmed.is_empty() && trimmed != "Razer" && trimmed != "Razer Device" {
            // Clean up dongle suffixes to look sleek
            let cleaned = trimmed
                .replace(" Dongle", "")
                .replace(" 2.4", "")
                .replace(" Transceiver", "");
            return cleaned;
        }
    }

    match pid {
        0x0078 | 0x007A => "Razer Viper Ultimate".to_string(),
        0x008A | 0x008C => "Razer DeathAdder V2 Pro".to_string(),
        0x0083 | 0x0088 => "Razer Basilisk Ultimate".to_string(),
        0x0090 | 0x0092 => "Razer Naga Pro".to_string(),
        0x0086 => "Razer BlackWidow V3 Pro".to_string(),
        0x00A5 | 0x00A6 => "Razer Viper V2 Pro".to_string(),
        0x00B6 | 0x00B7 => "Razer DeathAdder V3 Pro".to_string(),
        0x0099 | 0x009A => "Razer Basilisk V3 Pro".to_string(),
        0x00AB => "Razer Cobra Pro".to_string(),
        0x00B9 | 0x00BA => "Razer Viper V3 Pro".to_string(),
        0x00BC => "Razer DeathAdder V3 HyperSpeed".to_string(),
        0x025A => "Razer BlackShark V2 Pro".to_string(),
        0x0287 => "Razer BlackShark V2 Pro (2023)".to_string(),
        0x026D => "Razer Barracuda X".to_string(),
        0x0283 => "Razer Barracuda Pro".to_string(),
        0x008F => "Razer DeathStalker V2 Pro".to_string(),
        0x00A3 => "Razer DeathStalker V2 Pro TKL".to_string(),
        _ => format!("Razer Wireless Device ({:04x})", pid),
    }
}

impl BatteryCollector for RazerCollector {
    fn poll_devices(&self) -> Vec<DeviceBattery> {
        let mut devices = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();

        let api = match HidApi::new() {
            Ok(api) => api,
            Err(_) => return devices,
        };

        for dev_info in api.device_list() {
            if dev_info.vendor_id() != RAZER_VID {
                continue;
            }

            let pid = dev_info.product_id();
            let serial = dev_info
                .serial_number()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .unwrap_or_else(|| dev_info.path().to_string_lossy().to_string());

            let id = format!("razer_{:04x}_{}", pid, serial);
            if seen_ids.contains(&id) {
                continue;
            }

            // Attempt to open device and query battery status
            let Ok(device) = dev_info.open_device(&api) else {
                continue;
            };

            // Query battery level (class 0x07, command 0x80)
            let bat_req = create_razer_request(0x07, 0x80, 0x02, &[]);
            if device.send_feature_report(&bat_req).is_err() {
                continue;
            }

            std::thread::sleep(std::time::Duration::from_millis(10));

            let mut bat_resp = [0u8; 91];
            bat_resp[0] = 0x00; // Query report ID 0
            if device.get_feature_report(&mut bat_resp).is_err() {
                continue;
            }

            let Some(percentage) = parse_razer_battery_response(&bat_resp) else {
                continue;
            };

            // Query charging state (class 0x07, command 0x84)
            let mut is_charging = false;
            let chg_req = create_razer_request(0x07, 0x84, 0x02, &[]);
            if device.send_feature_report(&chg_req).is_ok() {
                std::thread::sleep(std::time::Duration::from_millis(5));
                let mut chg_resp = [0u8; 91];
                chg_resp[0] = 0x00;
                if device.get_feature_report(&mut chg_resp).is_ok() {
                    if let Some(chg) = parse_razer_charging_response(&chg_resp) {
                        is_charging = chg;
                    }
                }
            }

            let name = resolve_razer_name(pid, dev_info.product_string());
            let device_type = DeviceType::guess_from_name(&name);

            seen_ids.insert(id.clone());
            devices.push(DeviceBattery::new(
                id,
                name,
                device_type,
                percentage,
                is_charging,
                true,
                BatterySource::RazerHid,
            ));
        }

        devices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_razer_request_structure_and_checksum() {
        let req = create_razer_request(0x07, 0x80, 0x02, &[]);
        assert_eq!(req[0], 0x00); // Report ID
        assert_eq!(req[1], 0x00); // Status
        assert_eq!(req[2], 0x1F); // Transaction ID
        assert_eq!(req[6], 0x02); // Data size
        assert_eq!(req[7], 0x07); // Class Power
        assert_eq!(req[8], 0x80); // Command Get Battery

        // Checksum verification
        let mut expected_crc = 0u8;
        for &b in &req[3..89] {
            expected_crc ^= b;
        }
        assert_eq!(req[89], expected_crc);
        assert_eq!(req[90], 0x00);
    }

    #[test]
    fn test_parse_razer_battery_response_scaled_and_direct() {
        // Raw byte 255 -> 100%
        let mut resp = [0u8; 91];
        resp[0] = 0x00;
        resp[1] = 0x02; // Success
        resp[7] = 0x07; // Power
        resp[8] = 0x80; // Battery
        resp[10] = 255;
        assert_eq!(parse_razer_battery_response(&resp), Some(100));

        // Raw byte 128 -> 50%
        resp[10] = 128;
        assert_eq!(parse_razer_battery_response(&resp), Some(50));

        // Direct percentage (e.g. 75 <= 100)
        resp[10] = 75;
        assert_eq!(parse_razer_battery_response(&resp), Some(75));

        // Empty / Zero
        resp[10] = 0;
        assert_eq!(parse_razer_battery_response(&resp), Some(0));

        // Error status (0x01 = Device Busy or 0x03 = Failure)
        resp[1] = 0x03;
        assert_eq!(parse_razer_battery_response(&resp), None);
    }

    #[test]
    fn test_parse_razer_charging_response() {
        let mut resp = [0u8; 91];
        resp[0] = 0x00;
        resp[1] = 0x02;
        resp[7] = 0x07;
        resp[8] = 0x84; // Charging
        resp[10] = 0x01; // Charging
        assert_eq!(parse_razer_charging_response(&resp), Some(true));

        resp[10] = 0x00; // Not charging
        assert_eq!(parse_razer_charging_response(&resp), Some(false));
    }

    #[test]
    fn test_resolve_razer_name() {
        assert_eq!(
            resolve_razer_name(0x0078, None),
            "Razer Viper Ultimate"
        );
        assert_eq!(
            resolve_razer_name(0x00B6, None),
            "Razer DeathAdder V3 Pro"
        );
        assert_eq!(
            resolve_razer_name(0x9999, Some("Razer DeathAdder V3 Pro Dongle")),
            "Razer DeathAdder V3 Pro"
        );
    }
}
