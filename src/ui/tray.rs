use tray_icon::menu::{Menu, MenuId, MenuItem};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

pub struct SystemTray {
    _tray_icon: TrayIcon,
    pub settings_item_id: MenuId,
    pub refresh_item_id: MenuId,
    pub quit_item_id: MenuId,
}

impl SystemTray {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Create 32x32 RGBA icon buffer for tray icon
        let width = 32u32;
        let height = 32u32;
        let mut rgba = vec![0u8; (width * height * 4) as usize];

        // Draw battery icon into rgba buffer
        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                // Outer battery border: 4..28 x, 8..24 y
                let in_shell = x >= 4 && x <= 26 && y >= 9 && y <= 23;
                let on_shell_border = (x == 4 || x == 26 || y == 9 || y == 23) && in_shell;
                let in_tip = x >= 27 && x <= 28 && y >= 13 && y <= 19;
                let in_fill = x >= 6 && x <= 22 && y >= 11 && y <= 21;

                if on_shell_border || in_tip {
                    rgba[idx] = 230;
                    rgba[idx + 1] = 230;
                    rgba[idx + 2] = 230;
                    rgba[idx + 3] = 255;
                } else if in_fill {
                    // Mint neon green charge
                    rgba[idx] = 34;
                    rgba[idx + 1] = 197;
                    rgba[idx + 2] = 94;
                    rgba[idx + 3] = 255;
                }
            }
        }

        let icon = Icon::from_rgba(rgba, width, height)?;

        let menu = Menu::new();
        let settings_item = MenuItem::new("Settings", true, None);
        let refresh_item = MenuItem::new("Refresh Devices", true, None);
        let quit_item = MenuItem::new("Exit", true, None);

        let settings_id = settings_item.id().clone();
        let refresh_id = refresh_item.id().clone();
        let quit_id = quit_item.id().clone();

        menu.append(&settings_item)?;
        menu.append(&refresh_item)?;
        menu.append(&quit_item)?;

        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("BatteryDock - Peripheral Battery Monitor")
            .with_icon(icon)
            .build()?;

        Ok(Self {
            _tray_icon: tray_icon,
            settings_item_id: settings_id,
            refresh_item_id: refresh_id,
            quit_item_id: quit_id,
        })
    }
}
