use super::{BatteryCollector, BatterySource, DeviceBattery, DeviceType};
use hidapi::{HidApi, HidDevice};
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

pub struct LogitechHidppCollector;

/// Encodes a client-masked RFC 6455 text frame (supports payloads up to 64KB+).
pub fn ws_encode_text_frame(payload: &str) -> Vec<u8> {
    let bytes = payload.as_bytes();
    let len = bytes.len();
    let mask = [0x37u8, 0xFA, 0x21, 0x9C];

    let mut frame = Vec::with_capacity(len + 14);
    frame.push(0x81); // FIN + text frame
    if len < 126 {
        frame.push(0x80 | (len as u8));
    } else if len <= 65535 {
        frame.push(0x80 | 126);
        frame.extend_from_slice(&(len as u16).to_be_bytes());
    } else {
        frame.push(0x80 | 127);
        frame.extend_from_slice(&(len as u64).to_be_bytes());
    }
    frame.extend_from_slice(&mask);
    for (i, &b) in bytes.iter().enumerate() {
        frame.push(b ^ mask[i & 3]);
    }
    frame
}

/// Decodes one complete RFC 6455 frame from `buf`, returning `(opcode, payload, consumed_bytes)`.
pub fn ws_try_decode_frame(buf: &[u8]) -> Option<(u8, Vec<u8>, usize)> {
    if buf.len() < 2 {
        return None;
    }
    let opcode = buf[0] & 0x0F;
    let masked = (buf[1] & 0x80) != 0;
    let mut offset = 2usize;

    let payload_len: usize = match buf[1] & 0x7F {
        n if n < 126 => n as usize,
        126 => {
            if buf.len() < offset + 2 {
                return None;
            }
            let v = u16::from_be_bytes([buf[offset], buf[offset + 1]]) as usize;
            offset += 2;
            v
        }
        _ => {
            if buf.len() < offset + 8 {
                return None;
            }
            let v = u64::from_be_bytes([
                buf[offset],
                buf[offset + 1],
                buf[offset + 2],
                buf[offset + 3],
                buf[offset + 4],
                buf[offset + 5],
                buf[offset + 6],
                buf[offset + 7],
            ]) as usize;
            offset += 8;
            v
        }
    };

    let mask_key = if masked {
        if buf.len() < offset + 4 {
            return None;
        }
        let k = [
            buf[offset],
            buf[offset + 1],
            buf[offset + 2],
            buf[offset + 3],
        ];
        offset += 4;
        Some(k)
    } else {
        None
    };

    if buf.len() < offset + payload_len {
        return None;
    }

    let mut payload = buf[offset..offset + payload_len].to_vec();
    if let Some(k) = mask_key {
        for (i, b) in payload.iter_mut().enumerate() {
            *b ^= k[i & 3];
        }
    }
    Some((opcode, payload, offset + payload_len))
}

fn ws_read_matching_json(
    stream: &mut TcpStream,
    rx_buf: &mut Vec<u8>,
    expected_path: &str,
    timeout: Duration,
) -> Option<String> {
    let deadline = Instant::now() + timeout;
    let mut chunk = [0u8; 4096];

    loop {
        // First drain any already-buffered frames
        loop {
            let Some((opcode, payload, consumed)) = ws_try_decode_frame(rx_buf) else {
                break;
            };
            rx_buf.drain(..consumed);

            match opcode {
                0x8 => return None, // Connection close
                0x9 => {
                    // Respond to Ping with masked Pong
                    let mut pong = vec![0x8A, 0x80 | (payload.len().min(125) as u8), 0, 0, 0, 0];
                    pong.extend_from_slice(&payload[..payload.len().min(125)]);
                    let _ = stream.write_all(&pong);
                }
                0x1 | 0x2 | 0x0 => {
                    let text = String::from_utf8_lossy(&payload).to_string();
                    if text.contains(expected_path) {
                        return Some(text);
                    }
                }
                _ => {}
            }
        }

        if Instant::now() >= deadline {
            return None;
        }

        match stream.read(&mut chunk) {
            Ok(0) => return None,
            Ok(n) => rx_buf.extend_from_slice(&chunk[..n]),
            Err(e)
                if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut =>
            {
                if Instant::now() >= deadline {
                    return None;
                }
            }
            Err(_) => return None,
        }
    }
}

