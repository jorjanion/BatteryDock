use crate::config::DockPositionMode;
use std::ffi::c_void;
use windows::core::{w, PWSTR};
use windows::Win32::Foundation::{CloseHandle, BOOL, COLORREF, HWND, LPARAM, POINT, RECT, WPARAM};
use windows::Win32::Graphics::Dwm::{
    DwmGetWindowAttribute, DWMWA_CLOAKED, DWMWA_EXTENDED_FRAME_BOUNDS,
};
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MonitorFromRect, MonitorFromWindow, HMONITOR, MONITORINFO,
    MONITOR_DEFAULTTONEAREST, MONITOR_DEFAULTTONULL, MONITOR_DEFAULTTOPRIMARY,
};
use windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::Win32::UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_LBUTTON, VK_RBUTTON};
use windows::Win32::UI::Shell::{
    SHAppBarMessage, SHQueryUserNotificationState, ABM_GETTASKBARPOS, APPBARDATA,
    QUNS_RUNNING_D3D_FULL_SCREEN,
};
use windows::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreatePopupMenu, DestroyMenu, FindWindowExW, FindWindowW, GetClassNameW,
    GetCursorPos, GetDesktopWindow, GetForegroundWindow, GetLayeredWindowAttributes,
    GetShellWindow, GetSystemMetrics, GetTopWindow, GetWindow, GetWindowLongW, GetWindowRect,
    GetWindowThreadProcessId, IsIconic, IsWindow, IsWindowVisible, PostMessageW,
    SetForegroundWindow, SetWindowLongW, SetWindowPos, ShowWindow, TrackPopupMenuEx, GWL_EXSTYLE,
    GW_HWNDNEXT, GW_HWNDPREV, HWND_TOPMOST, LWA_ALPHA, MF_SEPARATOR, MF_STRING, SM_CXSCREEN,
    SM_CYSCREEN, SM_SWAPBUTTON, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE,
    SWP_NOZORDER, SW_HIDE, SW_SHOWNOACTIVATE, TPM_NONOTIFY, TPM_RETURNCMD, TPM_RIGHTBUTTON,
    WM_NULL, WS_EX_APPWINDOW, WS_EX_LAYERED, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT,
};

#[derive(Debug, Clone)]
pub struct TaskbarGeometry {
    pub taskbar_rect: RECT,
    pub tray_rect: Option<RECT>,
    #[allow(dead_code)]
    pub taskbar_hwnd: isize,
    #[allow(dead_code)]
    pub monitor_rect: RECT,
    pub dpi_scale: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CandidateWindowRect {
    pub rect: RECT,
    pub is_menu_or_tooltip: bool,
}

#[derive(Debug, Clone)]
pub struct MonitorVisibilityInput {
    pub monitor_rect: RECT,
    pub taskbar_rect: Option<RECT>,
    pub taskbar_visible: bool,
    pub taskbar_cloaked: bool,
    pub require_taskbar_window: bool,
    pub windows_above_taskbar: Vec<CandidateWindowRect>,
    pub quns_d3d_fullscreen: bool,
    pub dock_rect: RECT,
}

#[inline]
fn rect_intersection(a: RECT, b: RECT) -> Option<RECT> {
    let left = a.left.max(b.left);
    let top = a.top.max(b.top);
    let right = a.right.min(b.right);
    let bottom = a.bottom.min(b.bottom);
    if right > left && bottom > top {
        Some(RECT {
            left,
            top,
            right,
            bottom,
        })
    } else {
        None
    }
}

#[inline]
fn rect_covers_monitor(win: RECT, monitor: RECT) -> bool {
    const TOL: i32 = 2;
    win.left <= monitor.left + TOL
        && win.top <= monitor.top + TOL
        && win.right >= monitor.right - TOL
        && win.bottom >= monitor.bottom - TOL
}

/// Pure, testable visibility evaluator:
/// Returns `true` when BatteryDock should be VISIBLE (taskbar visible and unobstructed),
/// and `false` when it should be HIDDEN (taskbar hidden/auto-hid/obscured or fullscreen app active).
pub fn evaluate_dock_should_be_visible(input: &MonitorVisibilityInput) -> bool {
    if input.quns_d3d_fullscreen {
        return false;
    }

    if let Some(tb) = input.taskbar_rect {
        if !input.taskbar_visible || input.taskbar_cloaked {
            return false;
        }

        // Auto-hide taskbars stay "visible" in Win32 while sliding off-screen to a 1-2px edge sliver.
        let Some(tb_vis) = rect_intersection(tb, input.monitor_rect) else {
            return false;
        };
        let tb_vis_w = tb_vis.right - tb_vis.left;
        let tb_vis_h = tb_vis.bottom - tb_vis.top;
        if tb_vis_w < 16 || tb_vis_h < 16 {
            return false;
        }

        let dock_w = (input.dock_rect.right - input.dock_rect.left).max(0);
        let dock_h = (input.dock_rect.bottom - input.dock_rect.top).max(0);
        let dock_area = dock_w as i64 * dock_h as i64;

        for w in &input.windows_above_taskbar {
            if w.is_menu_or_tooltip {
                continue;
            }
            if rect_covers_monitor(w.rect, input.monitor_rect) {
                return false;
            }
            if let Some(occ) = rect_intersection(w.rect, tb_vis) {
                let occ_w = occ.right - occ.left;
                let occ_h = occ.bottom - occ.top;
                // Covers >=50% of taskbar width and essentially full taskbar height
                if occ_w * 2 >= tb_vis_w && occ_h >= (tb_vis_h - 4).max(12) {
                    return false;
                }
                // Covers >=75% of the dock pill while overlapping most of taskbar thickness
                if dock_area > 0 {
                    if let Some(dock_occ) = rect_intersection(w.rect, input.dock_rect) {
                        let dock_occ_area = (dock_occ.right - dock_occ.left) as i64
                            * (dock_occ.bottom - dock_occ.top) as i64;
                        if dock_occ_area * 4 >= dock_area * 3 && occ_h * 2 >= tb_vis_h {
                            return false;
                        }
                    }
                }
            }
        }

        true
    } else {
        if input.require_taskbar_window {
            return false;
        }

        // Custom mode on a monitor where Windows secondary taskbar is disabled:
        // hide only when a non-menu window covers that monitor fullscreen or covers the dock rect.
        let dock_w = (input.dock_rect.right - input.dock_rect.left).max(0);
        let dock_h = (input.dock_rect.bottom - input.dock_rect.top).max(0);
        let dock_area = dock_w as i64 * dock_h as i64;

        for w in &input.windows_above_taskbar {
            if w.is_menu_or_tooltip {
                continue;
            }
            if rect_covers_monitor(w.rect, input.monitor_rect) {
                return false;
            }
            if dock_area > 0 {
                if let Some(dock_occ) = rect_intersection(w.rect, input.dock_rect) {
                    let dock_occ_area = (dock_occ.right - dock_occ.left) as i64
                        * (dock_occ.bottom - dock_occ.top) as i64;
                    if dock_occ_area * 4 >= dock_area * 3 {
                        return false;
                    }
                }
            }
        }
        true
    }
}

unsafe fn get_monitor_rect_and_scale(hmon: HMONITOR) -> (RECT, f32) {
    let mut mi = MONITORINFO {
        cbSize: std::mem::size_of::<MONITORINFO>() as u32,
        ..Default::default()
    };
    let rect = if GetMonitorInfoW(hmon, &mut mi).as_bool() {
        mi.rcMonitor
    } else {
        let w = GetSystemMetrics(SM_CXSCREEN).max(1920);
        let h = GetSystemMetrics(SM_CYSCREEN).max(1080);
        RECT {
            left: 0,
            top: 0,
            right: w,
            bottom: h,
        }
    };

    let mut dpi_x = 96u32;
    let mut dpi_y = 96u32;
    let scale = if GetDpiForMonitor(hmon, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y).is_ok()
        && dpi_x >= 72
    {
        (dpi_x as f32 / 96.0).clamp(0.75, 4.0)
    } else {
        1.0
    };
    (rect, scale)
}

unsafe fn get_window_class_lower(hwnd: HWND) -> String {
    let mut buf = [0u16; 96];
    let len = GetClassNameW(hwnd, &mut buf);
    if len <= 0 {
        String::new()
    } else {
        String::from_utf16_lossy(&buf[..len as usize]).to_ascii_lowercase()
    }
}

unsafe fn get_process_exe_lower(pid: u32) -> Option<String> {
    let Ok(handle) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) else {
        return None;
    };
    let mut buf = [0u16; 512];
    let mut size = buf.len() as u32;
    let res = QueryFullProcessImageNameW(
        handle,
        PROCESS_NAME_WIN32,
        PWSTR(buf.as_mut_ptr()),
        &mut size,
    );
    let _ = CloseHandle(handle);
    if res.is_ok() && size > 0 {
        let full = String::from_utf16_lossy(&buf[..size as usize]);
        let exe = full.rsplit('\\').next().unwrap_or(&full);
        Some(exe.to_ascii_lowercase())
    } else {
        None
    }
}

