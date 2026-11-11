#![windows_subsystem = "windows"]

mod battery;
mod config;
mod notifications;
mod ui;

use battery::engine::BatteryEngine;
use battery::DeviceBattery;
use config::{AppConfig, DockPositionMode};
use notifications::NotificationManager;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use ui::dock::DockOverlay;
use ui::taskbar::{
    apply_overlay_ex_style, ensure_window_above_monitor_taskbar, is_window_visible_win32,
    set_window_physical_rect, set_window_visible_noactivate, TaskbarGeometry,
};
use ui::tray::SystemTray;
use windows::core::w;
use windows::Win32::Foundation::{GetLastError, ERROR_ALREADY_EXISTS, RECT};
use windows::Win32::System::Threading::CreateMutexW;
use windows::Win32::UI::HiDpi::{
    SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
};
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, IsWindow, SetForegroundWindow, ShowWindow, SW_RESTORE,
};

#[derive(Clone, PartialEq)]
struct DockLayoutParams {
    mode: DockPositionMode,
    custom_x: f32,
    custom_y: f32,
    logical_w: f32,
    logical_h: f32,
}

struct DockSyncState {
    our_hwnd: AtomicIsize,
    is_dragging: AtomicBool,
    is_dock_hidden: AtomicBool,
    just_unhid: AtomicBool,
    last_hidden_refresh: Mutex<Instant>,
    layout: Mutex<DockLayoutParams>,
}

struct BatteryDockApp {
    config: AppConfig,
    engine: BatteryEngine,
    notification_mgr: NotificationManager,
    dock: DockOverlay,
    devices: Vec<DeviceBattery>,
    open_settings: bool,
    reset_pos_flag: Arc<AtomicBool>,
    _tray: Option<SystemTray>,
    our_hwnd: isize,
    sync: Arc<DockSyncState>,
    last_dock_width: f32,
    last_position_mode: DockPositionMode,
    last_config_reload: Instant,
}

static MUTEX_HANDLE: std::sync::atomic::AtomicPtr<std::ffi::c_void> =
    std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

pub fn restart_app() {
    unsafe {
        let h = MUTEX_HANDLE.swap(std::ptr::null_mut(), Ordering::SeqCst);
        if !h.is_null() {
            let _ = windows::Win32::Foundation::CloseHandle(windows::Win32::Foundation::HANDLE(h));
        }

        if let Ok(exe) = std::env::current_exe() {
            let exe_str = exe.to_string_lossy().to_string();
            let exe_hstring = windows::core::HSTRING::from(exe_str);
            let open_hstring = windows::core::HSTRING::from("open");
            windows::Win32::UI::Shell::ShellExecuteW(
                windows::Win32::Foundation::HWND(std::ptr::null_mut()),
                windows::core::PCWSTR(open_hstring.as_ptr()),
                windows::core::PCWSTR(exe_hstring.as_ptr()),
                windows::core::PCWSTR(std::ptr::null()),
                windows::core::PCWSTR(std::ptr::null()),
                windows::Win32::UI::WindowsAndMessaging::SW_SHOW,
            );
        }
        std::process::exit(0);
    }
}

pub fn open_settings_window() {
    unsafe {
        if let Ok(existing) = FindWindowW(None, w!("BatteryDock Settings")) {
            if !existing.0.is_null() && IsWindow(existing).as_bool() {
                let _ = ShowWindow(existing, SW_RESTORE);
                let _ = SetForegroundWindow(existing);
                return;
            }
        }

        if let Ok(exe) = std::env::current_exe() {
            let exe_str = exe.to_string_lossy().to_string();
            let exe_hstring = windows::core::HSTRING::from(exe_str);
            let open_hstring = windows::core::HSTRING::from("open");
            let param_hstring = windows::core::HSTRING::from("--settings");
            windows::Win32::UI::Shell::ShellExecuteW(
                windows::Win32::Foundation::HWND(std::ptr::null_mut()),
                windows::core::PCWSTR(open_hstring.as_ptr()),
                windows::core::PCWSTR(exe_hstring.as_ptr()),
                windows::core::PCWSTR(param_hstring.as_ptr()),
                windows::core::PCWSTR(std::ptr::null()),
                windows::Win32::UI::WindowsAndMessaging::SW_SHOW,
            );
        }
    }
}

