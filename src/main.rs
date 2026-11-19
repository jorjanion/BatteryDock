#![windows_subsystem = "windows"]

mod battery;
pub mod config;
pub mod i18n;
mod notifications;
mod ui;
pub mod updater;

use battery::engine::BatteryEngine;
use battery::DeviceBattery;
use config::{AppConfig, DockPositionMode, OverlayCorner, OverlayHotkey};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use ui::dock::DockOverlay;
use ui::taskbar::{
    apply_overlay_ex_style, calculate_overlay_physical_rect, ensure_window_above_monitor_taskbar,
    ensure_window_topmost_standalone, get_active_foreground_monitor_info,
    is_hotkey_currently_pressed, is_window_visible_win32, make_window_transparent,
    set_window_physical_rect, set_window_visible_noactivate, TaskbarGeometry,
};
use ui::tray::SystemTray;
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{GetLastError, ERROR_ALREADY_EXISTS, HWND, RECT};
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
    game_overlay_enabled: bool,
    game_overlay_corner: OverlayCorner,
    game_overlay_margin: f32,
    game_overlay_custom_x_pct: f32,
    game_overlay_custom_y_pct: f32,
    game_overlay_always_on: bool,
    game_overlay_hotkey_enabled: bool,
    game_overlay_hotkey: OverlayHotkey,
}

struct DockSyncState {
    our_hwnd: AtomicIsize,
    is_dragging: AtomicBool,
    is_dock_hidden: AtomicBool,
    is_game_overlay_active: AtomicBool,
    hotkey_was_down: AtomicBool,
    just_unhid: AtomicBool,
    last_hidden_refresh: Mutex<Instant>,
    layout: Mutex<DockLayoutParams>,
}

