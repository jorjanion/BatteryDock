use crate::battery::DeviceBattery;
use crate::config::{AppConfig, DockPositionMode};
use crate::notifications::audio;

pub struct SettingsWindow;

impl SettingsWindow {
    pub fn render_contents(
        ui: &mut egui::Ui,
        open: &mut bool,
        config: &mut AppConfig,
        devices: &[DeviceBattery],
        on_refresh: impl FnOnce(),
    ) {
        let mut should_refresh = false;
        let lang = config.language;

        ui.add_space(8.0);

        // Header
        ui.horizontal(|ui| {
            ui.heading("🔋 BatteryDock");
            ui.label(egui::RichText::new(concat!("v", env!("CARGO_PKG_VERSION"))).weak());
        });
        ui.label(egui::RichText::new(lang.header_subtitle()).italics().weak());

        ui.add_space(6.0);
        ui.separator();
        ui.add_space(6.0);

        // Language Selector Section
        ui.heading(lang.heading_language());
        ui.add_space(4.0);
        let mut cur_lang = config.language;
        egui::ComboBox::from_id_salt("settings_language_combobox")
            .selected_text(cur_lang.display_name())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut cur_lang,
                    crate::i18n::Language::English,
                    crate::i18n::Language::English.display_name(),
                );
                ui.selectable_value(
                    &mut cur_lang,
                    crate::i18n::Language::Spanish,
                    crate::i18n::Language::Spanish.display_name(),
                );
            });
        if cur_lang != config.language {
            config.language = cur_lang;
            config.save();
        }

        ui.add_space(6.0);
        ui.separator();
        ui.add_space(6.0);

        // Alert Thresholds Section
        ui.heading(lang.heading_alert_thresholds());
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label(lang.low_battery_warning());
            let mut low = config.low_battery_threshold;
            if ui
                .add(egui::Slider::new(&mut low, 5..=40).suffix("%"))
                .changed()
            {
                config.low_battery_threshold = low;
                if config.critical_battery_threshold > config.low_battery_threshold {
                    config.critical_battery_threshold = config.low_battery_threshold;
                }
                config.save();
            }
        });

        ui.horizontal(|ui| {
            ui.label(lang.critical_battery_warning());
            let mut crit = config.critical_battery_threshold;
            if ui
                .add(egui::Slider::new(&mut crit, 1..=15).suffix("%"))
                .changed()
            {
                config.critical_battery_threshold = crit.min(config.low_battery_threshold);
                config.save();
            }
        });

        ui.add_space(6.0);
        ui.separator();
        ui.add_space(6.0);

        // Notification Toggles Section
        ui.heading(lang.heading_notifications());
        ui.add_space(4.0);

        let mut audio_on = config.audio_alert_enabled;
        if ui
            .checkbox(&mut audio_on, lang.play_chime_checkbox())
            .changed()
        {
            config.audio_alert_enabled = audio_on;
            config.save();
        }

        ui.horizontal(|ui| {
            ui.add_space(20.0);
            if ui.button(lang.test_chime_button()).clicked() {
                audio::play_gentle_chime();
            }
        });

        let mut visual_on = config.visual_alert_enabled;
        if ui
            .checkbox(&mut visual_on, lang.desktop_notifications_checkbox())
            .changed()
        {
            config.visual_alert_enabled = visual_on;
            config.save();
        }

        ui.add_space(6.0);
        ui.separator();
        ui.add_space(6.0);

        // Placement Section
        ui.heading(lang.heading_dock_placement());
        ui.add_space(4.0);

        let current_mode = config.position_mode;
        ui.radio_value(
            &mut config.position_mode,
            DockPositionMode::TaskbarLeftOfTray,
            lang.placement_tray_clock(),
        );
        ui.radio_value(
            &mut config.position_mode,
            DockPositionMode::TaskbarRight,
            lang.placement_far_right(),
        );
        ui.radio_value(
            &mut config.position_mode,
            DockPositionMode::TaskbarLeft,
            lang.placement_left_side(),
        );
        ui.radio_value(
            &mut config.position_mode,
            DockPositionMode::Custom,
            lang.placement_custom(),
        );

        if config.position_mode != current_mode {
            config.save();
        }

        if ui.button(lang.reset_position_button()).clicked() {
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
            ui.heading(lang.heading_connected_devices(devices.len()));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(lang.refresh_now_button()).clicked() {
                    should_refresh = true;
                }
            });
        });
        ui.add_space(4.0);

        if devices.is_empty() {
            ui.label(
                egui::RichText::new(lang.no_devices_detected_label())
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
                        ui.label(icon)
                            .on_hover_text(lang.device_type_name(&dev.device_type));
                        ui.label(egui::RichText::new(&dev.name).strong());

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let color = if dev.is_charging {
                                egui::Color32::from_rgb(56, 189, 248)
                            } else if dev.percentage <= config.critical_battery_threshold {
                                egui::Color32::from_rgb(239, 68, 68)
                            } else if dev.percentage <= config.low_battery_threshold {
                                egui::Color32::from_rgb(245, 158, 11)
                            } else {
                                egui::Color32::from_rgb(34, 197, 94)
                            };
                            let label = if dev.is_charging {
                                format!("{}%{}", dev.percentage, lang.charging_suffix())
                            } else {
                                format!("{}%", dev.percentage)
                            };
                            ui.label(egui::RichText::new(label).color(color).strong());
                            if dev.is_charging {
                                let (bolt_rect, _) = ui.allocate_exact_size(
                                    egui::Vec2::new(11.0, 14.0),
                                    egui::Sense::hover(),
                                );
                                crate::ui::icons::DevicePainter::paint_charging_icon(
                                    ui.painter(),
                                    bolt_rect,
                                    1.0,
                                );
                            }
                        });
                    });
                }
            });
        }

        ui.add_space(6.0);
        ui.separator();
        ui.add_space(6.0);

        let mut show_pct = config.show_percentage_text;
        if ui
            .checkbox(&mut show_pct, lang.show_percentage_checkbox())
            .changed()
        {
            config.show_percentage_text = show_pct;
            config.save();
        }

        // Windows Startup Option
        let mut start_win = config.start_with_windows;
        if ui
            .checkbox(&mut start_win, lang.start_with_windows_checkbox())
            .changed()
        {
            config.start_with_windows = start_win;
            let _ = set_run_at_startup(start_win);
            config.save();
        }

        ui.add_space(6.0);
        ui.separator();
        ui.add_space(6.0);

        // Application Updates
        ui.heading(lang.heading_updates());
        ui.add_space(4.0);

        let mut auto_up = config.auto_update;
        if ui
            .checkbox(&mut auto_up, lang.auto_update_checkbox())
            .changed()
        {
            config.auto_update = auto_up;
            config.save();
        }

        ui.horizontal(|ui| {
            if ui.button(lang.check_updates_now_button()).clicked() {
                crate::updater::check_for_updates_async(false);
            }

            match crate::updater::get_update_status() {
                crate::updater::UpdateStatus::Idle => {}
                crate::updater::UpdateStatus::Checking => {
                    ui.label(egui::RichText::new(lang.checking_github_label()).weak());
                }
                crate::updater::UpdateStatus::UpToDate => {
                    ui.label(
                        egui::RichText::new(lang.up_to_date_label())
                            .color(egui::Color32::from_rgb(34, 197, 94)),
                    );
                }
                crate::updater::UpdateStatus::UpdateAvailable {
                    version,
                    download_url,
                } => {
                    ui.label(
                        egui::RichText::new(lang.new_version_available_label(&version))
                            .color(egui::Color32::from_rgb(56, 189, 248))
                            .strong(),
                    );
                    if ui.button(lang.install_update_button()).clicked() {
                        let v = version.clone();
                        let u = download_url.clone();
                        std::thread::spawn(move || {
                            let _ = crate::updater::download_and_run_installer(&u, &v);
                        });
                    }
                }
                crate::updater::UpdateStatus::Downloading { version } => {
                    ui.label(
                        egui::RichText::new(lang.downloading_label(&version))
                            .color(egui::Color32::from_rgb(56, 189, 248)),
                    );
                }
                crate::updater::UpdateStatus::Installing { version } => {
                    ui.label(
                        egui::RichText::new(lang.installing_label(&version))
                            .color(egui::Color32::from_rgb(34, 197, 94))
                            .strong(),
                    );
                }
                crate::updater::UpdateStatus::Failed(err) => {
                    ui.label(
                        egui::RichText::new(lang.update_failed_label(&err))
                            .color(egui::Color32::from_rgb(239, 68, 68))
                            .weak(),
                    );
                }
            }
        });

        ui.add_space(10.0);
        if ui.button(lang.close_settings_button()).clicked() {
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

    let subkey: Vec<u16> =
        std::ffi::OsStr::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
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
                    let path_str = format!("\"{}\"", exe_path.to_string_lossy());
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

pub struct SettingsApp {
    config: AppConfig,
    engine: crate::battery::engine::BatteryEngine,
    devices: Vec<DeviceBattery>,
    is_open: bool,
    last_config_reload: std::time::Instant,
}

impl SettingsApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let config = AppConfig::load();
        let engine = crate::battery::engine::BatteryEngine::start(std::time::Duration::from_secs(
            config.poll_interval_secs,
        ));
        let devices = engine
            .receiver()
            .recv_timeout(std::time::Duration::from_millis(350))
            .unwrap_or_default();
        Self {
            config,
            engine,
            devices,
            is_open: true,
            last_config_reload: std::time::Instant::now(),
        }
    }

    fn reload_external_config_if_due(&mut self) {
        let path = AppConfig::config_path();
        sync_external_config_if_due(&mut self.config, &mut self.last_config_reload, &path);
    }
}