fn find_json_field_value_start<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    let needle = format!("\"{}\"", key);
    let mut search_from = 0usize;
    while let Some(rel) = json[search_from..].find(&needle) {
        let after_key = search_from + rel + needle.len();
        let rest = json[after_key..].trim_start();
        if let Some(after_colon) = rest.strip_prefix(':') {
            return Some(after_colon.trim_start());
        }
        search_from = after_key;
    }
    None
}

fn parse_json_string_field(json: &str, key: &str) -> Option<String> {
    let val = find_json_field_value_start(json, key)?;
    let inner = val.strip_prefix('"')?;
    let mut out = String::new();
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => return Some(out),
            '\\' => {
                if let Some(esc) = chars.next() {
                    out.push(match esc {
                        '"' => '"',
                        '\\' => '\\',
                        '/' => '/',
                        'n' => '\n',
                        't' => '\t',
                        other => other,
                    });
                }
            }
            other => out.push(other),
        }
    }
    None
}

fn parse_json_bool_field(json: &str, key: &str) -> Option<bool> {
    let val = find_json_field_value_start(json, key)?;
    if val.starts_with("true") {
        Some(true)
    } else if val.starts_with("false") {
        Some(false)
    } else {
        None
    }
}

fn parse_json_number_u8(json: &str, key: &str) -> Option<u8> {
    let val = find_json_field_value_start(json, key)?;
    let end = val
        .find(|c: char| !(c.is_ascii_digit() || c == '.'))
        .unwrap_or(val.len());
    if end == 0 {
        return None;
    }
    let num: f32 = val[..end].parse().ok()?;
    Some(num.round().clamp(0.0, 100.0) as u8)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GhubDeviceInfo {
    pub id: String,
    pub name: String,
    pub has_battery: bool,
}

/// Splits top-level device objects inside G HUB `/devices/list` JSON and extracts id, display name,
/// and whether the device supports battery reporting.
pub fn parse_ghub_devices_list(json: &str) -> Vec<GhubDeviceInfo> {
    let mut devices = Vec::new();
    let mut seen_ids = std::collections::HashSet::new();

    // Locate `"deviceInfos"` array if present, otherwise scan from start
    let slice = if let Some(val) = find_json_field_value_start(json, "deviceInfos") {
        val
    } else {
        json
    };

    let bytes = slice.as_bytes();
    let mut i = 0usize;
    let mut in_str = false;
    let mut escaped = false;
    let mut depth = 0i32;
    let mut obj_start: Option<usize> = None;

    while i < bytes.len() {
        let b = bytes[i];
        if in_str {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'"' {
                in_str = false;
            }
        } else {
            match b {
                b'"' => in_str = true,
                b'{' => {
                    if depth == 0 {
                        obj_start = Some(i);
                    }
                    depth += 1;
                }
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        if let Some(start) = obj_start.take() {
                            let obj = &slice[start..=i];
                            if let Some(id) = parse_json_string_field(obj, "id") {
                                if id.starts_with("dev") && seen_ids.insert(id.clone()) {
                                    let raw_name = parse_json_string_field(obj, "extendedDisplayName")
                                        .or_else(|| parse_json_string_field(obj, "displayName"))
                                        .unwrap_or_else(|| format!("Logitech Device {}", id));
                                    let name = if raw_name.to_ascii_lowercase().contains("logitech") {
                                        raw_name
                                    } else {
                                        format!("Logitech {}", raw_name)
                                    };
                                    let has_battery =
                                        parse_json_bool_field(obj, "hasBatteryStatus").unwrap_or(true);
                                    devices.push(GhubDeviceInfo {
                                        id,
                                        name,
                                        has_battery,
                                    });
                                }
                            }
                        }
                    }
                }
                b']' if depth == 0 && obj_start.is_none() => break,
                _ => {}
            }
        }
        i += 1;
    }

    devices
}

/// Parses a G HUB `/battery/{id}/state` JSON response, returning `Some((percentage, is_charging))`.
pub fn parse_ghub_battery_state(json: &str, expected_path: &str) -> Option<(u8, bool)> {
    if !json.contains(expected_path) {
        return None;
    }
    let code = parse_json_string_field(json, "code")?;
    if !code.eq_ignore_ascii_case("SUCCESS") {
        return None;
    }
    let pct = parse_json_number_u8(json, "percentage")?;
    let charging = parse_json_bool_field(json, "charging").unwrap_or(false);
    Some((pct, charging))
}