struct BatteryDockApp {
    config: AppConfig,
    engine: BatteryEngine,
    dock: DockOverlay,
    secondary_dock: DockOverlay,
    devices: Vec<DeviceBattery>,
    open_settings: bool,
    reset_pos_flag: Arc<AtomicBool>,
    _tray: Option<SystemTray>,
    our_hwnd: isize,
    sync: Arc<DockSyncState>,
    last_dock_width: f32,
    last_position_mode: DockPositionMode,
    last_language: i18n::Language,
    last_mouse_passthrough: bool,
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
            if std::process::Command::new(&exe).arg("--settings").spawn().is_err() {
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
        layout.game_overlay_enabled = cfg.game_overlay_enabled;
        layout.game_overlay_corner = cfg.game_overlay_corner;
        layout.game_overlay_margin = cfg.game_overlay_margin;
        layout.game_overlay_custom_x_pct = cfg.game_overlay_custom_x_pct;
        layout.game_overlay_custom_y_pct = cfg.game_overlay_custom_y_pct;
        layout.game_overlay_always_on = cfg.game_overlay_always_on;
        layout.game_overlay_hotkey_enabled = cfg.game_overlay_hotkey_enabled;
        layout.game_overlay_hotkey = cfg.game_overlay_hotkey;
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

    // Global edge-triggered hotkey to toggle Gaming / Fullscreen HUD Overlay on the fly
    if params.game_overlay_hotkey_enabled {
        let pressed = is_hotkey_currently_pressed(&params.game_overlay_hotkey);
        let was_pressed = sync.hotkey_was_down.swap(pressed, Ordering::AcqRel);
        if pressed && !was_pressed {
            let mut cfg = AppConfig::try_load().unwrap_or_default();
            cfg.game_overlay_enabled = !cfg.game_overlay_enabled;
            cfg.save();
            {
                let mut layout = sync.layout.lock().unwrap();
                layout.game_overlay_enabled = cfg.game_overlay_enabled;
            }
            params.game_overlay_enabled = cfg.game_overlay_enabled;
            sync.just_unhid.store(true, Ordering::Release);
            if let Some(c) = ctx {
                c.request_repaint();
            }
        }
    } else {
        sync.hotkey_was_down.store(false, Ordering::Release);
    }

    apply_overlay_ex_style(hwnd);

    let Some(tb) = TaskbarGeometry::detect() else {
        return;
    };

    let resolve_layout = |p: &DockLayoutParams| -> (RECT, i32, i32, bool, bool) {
        let (target_rect, phys_w, phys_h) = compute_target_physical_rect(p, &tb);
        let taskbar_visible =
            TaskbarGeometry::should_dock_be_visible(p.mode, target_rect, std::process::id());

        let is_game_hud = p.game_overlay_enabled && (!taskbar_visible || p.game_overlay_always_on);
        if is_game_hud {
            let (mon_rect, dpi) = if !taskbar_visible {
                get_active_foreground_monitor_info(tb.monitor_rect, tb.dpi_scale)
            } else {
                (tb.monitor_rect, tb.dpi_scale)
            };
            let (hud_rect, hw, hh) = calculate_overlay_physical_rect(
                mon_rect,
                dpi,
                p.logical_w,
                p.logical_h,
                p.game_overlay_corner,
                p.game_overlay_margin,
                p.game_overlay_custom_x_pct,
                p.game_overlay_custom_y_pct,
            );
            (hud_rect, hw, hh, true, true)
        } else {
            (target_rect, phys_w, phys_h, false, taskbar_visible)
        }
    };

    let (mut final_rect, mut final_w, mut final_h, mut is_game_hud, mut should_be_visible) =
        resolve_layout(&params);

    if should_be_visible && !currently_visible {
        if let Some(eng) = engine {
            params = refresh_layout_from_disk_and_engine(sync, eng);
            if let Ok(mut last) = sync.last_hidden_refresh.lock() {
                *last = Instant::now();
            }
            (final_rect, final_w, final_h, is_game_hud, should_be_visible) =
                resolve_layout(&params);
        }
    }

    let was_game_hud = sync
        .is_game_overlay_active
        .swap(is_game_hud, Ordering::AcqRel);
    if was_game_hud != is_game_hud {
        if let Some(c) = ctx {
            c.request_repaint();
        }
    }

    if !should_be_visible {
        if set_window_visible_noactivate(hwnd, false) {
            sync.is_dock_hidden.store(true, Ordering::Release);
        }
    } else {
        set_window_physical_rect(hwnd, final_rect.left, final_rect.top, final_w, final_h);
        if is_game_hud {
            ensure_window_topmost_standalone(hwnd);
        } else {
            ensure_window_above_monitor_taskbar(hwnd, final_rect);
        }
        if !currently_visible {
            set_window_visible_noactivate(hwnd, true);
            if is_game_hud {
                ensure_window_topmost_standalone(hwnd);
            } else {
                ensure_window_above_monitor_taskbar(hwnd, final_rect);
            }
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
        i18n::configure_egui_fonts(&cc.egui_ctx, config.language, false);
        let initial_devices: Vec<DeviceBattery> = Vec::new();
        let initial_w = DockOverlay::required_width(&initial_devices, &config);
        let initial_mode = config.position_mode;
        let initial_lang = config.language;

        let sync = Arc::new(DockSyncState {
            our_hwnd: AtomicIsize::new(our_hwnd),
            is_dragging: AtomicBool::new(false),
            is_dock_hidden: AtomicBool::new(false),
            is_game_overlay_active: AtomicBool::new(false),
            hotkey_was_down: AtomicBool::new(false),
            just_unhid: AtomicBool::new(false),
            last_hidden_refresh: Mutex::new(Instant::now()),
            layout: Mutex::new(DockLayoutParams {
                mode: config.position_mode,
                custom_x: config.custom_pos_x,
                custom_y: config.custom_pos_y,
                logical_w: initial_w,
                logical_h: 32.0,
                game_overlay_enabled: config.game_overlay_enabled,
                game_overlay_corner: config.game_overlay_corner,
                game_overlay_margin: config.game_overlay_margin,
                game_overlay_custom_x_pct: config.game_overlay_custom_x_pct,
                game_overlay_custom_y_pct: config.game_overlay_custom_y_pct,
                game_overlay_always_on: config.game_overlay_always_on,
                game_overlay_hotkey_enabled: config.game_overlay_hotkey_enabled,
                game_overlay_hotkey: config.game_overlay_hotkey,
            }),
        });

        // Immediately enforce overlay styles, initial taskbar rect, Z-order, and fullscreen check before frame 1
        sync_dock_window_once(&sync, None, None);

        let engine = BatteryEngine::start_with_notifications(
            Duration::from_secs(config.poll_interval_secs.max(1)),
            true,
        );

        if config.auto_update {
            updater::check_for_updates_async(true);
        }

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
            dock: DockOverlay::new(),
            secondary_dock: DockOverlay::new(),
            devices: initial_devices,
            open_settings: false,
            reset_pos_flag,
            _tray: tray,
            our_hwnd,
            sync,
            last_dock_width: initial_w,
            last_position_mode: initial_mode,
            last_language: initial_lang,
            last_mouse_passthrough: false,
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
        layout.game_overlay_enabled = self.config.game_overlay_enabled;
        layout.game_overlay_corner = self.config.game_overlay_corner;
        layout.game_overlay_margin = self.config.game_overlay_margin;
        layout.game_overlay_custom_x_pct = self.config.game_overlay_custom_x_pct;
        layout.game_overlay_custom_y_pct = self.config.game_overlay_custom_y_pct;
        layout.game_overlay_always_on = self.config.game_overlay_always_on;
        layout.game_overlay_hotkey_enabled = self.config.game_overlay_hotkey_enabled;
        layout.game_overlay_hotkey = self.config.game_overlay_hotkey;
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
                    if self.config.language != self.last_language {
                        self.last_language = self.config.language;
                        i18n::configure_egui_fonts(ctx, self.config.language, false);
                    }
                }
            }
        }