fn sync_external_config_if_due(
    config: &mut AppConfig,
    last_reload: &mut std::time::Instant,
    path: &std::path::Path,
) -> bool {
    if last_reload.elapsed() >= std::time::Duration::from_millis(250) {
        *last_reload = std::time::Instant::now();
        if let Some(loaded) = AppConfig::try_load_from_path(path) {
            if loaded != *config {
                *config = loaded;
                return true;
            }
        }
    }
    false
}

impl eframe::App for SettingsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.reload_external_config_if_due();

        while let Ok(updated) = self.engine.receiver().try_recv() {
            self.devices = updated;
        }

        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::from_rgb(22, 24, 29);
        visuals.window_fill = egui::Color32::from_rgb(22, 24, 29);
        ctx.set_visuals(visuals);

        let engine_ref = self.engine.clone();
        egui::CentralPanel::default()
            .frame(
                egui::Frame::NONE
                    .fill(egui::Color32::from_rgb(22, 24, 29))
                    .inner_margin(egui::Margin::same(16)),
            )
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    SettingsWindow::render_contents(
                        ui,
                        &mut self.is_open,
                        &mut self.config,
                        &self.devices,
                        move || engine_ref.trigger_refresh(),
                    );
                });
            });

        ctx.request_repaint_after(std::time::Duration::from_millis(250));

        if !self.is_open || ctx.input(|i| i.viewport().close_requested()) {
            std::process::exit(0);
        }
    }
}