/// Validates and parses an HID++ 2.0 battery status reply (`0x1004` get_status or `0x1000` getBatteryLevelStatus).
pub fn parse_hidpp_battery_response(
    resp: &[u8],
    expected_dev_idx: u8,
    expected_feat_idx: u8,
    expected_sw_id: u8,
) -> Option<(u8, bool)> {
    if resp.len() < 7 {
        return None;
    }
    if resp[0] != 0x11 || resp[1] != expected_dev_idx || resp[2] != expected_feat_idx {
        return None;
    }
    if (resp[3] & 0x0F) != (expected_sw_id & 0x0F) {
        return None;
    }
    let pct = resp[4].min(100);
    let charging = matches!(resp[6], 1 | 2 | 4);
    Some((pct, charging))
}

fn query_hidpp_feature_index(
    device: &HidDevice,
    dev_idx: u8,
    feature_hi: u8,
    feature_lo: u8,
    sw_id: u8,
) -> Option<u8> {
    let mut req = [0u8; 20];
    req[0] = 0x11;
    req[1] = dev_idx;
    req[2] = 0x00; // Root feature 0x0000
    req[3] = sw_id & 0x0F; // Function 0: getFeature(featureID)
    req[4] = feature_hi;
    req[5] = feature_lo;

    device.write(&req).ok()?;
    let mut resp = [0u8; 20];
    for _ in 0..5 {
        let Ok(n) = device.read_timeout(&mut resp, 40) else {
            break;
        };
        if n < 7 || resp[0] != 0x11 || resp[1] != dev_idx {
            continue;
        }
        if resp[2] == 0xFF {
            return None; // HID++ error response
        }
        if resp[2] == 0x00 && (resp[3] & 0x0F) == (sw_id & 0x0F) {
            return if resp[4] != 0 { Some(resp[4]) } else { None };
        }
    }
    None
}

fn query_hidpp_battery_feature(
    device: &HidDevice,
    dev_idx: u8,
    feat_idx: u8,
    function_id: u8,
    sw_id: u8,
) -> Option<(u8, bool)> {
    let mut req = [0u8; 20];
    req[0] = 0x11;
    req[1] = dev_idx;
    req[2] = feat_idx;
    req[3] = ((function_id & 0x0F) << 4) | (sw_id & 0x0F);

    device.write(&req).ok()?;
    let mut resp = [0u8; 20];
    for _ in 0..5 {
        let Ok(n) = device.read_timeout(&mut resp, 40) else {
            break;
        };
        if n < 7 || resp[0] != 0x11 || resp[1] != dev_idx {
            continue;
        }
        if resp[2] == 0xFF {
            return None;
        }
        if let Some(parsed) = parse_hidpp_battery_response(&resp[..n], dev_idx, feat_idx, sw_id) {
            return Some(parsed);
        }
    }
    None
}

impl LogitechHidppCollector {
    pub fn new() -> Self {
        Self
    }

