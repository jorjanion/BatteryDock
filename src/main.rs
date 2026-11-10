#![windows_subsystem = "windows"]

mod battery;
mod config;
mod notifications;
mod ui;

use battery::engine::BatteryEngine;
use battery::DeviceBattery;
use config::{AppConfig, DockPositionMode};
use notifications::NotificationManager;
use std::time::Duration;
use ui::dock::DockOverlay;
use ui::settings_window::SettingsWindow;
use ui::taskbar::TaskbarGeometry;
use ui::tray::SystemTray;
use windows::core::w;
use windows::Win32::Foundation::{GetLastError, ERROR_ALREADY_EXISTS};
use windows::Win32::System::Threading::CreateMutexW;

enum TrayAction {
    Menu(tray_icon::menu::MenuEvent),
    Tray(tray_icon::TrayIconEvent),
}

struct BatteryDockApp {
    config: AppConfig,
    engine: BatteryEngine,
    notification_mgr: NotificationManager,
    dock: DockOverlay,
    devices: Vec<DeviceBattery>,
    open_settings: bool,
    _tray: Option<SystemTray>,
    tray_action_rx: crossbeam_channel::Receiver<TrayAction>,
    last_device_count: usize,
    last_position_mode: DockPositionMode,
    is_dock_hidden: bool,
}

impl BatteryDockApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure visual styles for egui
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::TRANSPARENT;
        visuals.window_fill = egui::Color32::from_rgb(22, 24, 29);
        visuals.window_stroke = egui::Stroke::new(1.0_f32, egui::Color32::from_white_alpha(35));
        cc.egui_ctx.set_visuals(visuals);

        let config = AppConfig::load();
        let engine = BatteryEngine::start(Duration::from_secs(config.poll_interval_secs));
        let tray = SystemTray::new().ok();

        let (tray_action_tx, tray_action_rx) = crossbeam_channel::unbounded::<TrayAction>();

        // Register event handlers for tray and menu so events are queued and wake up egui immediately
        let tx = tray_action_tx.clone();
        let ctx_clone = cc.egui_ctx.clone();
        tray_icon::menu::MenuEvent::set_event_handler(Some(move |event| {
            let _ = tx.send(TrayAction::Menu(event));
            ctx_clone.request_repaint();
        }));

        let tx2 = tray_action_tx.clone();
        let ctx_clone2 = cc.egui_ctx.clone();
        tray_icon::TrayIconEvent::set_event_handler(Some(move |event| {
            let _ = tx2.send(TrayAction::Tray(event));
            ctx_clone2.request_repaint();
        }));

        // Dedicated fast fullscreen watcher thread (120ms check, wakes up egui instantly on state change)
        let ctx_fs = cc.egui_ctx.clone();
        std::thread::spawn(move || {
            let mut last_fs = false;
            loop {
                std::thread::sleep(Duration::from_millis(120));
                let fs = TaskbarGeometry::is_fullscreen_app_active();
                if fs != last_fs {
                    last_fs = fs;
                    ctx_fs.request_repaint();
                }
            }
        });

        Self {
            config,
            engine,
            notification_mgr: NotificationManager::new(),
            dock: DockOverlay::new(),
            devices: Vec::new(),
            open_settings: false,
            _tray: tray,
            tray_action_rx,
            last_device_count: 0,
            last_position_mode: DockPositionMode::TaskbarLeftOfTray,
            is_dock_hidden: false,
        }
    }

    fn calculate_dock_width(&self) -> f32 {
        if self.devices.is_empty() {
            135.0
        } else {
            let dev_width = if self.config.show_percentage_text { 76.0 } else { 48.0 };
            (self.devices.len() as f32 * dev_width + 30.0).max(120.0)
        }
    }

    fn reposition_dock_to_taskbar(&self, ctx: &egui::Context, width: f32) {
        if let Some(tb) = TaskbarGeometry::detect() {
            let (target_x, target_y) = tb.calculate_dock_position(
                self.config.position_mode,
                width,
                32.0,
                self.config.custom_pos_x,
                self.config.custom_pos_y,
            );
            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(egui::Pos2::new(
                target_x, target_y,
            )));
        }
    }
}

