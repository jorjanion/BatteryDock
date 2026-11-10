use crate::battery::DeviceBattery;
use crate::config::{AppConfig, DockPositionMode};
use crate::notifications::audio;

pub struct SettingsWindow;

impl SettingsWindow {
    pub fn show(
        ctx: &egui::Context,
        open: &mut bool,
        config: &mut AppConfig,
        devices: &[DeviceBattery],
        on_refresh: impl FnOnce(),
    ) {
        if !*open {
            return;
        }

        let mut should_refresh = false;

        egui::Window::new("⚙ BatteryDock Settings")
            .open(open)
            .resizable(false)
            .collapsible(false)
            .default_width(380.0)
            .show(ctx, |ui| {
                ui.add_space(4.0);

                // Branding Header
                ui.horizontal(|ui| {
                    ui.heading("🔋 BatteryDock");
                    ui.label(egui::RichText::new("v0.1.0").weak());
                });
                ui.label(
                    egui::RichText::new("Zero-bloat taskbar peripheral battery overlay")
                        .italics()
                        .weak(),
                );

                ui.separator();

                // Alert Thresholds Section
                ui.heading("Alert Thresholds");
                ui.horizontal(|ui| {
                    ui.label("Low Battery Alert:");
                    let mut low = config.low_battery_threshold;
                    if ui.add(egui::Slider::new(&mut low, 5..=40).suffix("%")).changed() {
                        config.low_battery_threshold = low;
                        config.save();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Critical Alert:");
                    let mut crit = config.critical_battery_threshold;
                    if ui.add(egui::Slider::new(&mut crit, 1..=15).suffix("%")).changed() {
                        config.critical_battery_threshold = crit;
                        config.save();
                    }
                });

                ui.separator();

                // Notification Toggles
                ui.heading("Notifications");
                let mut audio_on = config.audio_alert_enabled;
                if ui.checkbox(&mut audio_on, "Play gentle chime on low battery").changed() {
                    config.audio_alert_enabled = audio_on;
                    config.save();
                }

                ui.horizontal(|ui| {
                    if ui.button("🔔 Test Sound Chime").clicked() {
                        audio::play_gentle_chime();
                    }
                });

                let mut visual_on = config.visual_alert_enabled;
                if ui.checkbox(&mut visual_on, "Show desktop alert notification").changed() {
                    config.visual_alert_enabled = visual_on;
                    config.save();
                }

                ui.separator();

                // Taskbar Dock Position
                ui.heading("Taskbar Placement");
                let current_mode = config.position_mode;

                ui.radio_value(
                    &mut config.position_mode,
                    DockPositionMode::TaskbarLeftOfTray,
                    "Taskbar: Left of System Tray (Clock)",
                );
                ui.radio_value(
                    &mut config.position_mode,
                    DockPositionMode::TaskbarRight,
                    "Taskbar: Far Right",
                );
                ui.radio_value(
                    &mut config.position_mode,
                    DockPositionMode::TaskbarLeft,
                    "Taskbar: Left Side",
                );
                ui.radio_value(
                    &mut config.position_mode,
                    DockPositionMode::Custom,
                    "Custom (Click & Drag dock anywhere)",
                );

                if config.position_mode != current_mode {
                    config.save();
                }

                if ui.button("Reset Dock to Default Position").clicked() {
                    config.position_mode = DockPositionMode::TaskbarLeftOfTray;
                    config.custom_pos_x = 0.0;
                    config.custom_pos_y = 0.0;
                    config.save();
                }

                ui.separator();

                // Connected Devices List
                ui.heading(format!("Connected Peripherals ({})", devices.len()));
                if devices.is_empty() {
                    ui.label(egui::RichText::new("No active wireless peripherals detected.").weak());
                } else {
                    egui::ScrollArea::vertical().max_height(140.0).show(ui, |ui| {
                        for dev in devices {
                            ui.horizontal(|ui| {
                                let icon = match dev.device_type {
                                    crate::battery::DeviceType::Headset => "🎧",
                                    crate::battery::DeviceType::Mouse => "🖱️",
                                    crate::battery::DeviceType::Keyboard => "⌨️",
                                    crate::battery::DeviceType::Gamepad => "🎮",
                                    crate::battery::DeviceType::VRHeadset => "🥽",
                                    crate::battery::DeviceType::Wheel => "🏎️",
                                    crate::battery::DeviceType::Camera => "📷",
                                    crate::battery::DeviceType::Unknown => "🔋",
                                };
                                ui.label(icon);
                                ui.label(egui::RichText::new(&dev.name).strong());
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let charging_mark = if dev.is_charging { " ⚡" } else { "" };
                                    ui.label(format!("{}{}", dev.percentage, charging_mark));
                                });
                            });
                        }
                    });
                }

                ui.add_space(4.0);
                if ui.button("🔄 Refresh Devices Now").clicked() {
                    should_refresh = true;
                }

                ui.separator();

                // Startup toggle
                let mut start_win = config.start_with_windows;
                if ui.checkbox(&mut start_win, "Start BatteryDock with Windows").changed() {
                    config.start_with_windows = start_win;
                    let _ = set_run_at_startup(start_win);
                    config.save();
                }
            });

        if should_refresh {
            on_refresh();
        }
    }
}

/// Sets or removes Windows Run registry key
fn set_run_at_startup(enable: bool) -> Result<(), ()> {
    use std::os::windows::ffi::OsStrExt;
    use windows::core::PCWSTR;
    use windows::Win32::System::Registry::{
        RegCloseKey, RegCreateKeyW, RegDeleteValueW, RegSetValueExW, HKEY_CURRENT_USER, REG_SZ,
    };

    let subkey: Vec<u16> = std::ffi::OsStr::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
        .encode_wide()
        .chain(Some(0))
        .collect();
    let app_name: Vec<u16> = std::ffi::OsStr::new("BatteryDock")
        .encode_wide()
        .chain(Some(0))
        .collect();

    unsafe {
        let mut hkey = windows::Win32::System::Registry::HKEY::default();
        if RegCreateKeyW(
            HKEY_CURRENT_USER,
            PCWSTR(subkey.as_ptr()),
            &mut hkey,
        )
        .is_ok()
        {
            if enable {
                if let Ok(exe_path) = std::env::current_exe() {
                    let path_str = exe_path.to_string_lossy().to_string();
                    let path_wide: Vec<u16> = std::ffi::OsStr::new(&path_str)
                        .encode_wide()
                        .chain(Some(0))
                        .collect();
                    let _ = RegSetValueExW(
                        hkey,
                        PCWSTR(app_name.as_ptr()),
                        0,
                        REG_SZ,
                        Some(std::slice::from_raw_parts(
                            path_wide.as_ptr() as *const u8,
                            path_wide.len() * 2,
                        )),
                    );
                }
            } else {
                let _ = RegDeleteValueW(hkey, PCWSTR(app_name.as_ptr()));
            }
            let _ = RegCloseKey(hkey);
        }
    }
    Ok(())
}
