use super::{BatteryCollector, BatterySource, DeviceBattery, DeviceType};
use std::process::Command;

#[allow(dead_code)]
pub struct AppleIokitCollector;

#[allow(dead_code)]
impl AppleIokitCollector {
    pub fn new() -> Self {
        Self
    }
}

/// Parses the output of `ioreg -r -c AppleDeviceManagementHIDEventService -l`
#[allow(dead_code)]
pub fn parse_ioreg_hid_output(output: &str) -> Vec<DeviceBattery> {
    let mut devices = Vec::new();
    let mut current_product: Option<String> = None;
    let mut current_address: Option<String> = None;
    let mut current_percent: Option<u8> = None;
    let mut current_left: Option<u8> = None;
    let mut current_right: Option<u8> = None;
    let mut current_case: Option<u8> = None;
    let mut current_charging: bool = false;

    let flush_device = |product: &Option<String>,
                        address: &Option<String>,
                        percent: &Option<u8>,
                        left: &Option<u8>,
                        right: &Option<u8>,
                        case: &Option<u8>,
                        charging: bool,
                        devs: &mut Vec<DeviceBattery>| {
        let name = product.as_deref().unwrap_or("Apple Accessory").trim();
        if name.is_empty() {
            return;
        }
        let addr = address.as_deref().unwrap_or(name);

        // Check if AirPods split channels are available
        let has_split = left.is_some() || right.is_some();
        if has_split {
            if let Some(pct) = left {
                devs.push(DeviceBattery::new(
                    format!("{}_left", addr),
                    format!("{} (Left)", name),
                    DeviceType::Headset,
                    *pct,
                    charging,
                    true,
                    BatterySource::AppleIokit,
                ));
            }
            if let Some(pct) = right {
                devs.push(DeviceBattery::new(
                    format!("{}_right", addr),
                    format!("{} (Right)", name),
                    DeviceType::Headset,
                    *pct,
                    charging,
                    true,
                    BatterySource::AppleIokit,
                ));
            }
            if let Some(pct) = case {
                devs.push(DeviceBattery::new(
                    format!("{}_case", addr),
                    format!("{} (Case)", name),
                    DeviceType::Headset,
                    *pct,
                    charging,
                    true,
                    BatterySource::AppleIokit,
                ));
            }
        } else if let Some(pct) = percent {
            let dev_type = DeviceType::guess_from_name(name);
            devs.push(DeviceBattery::new(
                addr,
                name,
                dev_type,
                *pct,
                charging,
                true,
                BatterySource::AppleIokit,
            ));
        }
    };

    for line in output.lines() {
        let trimmed = line.trim();

        // New object header in ioreg tree
        if trimmed.starts_with("+-o AppleDeviceManagementHIDEventService")
            || trimmed.starts_with("o AppleDeviceManagementHIDEventService")
        {
            flush_device(
                &current_product,
                &current_address,
                &current_percent,
                &current_left,
                &current_right,
                &current_case,
                current_charging,
                &mut devices,
            );
            current_product = None;
            current_address = None;
            current_percent = None;
            current_left = None;
            current_right = None;
            current_case = None;
            current_charging = false;
            continue;
        }

        if let Some((k, v)) = trimmed.split_once('=') {
            let key = k.trim().trim_matches('"');
            let val = v.trim().trim_matches('"');

            match key {
                "Product" => {
                    current_product = Some(val.to_string());
                }
                "DeviceAddress" => {
                    current_address = Some(val.to_string());
                }
                "BatteryPercent" => {
                    if let Ok(num) = val.parse::<u8>() {
                        if num <= 100 {
                            current_percent = Some(num);
                        }
                    }
                }
                "BatteryPercentLeft" => {
                    if let Ok(num) = val.parse::<u8>() {
                        if num <= 100 {
                            current_left = Some(num);
                        }
                    }
                }
                "BatteryPercentRight" => {
                    if let Ok(num) = val.parse::<u8>() {
                        if num <= 100 {
                            current_right = Some(num);
                        }
                    }
                }
                "BatteryPercentCase" => {
                    if let Ok(num) = val.parse::<u8>() {
                        if num <= 100 {
                            current_case = Some(num);
                        }
                    }
                }
                "IsCharging" => {
                    let v_lower = val.to_lowercase();
                    current_charging = v_lower == "yes" || v_lower == "true" || v_lower == "1";
                }
                _ => {}
            }
        }
    }

    // Flush last object
    flush_device(
        &current_product,
        &current_address,
        &current_percent,
        &current_left,
        &current_right,
        &current_case,
        current_charging,
        &mut devices,
    );

    devices
}

