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

        let mut on_refresh_opt = Some(on_refresh);

        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("battery_dock_settings"),
            egui::ViewportBuilder::default()
                .with_title("BatteryDock Settings")
                .with_inner_size([460.0, 560.0])
                .with_min_inner_size([380.0, 420.0])
                .with_decorations(true)
                .with_resizable(true)
                .with_transparent(false),
            |ctx, _class| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        Self::render_contents(
                            ui,
                            open,
                            config,
                            devices,
                            || {
                                if let Some(cb) = on_refresh_opt.take() {
                                    cb();
                                }
                            },
                        );
                    });
                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    *open = false;
                }
            },
        );
    }

    pub fn render_contents(
        ui: &mut egui::Ui,
        open: &mut bool,
        config: &mut AppConfig,
        devices: &[DeviceBattery],
        on_refresh: impl FnOnce(),
    ) {
        let mut should_refresh = false;

        ui.add_space(8.0);

        // Header
        ui.horizontal(|ui| {
            ui.heading("🔋 BatteryDock");
            ui.label(egui::RichText::new(concat!("v", env!("CARGO_PKG_VERSION"))).weak());
        });
        ui.label(
            egui::RichText::new("Ultra-lightweight peripheral battery overlay for Windows")
                .italics()
                .weak(),
        );

        ui.add_space(6.0);
        ui.separator();
        ui.add_space(6.0);

        // Alert Thresholds Section
        ui.heading("⚡ Battery Alert Thresholds");
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label("Low Battery Warning:");
            let mut low = config.low_battery_threshold;
            if ui
                .add(egui::Slider::new(&mut low, 5..=40).suffix("%"))
                .changed()
            {
                config.low_battery_threshold = low;
                config.save();
            }
        });

        ui.horizontal(|ui| {
            ui.label("Critical Battery Warning:");
            let mut crit = config.critical_battery_threshold;
            if ui
                .add(egui::Slider::new(&mut crit, 1..=15).suffix("%"))
                .changed()
            {
                config.critical_battery_threshold = crit;
                config.save();
            }
        });

        ui.add_space(6.0);
        ui.separator();
        ui.add_space(6.0);

        // Notification Toggles Section
        ui.heading("🔔 Notifications & Sound");
        ui.add_space(4.0);

        let mut audio_on = config.audio_alert_enabled;
        if ui
            .checkbox(&mut audio_on, "Play gentle chime on low battery")
            .changed()
        {
            config.audio_alert_enabled = audio_on;
            config.save();
        }

        ui.horizontal(|ui| {
            ui.add_space(20.0);
            if ui.button("▶ Test Chime Sound").clicked() {
                audio::play_gentle_chime();
            }
        });

        let mut visual_on = config.visual_alert_enabled;
        if ui
            .checkbox(&mut visual_on, "Show desktop notifications")
            .changed()
        {
            config.visual_alert_enabled = visual_on;
            config.save();
        }

        ui.add_space(6.0);
        ui.separator();
        ui.add_space(6.0);

        // Placement Section
        ui.heading("📌 Dock Placement");
        ui.add_space(4.0);

        let current_mode = config.position_mode;
        ui.radio_value(
            &mut config.position_mode,
            DockPositionMode::TaskbarLeftOfTray,
            "Taskbar: Next to System Tray (Clock)",
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
            "Custom (Click & drag the dock anywhere)",
        );

        if config.position_mode != current_mode {
            config.save();
        }

        if ui.button("📍 Reset Dock Position to Clock").clicked() {
            config.position_mode = DockPositionMode::TaskbarLeftOfTray;
            config.custom_pos_x = 0.0;
            config.custom_pos_y = 0.0;
            config.save();
        }

        ui.add_space(6.0);
        ui.separator();
        ui.add_space(6.0);

        // Connected Devices Section
        ui.horizontal(|ui| {
            ui.heading(format!("🎧 Connected Devices ({})", devices.len()));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("🔄 Refresh Now").clicked() {
                    should_refresh = true;
                }
            });
        });
        ui.add_space(4.0);

        if devices.is_empty() {
            ui.label(
                egui::RichText::new("No active wireless peripherals detected.")
                    .italics()
                    .weak(),
            );
        } else {
            egui::Frame::group(ui.style()).show(ui, |ui| {
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

                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                let charging = if dev.is_charging { " ⚡" } else { "" };
                                let color = if dev.percentage <= config.critical_battery_threshold {
                                    egui::Color32::from_rgb(239, 68, 68)
                                } else if dev.percentage <= config.low_battery_threshold {
                                    egui::Color32::from_rgb(245, 158, 11)
                                } else {
                                    egui::Color32::from_rgb(34, 197, 94)
                                };
                                ui.label(
                                    egui::RichText::new(format!("{}%{}", dev.percentage, charging))
                                        .color(color)
                                        .strong(),
                                );
                            },
                        );
                    });
                }
            });
        }

        ui.add_space(6.0);
        ui.separator();
        ui.add_space(6.0);

        // Windows Startup Option
        let mut start_win = config.start_with_windows;
        if ui
            .checkbox(&mut start_win, "Start BatteryDock when Windows boots")
            .changed()
        {
            config.start_with_windows = start_win;
            let _ = set_run_at_startup(start_win);
            config.save();
        }

        ui.add_space(10.0);
        if ui.button("Close Settings").clicked() {
            *open = false;
        }

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
        if RegCreateKeyW(HKEY_CURRENT_USER, PCWSTR(subkey.as_ptr()), &mut hkey).is_ok() {
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