unsafe fn is_window_cloaked(hwnd: HWND) -> bool {
    let mut cloaked: u32 = 0;
    let hr = DwmGetWindowAttribute(
        hwnd,
        DWMWA_CLOAKED,
        &mut cloaked as *mut u32 as *mut c_void,
        std::mem::size_of::<u32>() as u32,
    );
    hr.is_ok() && cloaked != 0
}

unsafe fn get_effective_window_rect(hwnd: HWND) -> Option<RECT> {
    let mut ext = RECT::default();
    if DwmGetWindowAttribute(
        hwnd,
        DWMWA_EXTENDED_FRAME_BOUNDS,
        &mut ext as *mut RECT as *mut c_void,
        std::mem::size_of::<RECT>() as u32,
    )
    .is_ok()
        && ext.right > ext.left
        && ext.bottom > ext.top
    {
        return Some(ext);
    }

    let mut wr = RECT::default();
    if GetWindowRect(hwnd, &mut wr).is_ok() && wr.right > wr.left && wr.bottom > wr.top {
        Some(wr)
    } else {
        None
    }
}

unsafe fn is_shell_desktop_or_overlay_class(class_lower: &str) -> bool {
    matches!(
        class_lower,
        "progman"
            | "workerw"
            | "#32769"
            | "shell_traywnd"
            | "shell_secondarytraywnd"
            | "sysshadow"
            | "tooltips_class32"
            | "notifyiconoverflowwindow"
            | "toplevelwindowforoverflowxamlisland"
            | "xamlexplorerhostislandwindow"
            | "xamlexplorerhostislandwindow_wasdk"
            | "taskswitcherwnd"
            | "multitaskingviewframe"
            | "controlcenterwindow"
            | "shell_inputswitchtoplevelwindow"
            | "wnd_startfeed"
            | "traynotifywnd"
    )
}

pub fn is_shell_flyout_exe_and_class(exe_lower: Option<&str>, class_lower: &str) -> bool {
    let Some(exe) = exe_lower else {
        return false;
    };
    if matches!(
        exe,
        "startmenuexperiencehost.exe"
            | "searchhost.exe"
            | "searchapp.exe"
            | "shellexperiencehost.exe"
            | "shellhost.exe"
            | "searchui.exe"
            | "textinputhost.exe"
            | "widgets.exe"
            | "widgetservice.exe"
            | "microsoftstartfeedprovider.exe"
    ) {
        return true;
    }
    if exe == "explorer.exe" {
        return matches!(
            class_lower,
            "windows.ui.core.corewindow"
                | "xamlexplorerhostislandwindow"
                | "xamlexplorerhostislandwindow_wasdk"
                | "taskswitcherwnd"
                | "multitaskingviewframe"
                | "controlcenterwindow"
                | "shell_inputswitchtoplevelwindow"
        );
    }
    false
}

