use crate::battery::{DeviceBattery, DeviceType};
use crate::config::{AppConfig, DockPositionMode};
use crate::ui::icons::DevicePainter;
use crate::ui::taskbar::{
    get_cursor_physical_pos, get_window_physical_rect, is_primary_mouse_button_down,
    set_window_physical_pos, show_dock_context_menu, DockContextMenuAction,
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
        let visible: Vec<&DeviceBattery> = devices
            .iter()
            .filter(|d| !config.is_device_hidden(&d.id))
            .collect();

        if visible.is_empty() {
            let label_w = config.language.empty_dock_text_width();
            w += 16.0 + 6.0 + label_w + 4.0;
        } else {
            for (idx, dev) in visible.iter().enumerate() {
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
                let show_pct = config.show_percentage_text && !config.compact_mode;
                if show_pct {
                    w += 3.0 + Self::percentage_label_width(dev.percentage);
                }
            }
        }

        // Symmetrical right padding (8.0)
        w += 8.0;
        w.max(48.0)
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
        let lang = config.language;
        self.render_with_context_menu(
            ctx,
            config,
            devices,
            our_hwnd,
            open_settings,
            on_refresh,
            move |hwnd| show_dock_context_menu(hwnd, lang),
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn render_with_context_menu(
        &mut self,
        ctx: &egui::Context,
        config: &mut AppConfig,
        devices: &[DeviceBattery],
        our_hwnd: isize,
        open_settings: &mut bool,
        on_refresh: impl FnOnce(),
        context_menu_fn: impl FnOnce(isize) -> Option<DockContextMenuAction>,
    ) {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let pulse_alpha = ((elapsed * 4.0).sin() * 0.5 + 0.5).clamp(0.0, 1.0);

        let mut refresh_requested = false;
        let mut was_dragging_on_release = self.is_actively_dragging;

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(Color32::TRANSPARENT))
            .show(ctx, |ui| {
                let avail_rect = ui.available_rect_before_wrap();

                let is_light = config.is_effective_light_theme();
                let is_liquid = config.theme_mode == crate::config::ThemeMode::LiquidGlass;

                let (bg_color, border_stroke, divider_stroke) = if is_liquid {
                    (
                        Color32::from_rgba_premultiplied(12, 14, 18, 45),
                        Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(255, 255, 255, 38)),
                        Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(255, 255, 255, 22)),
                    )
                } else if is_light {
                    (
                        Color32::from_rgba_premultiplied(242, 244, 247, 228),
                        Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(0, 0, 0, 38)),
                        Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(0, 0, 0, 32)),
                    )
                } else {
                    (
                        Color32::from_rgba_premultiplied(16, 18, 22, 226),
                        Stroke::new(1.0_f32, Color32::from_white_alpha(32)),
                        Stroke::new(1.0_f32, Color32::from_white_alpha(28)),
                    )
                };

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

                // Content inside the pill with zero implicit egui item spacing so layout == required_width()
                ui.horizontal_centered(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.add_space(8.0);

                    let visible_devices: Vec<&DeviceBattery> = devices
                        .iter()
                        .filter(|d| !config.is_device_hidden(&d.id))
                        .collect();

                    if visible_devices.is_empty() {
                        let (icon_rect, _) =
                            ui.allocate_exact_size(Vec2::new(16.0, 16.0), egui::Sense::hover());
                        let empty_icon_color = if is_light {
                            Color32::from_black_alpha(150)
                        } else {
                            Color32::from_white_alpha(150)
                        };
                        DevicePainter::paint_device_icon(
                            ui.painter(),
                            icon_rect,
                            DeviceType::Unknown,
                            empty_icon_color,
                        );
                        ui.add_space(6.0);
                        let label_w = config.language.empty_dock_text_width();
                        let empty_text_color = if is_light {
                            Color32::from_black_alpha(175)
                        } else {
                            Color32::from_white_alpha(165)
                        };
                        ui.add_sized(
                            Vec2::new(label_w, 16.0),
                            egui::Label::new(
                                egui::RichText::new(config.language.no_devices_connected())
                                    .size(11.5)
                                    .color(empty_text_color),
                            )
                            .wrap_mode(egui::TextWrapMode::Extend)
                            .selectable(false),
                        );
                        ui.add_space(4.0);
                    } else {
                        for (idx, dev) in visible_devices.iter().enumerate() {
                            if idx > 0 {
                                ui.add_space(4.0);
                                let div_x = ui.cursor().min.x;
                                ui.painter().line_segment(
                                    [
                                        Pos2::new(div_x + 0.5, avail_rect.center().y - 8.0),
                                        Pos2::new(div_x + 0.5, avail_rect.center().y + 8.0),
                                    ],
                                    divider_stroke,
                                );
                                ui.add_space(1.0 + 6.0);
                            }

                            let seg_start_x = ui.cursor().min.x;

                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 0.0;

                                // Device Icon
                                let (icon_rect, _) = ui.allocate_exact_size(
                                    Vec2::new(16.0, 16.0),
                                    egui::Sense::hover(),
                                );
                                let icon_color = if is_light {
                                    if dev.is_asleep {
                                        Color32::from_black_alpha(110)
                                    } else if dev.is_charging {
                                        Color32::from_rgb(2, 132, 199)
                                    } else if dev.percentage <= config.critical_battery_threshold {
                                        Color32::from_rgb(220, 38, 38)
                                    } else {
                                        Color32::from_black_alpha(215)
                                    }
                                } else if dev.is_asleep {
                                    Color32::from_white_alpha(115)
                                } else if dev.is_charging {
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
                                DevicePainter::paint_battery_meter_themed(
                                    ui.painter(),
                                    meter_rect,
                                    dev.percentage,
                                    dev.is_charging,
                                    config.low_battery_threshold,
                                    config.critical_battery_threshold,
                                    if dev.is_asleep { 0.5 } else { pulse_alpha },
                                    is_light,
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

                                // Percentage text (hidden in compact mode)
                                let show_pct = config.show_percentage_text && !config.compact_mode;
                                if show_pct {
                                    ui.add_space(3.0);
                                    let text_color = if is_light {
                                        if dev.is_asleep {
                                            Color32::from_black_alpha(120)
                                        } else if dev.is_charging {
                                            Color32::from_rgb(2, 132, 199)
                                        } else if dev.percentage <= config.critical_battery_threshold {
                                            Color32::from_rgb(220, 38, 38)
                                        } else if dev.percentage <= config.low_battery_threshold {
                                            Color32::from_rgb(217, 119, 6)
                                        } else {
                                            Color32::from_black_alpha(225)
                                        }
                                    } else if dev.is_asleep {
                                        Color32::from_white_alpha(120)
                                    } else if dev.is_charging {
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
                                        .wrap_mode(egui::TextWrapMode::Extend)
                                        .selectable(false),
                                    );
                                }
                            });

                            let seg_end_x = ui.cursor().min.x;
                            let seg_rect = Rect::from_min_max(
                                Pos2::new(seg_start_x, avail_rect.min.y),
                                Pos2::new(seg_end_x, avail_rect.max.y),
                            );

                            let dev_hover_resp = ui.interact(
                                seg_rect,
                                ui.id().with(format!("dev_seg_{}", dev.id)),
                                egui::Sense::hover(),
                            );

                            // Subtle rounded hover glow over hovered peripheral
                            if dev_hover_resp.hovered() && !self.is_actively_dragging {
                                let highlight_rect = seg_rect.shrink2(Vec2::new(1.0, 3.0));
                                let highlight_color = if is_light {
                                    Color32::from_black_alpha(18)
                                } else if is_liquid {
                                    Color32::from_white_alpha(28)
                                } else {
                                    Color32::from_white_alpha(18)
                                };
                                ui.painter().rect_filled(
                                    highlight_rect,
                                    CornerRadius::same(7),
                                    highlight_color,
                                );
                            }

                            // Rich Telemetry Tooltip
                            let display_name = config.get_device_display_name(&dev.id, &dev.name).to_string();
                            let dev_copy = (*dev).clone();
                            let lang = config.language;
                            dev_hover_resp.on_hover_ui(|ui| {
                                ui.set_max_width(260.0);
                                ui.spacing_mut().item_spacing.y = 3.0;

                                // Header: Icon + Device Name
                                ui.horizontal(|ui| {
                                    let (icon_rect, _) = ui.allocate_exact_size(
                                        Vec2::new(14.0, 14.0),
                                        egui::Sense::hover(),
                                    );
                                    DevicePainter::paint_device_icon(
                                        ui.painter(),
                                        icon_rect,
                                        dev_copy.device_type,
                                        Color32::WHITE,
                                    );
                                    ui.label(egui::RichText::new(display_name).strong().size(12.5));
                                });

                                ui.separator();

                                // Protocol
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(lang.label_protocol()).weak().size(11.0));
                                    ui.label(egui::RichText::new(dev_copy.source.display_name()).size(11.0));
                                });

                                // Status
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(lang.label_status()).weak().size(11.0));
                                    let (status_text, status_color) = if dev_copy.is_charging {
                                        (lang.status_charging(), Color32::from_rgb(56, 189, 248))
                                    } else if dev_copy.is_asleep {
                                        (lang.status_asleep(), Color32::from_rgb(192, 132, 252))
                                    } else {
                                        (lang.status_active(), Color32::from_rgb(74, 222, 128))
                                    };
                                    ui.label(egui::RichText::new(status_text).color(status_color).strong().size(11.0));
                                });

                                // Estimated Autonomy / Time to Full
                                ui.horizontal(|ui| {
                                    let time_label = lang.label_estimated_time(dev_copy.is_charging);
                                    ui.label(egui::RichText::new(time_label).weak().size(11.0));
                                    if let Some(mins) = dev_copy.estimated_minutes_left {
                                        let time_str = lang.format_time_left(mins, dev_copy.is_charging);
                                        ui.label(egui::RichText::new(time_str).strong().size(11.0));
                                    } else {
                                        ui.label(egui::RichText::new("—").weak().size(11.0));
                                    }
                                });
                            });
                        }
                    }

                    // Symmetrical right padding
                    ui.add_space(8.0);
                });

                if dock_response.secondary_clicked() {
                    if let Some(action) = context_menu_fn(our_hwnd) {
                        match action {
                            DockContextMenuAction::Settings => *open_settings = true,
                            DockContextMenuAction::Refresh => refresh_requested = true,
                            DockContextMenuAction::ResetPosition => {
                                config.position_mode = DockPositionMode::TaskbarLeftOfTray;
                                config.custom_pos_x = 0.0;
                                config.custom_pos_y = 0.0;
                                config.save();
                            }
                            DockContextMenuAction::Restart => crate::restart_app(),
                            DockContextMenuAction::Exit => std::process::exit(0),
                        }
                    }
                }

                // Custom Win32 drag with >5px threshold (uses Win32 GetAsyncKeyState so release outside 32px pill never sticks)
                if our_hwnd != 0 {
                    let primary_pressed = ui.input(|i| i.pointer.primary_pressed());
                    let pointer_in_pill = ui.rect_contains_pointer(avail_rect);

                    if primary_pressed && pointer_in_pill {
                        if let (Some((cx, cy)), Some(win_r)) = (
                            get_cursor_physical_pos(),
                            get_window_physical_rect(our_hwnd),
                        ) {
                            self.drag_anchor = Some((cx, cy, win_r.left, win_r.top));
                            self.is_actively_dragging = false;
                        }
                    }

                    if let Some((cx0, cy0, wx0, wy0)) = self.drag_anchor {
                        let physical_down = is_primary_mouse_button_down();
                        if physical_down {
                            if let Some((cx, cy)) = get_cursor_physical_pos() {
                                let dx = cx - cx0;
                                let dy = cy - cy0;
                                if !self.is_actively_dragging && (dx.abs() > 5 || dy.abs() > 5) {
                                    self.is_actively_dragging = true;
                                }
                                if self.is_actively_dragging {
                                    set_window_physical_pos(our_hwnd, wx0 + dx, wy0 + dy);
                                }
                            }
                            ctx.request_repaint_after(Duration::from_millis(16));
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

                if dock_response.clicked() && !was_dragging_on_release {
                    *open_settings = true;
                }
            });

        if refresh_requested {
            on_refresh();
        }

        // Smooth animation timer while dragging, critical, or charging pulse is active
        let needs_pulse = devices
            .iter()
            .any(|d| d.is_charging || d.percentage <= config.critical_battery_threshold);
        if self.drag_anchor.is_some() || self.is_actively_dragging || needs_pulse {
            ctx.request_repaint_after(Duration::from_millis(if self.drag_anchor.is_some() {
                16
            } else {
                50
            }));
        } else {
            ctx.request_repaint_after(Duration::from_millis(1000));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battery::BatterySource;
    use crate::i18n::Language;

    fn sample_dev(pct: u8, charging: bool) -> DeviceBattery {
        DeviceBattery::new(
            format!("dev_{pct}"),
            "Sample Device".to_string(),
            DeviceType::Headset,
            pct,
            charging,
            true,
            BatterySource::LogitechHidpp,
        )
    }

    #[test]
    fn test_required_width_accounts_for_charging_icon_and_percentage_toggle() {
        let mut cfg = AppConfig {
            language: Language::English,
            ..Default::default()
        };
        let empty_w = DockOverlay::required_width(&[], &cfg);
        assert_eq!(empty_w, 162.0);

        cfg.language = Language::Spanish;
        let empty_es_w = DockOverlay::required_width(&[], &cfg);
        assert_eq!(empty_es_w, 194.0);
        cfg.language = Language::English;

        let one_normal = vec![sample_dev(27, false)];
        let one_charging = vec![sample_dev(27, true)];
        let one_full = vec![sample_dev(100, true)];

        let w_norm = DockOverlay::required_width(&one_normal, &cfg);
        let w_chg = DockOverlay::required_width(&one_charging, &cfg);
        let w_full = DockOverlay::required_width(&one_full, &cfg);

        assert_eq!(
            w_chg - w_norm,
            12.0,
            "Charging bolt badge must expand pill by 12px"
        );
        assert_eq!(w_full - w_chg, 6.0, "100% must reserve 6px more than 27%");

        cfg.show_percentage_text = false;
        let w_no_text = DockOverlay::required_width(&one_charging, &cfg);
        assert!(w_no_text < w_chg);
    }

    #[test]
    fn test_dock_render_fits_required_width_and_renders_charging_bolt() {
        let mut cfg = AppConfig::default();
        let cases = vec![
            vec![],
            vec![sample_dev(27, false)],
            vec![sample_dev(27, true)],
            vec![sample_dev(100, true)],
            vec![sample_dev(5, false), sample_dev(88, true)],
        ];

        for devs in cases {
            let req_w = DockOverlay::required_width(&devs, &cfg);
            let ctx = egui::Context::default();
            let mut dock = DockOverlay::new();
            let mut open_settings = false;
            let raw_input = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(req_w, 32.0))),
                ..Default::default()
            };

            let out = ctx.run(raw_input, |ctx| {
                dock.render(ctx, &mut cfg, &devs, 0, &mut open_settings, || {});
            });

            // Collect all rendered text galleys & verify zero unwanted line wrapping or horizontal overflow
            let mut saw_text = false;
            let mut max_right = 0.0_f32;
            for clipped in &out.shapes {
                let bb = clipped.shape.visual_bounding_rect();
                if bb.is_finite() && bb.width() > 0.0 {
                    max_right = max_right.max(bb.right());
                }
                if let egui::epaint::Shape::Text(text_shape) = &clipped.shape {
                    saw_text = true;
                    assert_eq!(
                        text_shape.galley.rows.len(),
                        1,
                        "Text '{}' wrapped onto multiple lines inside {}px dock",
                        text_shape.galley.text(),
                        req_w
                    );
                }
            }
            assert!(saw_text, "Expected rendered text inside dock");
            assert!(
                max_right <= req_w + 1.5,
                "Rendered content overflowed right edge: max_right={max_right} > req_w={req_w}"
            );
        }

        // Charging pill must emit additional convex polygon shapes for the internal + external lightning bolts
        let ctx = egui::Context::default();
        let mut dock = DockOverlay::new();
        let mut open_settings = false;
        let mut run_count_shapes = |devs: &[DeviceBattery]| -> usize {
            let w = DockOverlay::required_width(devs, &cfg);
            let raw = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(w, 32.0))),
                ..Default::default()
            };
            let out = ctx.run(raw, |ctx| {
                dock.render(ctx, &mut cfg, devs, 0, &mut open_settings, || {});
            });
            out.shapes.len()
        };
        let norm_shapes = run_count_shapes(&[sample_dev(50, false)]);
        let chg_shapes = run_count_shapes(&[sample_dev(50, true)]);
        assert!(
            chg_shapes >= norm_shapes + 6,
            "Charging state must paint lightning bolt shapes (norm={norm_shapes}, chg={chg_shapes})"
        );
    }

    #[test]
    fn test_settings_gear_right_click_opens_context_menu() {
        let mut cfg = AppConfig::default();
        let devs = vec![sample_dev(75, false)];
        let w = DockOverlay::required_width(&devs, &cfg);
        let gear_pos = Pos2::new(w - 13.0, 16.0);

        let ctx = egui::Context::default();
        let mut dock = DockOverlay::new();
        let mut open_settings = false;
        let mut menu_opened_count = 0usize;

        // Frame 1: initialize widget layout
        let raw_init = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(w, 32.0))),
            ..Default::default()
        };
        let _ = ctx.run(raw_init, |ctx| {
            dock.render_with_context_menu(
                ctx,
                &mut cfg,
                &devs,
                0,
                &mut open_settings,
                || {},
                |_| {
                    menu_opened_count += 1;
                    None
                },
            );
        });

        // Frame 2: secondary mouse press on gear icon
        let raw_down = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(w, 32.0))),
            events: vec![
                egui::Event::PointerMoved(gear_pos),
                egui::Event::PointerButton {
                    pos: gear_pos,
                    button: egui::PointerButton::Secondary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
            ..Default::default()
        };
        let _ = ctx.run(raw_down, |ctx| {
            dock.render_with_context_menu(
                ctx,
                &mut cfg,
                &devs,
                0,
                &mut open_settings,
                || {},
                |_| {
                    menu_opened_count += 1;
                    None
                },
            );
        });

        // Frame 3: secondary mouse release on gear icon -> must trigger context menu
        let raw_up = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(w, 32.0))),
            events: vec![egui::Event::PointerButton {
                pos: gear_pos,
                button: egui::PointerButton::Secondary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..Default::default()
        };
        let _ = ctx.run(raw_up, |ctx| {
            dock.render_with_context_menu(
                ctx,
                &mut cfg,
                &devs,
                0,
                &mut open_settings,
                || {},
                |_| {
                    menu_opened_count += 1;
                    None
                },
            );
        });

        assert_eq!(
            menu_opened_count, 1,
            "Right-clicking the dock pill must invoke the context menu"
        );
        assert!(
            !open_settings,
            "Right-clicking must not trigger left-click Settings open unless selected in menu"
        );
    }

    #[test]
    fn test_device_pill_right_click_opens_context_menu_and_left_click_opens_settings() {
        let mut cfg = AppConfig::default();
        let devs = vec![sample_dev(75, true)];
        let w = DockOverlay::required_width(&devs, &cfg);
        let pct_label_pos = Pos2::new(w - 35.0, 16.0);

        let ctx = egui::Context::default();
        let mut dock = DockOverlay::new();
        let mut open_settings = false;
        let mut menu_opened_count = 0usize;

        let run_frame = |dock: &mut DockOverlay,
                         cfg: &mut AppConfig,
                         open_settings: &mut bool,
                         menu_opened_count: &mut usize,
                         events: Vec<egui::Event>| {
            let raw = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(w, 32.0))),
                events,
                ..Default::default()
            };
            let _ = ctx.run(raw, |ctx| {
                dock.render_with_context_menu(
                    ctx,
                    cfg,
                    &devs,
                    0,
                    open_settings,
                    || {},
                    |_| {
                        *menu_opened_count += 1;
                        None
                    },
                );
            });
        };

        run_frame(
            &mut dock,
            &mut cfg,
            &mut open_settings,
            &mut menu_opened_count,
            vec![],
        );

        // Right-click on the percentage text / device area
        run_frame(
            &mut dock,
            &mut cfg,
            &mut open_settings,
            &mut menu_opened_count,
            vec![
                egui::Event::PointerMoved(pct_label_pos),
                egui::Event::PointerButton {
                    pos: pct_label_pos,
                    button: egui::PointerButton::Secondary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
        run_frame(
            &mut dock,
            &mut cfg,
            &mut open_settings,
            &mut menu_opened_count,
            vec![egui::Event::PointerButton {
                pos: pct_label_pos,
                button: egui::PointerButton::Secondary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            }],
        );
        assert_eq!(menu_opened_count, 1);
        assert!(!open_settings);

        // Left-click on the percentage text / device area opens Settings
        run_frame(
            &mut dock,
            &mut cfg,
            &mut open_settings,
            &mut menu_opened_count,
            vec![egui::Event::PointerButton {
                pos: pct_label_pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            }],
        );
        run_frame(
            &mut dock,
            &mut cfg,
            &mut open_settings,
            &mut menu_opened_count,
            vec![egui::Event::PointerButton {
                pos: pct_label_pos,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            }],
        );
        assert!(open_settings);
    }

    #[test]
    fn test_dock_renders_all_theme_modes() {
        let themes = [
            crate::config::ThemeMode::LiquidGlass,
            crate::config::ThemeMode::Dark,
            crate::config::ThemeMode::Light,
            crate::config::ThemeMode::System,
        ];
        let devs = vec![sample_dev(85, false), sample_dev(15, true)];
        let ctx = egui::Context::default();
        let mut dock = DockOverlay::new();

        for theme in themes {
            let mut cfg = AppConfig {
                theme_mode: theme,
                ..AppConfig::default()
            };
            let w = DockOverlay::required_width(&devs, &cfg);
            let raw = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(w, 32.0))),
                ..Default::default()
            };
            let mut open_settings = false;
            let _ = ctx.run(raw, |ctx| {
                dock.render(ctx, &mut cfg, &devs, 0, &mut open_settings, || {});
            });
        }
    }
}
