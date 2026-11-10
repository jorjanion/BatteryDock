use crate::battery::{DeviceBattery, DeviceType};
use crate::config::{AppConfig, DockPositionMode};
use crate::ui::icons::DevicePainter;
use egui::{Color32, CornerRadius, Pos2, Rect, Stroke, StrokeKind, Vec2};
use std::time::Instant;

pub struct DockOverlay {
    start_time: Instant,
}

impl DockOverlay {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        config: &mut AppConfig,
        devices: &[DeviceBattery],
        open_settings: &mut bool,
        on_refresh: impl FnOnce(),
    ) {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        // Pulse alpha for critical battery warning (sine wave between 0.0 and 1.0)
        let pulse_alpha = ((elapsed * 4.0).sin() * 0.5 + 0.5).clamp(0.0, 1.0);

        let mut refresh_requested = false;

        // Custom frameless central panel
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(Color32::TRANSPARENT))
            .show(ctx, |ui| {
                let avail_rect = ui.available_rect_before_wrap();

                // Frosted glass background pill
                let bg_color = Color32::from_rgba_premultiplied(18, 20, 24, 215);
                let border_stroke = Stroke::new(1.0_f32, Color32::from_white_alpha(35));
                ui.painter().rect_filled(avail_rect, CornerRadius::same(14), bg_color);
                ui.painter().rect_stroke(avail_rect, CornerRadius::same(14), border_stroke, StrokeKind::Inside);

                // Handle window dragging anywhere on the dock
                let dock_response = ui.interact(
                    avail_rect,
                    ui.id().with("dock_drag_area"),
                    egui::Sense::click_and_drag(),
                );

                if dock_response.drag_started() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }

                if dock_response.drag_stopped() {
                    if let Some(rect) = ctx.input(|i| i.viewport().outer_rect) {
                        config.custom_pos_x = rect.min.x;
                        config.custom_pos_y = rect.min.y;
                        config.position_mode = DockPositionMode::Custom;
                        config.save();
                    }
                }

                // Clicking the dock toggles settings
                if dock_response.clicked() {
                    *open_settings = !*open_settings;
                }

                dock_response.context_menu(|ui| {
                    ui.label(egui::RichText::new("🔋 BatteryDock").strong());
                    ui.separator();
                    if ui.button("⚙ Settings").clicked() {
                        *open_settings = true;
                        ui.close_menu();
                    }
                    if ui.button("🔄 Refresh Devices").clicked() {
                        refresh_requested = true;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("📌 Reset to Taskbar Clock").clicked() {
                        config.position_mode = DockPositionMode::TaskbarLeftOfTray;
                        config.save();
                        ui.close_menu();
                    }
                    if ui.button("❌ Exit BatteryDock").clicked() {
                        std::process::exit(0);
                    }
                });

                // Content inside the pill
                ui.horizontal_centered(|ui| {
                    ui.add_space(8.0);

                    if devices.is_empty() {
                        // Empty state: clean minimalist indicator
                        let icon_rect = Rect::from_center_size(
                            Pos2::new(ui.cursor().min.x + 8.0, avail_rect.center().y),
                            Vec2::new(14.0, 14.0),
                        );
                        DevicePainter::paint_device_icon(
                            ui.painter(),
                            icon_rect,
                            DeviceType::Unknown,
                            Color32::from_white_alpha(150),
                        );
                        ui.add_space(20.0);
                        ui.label(
                            egui::RichText::new("BatteryDock")
                                .size(11.5)
                                .color(Color32::from_white_alpha(160)),
                        );
                        ui.add_space(4.0);
                    } else {
                        for (idx, dev) in devices.iter().enumerate() {
                            if idx > 0 {
                                // Subtle divider between devices
                                ui.add_space(4.0);
                                let div_x = ui.cursor().min.x;
                                ui.painter().line_segment(
                                    [
                                        Pos2::new(div_x, avail_rect.center().y - 8.0),
                                        Pos2::new(div_x, avail_rect.center().y + 8.0),
                                    ],
                                    Stroke::new(1.0_f32, Color32::from_white_alpha(25)),
                                );
                                ui.add_space(6.0);
                            }

                            // Device Item
                            let item_response = ui.horizontal(|ui| {
                                // Device Icon
                                let icon_size = 16.0;
                                let (icon_rect, _) = ui.allocate_exact_size(
                                    Vec2::new(icon_size, icon_size),
                                    egui::Sense::hover(),
                                );
                                let icon_color = if dev.percentage <= config.critical_battery_threshold {
                                    Color32::from_rgb(239, 68, 68)
                                } else {
                                    Color32::from_white_alpha(220)
                                };
                                DevicePainter::paint_device_icon(
                                    ui.painter(),
                                    icon_rect,
                                    dev.device_type,
                                    icon_color,
                                );

                                ui.add_space(3.0);

                                // Battery Meter
                                let meter_w = 23.0;
                                let meter_h = 10.5;
                                let (meter_rect, _) = ui.allocate_exact_size(
                                    Vec2::new(meter_w, meter_h),
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

                                // Percentage text
                                if config.show_percentage_text {
                                    ui.add_space(2.0);
                                    let text_color = if dev.is_charging {
                                        Color32::from_rgb(56, 189, 248)
                                    } else if dev.percentage <= config.critical_battery_threshold {
                                        Color32::from_rgb(239, 68, 68)
                                    } else if dev.percentage <= config.low_battery_threshold {
                                        Color32::from_rgb(245, 158, 11)
                                    } else {
                                        Color32::from_white_alpha(215)
                                    };

                                    let charging_icon = if dev.is_charging { "⚡" } else { "" };
                                    ui.label(
                                        egui::RichText::new(format!("{}{}%", dev.percentage, charging_icon))
                                            .size(11.0)
                                            .color(text_color)
                                            .strong(),
                                    );
                                }
                            });

                            // Tooltip on hover
                            let charging_desc = if dev.is_charging {
                                " [Charging ⚡]"
                            } else {
                                ""
                            };
                            item_response.response.on_hover_ui(|ui| {
                                ui.label(egui::RichText::new(&dev.name).strong());
                                ui.label(format!(
                                    "Type: {} | Battery: {}%{}",
                                    dev.device_type.display_name(),
                                    dev.percentage,
                                    charging_desc
                                ));
                                ui.label(
                                    egui::RichText::new("Double-click dock to open settings")
                                        .size(10.0)
                                        .italics()
                                        .weak(),
                                );
                            });
                        }
                    }

                    // Settings gear icon at the end
                    ui.add_space(4.0);
                    let settings_btn = ui.add(
                        egui::Button::new(
                            egui::RichText::new("⚙")
                                .size(11.0)
                                .color(Color32::from_white_alpha(110)),
                        )
                        .frame(false),
                    );
                    if settings_btn.clicked() {
                        *open_settings = true;
                    }
                    settings_btn.on_hover_text("Open Settings");

                    ui.add_space(6.0);
                });
            });

        if refresh_requested {
            on_refresh();
        }

        // Request repaint smoothly only if critical battery animation is active
        let has_critical = devices
            .iter()
            .any(|d| d.percentage <= config.critical_battery_threshold && !d.is_charging);
        if has_critical {
            ctx.request_repaint_after(std::time::Duration::from_millis(50));
        } else {
            ctx.request_repaint_after(std::time::Duration::from_millis(1000));
        }
    }
}
