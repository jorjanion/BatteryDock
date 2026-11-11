#![windows_subsystem = "windows"]

mod battery;
mod config;
mod notifications;
mod ui;

use battery::engine::BatteryEngine;
use battery::DeviceBattery;
use config::{AppConfig, DockPositionMode};
use notifications::NotificationManager;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use ui::dock::DockOverlay;
use ui::settings_window::SettingsWindow;
use ui::taskbar::TaskbarGeometry;
use ui::tray::SystemTray;
use windows::core::w;
use windows::Win32::Foundation::{GetLastError, ERROR_ALREADY_EXISTS};
use windows::Win32::System::Threading::CreateMutexW;

struct BatteryDockApp {
    config: AppConfig,
    engine: BatteryEngine,
    notification_mgr: NotificationManager,
    dock: DockOverlay,
    devices: Vec<DeviceBattery>,
    open_settings: bool,
    open_settings_flag: Arc<AtomicBool>,
    reset_pos_flag: Arc<AtomicBool>,
    _tray: Option<SystemTray>,
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

        let open_settings_flag = Arc::new(AtomicBool::new(false));
        let open_settings_clone = open_settings_flag.clone();
        let reset_pos_flag = Arc::new(AtomicBool::new(false));
        let reset_pos_clone = reset_pos_flag.clone();
        let engine_clone = engine.clone();
        let ctx_clone = cc.egui_ctx.clone();

        let (quit_id, refresh_id, reset_id, restart_id, settings_id) = if let Some(t) = &tray {
            (
                Some(t.quit_item_id.clone()),
                Some(t.refresh_item_id.clone()),
                Some(t.reset_item_id.clone()),
                Some(t.restart_item_id.clone()),
                Some(t.settings_item_id.clone()),
            )
        } else {
            (None, None, None, None, None)
        };

        // Dedicated background thread for system tray:
        // Handles Exit, Restart, Reset, Refresh, and Settings completely decoupled from the GUI update loop!
        std::thread::spawn(move || {
            let menu_rx = tray_icon::menu::MenuEvent::receiver();
            let tray_rx = tray_icon::TrayIconEvent::receiver();
            let mut tick = 0u32;

            loop {
                // 1. Process menu clicks
                while let Ok(event) = menu_rx.try_recv() {
                    if let Some(qid) = &quit_id {
                        if &event.id == qid {
                            // Instant process exit
                            std::process::exit(0);
                        }
                    }
                    if let Some(reid) = &restart_id {
                        if &event.id == reid {
                            // Instant restart: launch delayed child that waits for mutex release, then exit
                            if let Ok(exe) = std::env::current_exe() {
                                let exe_str = exe.to_string_lossy().to_string();
                                let _ = std::process::Command::new("cmd")
                                    .args(["/c", "timeout /t 1 /nobreak >nul & start \"\""])
                                    .arg(exe_str)
                                    .spawn();
                            }
                            std::process::exit(0);
                        }
                    }
                    if let Some(rpid) = &reset_id {
                        if &event.id == rpid {
                            reset_pos_clone.store(true, Ordering::SeqCst);
                            ctx_clone.request_repaint();
                        }
                    }
                    if let Some(rid) = &refresh_id {
                        if &event.id == rid {
                            engine_clone.trigger_refresh();
                        }
                    }
                    if let Some(sid) = &settings_id {
                        if &event.id == sid {
                            open_settings_clone.store(true, Ordering::SeqCst);
                            ctx_clone.request_repaint();
                        }
                    }
                }

                // 2. Process tray icon clicks
                while let Ok(event) = tray_rx.try_recv() {
                    match event {
                        tray_icon::TrayIconEvent::Click {
                            button: tray_icon::MouseButton::Left,
                            button_state: tray_icon::MouseButtonState::Up,
                            ..
                        }
                        | tray_icon::TrayIconEvent::DoubleClick {
                            button: tray_icon::MouseButton::Left,
                            ..
                        } => {
                            let cur = open_settings_clone.load(Ordering::SeqCst);
                            open_settings_clone.store(!cur, Ordering::SeqCst);
                            ctx_clone.request_repaint();
                        }
                        _ => {}
                    }
                }

                // Heartbeat repaint request every ~150ms (5 * 30ms) to ensure event loop never hangs
                tick += 1;
                if tick >= 5 {
                    tick = 0;
                    ctx_clone.request_repaint();
                }

                std::thread::sleep(Duration::from_millis(30));
            }
        });

        Self {
            config,
            engine,
            notification_mgr: NotificationManager::new(),
            dock: DockOverlay::new(),
            devices: Vec::new(),
            open_settings: false,
            open_settings_flag,
            reset_pos_flag,
            _tray: tray,
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
        // 1. Process reset position request from tray
        if self.reset_pos_flag.swap(false, Ordering::SeqCst) {
            self.config.position_mode = DockPositionMode::TaskbarLeftOfTray;
            self.config.save();
            self.is_dock_hidden = false;
            let width = self.calculate_dock_width();
            self.reposition_dock_to_taskbar(ctx, width);
            self.engine.trigger_refresh();
            ctx.request_repaint();
        }

        // 2. Sync open_settings from atomic flag (e.g. if tray clicked)
        if self.open_settings_flag.swap(false, Ordering::SeqCst) {
            self.open_settings = true;
            // Ensure dock is not hidden when settings are opened
            self.is_dock_hidden = false;
            let width = self.calculate_dock_width();
            self.reposition_dock_to_taskbar(ctx, width);
            ctx.request_repaint();
        }

        // 3. Fullscreen app detection: hide dock when true fullscreen video or game is active
        let is_fullscreen = TaskbarGeometry::is_fullscreen_app_active();
        if is_fullscreen && !self.open_settings {
            if !self.is_dock_hidden {
                self.is_dock_hidden = true;
                // Make clickthrough and transparent without moving window off-screen
                ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(true));
            }
            ctx.request_repaint_after(Duration::from_millis(150));
            return;
        } else if self.is_dock_hidden {
            // Fullscreen has ended: restore interaction and redraw
            self.is_dock_hidden = false;
            ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(false));
            let width = self.calculate_dock_width();
            self.reposition_dock_to_taskbar(ctx, width);
            ctx.request_repaint();
        }

        // 2. Receive background battery updates
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

        // 3. Handle position mode changes from settings
        if self.config.position_mode != self.last_position_mode {
            self.last_position_mode = self.config.position_mode;
            let width = self.calculate_dock_width();
            self.reposition_dock_to_taskbar(ctx, width);
        }

        // 4. Render main dock overlay on taskbar
        let engine_ref = self.engine.clone();
        self.dock.render(
            ctx,
            &mut self.config,
            &self.devices,
            &mut self.open_settings,
            move || engine_ref.trigger_refresh(),
        );

        // 5. Render settings window if opened
        if self.open_settings {
            let engine_ref2 = self.engine.clone();
            SettingsWindow::show(
                ctx,
                &mut self.open_settings,
                &mut self.config,
                &self.devices,
                move || engine_ref2.trigger_refresh(),
            );
            if !self.open_settings {
                self.open_settings_flag.store(false, Ordering::SeqCst);
            }
        }
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
