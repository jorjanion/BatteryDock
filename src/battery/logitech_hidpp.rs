use super::{BatteryCollector, BatterySource, DeviceBattery, DeviceType};
use hidapi::HidApi;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub struct LogitechHidppCollector;

impl LogitechHidppCollector {
    pub fn new() -> Self {
        Self
    }

    /// If Logitech G HUB is active on port 9010, queries real-time battery status directly
    fn poll_ghub_ws() -> Option<Vec<DeviceBattery>> {
        let mut stream = TcpStream::connect("127.0.0.1:9010").ok()?;
        stream.set_read_timeout(Some(Duration::from_millis(300))).ok()?;
        stream.set_write_timeout(Some(Duration::from_millis(300))).ok()?;

        let handshake = "GET / HTTP/1.1\r\n\
Host: 127.0.0.1:9010\r\n\
Upgrade: websocket\r\n\
Connection: Upgrade\r\n\
Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
Sec-WebSocket-Version: 13\r\n\
Sec-WebSocket-Protocol: json\r\n\
Origin: file://\r\n\r\n";

        stream.write_all(handshake.as_bytes()).ok()?;
        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf).ok()?;
        let resp = String::from_utf8_lossy(&buf[..n]);
        if !resp.contains("101 Switching Protocols") {
            return None;
        }

        let mut results = Vec::new();

        // Query standard G HUB slots dev00000000 through dev00000003
        for slot in 0..=3 {
            let dev_id = format!("dev{:08x}", slot);
            let bat_path = format!("/battery/{}/state", dev_id);
            let bat_msg = format!(r#"{{"msg":"subscribe","verb":"GET","path":"{}"}}"#, bat_path);
            let p = bat_msg.as_bytes();
            let mut f = vec![0x81, 0x80 | (p.len() as u8), 0, 0, 0, 0];
            f.extend_from_slice(p);
            let _ = stream.write_all(&f);

            let mut accumulated = Vec::new();
            let mut chunk = [0u8; 2048];
            for _ in 0..4 {
                if let Ok(bytes) = stream.read(&mut chunk) {
                    accumulated.extend_from_slice(&chunk[..bytes]);
                    let s = String::from_utf8_lossy(&accumulated);
                    if s.contains(&bat_path) {
                        if s.contains("\"code\": \"SUCCESS\"") && s.contains("\"percentage\":") {
                            let pct = s
                                .split("\"percentage\":")
                                .nth(1)
                                .and_then(|x| x.split(',').next())
                                .and_then(|x| x.trim().parse::<u8>().ok())
                                .unwrap_or(0);

                            let charging = s.contains("\"charging\": true");

                            // Guess name by slot or connected HID devices
                            let name = if slot == 0 {
                                "Logitech G733 Gaming Headset".to_string()
                            } else {
                                format!("Logitech Wireless Device #{}", slot + 1)
                            };

                            results.push(DeviceBattery {
                                id: format!("ghub_{}", dev_id),
                                name: name.clone(),
                                device_type: DeviceType::guess_from_name(&name),
                                percentage: pct,
                                is_charging: charging,
                                is_connected: true,
                                source: BatterySource::LogitechHidpp,
                            });
                        }
                        break;
                    }
                }
            }
        }

        if results.is_empty() {
            None
        } else {
            Some(results)
        }
    }

    /// Fallback query directly via Logitech HID++ 2.0 (e.g. when G HUB is not installed or closed)
    fn query_direct_hidpp() -> Vec<DeviceBattery> {
        let mut devices = Vec::new();
        let api = match HidApi::new() {
            Ok(api) => api,
            Err(_) => return devices,
        };

        for dev_info in api.device_list() {
            if dev_info.vendor_id() == 0x046D {
                let usage_page = dev_info.usage_page();
                if usage_page == 0xFF00 || usage_page == 0xFF43 {
                    if let Ok(device) = api.open_path(dev_info.path()) {
                        for dev_idx in [0xFF, 0x01, 0x02] {
                            let mut req = [0u8; 20];
                            req[0] = 0x11;
                            req[1] = dev_idx;
                            req[2] = 0x00;
                            req[3] = 0x00;
                            req[4] = 0x10;
                            req[5] = 0x04;

                            if device.write(&req).is_ok() {
                                let mut resp = [0u8; 20];
                                if let Ok(n) = device.read_timeout(&mut resp, 80) {
                                    if n >= 7 && resp[0] == 0x11 && resp[1] == dev_idx && resp[4] != 0 {
                                        let feat_idx = resp[4];
                                        let mut bat_req = [0u8; 20];
                                        bat_req[0] = 0x11;
                                        bat_req[1] = dev_idx;
                                        bat_req[2] = feat_idx;
                                        bat_req[3] = 0x00;

                                        if device.write(&bat_req).is_ok() {
                                            let mut bat_resp = [0u8; 20];
                                            if let Ok(bn) = device.read_timeout(&mut bat_resp, 80) {
                                                if bn >= 7 && bat_resp[0] == 0x11 {
                                                    let pct = bat_resp[4].min(100);
                                                    let charging = bat_resp[6] == 1 || bat_resp[6] == 2;
                                                    let name = dev_info
                                                        .product_string()
                                                        .map(|s| s.to_string())
                                                        .unwrap_or_else(|| "Logitech Wireless Device".to_string());

                                                    devices.push(DeviceBattery {
                                                        id: format!("logi_hid_{:04x}_{}", dev_info.product_id(), dev_idx),
                                                        name: name.clone(),
                                                        device_type: DeviceType::guess_from_name(&name),
                                                        percentage: pct,
                                                        is_charging: charging,
                                                        is_connected: true,
                                                        source: BatterySource::LogitechHidpp,
                                                    });
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        devices
    }
}

impl BatteryCollector for LogitechHidppCollector {
    fn poll_devices(&self) -> Vec<DeviceBattery> {
        // Try G HUB IPC first if G HUB is active
        if let Some(ghub_devs) = Self::poll_ghub_ws() {
            if !ghub_devs.is_empty() {
                return ghub_devs;
            }
        }

        // Fallback to direct standalone HID++ communication
        Self::query_direct_hidpp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logitech_collector_poll() {
        let collector = LogitechHidppCollector::new();
        let devs = collector.poll_devices();
        println!("Logitech Devices polled: {}", devs.len());
        for d in &devs {
            println!("  -> {} | Battery: {}% [charging: {}]", d.name, d.percentage, d.is_charging);
        }
        // Poll should run cleanly without panicking regardless of whether physical hardware is connected
    }
}