unsafe fn is_shell_flyout_window(pid: u32, class_lower: &str) -> bool {
    let exe = get_process_exe_lower(pid);
    is_shell_flyout_exe_and_class(exe.as_deref(), class_lower)
}

unsafe fn is_window_zero_alpha(hwnd: HWND, ex_style: u32) -> bool {
    if (ex_style & WS_EX_LAYERED.0) == 0 {
        return false;
    }
    let mut cr = COLORREF(0);
    let mut alpha = 255u8;
    let mut flags = Default::default();
    GetLayeredWindowAttributes(hwnd, Some(&mut cr), Some(&mut alpha), Some(&mut flags)).is_ok()
        && (flags.0 & LWA_ALPHA.0 != 0)
        && alpha == 0
}

struct SecondaryTaskbarSearch {
    target_monitor: HMONITOR,
    found_hwnd: HWND,
}

unsafe extern "system" fn enum_secondary_taskbar_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let state = &mut *(lparam.0 as *mut SecondaryTaskbarSearch);
    let class = get_window_class_lower(hwnd);
    if class == "shell_secondarytraywnd" {
        let hmon = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
        if hmon == state.target_monitor {
            state.found_hwnd = hwnd;
            return BOOL(0);
        }
    }
    BOOL(1)
}

unsafe fn find_secondary_taskbar_for_monitor(target_monitor: HMONITOR) -> HWND {
    use windows::Win32::UI::WindowsAndMessaging::EnumWindows;

    let mut search = SecondaryTaskbarSearch {
        target_monitor,
        found_hwnd: HWND(std::ptr::null_mut()),
    };
    let _ = EnumWindows(
        Some(enum_secondary_taskbar_proc),
        LPARAM(&mut search as *mut SecondaryTaskbarSearch as isize),
    );
    search.found_hwnd
}

impl TaskbarGeometry {
    pub fn detect() -> Option<Self> {
        unsafe {
            // 1. Prefer live Shell_TrayWnd window geometry in physical pixels (accurate under PerMonitorV2)
            if let Ok(taskbar_hwnd) = FindWindowW(w!("Shell_TrayWnd"), None) {
                if !taskbar_hwnd.0.is_null() && IsWindow(taskbar_hwnd).as_bool() {
                    let mut taskbar_rect = RECT::default();
                    if GetWindowRect(taskbar_hwnd, &mut taskbar_rect).is_ok()
                        && taskbar_rect.right > taskbar_rect.left
                        && taskbar_rect.bottom > taskbar_rect.top
                    {
                        let tray_hwnd = FindWindowExW(
                            taskbar_hwnd,
                            HWND(std::ptr::null_mut()),
                            w!("TrayNotifyWnd"),
                            None,
                        )
                        .ok();
                        let mut tray_rect = None;
                        if let Some(tray) = tray_hwnd {
                            if !tray.0.is_null() {
                                let mut r = RECT::default();
                                if GetWindowRect(tray, &mut r).is_ok() && r.right > r.left {
                                    tray_rect = Some(r);
                                }
                            }
                        }
                        let hmon = MonitorFromWindow(taskbar_hwnd, MONITOR_DEFAULTTOPRIMARY);
                        let (monitor_rect, dpi_scale) = get_monitor_rect_and_scale(hmon);
                        return Some(Self {
                            taskbar_rect,
                            tray_rect,
                            taskbar_hwnd: taskbar_hwnd.0 as isize,
                            monitor_rect,
                            dpi_scale,
                        });
                    }
                }
            }

            // 2. Fallback to official Windows Shell AppBar API
            let mut abd = APPBARDATA {
                cbSize: std::mem::size_of::<APPBARDATA>() as u32,
                ..Default::default()
            };
            let res = SHAppBarMessage(ABM_GETTASKBARPOS, &mut abd);
            if res != 0 && abd.rc.right > abd.rc.left && abd.rc.bottom > abd.rc.top {
                let hmon = MonitorFromRect(&abd.rc, MONITOR_DEFAULTTOPRIMARY);
                let (monitor_rect, dpi_scale) = get_monitor_rect_and_scale(hmon);
                return Some(Self {
                    taskbar_rect: abd.rc,
                    tray_rect: None,
                    taskbar_hwnd: 0,
                    monitor_rect,
                    dpi_scale,
                });
            }

            // 3. Headless / fallback to primary screen bottom 48px
            let screen_w = GetSystemMetrics(SM_CXSCREEN);
            let screen_h = GetSystemMetrics(SM_CYSCREEN);
            if screen_w > 0 && screen_h > 0 {
                return Some(Self {
                    taskbar_rect: RECT {
                        left: 0,
                        top: screen_h - 48,
                        right: screen_w,
                        bottom: screen_h,
                    },
                    tray_rect: None,
                    taskbar_hwnd: 0,
                    monitor_rect: RECT {
                        left: 0,
                        top: 0,
                        right: screen_w,
                        bottom: screen_h,
                    },
                    dpi_scale: 1.0,
                });
            }

            None
        }
    }

    pub fn is_custom_rect_on_any_monitor(x: f32, y: f32, w: f32, h: f32) -> bool {
        if !x.is_finite() || !y.is_finite() || !w.is_finite() || !h.is_finite() {
            return false;
        }
        if x == 0.0 && y == 0.0 {
            return false;
        }
        let rc = RECT {
            left: x.round() as i32,
            top: y.round() as i32,
            right: (x + w.max(40.0)).round() as i32,
            bottom: (y + h.max(20.0)).round() as i32,
        };
        unsafe { !MonitorFromRect(&rc, MONITOR_DEFAULTTONULL).is_invalid() }
    }