impl eframe::App for BatteryDockApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Process all tray menu & click actions first
        while let Ok(action) = self.tray_action_rx.try_recv() {
            match action {
                TrayAction::Menu(menu_event) => {
                    if let Some(tray) = &self._tray {
                        if menu_event.id == tray.settings_item_id {
                            self.open_settings = true;
                            ctx.request_repaint();
                        } else if menu_event.id == tray.refresh_item_id {
                            self.engine.trigger_refresh();
                        } else if menu_event.id == tray.quit_item_id {
                            std::process::exit(0);
                        }
                    }
                }
                TrayAction::Tray(tray_event) => {
                    match tray_event {
                        tray_icon::TrayIconEvent::Click {
                            button: tray_icon::MouseButton::Left,
                            button_state: tray_icon::MouseButtonState::Up,
                            ..
                        }
                        | tray_icon::TrayIconEvent::DoubleClick {
                            button: tray_icon::MouseButton::Left,
                            ..
                        } => {
                            self.open_settings = !self.open_settings;
                            ctx.request_repaint();
                        }
                        _ => {}
                    }
                }
            }
        }

        // 2. Fullscreen app detection: when in fullscreen, pass clicks through and do not render dock
        let is_fullscreen = TaskbarGeometry::is_fullscreen_app_active();
        if is_fullscreen {
            if !self.is_dock_hidden {
                self.is_dock_hidden = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(true));
            }
            // If settings window was opened, keep rendering it
            let engine_ref2 = self.engine.clone();
            SettingsWindow::show(
                ctx,
                &mut self.open_settings,
                &mut self.config,
                &self.devices,
                move || engine_ref2.trigger_refresh(),
            );
            return;
        } else if self.is_dock_hidden {
            self.is_dock_hidden = false;
            ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(false));
        }

        // 3. Receive background battery updates
        if let Ok(updated_devices) = self.engine.receiver().try_recv() {
            self.notification_mgr
                .check_and_notify(&updated_devices, &self.config);
            self.devices = updated_devices;

            // Resize and reposition if device count changed
            if self.devices.len() != self.last_device_count {
                self.last_device_count = self.devices.len();
                let width = self.calculate_dock_width();
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::Vec2::new(
                    width, 32.0,
                )));

                if self.config.position_mode != DockPositionMode::Custom {
                    self.reposition_dock_to_taskbar(ctx, width);
                }
            }
        }

        // 4. Handle position mode changes from settings
        if self.config.position_mode != self.last_position_mode {
            self.last_position_mode = self.config.position_mode;
            let width = self.calculate_dock_width();
            self.reposition_dock_to_taskbar(ctx, width);
        }

        // 5. Render main dock overlay on taskbar
        let engine_ref = self.engine.clone();
        self.dock.render(
            ctx,
            &mut self.config,
            &self.devices,
            &mut self.open_settings,
            move || engine_ref.trigger_refresh(),
        );

        // 6. Render settings window if opened
        let engine_ref2 = self.engine.clone();
        SettingsWindow::show(
            ctx,
            &mut self.open_settings,
            &mut self.config,
            &self.devices,
            move || engine_ref2.trigger_refresh(),
        );
    }
}

fn main() -> eframe::Result<()> {
    // Ensure single instance running
    unsafe {
        let _ = CreateMutexW(None, true, w!("Global\\BatteryDockSingleInstanceMutex"));
        if GetLastError() == ERROR_ALREADY_EXISTS {
            eprintln!("BatteryDock is already running.");
            return Ok(());
        }
    }

    let initial_config = AppConfig::load();
    let initial_width = 135.0;
    let initial_height = 32.0;

    let (start_x, start_y) = if let Some(tb) = TaskbarGeometry::detect() {
        tb.calculate_dock_position(
            initial_config.position_mode,
            initial_width,
            initial_height,
            initial_config.custom_pos_x,
            initial_config.custom_pos_y,
        )
    } else {
        (100.0, 100.0)
    };

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("BatteryDock")
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_resizable(false)
            .with_inner_size([initial_width, initial_height])
            .with_position([start_x, start_y])
            .with_taskbar(false),
        ..Default::default()
    };

    eframe::run_native(
        "BatteryDock",
        native_options,
        Box::new(|cc| Ok(Box::new(BatteryDockApp::new(cc)))),
    )
}
