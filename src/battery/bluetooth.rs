use super::{BatteryCollector, BatterySource, DeviceBattery, DeviceType};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use windows::core::{implement, ComObject, Interface, HSTRING};
use windows::Devices::Bluetooth::{BluetoothDevice, BluetoothLEDevice};
use windows::Devices::Enumeration::{DeviceInformation, DeviceInformationKind};
use windows::Foundation::Collections::{IIterable, IIterable_Impl, IIterator, IIterator_Impl};
use windows::Foundation::IPropertyValue;
use windows::Win32::Foundation::E_BOUNDS;

#[implement(IIterable<HSTRING>)]
struct HstringIterable {
    values: Vec<HSTRING>,
}

impl IIterable_Impl<HSTRING> for HstringIterable_Impl {
    fn First(&self) -> windows::core::Result<IIterator<HSTRING>> {
        Ok(ComObject::new(HstringIterator {
            values: self.values.clone(),
            index: AtomicUsize::new(0),
        })
        .into_interface())
    }
}

#[implement(IIterator<HSTRING>)]
struct HstringIterator {
    values: Vec<HSTRING>,
    index: AtomicUsize,
}

impl IIterator_Impl<HSTRING> for HstringIterator_Impl {
    fn Current(&self) -> windows::core::Result<HSTRING> {
        let idx = self.index.load(Ordering::Relaxed);
        self.values
            .get(idx)
            .cloned()
            .ok_or_else(|| windows::core::Error::from(E_BOUNDS))
    }

    fn HasCurrent(&self) -> windows::core::Result<bool> {
        Ok(self.index.load(Ordering::Relaxed) < self.values.len())
    }

    fn MoveNext(&self) -> windows::core::Result<bool> {
        let cur = self.index.load(Ordering::Relaxed);
        if cur < self.values.len() {
            self.index.store(cur + 1, Ordering::Relaxed);
        }
        Ok(cur + 1 < self.values.len())
    }

    fn GetMany(&self, items: &mut [HSTRING]) -> windows::core::Result<u32> {
        let cur = self.index.load(Ordering::Relaxed);
        let avail = self.values.len().saturating_sub(cur);
        let count = avail.min(items.len());
        for (dst, src) in items[..count]
            .iter_mut()
            .zip(&self.values[cur..cur + count])
        {
            *dst = src.clone();
        }
        self.index.store(cur + count, Ordering::Relaxed);
        Ok(count as u32)
    }
}

fn make_hstring_iterable(keys: &[&str]) -> IIterable<HSTRING> {
    ComObject::new(HstringIterable {
        values: keys.iter().map(|&k| HSTRING::from(k)).collect(),
    })
    .into_interface()
}

pub struct BluetoothBatteryCollector;

impl BluetoothBatteryCollector {
    pub fn new() -> Self {
        Self
    }
}

/// Returns `true` only for Bluetooth/BLE PnP device instance IDs (strictly excluding internal laptop `ACPI\` batteries).
pub fn is_bluetooth_pnp_instance_id(instance_id: &str) -> bool {
    let upper = instance_id.trim().to_ascii_uppercase();
    upper.starts_with("BTHENUM\\")
        || upper.starts_with("BTHLE\\")
        || upper.starts_with("BTHLEDEVICE\\")
}

fn read_u8_prop(
    props: &windows::Foundation::Collections::IMapView<HSTRING, windows::core::IInspectable>,
    key: &str,
) -> Option<u8> {
    let val = props.Lookup(&HSTRING::from(key)).ok()?;
    let pv = val.cast::<IPropertyValue>().ok()?;
    if let Ok(u) = pv.GetUInt8() {
        Some(u.min(100))
    } else if let Ok(u) = pv.GetUInt32() {
        Some(u.min(100) as u8)
    } else if let Ok(i) = pv.GetInt32() {
        Some(i.clamp(0, 100) as u8)
    } else {
        None
    }
}