    pub fn dpi_scale_for_rect(rect: RECT) -> f32 {
        unsafe {
            let hmon = MonitorFromRect(&rect, MONITOR_DEFAULTTONEAREST);
            get_monitor_rect_and_scale(hmon).1
        }
    }

    /// Computes physical pixel `(x, y)` for the dock given logical width/height.
    pub fn calculate_dock_position(
        &self,
        mode: DockPositionMode,
        logical_dock_width: f32,
        logical_dock_height: f32,
        custom_x: f32,
        custom_y: f32,
    ) -> (f32, f32) {
        self.calculate_dock_position_scaled(
            mode,
            (logical_dock_width, logical_dock_height),
            (custom_x, custom_y),
            self.dpi_scale,
            Self::is_custom_rect_on_any_monitor,
        )
    }

    pub fn calculate_dock_position_scaled<F>(
        &self,
        mode: DockPositionMode,
        logical_size: (f32, f32),
        custom_pos: (f32, f32),
        scale: f32,
        is_custom_valid: F,
    ) -> (f32, f32)
    where
        F: Fn(f32, f32, f32, f32) -> bool,
    {
        let (logical_dock_width, logical_dock_height) = logical_size;
        let (custom_x, custom_y) = custom_pos;
        let scale = scale.clamp(0.75, 4.0);
        let phys_w = logical_dock_width * scale;
        let phys_h = logical_dock_height * scale;

        let tb = &self.taskbar_rect;
        let tb_h = (tb.bottom - tb.top) as f32;
        let clamped_h = phys_h.min((tb_h - 4.0).max(16.0));
        let center_y = tb.top as f32 + (tb_h - clamped_h) / 2.0;

        match mode {
            DockPositionMode::TaskbarLeftOfTray => {
                let target_x = if let Some(tray) = &self.tray_rect {
                    if tray.right > tray.left
                        && tray.left > tb.left + (120.0 * scale) as i32
                        && tray.left < tb.right
                    {
                        (tray.left as f32 - phys_w - (12.0 * scale))
                            .max(tb.left as f32 + (8.0 * scale))
                    } else {
                        (tb.right as f32 - (290.0 * scale) - phys_w)
                            .max(tb.left as f32 + (8.0 * scale))
                    }
                } else {
                    (tb.right as f32 - (290.0 * scale) - phys_w).max(tb.left as f32 + (8.0 * scale))
                };
                (target_x, center_y)
            }
            DockPositionMode::TaskbarRight => {
                let target_x =
                    (tb.right as f32 - phys_w - (12.0 * scale)).max(tb.left as f32 + (8.0 * scale));
                (target_x, center_y)
            }
            DockPositionMode::TaskbarLeft => {
                let target_x = tb.left as f32 + (65.0 * scale);
                (target_x, center_y)
            }
            DockPositionMode::Custom => {
                if is_custom_valid(custom_x, custom_y, phys_w, phys_h) {
                    (custom_x, custom_y)
                } else {
                    self.calculate_dock_position_scaled(
                        DockPositionMode::TaskbarLeftOfTray,
                        (logical_dock_width, logical_dock_height),
                        (0.0, 0.0),
                        scale,
                        is_custom_valid,
                    )
                }
            }
        }
    }

