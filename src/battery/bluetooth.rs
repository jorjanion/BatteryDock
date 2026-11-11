use super::{BatteryCollector, BatterySource, DeviceBattery, DeviceType};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use windows::core::{implement, ComObject, Interface, HSTRING};
use windows::Devices::Bluetooth::GenericAttributeProfile::{
    GattCharacteristicUuids, GattCommunicationStatus, GattServiceUuids,
};
use windows::Devices::Bluetooth::{BluetoothCacheMode, BluetoothDevice, BluetoothLEDevice};
use windows::Devices::Enumeration::{DeviceInformation, DeviceInformationKind};
use windows::Foundation::Collections::{IIterable, IIterable_Impl, IIterator, IIterator_Impl};
use windows::Foundation::IPropertyValue;
use windows::Storage::Streams::DataReader;
use windows::Win32::Foundation::E_BOUNDS;

const DEVPKEY_DEVICE_IS_CONNECTED: &str = "{83DA6326-97A6-4088-9453-A1923F573B29} 15";

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

/// Returns `true` only for Bluetooth/BLE PnP device instance IDs (strictly excluding internal laptop `ACPI\` batteries and wired USB HID).
pub fn is_bluetooth_pnp_instance_id(instance_id: &str) -> bool {
    let upper = instance_id.trim().to_ascii_uppercase();
    upper.starts_with("BTHENUM\\")
        || upper.starts_with("BTHLE\\")
        || upper.starts_with("BTHLEDEVICE\\")
        || upper.starts_with("HID\\{0000180F-")
        || upper.starts_with("HID\\{00001812-")
        || upper.starts_with("HID\\{00001124-")
}

/// Prefers the cleaner Bluetooth pairing/AEP friendly name when a PnP child node exposes a generic service name.
pub fn resolve_bluetooth_pnp_display_name(pnp_name: &str, aep_name: Option<&str>) -> String {
    let Some(aep) = aep_name.map(str::trim).filter(|s| !s.is_empty()) else {
        return pnp_name.to_string();
    };
    let lower = pnp_name.to_ascii_lowercase();
    if lower == "battery service"
        || lower.contains("generic attribute")
        || lower.contains("hands-free")
        || lower.contains("avrcp")
        || lower.contains("bluetooth low energy gatt")
        || lower.contains("hid device")
    {
        aep.to_string()
    } else {
        pnp_name.to_string()
    }
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
    } else if let Ok(u) = pv.GetUInt16() {
        Some(u != 0)
    } else if let Ok(i) = pv.GetInt16() {
        Some(i != 0)
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
        "System.Devices.Present",
        DEVPKEY_DEVICE_IS_CONNECTED,
    ])
}

struct AepMeta {
    name: String,
    is_connected: Option<bool>,
}

struct ConnectedBleCandidate {
    aep_id: HSTRING,
    dedup_key: String,
    name: String,
}