fn read_bool_prop(
    props: &windows::Foundation::Collections::IMapView<HSTRING, windows::core::IInspectable>,
    key: &str,
) -> Option<bool> {
    let val = props.Lookup(&HSTRING::from(key)).ok()?;
    let pv = val.cast::<IPropertyValue>().ok()?;
    if let Ok(b) = pv.GetBoolean() {
        Some(b)
    } else if let Ok(u) = pv.GetUInt8() {
        Some(u != 0)
    } else if let Ok(u) = pv.GetUInt32() {
        Some(u != 0)
    } else if let Ok(i) = pv.GetInt32() {
        Some(i != 0)
    } else {
        None
    }
}

fn read_string_prop(
    props: &windows::Foundation::Collections::IMapView<HSTRING, windows::core::IInspectable>,
    key: &str,
) -> Option<String> {
    let val = props.Lookup(&HSTRING::from(key)).ok()?;
    let pv = val.cast::<IPropertyValue>().ok()?;
    pv.GetString().ok().map(|h| h.to_string())
}

fn read_guid_prop(
    props: &windows::Foundation::Collections::IMapView<HSTRING, windows::core::IInspectable>,
    key: &str,
) -> Option<String> {
    let val = props.Lookup(&HSTRING::from(key)).ok()?;
    let pv = val.cast::<IPropertyValue>().ok()?;
    pv.GetGuid().ok().map(|g| format!("{g:?}"))
}

fn build_aep_additional_props() -> IIterable<HSTRING> {
    make_hstring_iterable(&[
        "System.Devices.Aep.IsConnected",
        "System.Devices.BatteryLife",
        "System.Devices.BatteryPlusCharging",
        "System.Devices.Aep.ContainerId",
    ])
}

fn build_pnp_additional_props() -> IIterable<HSTRING> {
    make_hstring_iterable(&[
        "System.Devices.BatteryLife",
        "System.Devices.BatteryPlusCharging",
        "System.Devices.DeviceInstanceId",
        "System.Devices.ContainerId",
        "System.Devices.IsPresent",
    ])
}

fn collect_aep_selector(selector: &HSTRING, out: &mut HashMap<String, DeviceBattery>) {
    let props_req = build_aep_additional_props();
    let Ok(op) =
        DeviceInformation::FindAllAsyncAqsFilterAndAdditionalProperties(selector, &props_req)
    else {
        return;
    };
    let Ok(results) = op.get() else {
        return;
    };

    for dev_info in results {
        let id = dev_info.Id().map(|h| h.to_string()).unwrap_or_default();
        let name = dev_info.Name().map(|h| h.to_string()).unwrap_or_default();
        if name.is_empty() {
            continue;
        }

        let Ok(props) = dev_info.Properties() else {
            continue;
        };

        let connected_opt = read_bool_prop(&props, "System.Devices.Aep.IsConnected");
        if connected_opt == Some(false) {
            continue;
        }
        let is_connected = connected_opt.unwrap_or(true);

        if let Some(percentage) = read_u8_prop(&props, "System.Devices.BatteryLife") {
            if is_connected && (percentage > 0 || connected_opt == Some(true)) {
                let is_charging =
                    read_bool_prop(&props, "System.Devices.BatteryPlusCharging").unwrap_or(false);
                let dedup_key = read_guid_prop(&props, "System.Devices.Aep.ContainerId")
                    .unwrap_or_else(|| name.to_ascii_lowercase());

                out.entry(dedup_key)
                    .and_modify(|existing| {
                        existing.is_charging |= is_charging;
                        existing.percentage = percentage;
                    })
                    .or_insert_with(|| DeviceBattery {
                        id: format!("bt_{}", id),
                        name: name.clone(),
                        device_type: DeviceType::guess_from_name(&name),
                        percentage,
                        is_charging,
                        is_connected,
                        source: BatterySource::BluetoothGatt,
                    });
            }
        }
    }
}