        // 3. Drain any pending battery updates (notifications run on BatteryEngine even when SW_HIDE)
        let mut latest_devices = None;
        while let Ok(updated_devices) = self.engine.receiver().try_recv() {
            latest_devices = Some(updated_devices);
        }
        if let Some(updated_devices) = latest_devices {
            self.devices = updated_devices;
            ctx.request_repaint();
        }
        if forced_reload {
            let snapshot = self.engine.latest_snapshot();
            if !snapshot.is_empty() || !self.devices.is_empty() {
                self.devices = snapshot;
                ctx.request_repaint();
            }
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

        // Toggle native click-through when in Gaming / Fullscreen HUD Overlay mode
        let in_game_hud = self.sync.is_game_overlay_active.load(Ordering::Acquire);
        let want_passthrough = in_game_hud && self.config.game_overlay_click_through;
        if want_passthrough != self.last_mouse_passthrough {
            self.last_mouse_passthrough = want_passthrough;
            ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(want_passthrough));
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

        // 5b. Render secondary dock overlays on all other active monitors if enabled
        if self.config.show_on_all_monitors {
            let all_monitors = TaskbarGeometry::detect_all();
            for mon in all_monitors.into_iter().filter(|m| !m.is_primary) {
                let mon_id = mon.monitor_id;
                let vp_id = egui::ViewportId::from_hash_of(("battery_dock_secondary", mon_id));
                let win_title = format!("BatteryDock-Secondary-{}", mon_id);

                let scale = mon.geometry.dpi_scale.max(0.75);
                let phys_w = (required_w * scale).round() as i32;
                let tb = &mon.geometry.taskbar_rect;
                let tb_h = (tb.bottom - tb.top) as f32;
                let phys_h = ((32.0 * scale).round() as i32).min((tb_h - 4.0).max(16.0) as i32);
                let center_y = tb.top as f32 + (tb_h - phys_h as f32) / 2.0;

                let target_x = if let Some(tray) = &mon.geometry.tray_rect {
                    if tray.right > tray.left
                        && tray.left > tb.left + (60.0 * scale) as i32
                        && tray.left < tb.right
                    {
                        (tray.left as f32 - phys_w as f32 - (12.0 * scale))
                            .max(tb.left as f32 + (8.0 * scale))
                    } else {
                        (tb.right as f32 - (155.0 * scale) - phys_w as f32)
                            .max(tb.left as f32 + (8.0 * scale))
                    }
                } else if self.config.position_mode == DockPositionMode::TaskbarLeft {
                    tb.left as f32 + (65.0 * scale)
                } else {
                    (tb.right as f32 - (155.0 * scale) - phys_w as f32)
                        .max(tb.left as f32 + (8.0 * scale))
                };

                let target_rect = RECT {
                    left: target_x.round() as i32,
                    top: center_y.round() as i32,
                    right: (target_x + phys_w as f32).round() as i32,
                    bottom: (center_y + phys_h as f32).round() as i32,
                };

                let should_be_visible = TaskbarGeometry::should_dock_be_visible(
                    DockPositionMode::TaskbarLeftOfTray,
                    target_rect,
                    std::process::id(),
                );

                let start_x_logical = target_x / scale;
                let start_y_logical = center_y / scale;

                let vp_builder = egui::ViewportBuilder::default()
                    .with_title(win_title.clone())
                    .with_decorations(false)
                    .with_transparent(true)
                    .with_always_on_top()
                    .with_resizable(false)
                    .with_inner_size([required_w, 32.0])
                    .with_position([start_x_logical, start_y_logical])
                    .with_taskbar(false)
                    .with_active(false)
                    .with_visible(should_be_visible);

                let engine_ref = self.engine.clone();
                let our_hwnd = self.our_hwnd;
                let devices_ref = &self.devices;
                let config_ref = &mut self.config;
                let open_settings_ref = &mut self.open_settings;
                let secondary_dock_ref = &mut self.secondary_dock;

                ctx.show_viewport_immediate(vp_id, vp_builder, |ctx, _class| {
                    unsafe {
                        let wide_title: Vec<u16> =
                            win_title.encode_utf16().chain(std::iter::once(0)).collect();
                        let sec_hwnd = FindWindowW(None, PCWSTR(wide_title.as_ptr()))
                            .unwrap_or(HWND(std::ptr::null_mut()));
                        if !sec_hwnd.0.is_null() && IsWindow(sec_hwnd).as_bool() {
                            let sec_hwnd_val = sec_hwnd.0 as isize;
                            apply_overlay_ex_style(sec_hwnd_val);
                            make_window_transparent(sec_hwnd_val);
                            if should_be_visible {
                                set_window_physical_rect(
                                    sec_hwnd_val,
                                    target_rect.left,
                                    target_rect.top,
                                    phys_w,
                                    phys_h,
                                );
                                ensure_window_above_monitor_taskbar(sec_hwnd_val, target_rect);
                                make_window_transparent(sec_hwnd_val);
                            }
                        }
                    }

                    let engine_cb = engine_ref.clone();
                    secondary_dock_ref.render_secondary(
                        ctx,
                        config_ref,
                        devices_ref,
                        our_hwnd,
                        open_settings_ref,
                        move || engine_cb.trigger_refresh(),
                    );
                });
            }
        }

        // Sync layout BEFORE clearing is_dragging so sync_watcher never snaps back for 1 tick
        self.sync_layout_params(self.last_dock_width);
        self.sync
            .is_dragging
            .store(self.dock.is_dragging(), Ordering::Release);

        // 6. Open standalone settings window if triggered via gear icon, dock click, or context menu
        if self.open_settings {
            self.open_settings = false;
            open_settings_window();
        }

        // Keep event loop alive to drain background battery events, animations, and taskbar sync
        ctx.request_repaint_after(Duration::from_millis(250));
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
            .with_taskbar(false)
            .with_active(false),
        ..Default::default()
    };

    eframe::run_native(
        "BatteryDock",
        native_options,
        Box::new(|cc| Ok(Box::new(BatteryDockApp::new(cc)))),
    )
}