pub fn run_standalone_settings() -> eframe::Result<()> {
    use windows::core::w;
    use windows::Win32::Foundation::{GetLastError, ERROR_ALREADY_EXISTS};
    use windows::Win32::System::Threading::CreateMutexW;
    use windows::Win32::UI::WindowsAndMessaging::{
        FindWindowW, SetForegroundWindow, ShowWindow, SW_RESTORE,
    };

    unsafe {
        let _mutex = CreateMutexW(None, true, w!("Global\\BatteryDockSettingsMutex"));
        if GetLastError() == ERROR_ALREADY_EXISTS {
            if let Ok(hwnd) = FindWindowW(None, w!("BatteryDock Settings")) {
                if !hwnd.0.is_null() {
                    let _ = ShowWindow(hwnd, SW_RESTORE);
                    let _ = SetForegroundWindow(hwnd);
                }
            }
            return Ok(());
        }
    }

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("BatteryDock Settings")
            .with_inner_size([480.0, 600.0])
            .with_min_inner_size([380.0, 420.0])
            .with_decorations(true)
            .with_resizable(true)
            .with_transparent(false),
        ..Default::default()
    };

    eframe::run_native(
        "BatteryDock Settings",
        native_options,
        Box::new(|cc| Ok(Box::new(SettingsApp::new(cc)))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_reloads_external_reset_position_without_overwriting() {
        let dir = std::env::temp_dir().join(format!(
            "batterydock_settings_reload_test_{}",
            std::process::id()
        ));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("config.toml");

        // Initial Settings state: Custom dragged position
        let mut settings_cfg = AppConfig {
            position_mode: DockPositionMode::Custom,
            custom_pos_x: 640.0,
            custom_pos_y: 420.0,
            ..AppConfig::default()
        };
        settings_cfg.save_to_path(&path);

        // External dock/tray action clicks "Reset to Taskbar Clock" and writes to disk
        let external_cfg = AppConfig {
            position_mode: DockPositionMode::TaskbarLeftOfTray,
            custom_pos_x: 0.0,
            custom_pos_y: 0.0,
            ..settings_cfg.clone()
        };
        external_cfg.save_to_path(&path);

        // Before 250ms elapsed, sync does not run
        let mut recent = std::time::Instant::now();
        assert!(!sync_external_config_if_due(
            &mut settings_cfg,
            &mut recent,
            &path
        ));
        assert_eq!(settings_cfg.position_mode, DockPositionMode::Custom);

        // Once due (>=250ms), Settings reloads the external reset cleanly
        let mut due = std::time::Instant::now() - std::time::Duration::from_millis(300);
        assert!(sync_external_config_if_due(
            &mut settings_cfg,
            &mut due,
            &path
        ));
        assert_eq!(
            settings_cfg.position_mode,
            DockPositionMode::TaskbarLeftOfTray
        );
        assert_eq!(settings_cfg.custom_pos_x, 0.0);
        assert_eq!(settings_cfg.custom_pos_y, 0.0);

        let _ = std::fs::remove_dir_all(&dir);
    }
}