    /// Checks whether the taskbar on the dock's target monitor is actually visible and unobstructed.
    pub fn should_dock_be_visible(
        mode: DockPositionMode,
        dock_phys_rect: RECT,
        our_pid: u32,
    ) -> bool {
        unsafe {
            let (target_monitor, tb_hwnd, require_taskbar_window) = match mode {
                DockPositionMode::Custom => {
                    let hmon = MonitorFromRect(&dock_phys_rect, MONITOR_DEFAULTTONEAREST);
                    let mut tb = HWND(std::ptr::null_mut());
                    if let Ok(primary_tb) = FindWindowW(w!("Shell_TrayWnd"), None) {
                        if !primary_tb.0.is_null()
                            && MonitorFromWindow(primary_tb, MONITOR_DEFAULTTOPRIMARY) == hmon
                        {
                            tb = primary_tb;
                        }
                    }
                    if tb.0.is_null() {
                        tb = find_secondary_taskbar_for_monitor(hmon);
                    }
                    (hmon, tb, false)
                }
                _ => {
                    let Ok(primary_tb) = FindWindowW(w!("Shell_TrayWnd"), None) else {
                        return false;
                    };
                    if primary_tb.0.is_null() || !IsWindow(primary_tb).as_bool() {
                        return false;
                    }
                    let hmon = MonitorFromWindow(primary_tb, MONITOR_DEFAULTTOPRIMARY);
                    (hmon, primary_tb, true)
                }
            };

            let (monitor_rect, _) = get_monitor_rect_and_scale(target_monitor);

            let (taskbar_rect, taskbar_visible, taskbar_cloaked) =
                if !tb_hwnd.0.is_null() && IsWindow(tb_hwnd).as_bool() {
                    let mut r = RECT::default();
                    let rect_opt = if GetWindowRect(tb_hwnd, &mut r).is_ok() {
                        Some(r)
                    } else {
                        None
                    };
                    let tb_ex = GetWindowLongW(tb_hwnd, GWL_EXSTYLE) as u32;
                    let tb_vis =
                        IsWindowVisible(tb_hwnd).as_bool() && !is_window_zero_alpha(tb_hwnd, tb_ex);
                    (rect_opt, tb_vis, is_window_cloaked(tb_hwnd))
                } else {
                    (None, false, false)
                };

            // Quick bailout if taskbar itself is absent/hidden/auto-hid
            if require_taskbar_window {
                let Some(tb_r) = taskbar_rect else {
                    return false;
                };
                if !taskbar_visible || taskbar_cloaked {
                    return false;
                }
                if let Some(vis) = rect_intersection(tb_r, monitor_rect) {
                    if (vis.right - vis.left) < 16 || (vis.bottom - vis.top) < 16 {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            // Check Direct3D exclusive fullscreen only when foreground app is on target_monitor
            let mut quns_d3d_fullscreen = false;
            if let Ok(state) = SHQueryUserNotificationState() {
                if state == QUNS_RUNNING_D3D_FULL_SCREEN {
                    let fg = GetForegroundWindow();
                    if !fg.0.is_null()
                        && fg != GetDesktopWindow()
                        && fg != GetShellWindow()
                        && MonitorFromWindow(fg, MONITOR_DEFAULTTONEAREST) == target_monitor
                    {
                        quns_d3d_fullscreen = true;
                    }
                }
            }

            // Walk top-down Z-order above tb_hwnd (or all top windows if secondary display has no taskbar)
            let mut windows_above = Vec::new();
            let mut cur = GetTopWindow(None).unwrap_or(HWND(std::ptr::null_mut()));
            let mut steps = 0usize;
            while !cur.0.is_null() && steps < 4096 {
                steps += 1;
                if !tb_hwnd.0.is_null() && cur == tb_hwnd {
                    break;
                }

                let next = GetWindow(cur, GW_HWNDNEXT).unwrap_or(HWND(std::ptr::null_mut()));

                if IsWindowVisible(cur).as_bool() && !IsIconic(cur).as_bool() {
                    let mut pid = 0u32;
                    GetWindowThreadProcessId(cur, Some(&mut pid));
                    if pid != 0 && pid != our_pid {
                        let ex_style = GetWindowLongW(cur, GWL_EXSTYLE) as u32;
                        let is_clickthrough = (ex_style & WS_EX_LAYERED.0 != 0)
                            && (ex_style & WS_EX_TRANSPARENT.0 != 0);
                        let is_zero_alpha = !is_clickthrough && is_window_zero_alpha(cur, ex_style);

                        if !is_clickthrough && !is_zero_alpha {
                            let class = get_window_class_lower(cur);
                            if !is_shell_desktop_or_overlay_class(&class) && !is_window_cloaked(cur)
                            {
                                if let Some(rect) = get_effective_window_rect(cur) {
                                    if rect_intersection(rect, monitor_rect).is_some()
                                        && (rect.right - rect.left) >= 16
                                        && (rect.bottom - rect.top) >= 16
                                    {
                                        // Only query process exe name when window is actually large enough to matter
                                        let might_occlude = rect_covers_monitor(rect, monitor_rect)
                                            || taskbar_rect
                                                .and_then(|tb_r| rect_intersection(rect, tb_r))
                                                .is_some();
                                        let skip_flyout =
                                            might_occlude && is_shell_flyout_window(pid, &class);
                                        if !skip_flyout {
                                            windows_above.push(CandidateWindowRect {
                                                rect,
                                                is_menu_or_tooltip: class == "#32768",
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                cur = next;
            }

            let input = MonitorVisibilityInput {
                monitor_rect,
                taskbar_rect,
                taskbar_visible,
                taskbar_cloaked,
                require_taskbar_window,
                windows_above_taskbar: windows_above,
                quns_d3d_fullscreen,
                dock_rect: dock_phys_rect,
            };

            evaluate_dock_should_be_visible(&input)
        }
    }

    /// Legacy convenience wrapper kept accurate via the new occlusion engine.
    #[allow(dead_code)]
    pub fn is_fullscreen_app_active() -> bool {
        let tb = Self::detect();
        let rect = tb.as_ref().map(|g| g.taskbar_rect).unwrap_or(RECT {
            left: 0,
            top: 1392,
            right: 120,
            bottom: 1424,
        });
        !Self::should_dock_be_visible(
            DockPositionMode::TaskbarLeftOfTray,
            rect,
            std::process::id(),
        )
    }
}

/// Ensures the overlay window has `WS_EX_TOOLWINDOW | WS_EX_TOPMOST` and NOT `WS_EX_APPWINDOW`.
pub fn apply_overlay_ex_style(hwnd_val: isize) {
    if hwnd_val == 0 {
        return;
    }
    unsafe {
        let hwnd = HWND(hwnd_val as *mut c_void);
        if !IsWindow(hwnd).as_bool() {
            return;
        }
        let ex = GetWindowLongW(hwnd, GWL_EXSTYLE);
        let desired =
            (ex | WS_EX_TOOLWINDOW.0 as i32 | WS_EX_TOPMOST.0 as i32) & !(WS_EX_APPWINDOW.0 as i32);
        if desired != ex {
            SetWindowLongW(hwnd, GWL_EXSTYLE, desired);
            let _ = SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_FRAMECHANGED,
            );
        }
    }
}

unsafe fn is_window_above_in_zorder(our_hwnd: HWND, tb_hwnd: HWND) -> bool {
    let mut cur = GetTopWindow(None).unwrap_or(HWND(std::ptr::null_mut()));
    let mut steps = 0usize;
    while !cur.0.is_null() && steps < 4096 {
        if cur == our_hwnd {
            return true;
        }
        if cur == tb_hwnd {
            return false;
        }
        cur = GetWindow(cur, GW_HWNDNEXT).unwrap_or(HWND(std::ptr::null_mut()));
        steps += 1;
    }
    false
}

/// Ensures `our_hwnd` sits strictly ABOVE `Shell_TrayWnd` in the `HWND_TOPMOST` Z-order band.
/// Fixes the Windows key / taskbar click burying BatteryDock behind `Shell_TrayWnd`.
pub fn ensure_window_above_taskbar(our_hwnd_val: isize, tb_hwnd_val: isize) {
    if our_hwnd_val == 0 || tb_hwnd_val == 0 {
        return;
    }
    unsafe {
        let our_hwnd = HWND(our_hwnd_val as *mut c_void);
        let tb_hwnd = HWND(tb_hwnd_val as *mut c_void);
        if !IsWindow(our_hwnd).as_bool() || !IsWindow(tb_hwnd).as_bool() {
            return;
        }

        let our_ex = GetWindowLongW(our_hwnd, GWL_EXSTYLE);
        let need_reorder =
            (our_ex & WS_EX_TOPMOST.0 as i32) == 0 || !is_window_above_in_zorder(our_hwnd, tb_hwnd);

        if need_reorder {
            let prev = GetWindow(tb_hwnd, GW_HWNDPREV).unwrap_or(HWND(std::ptr::null_mut()));
            let insert_after = if !prev.0.is_null()
                && prev != our_hwnd
                && (GetWindowLongW(prev, GWL_EXSTYLE) & WS_EX_TOPMOST.0 as i32) != 0
            {
                prev
            } else {
                HWND_TOPMOST
            };
            let res = SetWindowPos(
                our_hwnd,
                insert_after,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
            );
            if res.is_err() || !is_window_above_in_zorder(our_hwnd, tb_hwnd) {
                let _ = SetWindowPos(
                    our_hwnd,
                    HWND_TOPMOST,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
                );
            }
        }
    }
}

pub fn ensure_window_above_monitor_taskbar(our_hwnd_val: isize, target_rect: RECT) {
    if our_hwnd_val == 0 {
        return;
    }
    unsafe {
        let hmon = MonitorFromRect(&target_rect, MONITOR_DEFAULTTONEAREST);
        let mut tb = HWND(std::ptr::null_mut());
        if let Ok(primary_tb) = FindWindowW(w!("Shell_TrayWnd"), None) {
            if !primary_tb.0.is_null()
                && MonitorFromWindow(primary_tb, MONITOR_DEFAULTTOPRIMARY) == hmon
            {
                tb = primary_tb;
            }
        }
        if tb.0.is_null() {
            tb = find_secondary_taskbar_for_monitor(hmon);
        }
        if !tb.0.is_null() {
            ensure_window_above_taskbar(our_hwnd_val, tb.0 as isize);
        } else {
            // Ensure TOPMOST even on a monitor with no taskbar
            let our_hwnd = HWND(our_hwnd_val as *mut c_void);
            if IsWindow(our_hwnd).as_bool() {
                let our_ex = GetWindowLongW(our_hwnd, GWL_EXSTYLE);
                if (our_ex & WS_EX_TOPMOST.0 as i32) == 0 {
                    let _ = SetWindowPos(
                        our_hwnd,
                        HWND_TOPMOST,
                        0,
                        0,
                        0,
                        0,
                        SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
                    );
                }
            }
        }
    }
}

pub fn get_window_physical_rect(hwnd_val: isize) -> Option<RECT> {
    if hwnd_val == 0 {
        return None;
    }
    unsafe {
        let hwnd = HWND(hwnd_val as *mut c_void);
        if !IsWindow(hwnd).as_bool() {
            return None;
        }
        let mut r = RECT::default();
        if GetWindowRect(hwnd, &mut r).is_ok() {
            Some(r)
        } else {
            None
        }
    }
}

pub fn set_window_physical_rect(hwnd_val: isize, x: i32, y: i32, w: i32, h: i32) {
    if hwnd_val == 0 {
        return;
    }
    unsafe {
        let hwnd = HWND(hwnd_val as *mut c_void);
        if !IsWindow(hwnd).as_bool() {
            return;
        }
        let mut cur = RECT::default();
        if GetWindowRect(hwnd, &mut cur).is_ok() {
            let dw = cur.right - cur.left;
            let dh = cur.bottom - cur.top;
            if (cur.left - x).abs() <= 1
                && (cur.top - y).abs() <= 1
                && (dw - w).abs() <= 1
                && (dh - h).abs() <= 1
            {
                return;
            }
        }
        let _ = SetWindowPos(
            hwnd,
            HWND(std::ptr::null_mut()),
            x,
            y,
            w.max(40),
            h.max(20),
            SWP_NOZORDER | SWP_NOACTIVATE,
        );
    }
}

pub fn get_cursor_physical_pos() -> Option<(i32, i32)> {
    unsafe {
        let mut pt = POINT::default();
        if GetCursorPos(&mut pt).is_ok() {
            Some((pt.x, pt.y))
        } else {
            None
        }
    }
}

pub fn set_window_physical_pos(hwnd_val: isize, x: i32, y: i32) {
    if hwnd_val == 0 {
        return;
    }
    unsafe {
        let hwnd = HWND(hwnd_val as *mut c_void);
        if !IsWindow(hwnd).as_bool() {
            return;
        }
        let _ = SetWindowPos(
            hwnd,
            HWND(std::ptr::null_mut()),
            x,
            y,
            0,
            0,
            SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
        );
    }
}

pub fn is_window_visible_win32(hwnd_val: isize) -> bool {
    if hwnd_val == 0 {
        return false;
    }
    unsafe {
        let hwnd = HWND(hwnd_val as *mut c_void);
        IsWindow(hwnd).as_bool() && IsWindowVisible(hwnd).as_bool()
    }
}

/// Shows or hides the window purely via Win32 (`SW_HIDE` / `SW_SHOWNOACTIVATE`),
/// avoiding `winit` style rewrites. Returns `true` if visibility actually changed.
pub fn set_window_visible_noactivate(hwnd_val: isize, visible: bool) -> bool {
    if hwnd_val == 0 {
        return false;
    }
    unsafe {
        let hwnd = HWND(hwnd_val as *mut c_void);
        if !IsWindow(hwnd).as_bool() {
            return false;
        }
        let cur_vis = IsWindowVisible(hwnd).as_bool();
        if cur_vis == visible {
            return false;
        }
        let _ = ShowWindow(hwnd, if visible { SW_SHOWNOACTIVATE } else { SW_HIDE });
        true
    }
}

/// Checks live Win32 mouse button state (immune to uncaptured cursor exits outside the 32px pill).
pub fn is_primary_mouse_button_down() -> bool {
    unsafe {
        let swapped = GetSystemMetrics(SM_SWAPBUTTON) != 0;
        let vk = if swapped {
            VK_RBUTTON.0 as i32
        } else {
            VK_LBUTTON.0 as i32
        };
        (GetAsyncKeyState(vk) as u16 & 0x8000) != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DockContextMenuAction {
    Settings,
    Refresh,
    ResetPosition,
    Restart,
    Exit,
}

/// Shows a crisp native Win32 popup menu above/beside the dock pill (never clipped by the 32px viewport).
pub fn show_dock_context_menu(hwnd_val: isize) -> Option<DockContextMenuAction> {
    if hwnd_val == 0 {
        return None;
    }
    unsafe {
        let hwnd = HWND(hwnd_val as *mut c_void);
        if !IsWindow(hwnd).as_bool() {
            return None;
        }
        let Ok(menu) = CreatePopupMenu() else {
            return None;
        };

        const CMD_SETTINGS: usize = 1001;
        const CMD_REFRESH: usize = 1002;
        const CMD_RESET: usize = 1003;
        const CMD_RESTART: usize = 1004;
        const CMD_EXIT: usize = 1005;

        let _ = AppendMenuW(menu, MF_STRING, CMD_SETTINGS, w!("Settings"));
        let _ = AppendMenuW(menu, MF_STRING, CMD_REFRESH, w!("Refresh Devices"));
        let _ = AppendMenuW(menu, MF_SEPARATOR, 0, None);
        let _ = AppendMenuW(menu, MF_STRING, CMD_RESET, w!("Reset to Taskbar Clock"));
        let _ = AppendMenuW(menu, MF_STRING, CMD_RESTART, w!("Restart BatteryDock"));
        let _ = AppendMenuW(menu, MF_STRING, CMD_EXIT, w!("Exit BatteryDock"));

        let mut pt = POINT::default();
        let _ = GetCursorPos(&mut pt);
        let _ = SetForegroundWindow(hwnd);
        let cmd = TrackPopupMenuEx(
            menu,
            (TPM_RETURNCMD | TPM_NONOTIFY | TPM_RIGHTBUTTON).0,
            pt.x,
            pt.y,
            hwnd,
            None,
        );
        let _ = PostMessageW(hwnd, WM_NULL, WPARAM(0), LPARAM(0));
        let _ = DestroyMenu(menu);

        match cmd.0 as usize {
            CMD_SETTINGS => Some(DockContextMenuAction::Settings),
            CMD_REFRESH => Some(DockContextMenuAction::Refresh),
            CMD_RESET => Some(DockContextMenuAction::ResetPosition),
            CMD_RESTART => Some(DockContextMenuAction::Restart),
            CMD_EXIT => Some(DockContextMenuAction::Exit),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(l: i32, t: i32, r: i32, b: i32) -> RECT {
        RECT {
            left: l,
            top: t,
            right: r,
            bottom: b,
        }
    }

    fn base_primary_input() -> MonitorVisibilityInput {
        MonitorVisibilityInput {
            monitor_rect: rect(0, 0, 2560, 1440),
            taskbar_rect: Some(rect(0, 1392, 2560, 1440)),
            taskbar_visible: true,
            taskbar_cloaked: false,
            require_taskbar_window: true,
            windows_above_taskbar: vec![],
            quns_d3d_fullscreen: false,
            dock_rect: rect(2100, 1400, 2240, 1432),
        }
    }

    #[test]
    fn test_visibility_normal_desktop_and_maximized_window() {
        let mut input = base_primary_input();
        assert!(evaluate_dock_should_be_visible(&input));

        // Maximized window stops at work area top (1392), zero overlap with taskbar
        input.windows_above_taskbar.push(CandidateWindowRect {
            rect: rect(0, 0, 2560, 1392),
            is_menu_or_tooltip: false,
        });
        assert!(evaluate_dock_should_be_visible(&input));
    }

    #[test]
    fn test_visibility_hidden_on_fullscreen_video_or_game() {
        let mut input = base_primary_input();
        // Fullscreen YouTube / borderless game covers all 4 edges of monitor
        input.windows_above_taskbar.push(CandidateWindowRect {
            rect: rect(0, 0, 2560, 1440),
            is_menu_or_tooltip: false,
        });
        assert!(!evaluate_dock_should_be_visible(&input));

        // D3D exclusive fullscreen flag
        let mut input2 = base_primary_input();
        input2.quns_d3d_fullscreen = true;
        assert!(!evaluate_dock_should_be_visible(&input2));
    }

    #[test]
    fn test_visibility_hidden_when_taskbar_autohid_or_cloaked() {
        // Auto-hide slid off bottom leaving 2px sliver
        let mut input = base_primary_input();
        input.taskbar_rect = Some(rect(0, 1438, 2560, 1440));
        assert!(!evaluate_dock_should_be_visible(&input));

        // Auto-hide revealed again
        input.taskbar_rect = Some(rect(0, 1392, 2560, 1440));
        assert!(evaluate_dock_should_be_visible(&input));

        // Hidden via IsWindowVisible == false or DWM cloaked
        input.taskbar_visible = false;
        assert!(!evaluate_dock_should_be_visible(&input));
        input.taskbar_visible = true;
        input.taskbar_cloaked = true;
        assert!(!evaluate_dock_should_be_visible(&input));
    }

    #[test]
    fn test_visibility_ignores_context_menus_but_hides_if_taskbar_covered() {
        let mut input = base_primary_input();
        // Right-click menu overlapping dock rect does not cause flicker/hide
        input.windows_above_taskbar.push(CandidateWindowRect {
            rect: rect(2080, 1300, 2260, 1440),
            is_menu_or_tooltip: true,
        });
        assert!(evaluate_dock_should_be_visible(&input));

        // Non-menu app covering >=50% of taskbar width & full taskbar height hides dock
        input.windows_above_taskbar.push(CandidateWindowRect {
            rect: rect(0, 1390, 2560, 1440),
            is_menu_or_tooltip: false,
        });
        assert!(!evaluate_dock_should_be_visible(&input));
    }

    #[test]
    fn test_visibility_custom_mode_secondary_monitor_without_taskbar() {
        let input = MonitorVisibilityInput {
            monitor_rect: rect(-1080, -592, 0, 1328),
            taskbar_rect: None,
            taskbar_visible: false,
            taskbar_cloaked: false,
            require_taskbar_window: false,
            windows_above_taskbar: vec![],
            quns_d3d_fullscreen: false,
            dock_rect: rect(-500, 1280, -360, 1312),
        };
        assert!(evaluate_dock_should_be_visible(&input));

        let mut fullscreen_on_left = input;
        fullscreen_on_left
            .windows_above_taskbar
            .push(CandidateWindowRect {
                rect: rect(-1080, -592, 0, 1328),
                is_menu_or_tooltip: false,
            });
        assert!(!evaluate_dock_should_be_visible(&fullscreen_on_left));
    }

    #[test]
    fn test_position_math_100_and_150_dpi_and_negative_custom_coords() {
        let geo = TaskbarGeometry {
            taskbar_rect: rect(0, 1392, 2560, 1440),
            tray_rect: Some(rect(2250, 1392, 2560, 1440)),
            taskbar_hwnd: 0,
            monitor_rect: rect(0, 0, 2560, 1440),
            dpi_scale: 1.0,
        };

        // 100% DPI: centered inside 48px taskbar -> y = 1392 + (48 - 32)/2 = 1400
        let (x, y) = geo.calculate_dock_position_scaled(
            DockPositionMode::TaskbarLeftOfTray,
            (150.0, 32.0),
            (0.0, 0.0),
            1.0,
            |_, _, _, _| true,
        );
        assert_eq!(x, 2250.0 - 150.0 - 12.0);
        assert_eq!(y, 1400.0);

        // 150% DPI: 72px taskbar, 48px pill -> y = 1392 + (72 - 48)/2 = 1404
        let geo_150 = TaskbarGeometry {
            taskbar_rect: rect(0, 1368, 2560, 1440),
            tray_rect: Some(rect(2100, 1368, 2560, 1440)),
            taskbar_hwnd: 0,
            monitor_rect: rect(0, 0, 2560, 1440),
            dpi_scale: 1.5,
        };
        let (x150, y150) = geo_150.calculate_dock_position_scaled(
            DockPositionMode::TaskbarLeftOfTray,
            (150.0, 32.0),
            (0.0, 0.0),
            1.5,
            |_, _, _, _| true,
        );
        assert_eq!(x150, 2100.0 - (150.0 * 1.5) - (12.0 * 1.5));
        assert_eq!(y150, 1368.0 + (72.0 - 48.0) / 2.0);

        // Negative left-monitor custom coordinates must be preserved when valid
        let (cx, cy) = geo.calculate_dock_position_scaled(
            DockPositionMode::Custom,
            (150.0, 32.0),
            (-800.0, 1285.0),
            1.0,
            |x, y, _, _| x == -800.0 && y == 1285.0,
        );
        assert_eq!((cx, cy), (-800.0, 1285.0));

        // Stale minimized-style coordinates fallback cleanly to taskbar
        let (fx, fy) = geo.calculate_dock_position_scaled(
            DockPositionMode::Custom,
            (150.0, 32.0),
            (-32000.0, -32000.0),
            1.0,
            |x, y, _, _| x > -2000.0 && y > -2000.0,
        );
        assert_eq!(fx, 2250.0 - 150.0 - 12.0);
        assert_eq!(fy, 1400.0);
    }

    #[test]
    fn test_shell_flyout_classification() {
        assert!(is_shell_flyout_exe_and_class(
            Some("startmenuexperiencehost.exe"),
            "windows.ui.core.corewindow"
        ));
        assert!(is_shell_flyout_exe_and_class(
            Some("shellhost.exe"),
            "windows.ui.core.corewindow"
        ));
        assert!(is_shell_flyout_exe_and_class(
            Some("searchhost.exe"),
            "windows.ui.core.corewindow"
        ));
        assert!(is_shell_flyout_exe_and_class(
            Some("searchapp.exe"),
            "windows.ui.core.corewindow"
        ));
        assert!(is_shell_flyout_exe_and_class(
            Some("searchui.exe"),
            "windows.ui.core.corewindow"
        ));
        assert!(is_shell_flyout_exe_and_class(
            Some("widgetservice.exe"),
            "widgets"
        ));
        assert!(is_shell_flyout_exe_and_class(
            Some("explorer.exe"),
            "xamlexplorerhostislandwindow_wasdk"
        ));
        assert!(is_shell_flyout_exe_and_class(
            Some("explorer.exe"),
            "windows.ui.core.corewindow"
        ));
        // Normal File Explorer window or regular fullscreen UWP/browser must NOT be skipped
        assert!(!is_shell_flyout_exe_and_class(
            Some("explorer.exe"),
            "cabinetwclass"
        ));
        assert!(!is_shell_flyout_exe_and_class(
            Some("brave.exe"),
            "chrome_widgetwin_1"
        ));
        assert!(!is_shell_flyout_exe_and_class(
            Some("applicationframehost.exe"),
            "applicationframewindow"
        ));
        assert!(!is_shell_flyout_exe_and_class(
            None,
            "windows.ui.core.corewindow"
        ));
    }

    #[test]
    fn test_taskbar_detect_and_live_checks_do_not_panic() {
        let tb = TaskbarGeometry::detect().expect("Fallback taskbar geometry expected");
        assert!(tb.taskbar_rect.right > tb.taskbar_rect.left);
        assert!(tb.taskbar_rect.bottom > tb.taskbar_rect.top);
        let _ = TaskbarGeometry::is_fullscreen_app_active();
    }
}
