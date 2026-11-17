use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceType {
    Headset,
    Mouse,
    Keyboard,
    Gamepad,
    VRHeadset,
    Wheel,
    Camera,
    Unknown,
}

impl DeviceType {
    pub fn display_name(&self) -> &'static str {
        match self {
            DeviceType::Headset => "Headset",
            DeviceType::Mouse => "Mouse",
            DeviceType::Keyboard => "Keyboard",
            DeviceType::Gamepad => "Gamepad",
            DeviceType::VRHeadset => "VR Headset",
            DeviceType::Wheel => "Racing Wheel",
            DeviceType::Camera => "Camera",
            DeviceType::Unknown => "Peripheral",
        }
    }

    pub fn guess_from_name(name: &str) -> Self {
        let lower = name.to_lowercase();
        if lower.contains("headset")
            || lower.contains("headphone")
            || lower.contains("earbud")
            || lower.contains("airpod")
            || lower.contains("wh-")
            || lower.contains("wf-")
            || lower.contains("wi-")
            || lower.contains("bose")
            || lower.contains("quietcomfort")
            || lower.contains("sennheiser")
            || lower.contains("momentum")
            || lower.contains("hands-free")
            || lower.contains("jbl")
            || lower.contains("g733")
            || lower.contains("g935")
            || lower.contains("g533")
            || lower.contains("g535")
            || lower.contains("g435")
            || lower.contains("barracuda")
            || lower.contains("blackshark")
            || lower.contains("arctis")
            || lower.contains("cloud")
        {
            DeviceType::Headset
        } else if lower.contains("mouse")
            || lower.contains("ratón")
            || lower.contains("g pro")
            || lower.contains("superlight")
            || lower.contains("g502")
            || lower.contains("g305")
            || lower.contains("g703")
            || lower.contains("g903")
            || lower.contains("viper")
            || lower.contains("deathadder")
            || lower.contains("basilisk")
            || lower.contains("mx master")
            || lower.contains("mx anywhere")
        {
            DeviceType::Mouse
        } else if lower.contains("keyboard")
            || lower.contains("teclado")
            || lower.contains("g915")
            || lower.contains("keychron")
            || lower.contains("blackwidow")
            || lower.contains("huntsman")
            || lower.contains("apex")
        {
            DeviceType::Keyboard
        } else if lower.contains("controller")
            || lower.contains("mando")
            || lower.contains("gamepad")
            || lower.contains("xbox")
            || lower.contains("dualsense")
            || lower.contains("dualshock")
            || lower.contains("playstation")
            || lower.contains("switch pro")
            || lower.contains("8bitdo")
        {
            DeviceType::Gamepad
        } else if lower.contains("vr")
            || lower.contains("quest")
            || lower.contains("index")
            || lower.contains("vive")
            || lower.contains("oculus")
        {
            DeviceType::VRHeadset
        } else if lower.contains("wheel")
            || lower.contains("volante")
            || lower.contains("g29")
            || lower.contains("g923")
            || lower.contains("g920")
            || lower.contains("t300")
            || lower.contains("fanatec")
        {
            DeviceType::Wheel
        } else if lower.contains("camera") || lower.contains("cámara") || lower.contains("webcam")
        {
            DeviceType::Camera
        } else {
            DeviceType::Unknown
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatterySource {
    BluetoothGatt,
    WindowsPnp,
    LogitechHidpp,
    XInput,
    DualSenseHid,
    RazerHid,
    SteelSeriesHid,
    CorsairHid,
    GenericHid,
    NintendoHid,
    AudioHeadsetHid,
}

impl BatterySource {
    pub fn display_name(&self) -> &'static str {
        match self {
            BatterySource::BluetoothGatt => "Bluetooth GATT",
            BatterySource::WindowsPnp => "Windows PNP",
            BatterySource::LogitechHidpp => "Logitech LIGHTSPEED / HID++",
            BatterySource::XInput => "Xbox Wireless / XInput",
            BatterySource::DualSenseHid => "PlayStation DualSense / DualShock",
            BatterySource::RazerHid => "Razer HyperSpeed Wireless",
            BatterySource::SteelSeriesHid => "SteelSeries Quantum Wireless",
            BatterySource::CorsairHid => "Corsair SLIPSTREAM",
            BatterySource::GenericHid => "Universal HID Battery",
            BatterySource::NintendoHid => "Nintendo Switch",
            BatterySource::AudioHeadsetHid => "Wireless Audio Headset",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceBattery {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub percentage: u8,
    pub is_charging: bool,
    pub is_connected: bool,
    pub source: BatterySource,
    #[serde(default)]
    pub is_asleep: bool,
    #[serde(default)]
    pub estimated_minutes_left: Option<u32>,
}

impl DeviceBattery {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        device_type: DeviceType,
        percentage: u8,
        is_charging: bool,
        is_connected: bool,
        source: BatterySource,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            device_type,
            percentage,
            is_charging,
            is_connected,
            source,
            is_asleep: false,
            estimated_minutes_left: None,
        }
    }
}

pub trait BatteryCollector: Send + Sync {
    fn poll_devices(&self) -> Vec<DeviceBattery>;
}

pub mod audio_headsets;
pub mod bluetooth;
pub mod diagnostics;
pub mod dualsense;
pub mod engine;
pub mod generic_hid;
pub mod history;
pub mod intelligence;
pub mod logitech_hidpp;
pub mod nintendo;
pub mod razer;
pub mod xinput;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_type_guess() {
        assert_eq!(
            DeviceType::guess_from_name("Logitech G733 Gaming Headset"),
            DeviceType::Headset
        );
        assert_eq!(
            DeviceType::guess_from_name("Xbox Wireless Controller"),
            DeviceType::Gamepad
        );
        assert_eq!(
            DeviceType::guess_from_name("Logitech G PRO X Superlight"),
            DeviceType::Mouse
        );
        assert_eq!(
            DeviceType::guess_from_name("Logitech G915 TKL Wireless Mechanical Keyboard"),
            DeviceType::Keyboard
        );
        assert_eq!(
            DeviceType::guess_from_name("Sony WH-1000XM5"),
            DeviceType::Headset
        );
        assert_eq!(
            DeviceType::guess_from_name("Sony WH-CH720N"),
            DeviceType::Headset
        );
        assert_eq!(
            DeviceType::guess_from_name("WH-CH720N Hands-Free AG"),
            DeviceType::Headset
        );
        assert_eq!(
            DeviceType::guess_from_name(
                "Sony Interactive Entertainment Wireless Controller (DualSense)"
            ),
            DeviceType::Gamepad
        );
        assert_eq!(
            DeviceType::guess_from_name("Meta Quest 3"),
            DeviceType::VRHeadset
        );
        assert_eq!(DeviceType::Headset.display_name(), "Headset");
        assert_eq!(DeviceType::VRHeadset.display_name(), "VR Headset");
        assert_eq!(DeviceType::Unknown.display_name(), "Peripheral");
    }
}
