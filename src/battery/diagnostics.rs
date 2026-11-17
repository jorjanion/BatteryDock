use crate::battery::DeviceBattery;
use std::collections::BTreeSet;
use std::fmt::Write as _;

/// Generates an anonymized Markdown hardware diagnostic report (detected devices,
/// USB-HID VID/PID/UsagePage table, and Bluetooth PnP battery nodes) ready to paste
/// into a GitHub device-support issue.
pub fn build_hardware_diagnostics_report(detected_devices: &[DeviceBattery]) -> String {
    let mut out = String::with_capacity(2048);

    let _ = writeln!(out, "### BatteryDock Hardware Diagnostics");
    let _ = writeln!(out, "- **App Version:** v{}", env!("CARGO_PKG_VERSION"));
    let _ = writeln!(out, "- **OS:** {}", std::env::consts::OS);
    let _ = writeln!(out);

    let _ = writeln!(
        out,
        "#### 1. Detected Battery Devices ({})",
        detected_devices.len()
    );
    if detected_devices.is_empty() {
        let _ = writeln!(out, "- *(none)*");
    } else {
        for dev in detected_devices {
            let _ = writeln!(
                out,
                "- `{}` | type={:?} | source={} | battery={}% | charging={} | asleep={}",
                dev.name,
                dev.device_type,
                dev.source.display_name(),
                dev.percentage,
                dev.is_charging,
                dev.is_asleep
            );
        }
    }
    let _ = writeln!(out);

    // 2. USB-HID enumeration (deduplicated by VID/PID/UsagePage/Usage, no serial numbers)
    let _ = writeln!(out, "#### 2. USB-HID Interfaces (Anonymized)");
    match hidapi::HidApi::new() {
        Ok(api) => {
            let mut entries: BTreeSet<(u16, u16, u16, u16, String, String)> = BTreeSet::new();
            for info in api.device_list() {
                let mfg = info
                    .manufacturer_string()
                    .unwrap_or("")
                    .trim()
                    .to_string();
                let prod = info.product_string().unwrap_or("").trim().to_string();
                entries.insert((
                    info.vendor_id(),
                    info.product_id(),
                    info.usage_page(),
                    info.usage(),
                    mfg,
                    prod,
                ));
            }
            if entries.is_empty() {
                let _ = writeln!(out, "- *(no HID devices found)*");
            } else {
                let _ = writeln!(out, "```text");
                for (vid, pid, up, u, mfg, prod) in entries {
                    let label = match (mfg.is_empty(), prod.is_empty()) {
                        (false, false) => format!("{mfg} - {prod}"),
                        (false, true) => mfg,
                        (true, false) => prod,
                        (true, true) => "(unnamed)".to_string(),
                    };
                    let _ = writeln!(
                        out,
                        "VID_{vid:04X} PID_{pid:04X} | UsagePage=0x{up:04X} Usage=0x{u:04X} | {label}"
                    );
                }
                let _ = writeln!(out, "```");
            }
        }
        Err(e) => {
            let _ = writeln!(out, "- *(HID enumeration error: {e})*");
        }
    }

    let _ = writeln!(out);
    let _ = writeln!(out, "#### 3. Bluetooth & PnP Battery Nodes");
    let bt_lines = collect_bluetooth_diagnostic_lines();
    if bt_lines.is_empty() {
        let _ = writeln!(out, "- *(no Bluetooth battery nodes reported)*");
    } else {
        let _ = writeln!(out, "```text");
        for line in bt_lines {
            let _ = writeln!(out, "{line}");
        }
        let _ = writeln!(out, "```");
    }

    out
}

#[cfg(windows)]
fn collect_bluetooth_diagnostic_lines() -> Vec<String> {
    use windows::core::{Interface, HSTRING};
    use windows::Devices::Enumeration::{DeviceInformation, DeviceInformationKind};
    use windows::Foundation::IPropertyValue;

    let mut lines = Vec::new();
    let aqs = HSTRING::from("System.Devices.BatteryLife:>=0");
    let Ok(op) = DeviceInformation::FindAllAsyncWithKindAqsFilterAndAdditionalProperties(
        &aqs,
        None,
        DeviceInformationKind::Device,
    ) else {
        return lines;
    };
    let Ok(results) = op.get() else {
        return lines;
    };

    for dev in results {
        let name = dev.Name().map(|h| h.to_string()).unwrap_or_default();
        let id = dev.Id().map(|h| h.to_string()).unwrap_or_default();
        if !crate::battery::bluetooth::is_bluetooth_pnp_instance_id(&id) {
            continue;
        }
        let batt = dev
            .Properties()
            .ok()
            .and_then(|p| p.Lookup(&HSTRING::from("System.Devices.BatteryLife")).ok())
            .and_then(|v| v.cast::<IPropertyValue>().ok())
            .and_then(|pv| pv.GetUInt8().ok().or_else(|| pv.GetUInt32().ok().map(|x| x as u8)));

        // Redact trailing hardware instance/MAC segment after the second backslash for privacy
        let safe_id = redact_instance_id(&id);
        match batt {
            Some(pct) => lines.push(format!("{safe_id} | {name} | battery={pct}%")),
            None => lines.push(format!("{safe_id} | {name} | battery=?")),
        }
    }

    lines
}

#[cfg(not(windows))]
fn collect_bluetooth_diagnostic_lines() -> Vec<String> {
    Vec::new()
}

pub fn redact_instance_id(instance_id: &str) -> String {
    let mut parts = instance_id.split('\\');
    match (parts.next(), parts.next()) {
        (Some(p1), Some(p2)) => format!("{p1}\\{p2}\\<redacted>"),
        _ => instance_id.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battery::{BatterySource, DeviceType};

    #[test]
    fn test_redact_instance_id_hides_mac_suffix() {
        let raw = "BTHENUM\\{0000111E-0000-1000-8000-00805F9B34FB}_VID&0002054C_PID&0EAF\\A&31DAB596&0&00A41C94297E_C00000000";
        let redacted = redact_instance_id(raw);
        assert_eq!(
            redacted,
            "BTHENUM\\{0000111E-0000-1000-8000-00805F9B34FB}_VID&0002054C_PID&0EAF\\<redacted>"
        );
        assert!(!redacted.contains("00A41C94297E"));
    }

    #[test]
    fn test_build_hardware_diagnostics_report_contains_sections() {
        let dev = DeviceBattery::new(
            "dev_1",
            "Test Headset",
            DeviceType::Headset,
            72,
            false,
            true,
            BatterySource::LogitechHidpp,
        );
        let report = build_hardware_diagnostics_report(&[dev]);
        assert!(report.contains("### BatteryDock Hardware Diagnostics"));
        assert!(report.contains("Test Headset"));
        assert!(report.contains("battery=72%"));
        assert!(report.contains("#### 2. USB-HID Interfaces"));
        assert!(report.contains("#### 3. Bluetooth & PnP Battery Nodes"));
    }
}