/// Parses the output of `ioreg -r -c IOBluetoothDevice -l` for third-party Bluetooth accessories
#[allow(dead_code)]
pub fn parse_ioreg_bluetooth_output(output: &str) -> Vec<DeviceBattery> {
    let mut devices = Vec::new();
    let mut current_name: Option<String> = None;
    let mut current_address: Option<String> = None;
    let mut current_percent: Option<u8> = None;
    let mut is_connected = false;

    let flush_bt = |name: &Option<String>,
                    address: &Option<String>,
                    percent: &Option<u8>,
                    connected: bool,
                    devs: &mut Vec<DeviceBattery>| {
        if !connected {
            return;
        }
        if let (Some(n), Some(pct)) = (name, percent) {
            let addr = address.as_deref().unwrap_or(n.as_str());
            let dev_type = DeviceType::guess_from_name(n);
            devs.push(DeviceBattery::new(
                addr,
                n,
                dev_type,
                *pct,
                false,
                true,
                BatterySource::AppleIokit,
            ));
        }
    };

    for line in output.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("+-o IOBluetoothDevice") || trimmed.starts_with("o IOBluetoothDevice")
        {
            flush_bt(
                &current_name,
                &current_address,
                &current_percent,
                is_connected,
                &mut devices,
            );
            current_name = None;
            current_address = None;
            current_percent = None;
            is_connected = false;
            continue;
        }

        if let Some((k, v)) = trimmed.split_once('=') {
            let key = k.trim().trim_matches('"');
            let val = v.trim().trim_matches('"');

            match key {
                "BluetoothDeviceName" | "Name" => {
                    current_name = Some(val.to_string());
                }
                "BluetoothDeviceAddress" | "Address" => {
                    current_address = Some(val.to_string());
                }
                "BatteryPercent" => {
                    if let Ok(pct) = val.parse::<u8>() {
                        if pct <= 100 {
                            current_percent = Some(pct);
                        }
                    }
                }
                "Connected" => {
                    let v_lower = val.to_lowercase();
                    is_connected = v_lower == "yes" || v_lower == "true" || v_lower == "1";
                }
                _ => {}
            }
        }
    }

    flush_bt(
        &current_name,
        &current_address,
        &current_percent,
        is_connected,
        &mut devices,
    );

    devices
}