fn collect_aep_selector_inner(
    selector: &HSTRING,
    is_ble: bool,
    out: &mut HashMap<String, DeviceBattery>,
    aep_meta: &mut HashMap<String, AepMeta>,
) -> windows::core::Result<Vec<ConnectedBleCandidate>> {
    let props_req = build_aep_additional_props();
    let op = DeviceInformation::FindAllAsyncWithKindAqsFilterAndAdditionalProperties(
        selector,
        &props_req,
        DeviceInformationKind::AssociationEndpoint,
    )?;
    let results = op.get()?;
    let mut ble_candidates = Vec::new();

    for dev_info in results {
        let id_h = match dev_info.Id() {
            Ok(h) => h,
            Err(_) => continue,
        };
        let id = id_h.to_string();
        let name = dev_info.Name().map(|h| h.to_string()).unwrap_or_default();
        if name.is_empty() {
            continue;
        }

        let Ok(props) = dev_info.Properties() else {
            continue;
        };

        let connected_opt = read_bool_prop(&props, "System.Devices.Aep.IsConnected");
        let dedup_key = read_guid_prop(&props, "System.Devices.Aep.ContainerId")
            .unwrap_or_else(|| name.to_ascii_lowercase());

        aep_meta
            .entry(dedup_key.clone())
            .and_modify(|m| {
                if m.is_connected != Some(true) && connected_opt.is_some() {
                    m.is_connected = connected_opt;
                }
            })
            .or_insert_with(|| AepMeta {
                name: name.clone(),
                is_connected: connected_opt,
            });

        if connected_opt == Some(false) {
            continue;
        }
        let is_connected = connected_opt.unwrap_or(true);

        if let Some(percentage) = read_u8_prop(&props, "System.Devices.BatteryLife") {
            if is_connected && (percentage > 0 || connected_opt == Some(true)) {
                let is_charging =
                    read_bool_prop(&props, "System.Devices.BatteryPlusCharging").unwrap_or(false);

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
        } else if is_ble && connected_opt == Some(true) {
            ble_candidates.push(ConnectedBleCandidate {
                aep_id: id_h,
                dedup_key,
                name,
            });
        }
    }

    Ok(ble_candidates)
}

fn collect_bluetooth_pnp_nodes_inner(
    out: &mut HashMap<String, DeviceBattery>,
    aep_meta: &HashMap<String, AepMeta>,
) -> windows::core::Result<()> {
    let aqs = HSTRING::from("System.Devices.BatteryLife:>=0");
    let props_req = build_pnp_additional_props();
    let op = DeviceInformation::FindAllAsyncWithKindAqsFilterAndAdditionalProperties(
        &aqs,
        &props_req,
        DeviceInformationKind::Device,
    )?;
    let results = op.get()?;

    for dev_info in results {
        let id = dev_info.Id().map(|h| h.to_string()).unwrap_or_default();
        let raw_name = dev_info.Name().map(|h| h.to_string()).unwrap_or_default();
        if raw_name.is_empty() {
            continue;
        }

        let Ok(props) = dev_info.Properties() else {
            continue;
        };

        if read_bool_prop(&props, "System.Devices.Present") == Some(false) {
            continue;
        }
        if read_bool_prop(&props, DEVPKEY_DEVICE_IS_CONNECTED) == Some(false) {
            continue;
        }

        let Some(instance_id) = read_string_prop(&props, "System.Devices.DeviceInstanceId") else {
            continue;
        };
        if !is_bluetooth_pnp_instance_id(&instance_id) {
            continue;
        }

        let dedup_key = read_guid_prop(&props, "System.Devices.ContainerId")
            .unwrap_or_else(|| raw_name.to_ascii_lowercase());
        let meta = aep_meta.get(&dedup_key);
        if meta.and_then(|m| m.is_connected) == Some(false) {
            continue;
        }

        if let Some(percentage) = read_u8_prop(&props, "System.Devices.BatteryLife") {
            let is_charging =
                read_bool_prop(&props, "System.Devices.BatteryPlusCharging").unwrap_or(false);
            let name = resolve_bluetooth_pnp_display_name(&raw_name, meta.map(|m| m.name.as_str()));

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

    Ok(())
}

fn query_ble_gatt_battery_level(aep_id: &HSTRING) -> Option<u8> {
    let ble_dev = BluetoothLEDevice::FromIdAsync(aep_id).ok()?.get().ok()?;
    let battery_uuid = GattServiceUuids::Battery().ok()?;
    let services_res = ble_dev
        .GetGattServicesForUuidAsync(battery_uuid)
        .ok()?
        .get()
        .ok()?;
    if services_res.Status().ok()? != GattCommunicationStatus::Success {
        let _ = ble_dev.Close();
        return None;
    }

    let services = services_res.Services().ok()?;
    let mut found_pct = None;
    if let Ok(level_uuid) = GattCharacteristicUuids::BatteryLevel() {
        for service in services {
            if let Ok(chars_res) = service
                .GetCharacteristicsForUuidAsync(level_uuid)
                .and_then(|op| op.get())
            {
                if chars_res.Status().ok() == Some(GattCommunicationStatus::Success) {
                    if let Ok(chars) = chars_res.Characteristics() {
                        for ch in chars {
                            let read_res = ch
                                .ReadValueWithCacheModeAsync(BluetoothCacheMode::Cached)
                                .and_then(|op| op.get())
                                .ok()
                                .filter(|r| {
                                    r.Status().ok() == Some(GattCommunicationStatus::Success)
                                })
                                .or_else(|| ch.ReadValueAsync().and_then(|op| op.get()).ok());

                            if let Some(res) = read_res {
                                if res.Status().ok() == Some(GattCommunicationStatus::Success) {
                                    if let Ok(buf) = res.Value() {
                                        if buf.Length().unwrap_or(0) >= 1 {
                                            if let Ok(reader) = DataReader::FromBuffer(&buf) {
                                                if let Ok(pct) = reader.ReadByte() {
                                                    found_pct = Some(pct.min(100));
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
            let _ = service.Close();
            if found_pct.is_some() {
                break;
            }
        }
    }

    let _ = ble_dev.Close();
    found_pct
}

fn collect_ble_gatt_fallback(
    candidates: Vec<ConnectedBleCandidate>,
    out: &mut HashMap<String, DeviceBattery>,
) {
    for candidate in candidates {
        if out.contains_key(&candidate.dedup_key) {
            continue;
        }
        if let Some(percentage) = query_ble_gatt_battery_level(&candidate.aep_id) {
            out.insert(
                candidate.dedup_key,
                DeviceBattery {
                    id: format!("bt_gatt_{}", candidate.aep_id),
                    name: candidate.name.clone(),
                    device_type: DeviceType::guess_from_name(&candidate.name),
                    percentage,
                    is_charging: false,
                    is_connected: true,
                    source: BatterySource::BluetoothGatt,
                },
            );
        }
    }
}

impl BatteryCollector for BluetoothBatteryCollector {
    fn poll_devices(&self) -> Vec<DeviceBattery> {
        let mut by_container: HashMap<String, DeviceBattery> = HashMap::new();
        let mut aep_meta: HashMap<String, AepMeta> = HashMap::new();
        let mut ble_candidates: Vec<ConnectedBleCandidate> = Vec::new();

        // 1. Classic Bluetooth paired endpoints (fast, no 30s radio inquiry)
        if let Ok(bt_selector) = BluetoothDevice::GetDeviceSelectorFromPairingState(true) {
            let _ =
                collect_aep_selector_inner(&bt_selector, false, &mut by_container, &mut aep_meta);
        }

        // 2. Bluetooth Low Energy (GATT) paired endpoints
        if let Ok(ble_selector) = BluetoothLEDevice::GetDeviceSelectorFromPairingState(true) {
            if let Ok(cands) =
                collect_aep_selector_inner(&ble_selector, true, &mut by_container, &mut aep_meta)
            {
                ble_candidates = cands;
            }
        }

        // 3. Bluetooth PnP device nodes (HFP headsets / BLE HID battery nodes)
        let _ = collect_bluetooth_pnp_nodes_inner(&mut by_container, &aep_meta);

        // 4. Direct BLE GATT Battery Service (0x180F / 0x2A19) fallback for connected BLE devices
        if !ble_candidates.is_empty() {
            collect_ble_gatt_fallback(ble_candidates, &mut by_container);
        }

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
        assert!(is_bluetooth_pnp_instance_id(
            "HID\\{00001812-0000-1000-8000-00805f9b34fb}_Dev_VID&02046d\\8&1234"
        ));
        // Must NEVER include internal laptop ACPI or wired/dongle USB HID UPS batteries
        assert!(!is_bluetooth_pnp_instance_id("ACPI\\PNP0C0A\\1"));
        assert!(!is_bluetooth_pnp_instance_id(
            "HID\\VID_046D&PID_0AB5\\7&1234"
        ));
        assert!(!is_bluetooth_pnp_instance_id(""));
    }

    #[test]
    fn test_resolve_bluetooth_pnp_display_name_prefers_aep_over_generic() {
        assert_eq!(
            resolve_bluetooth_pnp_display_name("Battery Service", Some("Logitech MX Master 3S")),
            "Logitech MX Master 3S"
        );
        assert_eq!(
            resolve_bluetooth_pnp_display_name("Hands-Free AG Audio", Some("Sony WH-1000XM5")),
            "Sony WH-1000XM5"
        );
        assert_eq!(
            resolve_bluetooth_pnp_display_name("Sony WH-1000XM5 Hands-Free", None),
            "Sony WH-1000XM5 Hands-Free"
        );
        assert_eq!(
            resolve_bluetooth_pnp_display_name("AirPods Pro", Some("AirPods Pro")),
            "AirPods Pro"
        );
    }

    #[test]
    fn test_winrt_bluetooth_and_pnp_queries_succeed_without_property_syntax_error() {
        let mut by_container = HashMap::new();
        let mut aep_meta = HashMap::new();

        let bt_sel = BluetoothDevice::GetDeviceSelectorFromPairingState(true)
            .expect("BluetoothDevice::GetDeviceSelectorFromPairingState failed");
        let bt_res = collect_aep_selector_inner(&bt_sel, false, &mut by_container, &mut aep_meta);
        assert!(
            bt_res.is_ok(),
            "Classic BT AEP query failed: {:?}",
            bt_res.err()
        );

        let ble_sel = BluetoothLEDevice::GetDeviceSelectorFromPairingState(true)
            .expect("BluetoothLEDevice::GetDeviceSelectorFromPairingState failed");
        let ble_res = collect_aep_selector_inner(&ble_sel, true, &mut by_container, &mut aep_meta);
        assert!(ble_res.is_ok(), "BLE AEP query failed: {:?}", ble_res.err());

        let pnp_res = collect_bluetooth_pnp_nodes_inner(&mut by_container, &aep_meta);
        assert!(
            pnp_res.is_ok(),
            "Bluetooth PnP query must succeed without WinRT property key syntax errors: {:?}",
            pnp_res.err()
        );
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