fn compute_target_physical_rect(
    params: &DockLayoutParams,
    tb: &TaskbarGeometry,
) -> (RECT, i32, i32) {
    let custom_probe_w = params.logical_w * tb.dpi_scale;
    let custom_probe_h = params.logical_h * tb.dpi_scale;
    let scale = if params.mode == DockPositionMode::Custom
        && TaskbarGeometry::is_custom_rect_on_any_monitor(
            params.custom_x,
            params.custom_y,
            custom_probe_w,
            custom_probe_h,
        ) {
        let probe = RECT {
            left: params.custom_x.round() as i32,
            top: params.custom_y.round() as i32,
            right: (params.custom_x + custom_probe_w).round() as i32,
            bottom: (params.custom_y + custom_probe_h).round() as i32,
        };
        TaskbarGeometry::dpi_scale_for_rect(probe)
    } else {
        tb.dpi_scale
    };

    let (tx, ty) = tb.calculate_dock_position_scaled(
        params.mode,
        (params.logical_w, params.logical_h),
        (params.custom_x, params.custom_y),
        scale,
        TaskbarGeometry::is_custom_rect_on_any_monitor,
    );

    let phys_w = (params.logical_w * scale).round() as i32;
    let tb_h = tb.taskbar_rect.bottom - tb.taskbar_rect.top;
    let raw_phys_h = (params.logical_h * scale).round() as i32;
    let phys_h = if params.mode == DockPositionMode::Custom {
        raw_phys_h
    } else {
        raw_phys_h.min((tb_h - 4).max(16))
    };

    let x = tx.round() as i32;
    let y = ty.round() as i32;
    let rect = RECT {
        left: x,
        top: y,
        right: x + phys_w,
        bottom: y + phys_h,
    };
    (rect, phys_w, phys_h)
}

fn refresh_layout_from_disk_and_engine(
    sync: &DockSyncState,
    engine: &BatteryEngine,
) -> DockLayoutParams {
    let devices = engine.latest_snapshot();
    if let Some(cfg) = AppConfig::try_load() {
        let logical_w = DockOverlay::required_width(&devices, &cfg);
        let mut layout = sync.layout.lock().unwrap();
        layout.mode = cfg.position_mode;
        layout.custom_x = cfg.custom_pos_x;
        layout.custom_y = cfg.custom_pos_y;
        layout.logical_w = logical_w;
        layout.logical_h = 32.0;
        return layout.clone();
    }
    sync.layout.lock().unwrap().clone()
}

fn sync_dock_window_once(
    sync: &DockSyncState,
    engine: Option<&BatteryEngine>,
    ctx: Option<&egui::Context>,
) {
    let hwnd = sync.our_hwnd.load(Ordering::Acquire);
    if hwnd == 0 {
        return;
    }
    if sync.is_dragging.load(Ordering::Acquire) {
        return;
    }

    let currently_visible = is_window_visible_win32(hwnd);
    let mut params = if !currently_visible {
        if let Some(eng) = engine {
            let mut due = false;
            if let Ok(mut last) = sync.last_hidden_refresh.lock() {
                if last.elapsed() >= Duration::from_millis(250) {
                    *last = Instant::now();
                    due = true;
                }
            }
            if due {
                refresh_layout_from_disk_and_engine(sync, eng)
            } else {
                sync.layout.lock().unwrap().clone()
            }
        } else {
            sync.layout.lock().unwrap().clone()
        }
    } else {
        sync.layout.lock().unwrap().clone()
    };

    apply_overlay_ex_style(hwnd);

    let Some(tb) = TaskbarGeometry::detect() else {
        return;
    };

    let (mut target_rect, mut phys_w, mut phys_h) = compute_target_physical_rect(&params, &tb);
    let mut should_be_visible =
        TaskbarGeometry::should_dock_be_visible(params.mode, target_rect, std::process::id());

    if should_be_visible && !currently_visible {
        if let Some(eng) = engine {
            params = refresh_layout_from_disk_and_engine(sync, eng);
            if let Ok(mut last) = sync.last_hidden_refresh.lock() {
                *last = Instant::now();
            }
            (target_rect, phys_w, phys_h) = compute_target_physical_rect(&params, &tb);
            should_be_visible = TaskbarGeometry::should_dock_be_visible(
                params.mode,
                target_rect,
                std::process::id(),
            );
        }
    }

    if !should_be_visible {
        if set_window_visible_noactivate(hwnd, false) {
            sync.is_dock_hidden.store(true, Ordering::Release);
        }
    } else {
        set_window_physical_rect(hwnd, target_rect.left, target_rect.top, phys_w, phys_h);
        ensure_window_above_monitor_taskbar(hwnd, target_rect);
        if !currently_visible {
            set_window_visible_noactivate(hwnd, true);
            ensure_window_above_monitor_taskbar(hwnd, target_rect);
            sync.is_dock_hidden.store(false, Ordering::Release);
            sync.just_unhid.store(true, Ordering::Release);
            if let Some(c) = ctx {
                c.request_repaint();
            }
        }
    }
}

