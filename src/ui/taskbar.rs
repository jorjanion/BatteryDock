use crate::config::DockPositionMode;
use windows::core::w;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::Shell::{SHAppBarMessage, ABM_GETTASKBARPOS, APPBARDATA};
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowExW, FindWindowW, GetSystemMetrics, GetWindowRect, SM_CXSCREEN, SM_CYSCREEN,
};

pub struct TaskbarGeometry {
    pub taskbar_rect: RECT,
    pub tray_rect: Option<RECT>,
}

impl TaskbarGeometry {
    pub fn detect() -> Option<Self> {
        unsafe {
            // 1. First try official Windows Shell AppBar API
            let mut abd = APPBARDATA {
                cbSize: std::mem::size_of::<APPBARDATA>() as u32,
                ..Default::default()
            };
            let res = SHAppBarMessage(ABM_GETTASKBARPOS, &mut abd);
            if res != 0 && (abd.rc.right > abd.rc.left && abd.rc.bottom > abd.rc.top) {
                let tray_hwnd = FindWindowW(w!("Shell_TrayWnd"), None)
                    .ok()
                    .and_then(|tb| FindWindowExW(tb, HWND(std::ptr::null_mut()), w!("TrayNotifyWnd"), None).ok());
                let mut tray_rect = None;
                if let Some(tray) = tray_hwnd {
                    if !tray.0.is_null() {
                        let mut r = RECT::default();
                        if GetWindowRect(tray, &mut r).is_ok() {
                            tray_rect = Some(r);
                        }
                    }
                }
                return Some(Self {
                    taskbar_rect: abd.rc,
                    tray_rect,
                });
            }

            // 2. Fallback to FindWindowW for Shell_TrayWnd
            if let Ok(taskbar_hwnd) = FindWindowW(w!("Shell_TrayWnd"), None) {
                if !taskbar_hwnd.0.is_null() {
                    let mut taskbar_rect = RECT::default();
                    if GetWindowRect(taskbar_hwnd, &mut taskbar_rect).is_ok() {
                        let tray_hwnd = FindWindowExW(taskbar_hwnd, HWND(std::ptr::null_mut()), w!("TrayNotifyWnd"), None).ok();
                        let mut tray_rect = None;
                        if let Some(tray) = tray_hwnd {
                            if !tray.0.is_null() {
                                let mut r = RECT::default();
                                if GetWindowRect(tray, &mut r).is_ok() {
                                    tray_rect = Some(r);
                                }
                            }
                        }
                        return Some(Self {
                            taskbar_rect,
                            tray_rect,
                        });
                    }
                }
            }

            // 3. Fallback to primary screen bottom 48px
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
                });
            }

            None
        }
    }

    /// Checks if a true fullscreen application/video or game is currently active
    pub fn is_fullscreen_app_active() -> bool {
        use windows::Win32::Foundation::RECT;
        use windows::Win32::Graphics::Gdi::{
            GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
        };
        use windows::Win32::UI::Shell::{
            SHQueryUserNotificationState, QUNS_BUSY, QUNS_PRESENTATION_MODE,
            QUNS_RUNNING_D3D_FULL_SCREEN,
        };
        use windows::Win32::UI::WindowsAndMessaging::{
            GetClassNameW, GetForegroundWindow, GetWindowRect,
        };

        unsafe {
            // 1. Check official Windows Shell Query state
            if let Ok(qstate) = SHQueryUserNotificationState() {
                if qstate == QUNS_BUSY
                    || qstate == QUNS_RUNNING_D3D_FULL_SCREEN
                    || qstate == QUNS_PRESENTATION_MODE
                {
                    return true;
                }
            }

            // 2. Check foreground window bounds
            let fg = GetForegroundWindow();
            if fg.0.is_null() {
                return false;
            }

            // Ignore shell desktop / taskbar windows
            let mut class_buf = [0u16; 64];
            let len = GetClassNameW(fg, &mut class_buf);
            if len > 0 {
                let class = String::from_utf16_lossy(&class_buf[..len as usize]);
                if class == "Shell_TrayWnd"
                    || class == "Progman"
                    || class == "WorkerW"
                    || class == "Shell_SecondaryTrayWnd"
                {
                    return false;
                }
            }

            let mut win_rect = RECT::default();
            if GetWindowRect(fg, &mut win_rect).is_ok() {
                let monitor = MonitorFromWindow(fg, MONITOR_DEFAULTTONEAREST);
                let mut mi = MONITORINFO {
                    cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                    ..Default::default()
                };
                if GetMonitorInfoW(monitor, &mut mi).as_bool() {
                    let m = mi.rcMonitor;
                    // A true fullscreen app, video (e.g. YouTube), or game covers all 4 edges of the physical screen.
                    // A normal maximized window does NOT cover the taskbar (its rect stops at rcWork).
                    if win_rect.left <= m.left
                        && win_rect.top <= m.top
                        && win_rect.right >= m.right
                        && win_rect.bottom >= m.bottom
                    {
                        return true;
                    }
                }
            }
            false
        }
    }

    pub fn calculate_dock_position(
        &self,
        mode: DockPositionMode,
        dock_width: f32,
        dock_height: f32,
        custom_x: f32,
        custom_y: f32,
    ) -> (f32, f32) {
        let tb = &self.taskbar_rect;
        let tb_h = (tb.bottom - tb.top) as f32;

        let center_y = tb.top as f32 + (tb_h - dock_height) / 2.0;

        match mode {
            DockPositionMode::TaskbarLeftOfTray => {
                // In Windows 11 / 10, the tray area (clock, sound, network, chevron)
                // occupies approximately 280-320px from the right edge.
                let target_x = if let Some(tray) = &self.tray_rect {
                    if tray.left > tb.left + 200 && tray.left < tb.right {
                        (tray.left as f32 - dock_width - 14.0).max(tb.left as f32 + 10.0)
                    } else {
                        (tb.right as f32 - 290.0 - dock_width).max(tb.left as f32 + 10.0)
                    }
                } else {
                    (tb.right as f32 - 290.0 - dock_width).max(tb.left as f32 + 10.0)
                };
                (target_x, center_y)
            }
            DockPositionMode::TaskbarRight => {
                let target_x = (tb.right as f32 - dock_width - 12.0).max(tb.left as f32 + 10.0);
                (target_x, center_y)
            }
            DockPositionMode::TaskbarLeft => {
                let target_x = tb.left as f32 + 65.0;
                (target_x, center_y)
            }
            DockPositionMode::Custom => {
                // Sanity check: ensure custom position is within valid screen bounds
                if (custom_x > -100.0 && custom_x < 10000.0)
                    && (custom_y > -100.0 && custom_y < 5000.0)
                    && (custom_x != 0.0 || custom_y != 0.0)
                {
                    (custom_x, custom_y)
                } else {
                    self.calculate_dock_position(
                        DockPositionMode::TaskbarLeftOfTray,
                        dock_width,
                        dock_height,
                        0.0,
                        0.0,
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taskbar_detect() {
        let tb_opt = TaskbarGeometry::detect();
        assert!(tb_opt.is_some(), "Taskbar detection should always succeed via fallback");
        let tb = tb_opt.unwrap();
        let (x, y) = tb.calculate_dock_position(DockPositionMode::TaskbarLeftOfTray, 120.0, 32.0, 0.0, 0.0);
        assert!(x > 0.0);
        assert!(y > 0.0);
    }

    #[test]
    fn test_fullscreen_detect_does_not_panic() {
        let _ = TaskbarGeometry::is_fullscreen_app_active();
    }
}