fn collect_bluetooth_pnp_nodes(out: &mut HashMap<String, DeviceBattery>) {
    let aqs = HSTRING::from("System.Devices.BatteryLife:>=0");
    let props_req = build_pnp_additional_props();
    let Ok(op) = DeviceInformation::FindAllAsyncWithKindAqsFilterAndAdditionalProperties(
        &aqs,
        &props_req,
        DeviceInformationKind::Device,
    ) else {
        return;
    };
    let Ok(results) = op.get() else {
        return;
    };

    for dev_info in results {
        let id = dev_info.Id().map(|h| h.to_string()).unwrap_or_default();
        let name = dev_info.Name().map(|h| h.to_string()).unwrap_or_default();
        if name.is_empty() {
            continue;
        }

        let Ok(props) = dev_info.Properties() else {
            continue;
        };

        if read_bool_prop(&props, "System.Devices.IsPresent") == Some(false) {
            continue;
        }

        let Some(instance_id) = read_string_prop(&props, "System.Devices.DeviceInstanceId") else {
            continue;
        };
        if !is_bluetooth_pnp_instance_id(&instance_id) {
            continue;
        }

        if let Some(percentage) = read_u8_prop(&props, "System.Devices.BatteryLife") {
            let is_charging =
                read_bool_prop(&props, "System.Devices.BatteryPlusCharging").unwrap_or(false);
            let dedup_key = read_guid_prop(&props, "System.Devices.ContainerId")
                .unwrap_or_else(|| name.to_ascii_lowercase());

            out.entry(dedup_key)
                .and_modify(|existing| {
                    existing.is_charging |= is_charging;
                })
                .or_insert_with(|| DeviceBattery {
                    id: format!("bt_pnp_{}", id),
                    name: name.clone(),
                    device_type: DeviceType::guess_from_name(&name),
                    percentage,
                    is_charging,
                    is_connected: true,
                    source: BatterySource::WindowsPnp,
                });
        }
    }
}

impl BatteryCollector for BluetoothBatteryCollector {
    fn poll_devices(&self) -> Vec<DeviceBattery> {
        let mut by_container: HashMap<String, DeviceBattery> = HashMap::new();

        // 1. Classic Bluetooth paired endpoints (fast, no 30s radio inquiry)
        if let Ok(bt_selector) = BluetoothDevice::GetDeviceSelectorFromPairingState(true) {
            collect_aep_selector(&bt_selector, &mut by_container);
        }

        // 2. Bluetooth Low Energy (GATT) paired endpoints (correct Windows BLE protocol GUID)
        if let Ok(ble_selector) = BluetoothLEDevice::GetDeviceSelectorFromPairingState(true) {
            collect_aep_selector(&ble_selector, &mut by_container);
        }

        // 3. Bluetooth PnP device nodes (HFP headsets / BLE HID battery nodes)
        collect_bluetooth_pnp_nodes(&mut by_container);

        let mut devices: Vec<DeviceBattery> = by_container.into_values().collect();
        devices.sort_by(|a, b| a.name.cmp(&b.name));
        devices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_bluetooth_pnp_instance_id_filters_internal_laptop_batteries() {
        assert!(is_bluetooth_pnp_instance_id(
            "BTHENUM\\{0000111e-0000-1000-8000-00805f9b34fb}_LOCALMFG&0002\\8&12345"
        ));
        assert!(is_bluetooth_pnp_instance_id(
            "BTHLE\\Dev_AABBCCDDEEFF\\7&123456&0&AABBCCDDEEFF"
        ));
        assert!(is_bluetooth_pnp_instance_id(
            "bthledevice\\{0000180f-0000-1000-8000-00805f9b34fb}_dev_vid&02045e"
        ));
        // Must NEVER include internal laptop ACPI or HID UPS batteries
        assert!(!is_bluetooth_pnp_instance_id("ACPI\\PNP0C0A\\1"));
        assert!(!is_bluetooth_pnp_instance_id(
            "HID\\VID_046D&PID_0AB5\\7&1234"
        ));
        assert!(!is_bluetooth_pnp_instance_id(""));
    }

    #[test]
    fn test_bluetooth_collector_poll_does_not_panic_and_is_fast() {
        let start = std::time::Instant::now();
        let collector = BluetoothBatteryCollector::new();
        let _ = collector.poll_devices();
        assert!(
            start.elapsed() < std::time::Duration::from_secs(5),
            "Bluetooth poll must not stall on active 30s radio inquiry"
        );
    }
}