impl BatteryDockApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::TRANSPARENT;
        visuals.window_fill = egui::Color32::from_rgb(22, 24, 29);
        visuals.window_stroke = egui::Stroke::new(1.0_f32, egui::Color32::from_white_alpha(35));
        cc.egui_ctx.set_visuals(visuals);

        let our_hwnd: isize = cc
            .window_handle()
            .ok()
            .and_then(|h| match h.as_raw() {
                RawWindowHandle::Win32(win32) => Some(win32.hwnd.get()),
                _ => None,
            })
            .unwrap_or(0);

        let config = AppConfig::load();
        let initial_devices: Vec<DeviceBattery> = Vec::new();
        let initial_w = DockOverlay::required_width(&initial_devices, &config);
        let initial_mode = config.position_mode;

        let sync = Arc::new(DockSyncState {
            our_hwnd: AtomicIsize::new(our_hwnd),
            is_dragging: AtomicBool::new(false),
            is_dock_hidden: AtomicBool::new(false),
            just_unhid: AtomicBool::new(false),
            last_hidden_refresh: Mutex::new(Instant::now()),
            layout: Mutex::new(DockLayoutParams {
                mode: config.position_mode,
                custom_x: config.custom_pos_x,
                custom_y: config.custom_pos_y,
                logical_w: initial_w,
                logical_h: 32.0,
            }),
        });

        // Immediately enforce overlay styles, initial taskbar rect, Z-order, and fullscreen check before frame 1
        sync_dock_window_once(&sync, None, None);

        let engine = BatteryEngine::start(Duration::from_secs(config.poll_interval_secs));
        let tray = SystemTray::new().ok();

        let reset_pos_flag = Arc::new(AtomicBool::new(false));
        let reset_pos_clone = reset_pos_flag.clone();
        let engine_clone = engine.clone();
        let ctx_tray = cc.egui_ctx.clone();
        let sync_tray = sync.clone();

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

        // Dedicated System Tray listener thread (works even while dock is SW_HIDE in fullscreen apps)
        std::thread::spawn(move || {
            let menu_rx = tray_icon::menu::MenuEvent::receiver();
            let tray_rx = tray_icon::TrayIconEvent::receiver();

            loop {
                while let Ok(event) = menu_rx.try_recv() {
                    if let Some(qid) = &quit_id {
                        if event.id == *qid {
                            std::process::exit(0);
                        }
                    }
                    if let Some(reid) = &restart_id {
                        if event.id == *reid {
                            restart_app();
                        }
                    }
                    if let Some(rpid) = &reset_id {
                        if event.id == *rpid {
                            let mut cfg = AppConfig::load();
                            cfg.position_mode = DockPositionMode::TaskbarLeftOfTray;
                            cfg.custom_pos_x = 0.0;
                            cfg.custom_pos_y = 0.0;
                            cfg.save();
                            {
                                let mut layout = sync_tray.layout.lock().unwrap();
                                layout.mode = DockPositionMode::TaskbarLeftOfTray;
                                layout.custom_x = 0.0;
                                layout.custom_y = 0.0;
                            }
                            reset_pos_clone.store(true, Ordering::SeqCst);
                            engine_clone.trigger_refresh();
                            ctx_tray.request_repaint();
                        }
                    }
                    if let Some(rid) = &refresh_id {
                        if event.id == *rid {
                            engine_clone.trigger_refresh();
                        }
                    }
                    if let Some(sid) = &settings_id {
                        if event.id == *sid {
                            open_settings_window();
                        }
                    }
                }

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
                            open_settings_window();
                        }
                        _ => {}
                    }
                }

                std::thread::sleep(Duration::from_millis(30));
            }
        });

        // Dedicated 25ms Win32 visibility, Z-order (above Shell_TrayWnd), and taskbar sync thread
        let sync_watcher = sync.clone();
        let engine_watcher = engine.clone();
        let ctx_watcher = cc.egui_ctx.clone();
        std::thread::spawn(move || loop {
            sync_dock_window_once(&sync_watcher, Some(&engine_watcher), Some(&ctx_watcher));
            std::thread::sleep(Duration::from_millis(25));
        });

        Self {
            config,
            engine,
            notification_mgr: NotificationManager::new(),
            dock: DockOverlay::new(),
            devices: initial_devices,
            open_settings: false,
            reset_pos_flag,
            _tray: tray,
            our_hwnd,
            sync,
            last_dock_width: initial_w,
            last_position_mode: initial_mode,
            last_config_reload: Instant::now(),
        }
    }

    fn sync_layout_params(&self, width: f32) {
        let mut layout = self.sync.layout.lock().unwrap();
        layout.mode = self.config.position_mode;
        layout.custom_x = self.config.custom_pos_x;
        layout.custom_y = self.config.custom_pos_y;
        layout.logical_w = width;
        layout.logical_h = 32.0;
    }
}