    /// Queries Logitech G HUB's local WebSocket IPC server on port 9010 if active.
    fn poll_ghub_ws() -> Option<Vec<DeviceBattery>> {
        let mut stream = TcpStream::connect("127.0.0.1:9010").ok()?;
        stream.set_read_timeout(Some(Duration::from_millis(250))).ok()?;
        stream.set_write_timeout(Some(Duration::from_millis(250))).ok()?;

        let handshake = "GET / HTTP/1.1\r\n\
Host: 127.0.0.1:9010\r\n\
Upgrade: websocket\r\n\
Connection: Upgrade\r\n\
Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
Sec-WebSocket-Version: 13\r\n\
Sec-WebSocket-Protocol: json\r\n\
Origin: file://\r\n\r\n";

        stream.write_all(handshake.as_bytes()).ok()?;

        // Read until end of HTTP 101 headers cleanly, keeping any trailing WS bytes in rx_buf
        let mut raw = Vec::with_capacity(2048);
        let mut tmp = [0u8; 1024];
        let header_end;
        loop {
            let n = stream.read(&mut tmp).ok()?;
            if n == 0 {
                return None;
            }
            raw.extend_from_slice(&tmp[..n]);
            if let Some(pos) = raw.windows(4).position(|w| w == b"\r\n\r\n") {
                header_end = pos + 4;
                break;
            }
            if raw.len() > 8192 {
                return None;
            }
        }

        let headers = String::from_utf8_lossy(&raw[..header_end]);
        if !headers.contains("101 Switching Protocols") {
            return None;
        }

        let mut rx_buf = raw[header_end..].to_vec();

        // 1. Query actual connected/known devices from /devices/list
        let list_req = r#"{"verb":"GET","path":"/devices/list"}"#;
        stream.write_all(&ws_encode_text_frame(list_req)).ok()?;

        let dev_infos = if let Some(list_json) =
            ws_read_matching_json(&mut stream, &mut rx_buf, "/devices/list", Duration::from_millis(500))
        {
            parse_ghub_devices_list(&list_json)
        } else {
            Vec::new()
        };

        let query_targets: Vec<GhubDeviceInfo> = if dev_infos.is_empty() {
            (0..=3)
                .map(|slot| GhubDeviceInfo {
                    id: format!("dev{:08x}", slot),
                    name: format!("Logitech Wireless Device #{}", slot + 1),
                    has_battery: true,
                })
                .collect()
        } else {
            dev_infos
                .into_iter()
                .filter(|d| d.has_battery)
                .collect()
        };

        // 2. Query /battery/{id}/state for each battery-capable device
        let mut results = Vec::new();
        for target in query_targets {
            let bat_path = format!("/battery/{}/state", target.id);
            let bat_req = format!(r#"{{"verb":"GET","path":"{}"}}"#, bat_path);
            if stream.write_all(&ws_encode_text_frame(&bat_req)).is_err() {
                break;
            }
            if let Some(bat_json) = ws_read_matching_json(
                &mut stream,
                &mut rx_buf,
                &bat_path,
                Duration::from_millis(400),
            ) {
                if let Some((pct, charging)) = parse_ghub_battery_state(&bat_json, &bat_path) {
                    results.push(DeviceBattery {
                        id: format!("ghub_{}", target.id),
                        name: target.name.clone(),
                        device_type: DeviceType::guess_from_name(&target.name),
                        percentage: pct,
                        is_charging: charging,
                        is_connected: true,
                        source: BatterySource::LogitechHidpp,
                    });
                }
            }
        }

        if results.is_empty() {
            None
        } else {
            Some(results)
        }
    }

    /// Fallback query directly via Logitech HID++ 2.0 (when G HUB is closed or not installed).
    fn query_direct_hidpp() -> Vec<DeviceBattery> {
        let mut devices = Vec::new();
        let api = match HidApi::new() {
            Ok(api) => api,
            Err(_) => return devices,
        };

        let mut seen_keys = std::collections::HashSet::new();

        for dev_info in api.device_list() {
            if dev_info.vendor_id() != 0x046D {
                continue;
            }
            let usage_page = dev_info.usage_page();
            if usage_page != 0xFF00 && usage_page != 0xFF43 {
                continue;
            }

            let Ok(device) = api.open_path(dev_info.path()) else {
                continue;
            };

            let sw_id = 0x01;
            let base_name = dev_info
                .product_string()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "Logitech Wireless Device".to_string());

            for dev_idx in [0xFFu8, 0x01, 0x02, 0x03, 0x04] {
                let dedup_key = (dev_info.product_id(), dev_idx);
                if seen_keys.contains(&dedup_key) {
                    continue;
                }

                // Prefer Unified Battery 0x1004 (function 1 = get_status), fallback 0x1000 (function 0)
                let battery_opt = if let Some(feat_1004) =
                    query_hidpp_feature_index(&device, dev_idx, 0x10, 0x04, sw_id)
                {
                    query_hidpp_battery_feature(&device, dev_idx, feat_1004, 1, sw_id)
                } else if let Some(feat_1000) =
                    query_hidpp_feature_index(&device, dev_idx, 0x10, 0x00, sw_id)
                {
                    query_hidpp_battery_feature(&device, dev_idx, feat_1000, 0, sw_id)
                } else {
                    None
                };

                if let Some((pct, charging)) = battery_opt {
                    seen_keys.insert(dedup_key);
                    devices.push(DeviceBattery {
                        id: format!("logi_hid_{:04x}_{}", dev_info.product_id(), dev_idx),
                        name: base_name.clone(),
                        device_type: DeviceType::guess_from_name(&base_name),
                        percentage: pct,
                        is_charging: charging,
                        is_connected: true,
                        source: BatterySource::LogitechHidpp,
                    });
                    if dev_idx == 0xFF {
                        break; // Direct dongle device responded on 0xFF
                    }
                }
            }
        }

        devices
    }
}

