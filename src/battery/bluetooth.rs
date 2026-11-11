use super::{BatteryCollector, BatterySource, DeviceBattery, DeviceType};
use windows::core::{HSTRING, Interface};
use windows::Devices::Enumeration::DeviceInformation;
use windows::Foundation::IPropertyValue;

pub struct BluetoothBatteryCollector;

impl BluetoothBatteryCollector {
    pub fn new() -> Self {
        Self
    }
}

impl BatteryCollector for BluetoothBatteryCollector {
    fn poll_devices(&self) -> Vec<DeviceBattery> {
        let mut devices = Vec::new();

        // Bluetooth AEP protocol ID
        let aqs = HSTRING::from(
            "System.Devices.Aep.ProtocolId:=\"{E0CBF06C-CD8B-4647-BB8A-263B43F0F974}\" OR \
             System.Devices.Aep.ProtocolId:=\"{BB7BB05E-5972-42B5-BB20-767BD5034CEE}\"",
        );

        let async_op = match DeviceInformation::FindAllAsyncAqsFilter(&aqs) {
            Ok(op) => op,
            Err(_) => return devices,
        };

        let results = match async_op.get() {
            Ok(res) => res,
            Err(_) => return devices,
        };

        for dev_info in results {
            let id = dev_info.Id().map(|h| h.to_string()).unwrap_or_default();
            let name = dev_info.Name().map(|h| h.to_string()).unwrap_or_default();

            if name.is_empty() {
                continue;
            }

            if let Ok(props) = dev_info.Properties() {
                // Check if device is connected (None = property absent on driver; Some(false) = explicitly disconnected)
                let connected_opt: Option<bool> =
                    match props.Lookup(&HSTRING::from("System.Devices.Aep.IsConnected")) {
                        Ok(val) => val
                            .cast::<IPropertyValue>()
                            .ok()
                            .and_then(|pv| pv.GetBoolean().ok()),
                        Err(_) => None,
                    };
                if connected_opt == Some(false) {
                    continue;
                }
                let is_connected = connected_opt.unwrap_or(true);

                // Check battery level (0-100)
                if let Ok(battery_val) =
                    props.Lookup(&HSTRING::from("System.Devices.BatteryLife"))
                {
                    if let Ok(prop_val) = battery_val.cast::<IPropertyValue>() {
                        let level: Option<u8> = if let Ok(u) = prop_val.GetUInt8() {
                            Some(u.min(100))
                        } else if let Ok(u) = prop_val.GetUInt32() {
                            Some(u.min(100) as u8)
                        } else if let Ok(i) = prop_val.GetInt32() {
                            Some((i.clamp(0, 100)) as u8)
                        } else {
                            None
                        };

                        if let Some(percentage) = level {
                            if is_connected && (percentage > 0 || connected_opt == Some(true)) {
                                let device_type = DeviceType::guess_from_name(&name);
                                devices.push(DeviceBattery {
                                    id: format!("bt_{}", id),
                                    name,
                                    device_type,
                                    percentage,
                                    is_charging: false,
                                    is_connected,
                                    source: BatterySource::BluetoothGatt,
                                });
                            }
                        }
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
    fn test_bluetooth_collector_poll_does_not_panic() {
        let collector = BluetoothBatteryCollector::new();
        let _ = collector.poll_devices();
    }
}
