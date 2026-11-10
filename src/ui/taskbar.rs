use crate::config::DockPositionMode;
use windows::core::w;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowExW, FindWindowW, GetWindowRect};

pub struct TaskbarGeometry {
    pub taskbar_rect: RECT,
    pub tray_rect: Option<RECT>,
}

impl TaskbarGeometry {
    pub fn detect() -> Option<Self> {
        unsafe {
            let taskbar_hwnd = FindWindowW(w!("Shell_TrayWnd"), None).ok()?;
            if taskbar_hwnd.0.is_null() {
                return None;
            }

            let mut taskbar_rect = RECT::default();
            if GetWindowRect(taskbar_hwnd, &mut taskbar_rect).is_err() {
                return None;
            }

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

            Some(Self {
                taskbar_rect,
                tray_rect,
            })
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
        let _tb_w = (tb.right - tb.left) as f32;
        let tb_h = (tb.bottom - tb.top) as f32;

        let center_y = tb.top as f32 + (tb_h - dock_height) / 2.0;

        match mode {
            DockPositionMode::TaskbarLeftOfTray => {
                if let Some(tray) = &self.tray_rect {
                    let target_x = tray.left as f32 - dock_width - 12.0;
                    (target_x.max(tb.left as f32 + 10.0), center_y)
                } else {
                    // Fallback: estimate tray width ~200px
                    let target_x = tb.right as f32 - 220.0 - dock_width;
                    (target_x.max(tb.left as f32 + 10.0), center_y)
                }
            }
            DockPositionMode::TaskbarRight => {
                let target_x = tb.right as f32 - dock_width - 8.0;
                (target_x, center_y)
            }
            DockPositionMode::TaskbarLeft => {
                // Next to start menu / widgets on left
                let target_x = tb.left as f32 + 65.0;
                (target_x, center_y)
            }
            DockPositionMode::Custom => {
                if custom_x != 0.0 || custom_y != 0.0 {
                    (custom_x, custom_y)
                } else {
                    // Default to left of tray if no custom position set yet
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