impl BatteryCollector for LogitechHidppCollector {
    fn poll_devices(&self) -> Vec<DeviceBattery> {
        if let Some(ghub_devs) = Self::poll_ghub_ws() {
            if !ghub_devs.is_empty() {
                return ghub_devs;
            }
        }
        Self::query_direct_hidpp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_frame_encode_and_decode_roundtrip() {
        let msg = r#"{"verb":"GET","path":"/devices/list"}"#;
        let encoded = ws_encode_text_frame(msg);
        let (opcode, payload, consumed) = ws_try_decode_frame(&encoded).expect("Decode failed");
        assert_eq!(opcode, 0x1);
        assert_eq!(consumed, encoded.len());
        assert_eq!(String::from_utf8(payload).unwrap(), msg);

        // Partial frame should return None until rest arrives
        assert!(ws_try_decode_frame(&encoded[..encoded.len() - 1]).is_none());

        // Extended 16-bit payload (>125 bytes)
        let long_msg = "x".repeat(300);
        let long_enc = ws_encode_text_frame(&long_msg);
        let (op2, pay2, cons2) = ws_try_decode_frame(&long_enc).expect("Extended decode failed");
        assert_eq!(op2, 0x1);
        assert_eq!(cons2, long_enc.len());
        assert_eq!(pay2.len(), 300);
    }

    #[test]
    fn test_parse_ghub_devices_list_and_battery_state() {
        let list_json = r#"{
            "path": "/devices/list",
            "payload": {
                "deviceInfos": [
                    {
                        "id": "dev00000000",
                        "extendedDisplayName": "G733 Gaming Headset",
                        "capabilities": { "hasBatteryStatus": true },
                        "state": "NOT_CONNECTED"
                    },
                    {
                        "id": "dev00000001",
                        "extendedDisplayName": "G502 HERO Gaming Mouse",
                        "capabilities": { "hasBatteryStatus": false },
                        "state": "ACTIVE"
                    },
                    {
                        "id": "dev00000002",
                        "displayName": "Logitech G PRO X SUPERLIGHT",
                        "capabilities": { "hasBatteryStatus": true },
                        "state": "ACTIVE"
                    }
                ]
            },
            "result": { "code": "SUCCESS" }
        }"#;

        let parsed = parse_ghub_devices_list(list_json);
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[0].id, "dev00000000");
        assert_eq!(parsed[0].name, "Logitech G733 Gaming Headset");
        assert!(parsed[0].has_battery);
        assert!(!parsed[1].has_battery);
        assert_eq!(parsed[2].name, "Logitech G PRO X SUPERLIGHT");
        assert!(parsed[2].has_battery);

        let charging_json = r#"{"path":"/battery/dev00000000/state","payload":{"charging":true,"percentage":64.0},"result":{"code":"SUCCESS"}}"#;
        assert_eq!(
            parse_ghub_battery_state(charging_json, "/battery/dev00000000/state"),
            Some((64, true))
        );

        let discharging_json = r#"{"path": "/battery/dev00000000/state", "payload": {"charging": false, "percentage": 27}, "result": {"code": "SUCCESS"}}"#;
        assert_eq!(
            parse_ghub_battery_state(discharging_json, "/battery/dev00000000/state"),
            Some((27, false))
        );

        assert_eq!(
            parse_ghub_battery_state(discharging_json, "/battery/dev00000002/state"),
            None
        );
    }

    #[test]
    fn test_parse_hidpp_battery_response_validation() {
        // Valid 0x1004 / 0x1000 reply on feat 0x05, dev 0xFF, sw_id 1: 78%, charging (1)
        let valid = [0x11u8, 0xFF, 0x05, 0x11, 78, 0, 1];
        assert_eq!(parse_hidpp_battery_response(&valid, 0xFF, 0x05, 1), Some((78, true)));

        // Discharging status (0)
        let discharging = [0x11u8, 0xFF, 0x05, 0x11, 42, 0, 0];
        assert_eq!(parse_hidpp_battery_response(&discharging, 0xFF, 0x05, 1), Some((42, false)));

        // HID++ error frame (feat = 0xFF) must be rejected
        let err_frame = [0x11u8, 0xFF, 0xFF, 0x11, 0x08, 0, 0];
        assert_eq!(parse_hidpp_battery_response(&err_frame, 0xFF, 0x05, 1), None);

        // Unrelated notification feature must be rejected
        let wrong_feat = [0x11u8, 0xFF, 0x02, 0x11, 55, 0, 1];
        assert_eq!(parse_hidpp_battery_response(&wrong_feat, 0xFF, 0x05, 1), None);
    }

    #[test]
    fn test_logitech_collector_poll() {
        let collector = LogitechHidppCollector::new();
        let devs = collector.poll_devices();
        println!("Logitech Devices polled: {}", devs.len());
        for d in &devs {
            println!(
                "  -> {} | Battery: {}% [charging: {}]",
                d.name, d.percentage, d.is_charging
            );
        }
    }
}