impl eframe::App for BatteryDockApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Handle reset position signal from tray
        if self.reset_pos_flag.swap(false, Ordering::SeqCst) {
            self.config.position_mode = DockPositionMode::TaskbarLeftOfTray;
            self.config.custom_pos_x = 0.0;
            self.config.custom_pos_y = 0.0;
            self.last_position_mode = self.config.position_mode;
            self.engine.trigger_refresh();
        }

        // 2. Reload ALL config fields from disk before processing notifications
        let forced_reload = self.sync.just_unhid.swap(false, Ordering::AcqRel);
        if !self.dock.is_dragging()
            && (forced_reload || self.last_config_reload.elapsed() >= Duration::from_millis(500))
        {
            self.last_config_reload = Instant::now();
            if let Some(loaded) = AppConfig::try_load() {
                if loaded != self.config {
                    self.config = loaded;
                    self.last_position_mode = self.config.position_mode;
                }
            }
        }

        // 3. Drain all pending battery updates and evaluate notifications only on the latest snapshot
        let mut latest_devices = None;
        while let Ok(updated_devices) = self.engine.receiver().try_recv() {
            latest_devices = Some(updated_devices);
        }
        if let Some(updated_devices) = latest_devices {
            self.notification_mgr
                .check_and_notify(&updated_devices, &self.config);
            self.devices = updated_devices;
        }

        // 4. Resize viewport if device list, charging state, or percentage text toggle changed pill width
        let required_w = DockOverlay::required_width(&self.devices, &self.config);
        if (required_w - self.last_dock_width).abs() > 0.5 {
            self.last_dock_width = required_w;
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::Vec2::new(
                required_w, 32.0,
            )));
        }

        if self.config.position_mode != self.last_position_mode {
            self.last_position_mode = self.config.position_mode;
        }

        // 5. Render main dock overlay
        let engine_ref = self.engine.clone();
        self.dock.render(
            ctx,
            &mut self.config,
            &self.devices,
            self.our_hwnd,
            &mut self.open_settings,
            move || engine_ref.trigger_refresh(),
        );

        self.sync
            .is_dragging
            .store(self.dock.is_dragging(), Ordering::Release);
        self.sync_layout_params(self.last_dock_width);

        // 6. Open standalone settings window if triggered via gear icon, dock click, or context menu
        if self.open_settings {
            self.open_settings = false;
            open_settings_window();
        }
    }
}

fn main() -> eframe::Result<()> {
    // Enable Per-Monitor V2 DPI awareness before querying monitor/taskbar geometry
    unsafe {
        let _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
    }

    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--settings" || a == "-s") {
        return ui::settings_window::run_standalone_settings();
    }

    unsafe {
        let mutex = CreateMutexW(None, true, w!("Global\\BatteryDockSingleInstanceMutex"));
        if GetLastError() == ERROR_ALREADY_EXISTS {
            eprintln!("BatteryDock is already running.");
            return Ok(());
        }
        if let Ok(h) = mutex {
            MUTEX_HANDLE.store(h.0, Ordering::SeqCst);
        }
    }

    let initial_config = AppConfig::load();
    let initial_width = DockOverlay::required_width(&[], &initial_config);
    let initial_height = 32.0;

    let (start_x_logical, start_y_logical) = if let Some(tb) = TaskbarGeometry::detect() {
        let (px, py) = tb.calculate_dock_position(
            initial_config.position_mode,
            initial_width,
            initial_height,
            initial_config.custom_pos_x,
            initial_config.custom_pos_y,
        );
        let scale = tb.dpi_scale.max(0.75);
        (px / scale, py / scale)
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
            .with_position([start_x_logical, start_y_logical])
            .with_taskbar(false),
        ..Default::default()
    };

    eframe::run_native(
        "BatteryDock",
        native_options,
        Box::new(|cc| Ok(Box::new(BatteryDockApp::new(cc)))),
    )
}