impl BatteryCollector for AppleIokitCollector {
    fn poll_devices(&self) -> Vec<DeviceBattery> {
        let mut results = Vec::new();

        // 1. Query AppleDeviceManagementHIDEventService (AirPods, Magic Mouse, Magic Keyboard, Beats)
        if let Ok(out) = Command::new("/usr/sbin/ioreg")
            .args(["-r", "-c", "AppleDeviceManagementHIDEventService", "-l"])
            .output()
        {
            if out.status.success() {
                if let Ok(text) = String::from_utf8(out.stdout) {
                    let devs = parse_ioreg_hid_output(&text);
                    results.extend(devs);
                }
            }
        }

        // 2. Query IOBluetoothDevice for third-party Bluetooth accessories
        if let Ok(out) = Command::new("/usr/sbin/ioreg")
            .args(["-r", "-c", "IOBluetoothDevice", "-l"])
            .output()
        {
            if out.status.success() {
                if let Ok(text) = String::from_utf8(out.stdout) {
                    let devs = parse_ioreg_bluetooth_output(&text);
                    for d in devs {
                        if !results.iter().any(|r| r.id == d.id || r.name == d.name) {
                            results.push(d);
                        }
                    }
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ioreg_magic_mouse_and_keyboard() {
        let ioreg_sample = r#"
+-o AppleDeviceManagementHIDEventService  <class AppleDeviceManagementHIDEventService, id 0x100000a, registered, matched, active>
  {
    "BatteryPercent" = 82
    "DeviceAddress" = "60-c5-47-11-22-33"
    "IsCharging" = No
    "Product" = "Magic Mouse"
    "PrimaryUsage" = 2
  }
+-o AppleDeviceManagementHIDEventService  <class AppleDeviceManagementHIDEventService, id 0x100000b, registered, matched, active>
  {
    "BatteryPercent" = 45
    "DeviceAddress" = "60-c5-47-44-55-66"
    "IsCharging" = Yes
    "Product" = "Magic Keyboard with Touch ID"
    "PrimaryUsage" = 6
  }
"#;
        let devices = parse_ioreg_hid_output(ioreg_sample);
        assert_eq!(devices.len(), 2);

        assert_eq!(devices[0].name, "Magic Mouse");
        assert_eq!(devices[0].percentage, 82);
        assert!(!devices[0].is_charging);
        assert_eq!(devices[0].device_type, DeviceType::Mouse);

        assert_eq!(devices[1].name, "Magic Keyboard with Touch ID");
        assert_eq!(devices[1].percentage, 45);
        assert!(devices[1].is_charging);
        assert_eq!(devices[1].device_type, DeviceType::Keyboard);
    }

    #[test]
    fn test_parse_ioreg_airpods_pro_channels() {
        let airpods_sample = r#"
+-o AppleDeviceManagementHIDEventService  <class AppleDeviceManagementHIDEventService, id 0x100000c>
  {
    "BatteryPercentLeft" = 90
    "BatteryPercentRight" = 85
    "BatteryPercentCase" = 100
    "DeviceAddress" = "9c-20-7b-aa-bb-cc"
    "IsCharging" = No
    "Product" = "AirPods Pro"
  }
"#;
        let devices = parse_ioreg_hid_output(airpods_sample);
        assert_eq!(devices.len(), 3);

        let left = devices.iter().find(|d| d.name.contains("(Left)")).unwrap();
        assert_eq!(left.percentage, 90);
        assert_eq!(left.device_type, DeviceType::Headset);

        let right = devices.iter().find(|d| d.name.contains("(Right)")).unwrap();
        assert_eq!(right.percentage, 85);
        assert_eq!(right.device_type, DeviceType::Headset);

        let case = devices.iter().find(|d| d.name.contains("(Case)")).unwrap();
        assert_eq!(case.percentage, 100);
        assert_eq!(case.device_type, DeviceType::Headset);
    }

    #[test]
    fn test_parse_ioreg_bluetooth_generic() {
        let bt_sample = r#"
+-o IOBluetoothDevice  <class IOBluetoothDevice>
  {
    "BluetoothDeviceName" = "Sony WH-1000XM4"
    "BluetoothDeviceAddress" = "ac-80-0a-11-22-33"
    "BatteryPercent" = 70
    "Connected" = Yes
  }
+-o IOBluetoothDevice  <class IOBluetoothDevice>
  {
    "BluetoothDeviceName" = "Disconnected Device"
    "BatteryPercent" = 50
    "Connected" = No
  }
"#;
        let devices = parse_ioreg_bluetooth_output(bt_sample);
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].name, "Sony WH-1000XM4");
        assert_eq!(devices[0].percentage, 70);
        assert_eq!(devices[0].device_type, DeviceType::Headset);
    }
}
