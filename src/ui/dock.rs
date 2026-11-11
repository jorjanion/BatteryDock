use crate::battery::{DeviceBattery, DeviceType};
use crate::config::{AppConfig, DockPositionMode};
use crate::ui::icons::DevicePainter;
use crate::ui::taskbar::{
    get_cursor_physical_pos, get_window_physical_rect, set_window_physical_pos,
};
use egui::{Color32, CornerRadius, Pos2, Rect, Stroke, StrokeKind, Vec2};
use std::time::{Duration, Instant};

pub struct DockOverlay {
    start_time: Instant,
    drag_anchor: Option<(i32, i32, i32, i32)>,
    is_actively_dragging: bool,
}

impl DockOverlay {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            drag_anchor: None,
            is_actively_dragging: false,
        }
    }

    pub fn is_dragging(&self) -> bool {
        self.is_actively_dragging
    }

    #[inline]
    fn percentage_label_width(percentage: u8) -> f32 {
        if percentage >= 100 {
            32.0
        } else if percentage >= 10 {
            26.0
        } else {
            20.0
        }
    }

    /// Computes the exact logical width of the dock pill matching `render()` pixel-for-pixel.
    pub fn required_width(devices: &[DeviceBattery], config: &AppConfig) -> f32 {
        let mut w = 8.0_f32; // left padding

        if devices.is_empty() {
            // icon (16) + gap (6) + text ("BatteryDock", ~70) + gap (4)
            w += 16.0 + 6.0 + 70.0 + 4.0;
        } else {
            for (idx, dev) in devices.iter().enumerate() {
                if idx > 0 {
                    // divider: space(4) + 1px line + space(6)
                    w += 11.0;
                }
                // device icon (16) + space(3) + battery meter (23)
                w += 16.0 + 3.0 + 23.0;
                if dev.is_charging {
                    // space(2) + crisp vector charging lightning bolt (10)
                    w += 2.0 + 10.0;
                }
                if config.show_percentage_text {
                    w += 3.0 + Self::percentage_label_width(dev.percentage);
                }
            }
        }

        // space(4) + settings gear hit target (14) + right padding (6)
        w += 4.0 + 14.0 + 6.0;
        w.max(96.0)
    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        config: &mut AppConfig,
        devices: &[DeviceBattery],
        our_hwnd: isize,
        open_settings: &mut bool,
        on_refresh: impl FnOnce(),
    ) {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let pulse_alpha = ((elapsed * 4.0).sin() * 0.5 + 0.5).clamp(0.0, 1.0);

        let mut refresh_requested = false;
        let mut was_dragging_on_release = self.is_actively_dragging;

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(Color32::TRANSPARENT))
            .show(ctx, |ui| {
                let avail_rect = ui.available_rect_before_wrap();

                // Frosted dark pill background
                let bg_color = Color32::from_rgba_premultiplied(18, 20, 24, 224);
                let border_stroke = Stroke::new(1.0_f32, Color32::from_white_alpha(38));
                ui.painter()
                    .rect_filled(avail_rect, CornerRadius::same(14), bg_color);
                ui.painter().rect_stroke(
                    avail_rect,
                    CornerRadius::same(14),
                    border_stroke,
                    StrokeKind::Inside,
                );

                // Base click/drag area across pill (children added after still receive hover/clicks)
                let dock_response = ui.interact(
                    avail_rect,
                    ui.id().with("dock_drag_area"),
                    egui::Sense::click_and_drag(),
                );

                dock_response.context_menu(|ui| {
                    ui.label(egui::RichText::new("BatteryDock").strong());
                    ui.separator();
                    if ui.button("Settings").clicked() {
                        *open_settings = true;
                        ui.close_menu();
                    }
                    if ui.button("Refresh Devices").clicked() {
                        refresh_requested = true;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Reset to Taskbar Clock").clicked() {
                        config.position_mode = DockPositionMode::TaskbarLeftOfTray;
                        config.save();
                        ui.close_menu();
                    }
                    if ui.button("Restart BatteryDock").clicked() {
                        crate::restart_app();
                    }
                    if ui.button("Exit BatteryDock").clicked() {
                        std::process::exit(0);
                    }
                });

                let mut settings_clicked = false;
                let mut settings_pressed = false;

                // Content inside the pill with zero implicit egui item spacing so layout == required_width()
                ui.horizontal_centered(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.add_space(8.0);

                    if devices.is_empty() {
                        let (icon_rect, _) =
                            ui.allocate_exact_size(Vec2::new(16.0, 16.0), egui::Sense::hover());
                        DevicePainter::paint_device_icon(
                            ui.painter(),
                            icon_rect,
                            DeviceType::Unknown,
                            Color32::from_white_alpha(150),
                        );
                        ui.add_space(6.0);
                        ui.add_sized(
                            Vec2::new(70.0, 16.0),
                            egui::Label::new(
                                egui::RichText::new("BatteryDock")
                                    .size(11.5)
                                    .color(Color32::from_white_alpha(165)),
                            )
                            .selectable(false),
                        );
                        ui.add_space(4.0);
                    } else {
                        for (idx, dev) in devices.iter().enumerate() {
                            if idx > 0 {
                                ui.add_space(4.0);
                                let div_x = ui.cursor().min.x;
                                ui.painter().line_segment(
                                    [
                                        Pos2::new(div_x + 0.5, avail_rect.center().y - 8.0),
                                        Pos2::new(div_x + 0.5, avail_rect.center().y + 8.0),
                                    ],
                                    Stroke::new(1.0_f32, Color32::from_white_alpha(28)),
                                );
                                ui.add_space(1.0 + 6.0);
                            }

                            let item_response = ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 0.0;

                                // Device Icon
                                let (icon_rect, _) = ui.allocate_exact_size(
                                    Vec2::new(16.0, 16.0),
                                    egui::Sense::hover(),
                                );
                                let icon_color = if dev.is_charging {
                                    Color32::from_rgb(125, 211, 252)
                                } else if dev.percentage <= config.critical_battery_threshold {
                                    Color32::from_rgb(239, 68, 68)
                                } else {
                                    Color32::from_white_alpha(225)
                                };
                                DevicePainter::paint_device_icon(
                                    ui.painter(),
                                    icon_rect,
                                    dev.device_type,
                                    icon_color,
                                );

                                ui.add_space(3.0);

                                // Battery Meter
                                let (meter_rect, _) = ui.allocate_exact_size(
                                    Vec2::new(23.0, 10.5),
                                    egui::Sense::hover(),
                                );
                                DevicePainter::paint_battery_meter(
                                    ui.painter(),
                                    meter_rect,
                                    dev.percentage,
                                    dev.is_charging,
                                    config.low_battery_threshold,
                                    config.critical_battery_threshold,
                                    pulse_alpha,
                                );

                                // Dedicated crisp charging badge when peripheral is charging
                                if dev.is_charging {
                                    ui.add_space(2.0);
                                    let (bolt_rect, _) = ui.allocate_exact_size(
                                        Vec2::new(10.0, 13.0),
                                        egui::Sense::hover(),
                                    );
                                    DevicePainter::paint_charging_icon(
                                        ui.painter(),
                                        bolt_rect,
                                        pulse_alpha,
                                    );
                                }

                                // Percentage text
                                if config.show_percentage_text {
                                    ui.add_space(3.0);
                                    let text_color = if dev.is_charging {
                                        Color32::from_rgb(56, 189, 248)
                                    } else if dev.percentage <= config.critical_battery_threshold {
                                        Color32::from_rgb(239, 68, 68)
                                    } else if dev.percentage <= config.low_battery_threshold {
                                        Color32::from_rgb(245, 158, 11)
                                    } else {
                                        Color32::from_white_alpha(220)
                                    };

                                    let pct_w = Self::percentage_label_width(dev.percentage);
                                    ui.add_sized(
                                        Vec2::new(pct_w, 16.0),
                                        egui::Label::new(
                                            egui::RichText::new(format!("{}%", dev.percentage))
                                                .size(11.0)
                                                .color(text_color)
                                                .strong(),
                                        )
                                        .selectable(false),
                                    );
                                }
                            });

                            let charging_desc = if dev.is_charging { " (Charging)" } else { "" };
                            item_response.response.on_hover_ui(|ui| {
                                ui.label(egui::RichText::new(&dev.name).strong());
                                ui.label(format!(
                                    "Type: {} | Battery: {}%{}",
                                    dev.device_type.display_name(),
                                    dev.percentage,
                                    charging_desc
                                ));
                                ui.label(
                                    egui::RichText::new("Click for Settings • Drag to move")
                                        .size(10.0)
                                        .italics()
                                        .weak(),
                                );
                            });
                        }
                    }

                    // Crisp vector Settings gear icon button at right end
                    ui.add_space(4.0);
                    let (gear_rect, settings_btn) =
                        ui.allocate_exact_size(Vec2::new(14.0, 20.0), egui::Sense::click());
                    let gear_color = if settings_btn.hovered() {
                        Color32::from_white_alpha(235)
                    } else {
                        Color32::from_white_alpha(125)
                    };
                    DevicePainter::paint_settings_gear(
                        ui.painter(),
                        Rect::from_center_size(gear_rect.center(), Vec2::new(11.0, 11.0)),
                        gear_color,
                    );
                    if settings_btn.clicked() {
                        settings_clicked = true;
                    }
                    settings_pressed = settings_btn.is_pointer_button_down_on() || settings_btn.hovered();
                    settings_btn.on_hover_text("Open Settings");

                    ui.add_space(6.0);
                });

                // Custom Win32 drag with >5px threshold (immune to click-jitter & taskbar edge clamping)
                if our_hwnd != 0 {
                    let primary_pressed = ui.input(|i| i.pointer.primary_pressed());
                    let primary_down = ui.input(|i| i.pointer.primary_down());

                    if primary_pressed && !settings_pressed && dock_response.hovered() {
                        if let (Some((cx, cy)), Some(win_r)) =
                            (get_cursor_physical_pos(), get_window_physical_rect(our_hwnd))
                        {
                            self.drag_anchor = Some((cx, cy, win_r.left, win_r.top));
                            self.is_actively_dragging = false;
                        }
                    }

                    if let Some((cx0, cy0, wx0, wy0)) = self.drag_anchor {
                        if primary_down {
                            if let Some((cx, cy)) = get_cursor_physical_pos() {
                                let dx = cx - cx0;
                                let dy = cy - cy0;
                                if !self.is_actively_dragging && (dx.abs() > 5 || dy.abs() > 5) {
                                    self.is_actively_dragging = true;
                                }
                                if self.is_actively_dragging {
                                    set_window_physical_pos(our_hwnd, wx0 + dx, wy0 + dy);
                                    ctx.request_repaint_after(Duration::from_millis(16));
                                }
                            }
                        } else {
                            was_dragging_on_release = self.is_actively_dragging;
                            if self.is_actively_dragging {
                                if let Some(final_r) = get_window_physical_rect(our_hwnd) {
                                    config.custom_pos_x = final_r.left as f32;
                                    config.custom_pos_y = final_r.top as f32;
                                    config.position_mode = DockPositionMode::Custom;
                                    config.save();
                                }
                            }
                            self.drag_anchor = None;
                            self.is_actively_dragging = false;
                        }
                    }
                }

                if settings_clicked || (dock_response.clicked() && !was_dragging_on_release) {
                    *open_settings = true;
                }
            });

        if refresh_requested {
            on_refresh();
        }

        // Smooth animation timer only while critical or charging pulse is active
        let needs_pulse = devices.iter().any(|d| {
            d.is_charging || d.percentage <= config.critical_battery_threshold
        });
        if self.is_actively_dragging || needs_pulse {
            ctx.request_repaint_after(Duration::from_millis(50));
        } else {
            ctx.request_repaint_after(Duration::from_millis(1000));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battery::BatterySource;

    fn sample_dev(pct: u8, charging: bool) -> DeviceBattery {
        DeviceBattery {
            id: format!("dev_{pct}"),
            name: "Sample Device".to_string(),
            device_type: DeviceType::Headset,
            percentage: pct,
            is_charging: charging,
            is_connected: true,
            source: BatterySource::LogitechHidpp,
        }
    }

    #[test]
    fn test_required_width_accounts_for_charging_icon_and_percentage_toggle() {
        let mut cfg = AppConfig::default();
        let empty_w = DockOverlay::required_width(&[], &cfg);
        assert_eq!(empty_w, 128.0);

        let one_normal = vec![sample_dev(27, false)];
        let one_charging = vec![sample_dev(27, true)];
        let one_full = vec![sample_dev(100, true)];

        let w_norm = DockOverlay::required_width(&one_normal, &cfg);
        let w_chg = DockOverlay::required_width(&one_charging, &cfg);
        let w_full = DockOverlay::required_width(&one_full, &cfg);

        assert_eq!(w_chg - w_norm, 12.0, "Charging bolt badge must expand pill by 12px");
        assert_eq!(w_full - w_chg, 6.0, "100% must reserve 6px more than 27%");

        cfg.show_percentage_text = false;
        let w_no_text = DockOverlay::required_width(&one_charging, &cfg);
        assert!(w_no_text < w_chg);
    }
}
