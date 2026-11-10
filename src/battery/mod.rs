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
            || lower.contains("wh-1000")
            || lower.contains("wf-1000")
            || lower.contains("bose")
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
        } else if lower.contains("camera") || lower.contains("cámara") || lower.contains("webcam") {
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
}

pub trait BatteryCollector: Send + Sync {
    fn poll_devices(&self) -> Vec<DeviceBattery>;
}

pub mod bluetooth;
pub mod dualsense;
pub mod engine;
pub mod logitech_hidpp;
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
            DeviceType::guess_from_name("Sony Interactive Entertainment Wireless Controller (DualSense)"),
            DeviceType::Gamepad
        );
        assert_eq!(
            DeviceType::guess_from_name("Meta Quest 3"),
            DeviceType::VRHeadset
        );
    }
}
