use crate::battery::DeviceBattery;
use crate::config::{AppConfig, DockPositionMode};
use crate::notifications::audio;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingsTab {
    #[default]
    Devices,
    Alerts,
    Customization,
}

pub struct SettingsWindow;

impl SettingsWindow {
    fn paint_reorder_button(ui: &mut egui::Ui, up: bool, enabled: bool) -> egui::Response {
        let sense = if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        };
        let (rect, resp) = ui.allocate_exact_size(egui::Vec2::new(20.0, 20.0), sense);
        let bg = if enabled && resp.hovered() {
            egui::Color32::from_white_alpha(28)
        } else {
            egui::Color32::from_white_alpha(10)
        };
        let fg = if !enabled {
            egui::Color32::from_white_alpha(45)
        } else if resp.hovered() {
            egui::Color32::WHITE
        } else {
            egui::Color32::from_white_alpha(180)
        };
        ui.painter()
            .rect_filled(rect, egui::CornerRadius::same(5), bg);
        let c = rect.center();
        let pts = if up {
            vec![
                egui::Pos2::new(c.x - 4.0, c.y + 2.0),
                egui::Pos2::new(c.x, c.y - 2.5),
                egui::Pos2::new(c.x + 4.0, c.y + 2.0),
            ]
        } else {
            vec![
                egui::Pos2::new(c.x - 4.0, c.y - 2.0),
                egui::Pos2::new(c.x, c.y + 2.5),
                egui::Pos2::new(c.x + 4.0, c.y - 2.0),
            ]
        };
        ui.painter()
            .add(egui::Shape::line(pts, egui::Stroke::new(1.5_f32, fg)));
        resp
    }

    fn paint_battery_sparkline(
        ui: &mut egui::Ui,
        points: &[crate::battery::history::BatteryPoint],
        current_pct: u8,
        is_charging: bool,
        low_thresh: u8,
        crit_thresh: u8,
        lang: crate::i18n::Language,
    ) {
        let accent = if is_charging {
            egui::Color32::from_rgb(56, 189, 248)
        } else if current_pct <= crit_thresh {
            egui::Color32::from_rgb(239, 68, 68)
        } else if current_pct <= low_thresh {
            egui::Color32::from_rgb(245, 158, 11)
        } else {
            egui::Color32::from_rgb(34, 197, 94)
        };
        let fill_col = egui::Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 30);

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(lang.battery_history_label())
                    .size(10.5)
                    .weak(),
            );
            ui.with_layout(
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| {
                    if points.len() >= 2 {
                        let min_p = points.iter().map(|p| p.percentage).min().unwrap_or(current_pct);
                        let max_p = points.iter().map(|p| p.percentage).max().unwrap_or(current_pct);
                        ui.label(
                            egui::RichText::new(format!("Min {}% • Max {}%", min_p, max_p))
                                .size(10.0)
                                .weak(),
                        );
                    } else {
                        ui.label(
                            egui::RichText::new(lang.battery_history_collecting())
                                .size(10.0)
                                .weak(),
                        );
                    }
                },
            );
        });

        ui.add_space(2.0);
        let chart_w = ui.available_width().max(120.0);
        let chart_h = 36.0;
        let (rect, _) =
            ui.allocate_exact_size(egui::Vec2::new(chart_w, chart_h), egui::Sense::hover());

        let painter = ui.painter();
        painter.rect_filled(
            rect,
            egui::CornerRadius::same(6),
            egui::Color32::from_rgb(18, 20, 25),
        );
        painter.rect_stroke(
            rect,
            egui::CornerRadius::same(6),
            egui::Stroke::new(1.0_f32, egui::Color32::from_white_alpha(15)),
            egui::StrokeKind::Inside,
        );

        let pad_x = 6.0;
        let pad_y = 5.0;
        let inner = rect.shrink2(egui::Vec2::new(pad_x, pad_y));
        if inner.width() <= 2.0 || inner.height() <= 2.0 {
            return;
        }

        // Subtle 50% dashed/faint midline
        let mid_y = inner.center().y;
        painter.line_segment(
            [
                egui::Pos2::new(inner.min.x, mid_y),
                egui::Pos2::new(inner.max.x, mid_y),
            ],
            egui::Stroke::new(1.0_f32, egui::Color32::from_white_alpha(12)),
        );

        let pct_to_y = |pct: u8| -> f32 {
            let frac = (pct.min(100) as f32) / 100.0;
            inner.max.y - frac * inner.height()
        };

        let mut coords: Vec<egui::Pos2> = Vec::new();
        if points.len() >= 2 {
            let t0 = points.first().unwrap().timestamp_secs;
            let t1 = points.last().unwrap().timestamp_secs;
            let span = t1.saturating_sub(t0);
            let last_idx = (points.len() - 1) as f32;
            for (i, pt) in points.iter().enumerate() {
                let x_frac = if span > 0 {
                    (pt.timestamp_secs.saturating_sub(t0) as f32) / (span as f32)
                } else {
                    (i as f32) / last_idx
                };
                let x = inner.min.x + x_frac * inner.width();
                let y = pct_to_y(pt.percentage);
                coords.push(egui::Pos2::new(x, y));
            }
        } else {
            let y = pct_to_y(current_pct);
            coords.push(egui::Pos2::new(inner.min.x, y));
            coords.push(egui::Pos2::new(inner.max.x, y));
        }

        // Soft area fill under curve using convex trapezoid slices
        for win in coords.windows(2) {
            let p0 = win[0];
            let p1 = win[1];
            if (p1.x - p0.x).abs() > 0.1 {
                let poly = vec![
                    egui::Pos2::new(p0.x, inner.max.y),
                    p0,
                    p1,
                    egui::Pos2::new(p1.x, inner.max.y),
                ];
                painter.add(egui::Shape::convex_polygon(
                    poly,
                    fill_col,
                    egui::Stroke::NONE,
                ));
            }
        }

        // Crisp trend line
        painter.add(egui::Shape::line(
            coords.clone(),
            egui::Stroke::new(1.8_f32, accent),
        ));

        // Glowing endpoint dot
        if let Some(&last) = coords.last() {
            painter.circle_filled(last, 4.0, fill_col);
            painter.circle_filled(last, 2.3, accent);
        }
    }

    pub fn render_contents(
        ui: &mut egui::Ui,
        active_tab: &mut SettingsTab,
        open: &mut bool,
        config: &mut AppConfig,
        devices: &[DeviceBattery],
        history: &crate::battery::history::BatteryHistoryStore,
        on_refresh: impl FnOnce(),
    ) {
        let mut should_refresh = false;
        let lang = config.language;

        // Sleek Windows 11 Fluent Header
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.heading("BatteryDock");
            ui.add_space(4.0);
            let ver_label = egui::RichText::new(concat!("v", env!("CARGO_PKG_VERSION")))
                .size(11.0)
                .color(egui::Color32::from_white_alpha(140));
            ui.label(ver_label);

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .button(
                        egui::RichText::new(lang.refresh_now_button())
                            .size(11.5)
                            .color(egui::Color32::from_white_alpha(220)),
                    )
                    .clicked()
                {
                    should_refresh = true;
                }
            });
        });

        // Subtitle status badge with painted vector dot
        ui.horizontal(|ui| {
            let (dot_rect, _) =
                ui.allocate_exact_size(egui::Vec2::new(8.0, 8.0), egui::Sense::hover());
            ui.painter().circle_filled(
                dot_rect.center(),
                3.5,
                egui::Color32::from_rgb(34, 197, 94),
            );
            ui.label(
                egui::RichText::new(lang.status_nominal(devices.len()))
                    .size(11.5)
                    .color(egui::Color32::from_rgb(34, 197, 94)),
            );
        });

        ui.add_space(8.0);

        // Segmented Tab Selector
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            let tabs = [
                (SettingsTab::Devices, lang.tab_devices()),
                (SettingsTab::Alerts, lang.tab_alerts()),
                (SettingsTab::Customization, lang.tab_customization()),
            ];

            for (tab, label) in tabs {
                let is_selected = *active_tab == tab;
                let (bg_color, text_color) = if is_selected {
                    (
                        egui::Color32::from_rgb(37, 99, 235),
                        egui::Color32::WHITE,
                    )
                } else {
                    (
                        egui::Color32::from_white_alpha(18),
                        egui::Color32::from_white_alpha(175),
                    )
                };

                let btn = egui::Button::new(
                    egui::RichText::new(label)
                        .size(12.0)
                        .color(text_color)
                        .strong(),
                )
                .fill(bg_color)
                .corner_radius(8.0)
                .min_size(egui::Vec2::new(0.0, 28.0));

                if ui.add(btn).clicked() {
                    *active_tab = tab;
                }
            }
        });

        ui.add_space(10.0);

        let card_frame = egui::Frame::NONE
            .fill(egui::Color32::from_rgb(26, 28, 34))
            .corner_radius(10.0)
            .stroke(egui::Stroke::new(
                1.0_f32,
                egui::Color32::from_white_alpha(22),
            ))
            .inner_margin(egui::Margin::same(12));

        // Tab Content
        match active_tab {
            SettingsTab::Devices => {
                if devices.is_empty() {
                    card_frame.show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(16.0);
                            let (icon_rect, _) = ui.allocate_exact_size(
                                egui::Vec2::new(28.0, 28.0),
                                egui::Sense::hover(),
                            );
                            crate::ui::icons::DevicePainter::paint_device_icon(
                                ui.painter(),
                                icon_rect,
                                crate::battery::DeviceType::Unknown,
                                egui::Color32::from_white_alpha(120),
                            );
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new(lang.no_devices_detected_label())
                                    .size(13.0)
                                    .strong(),
                            );
                            ui.add_space(4.0);
                            ui.label(
                                egui::RichText::new(
                                    "Connect wireless peripherals via Bluetooth or 2.4GHz USB receiver.",
                                )
                                .size(11.5)
                                .weak(),
                            );
                            ui.add_space(14.0);
                        });
                    });
                } else {
                    let mut sorted_devices: Vec<&DeviceBattery> = devices.iter().collect();
                    sorted_devices.sort_by_key(|d| config.device_sort_key(&d.id));
                    let current_ids: Vec<String> =
                        sorted_devices.iter().map(|d| d.id.clone()).collect();
                    let total_devs = sorted_devices.len();
                    let mut pending_move: Option<(String, isize)> = None;

                    let mut cfg_changed = false;
                    for (idx, dev) in sorted_devices.iter().enumerate() {
                        card_frame.show(ui, |ui| {
                            ui.horizontal(|ui| {
                                let (icon_rect, _) = ui.allocate_exact_size(
                                    egui::Vec2::new(18.0, 18.0),
                                    egui::Sense::hover(),
                                );
                                crate::ui::icons::DevicePainter::paint_device_icon(
                                    ui.painter(),
                                    icon_rect,
                                    dev.device_type,
                                    egui::Color32::from_white_alpha(225),
                                );
                                ui.add_space(6.0);

                                let display_name = config.get_device_display_name(&dev.id, &dev.name);
                                ui.vertical(|ui| {
                                    ui.label(
                                        egui::RichText::new(display_name)
                                            .size(13.5)
                                            .strong(),
                                    );
                                    ui.label(
                                        egui::RichText::new(dev.source.display_name())
                                            .size(11.0)
                                            .weak(),
                                    );
                                });

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if total_devs > 1 {
                                            let can_down = idx + 1 < total_devs;
                                            let down_resp =
                                                Self::paint_reorder_button(ui, false, can_down)
                                                    .on_hover_text(lang.move_device_down_tooltip());
                                            if can_down && down_resp.clicked() {
                                                pending_move = Some((dev.id.clone(), 1));
                                            }

                                            let can_up = idx > 0;
                                            let up_resp =
                                                Self::paint_reorder_button(ui, true, can_up)
                                                    .on_hover_text(lang.move_device_up_tooltip());
                                            if can_up && up_resp.clicked() {
                                                pending_move = Some((dev.id.clone(), -1));
                                            }
                                            ui.add_space(4.0);
                                        }

                                        let color = if dev.is_charging {
                                            egui::Color32::from_rgb(56, 189, 248)
                                        } else if dev.percentage <= config.critical_battery_threshold {
                                            egui::Color32::from_rgb(239, 68, 68)
                                        } else if dev.percentage <= config.low_battery_threshold {
                                            egui::Color32::from_rgb(245, 158, 11)
                                        } else {
                                            egui::Color32::from_rgb(34, 197, 94)
                                        };

                                        let label = if dev.is_charging {
                                            format!("{}%{}", dev.percentage, lang.charging_suffix())
                                        } else {
                                            format!("{}%", dev.percentage)
                                        };
                                        ui.label(
                                            egui::RichText::new(label)
                                                .color(color)
                                                .strong()
                                                .size(14.0),
                                        );

                                        if dev.is_charging {
                                            let (bolt_rect, _) = ui.allocate_exact_size(
                                                egui::Vec2::new(11.0, 14.0),
                                                egui::Sense::hover(),
                                            );
                                            crate::ui::icons::DevicePainter::paint_charging_icon(
                                                ui.painter(),
                                                bolt_rect,
                                                1.0,
                                            );
                                        }
                                    },
                                );
                            });

                            ui.add_space(6.0);
                            ui.separator();
                            ui.add_space(4.0);

                            // Status and Autonomy Telemetry
                            ui.horizontal(|ui| {
                                let (st_txt, st_col) = if dev.is_charging {
                                    (
                                        lang.status_charging(),
                                        egui::Color32::from_rgb(56, 189, 248),
                                    )
                                } else if dev.is_asleep {
                                    (
                                        lang.status_asleep(),
                                        egui::Color32::from_rgb(192, 132, 252),
                                    )
                                } else {
                                    (
                                        lang.status_active(),
                                        egui::Color32::from_rgb(74, 222, 128),
                                    )
                                };

                                ui.label(
                                    egui::RichText::new(st_txt)
                                        .color(st_col)
                                        .strong()
                                        .size(11.5),
                                );

                                if let Some(mins) = dev.estimated_minutes_left {
                                    let time_str = lang.format_time_left(mins, dev.is_charging);
                                    ui.label(
                                        egui::RichText::new(format!("• {time_str}"))
                                            .size(11.5)
                                            .weak(),
                                    );
                                }
                            });

                            ui.add_space(6.0);
                            let (dev_low, dev_crit) = config.get_device_thresholds(&dev.id);
                            Self::paint_battery_sparkline(
                                ui,
                                history.points_for(&dev.id),
                                dev.percentage,
                                dev.is_charging,
                                dev_low,
                                dev_crit,
                                lang,
                            );

                            ui.add_space(6.0);

                            // Collapsible Per-Device Customization
                            let dev_cfg =
                                config.device_configs.entry(dev.id.clone()).or_default();
                            ui.collapsing(lang.heading_per_device_settings(), |ui| {
                                ui.add_space(2.0);
                                // Nickname edit
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("Nickname:").size(11.5));
                                    let mut name_buf =
                                        dev_cfg.custom_name.clone().unwrap_or_default();
                                    if ui.text_edit_singleline(&mut name_buf).changed() {
                                        dev_cfg.custom_name = if name_buf.trim().is_empty() {
                                            None
                                        } else {
                                            Some(name_buf)
                                        };
                                        cfg_changed = true;
                                    }
                                });

                                // Custom alert threshold slider
                                ui.horizontal(|ui| {
                                    let mut use_custom = dev_cfg.low_threshold.is_some();
                                    if ui
                                        .checkbox(&mut use_custom, lang.label_custom_low_alert())
                                        .changed()
                                    {
                                        if use_custom {
                                            dev_cfg.low_threshold =
                                                Some(config.low_battery_threshold);
                                        } else {
                                            dev_cfg.low_threshold = None;
                                        }
                                        cfg_changed = true;
                                    }
                                    if let Some(ref mut low_thresh) = dev_cfg.low_threshold {
                                        if ui
                                            .add(
                                                egui::Slider::new(low_thresh, 5..=50)
                                                    .suffix("%"),
                                            )
                                            .changed()
                                        {
                                            cfg_changed = true;
                                        }
                                    }
                                });

                                // Mute & Hide toggles
                                ui.horizontal(|ui| {
                                    if ui
                                        .checkbox(
                                            &mut dev_cfg.mute_alerts,
                                            lang.label_mute_alerts(),
                                        )
                                        .changed()
                                    {
                                        cfg_changed = true;
                                    }
                                    if ui
                                        .checkbox(
                                            &mut dev_cfg.hidden,
                                            lang.label_hide_from_dock(),
                                        )
                                        .changed()
                                    {
                                        cfg_changed = true;
                                    }
                                });
                            });
                        });
                        ui.add_space(4.0);
                    }
                    if let Some((dev_id, delta)) = pending_move {
                        if config.move_device_order(&current_ids, &dev_id, delta) {
                            cfg_changed = true;
                        }
                    }
                    if cfg_changed {
                        config.save();
                    }
                }
            }
            SettingsTab::Alerts => {
                card_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(lang.heading_alert_thresholds())
                            .strong()
                            .size(13.0),
                    );
                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        ui.label(lang.low_battery_warning());
                        let mut low = config.low_battery_threshold;
                        if ui
                            .add(egui::Slider::new(&mut low, 5..=40).suffix("%"))
                            .changed()
                        {
                            config.low_battery_threshold = low;
                            if config.critical_battery_threshold > config.low_battery_threshold {
                                config.critical_battery_threshold = config.low_battery_threshold;
                            }
                            config.save();
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(lang.critical_battery_warning());
                        let mut crit = config.critical_battery_threshold;
                        if ui
                            .add(egui::Slider::new(&mut crit, 1..=15).suffix("%"))
                            .changed()
                        {
                            config.critical_battery_threshold =
                                crit.min(config.low_battery_threshold);
                            config.save();
                        }
                    });
                });

                ui.add_space(8.0);

                card_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(lang.heading_notifications())
                            .strong()
                            .size(13.0),
                    );
                    ui.add_space(6.0);

                    let mut audio_on = config.audio_alert_enabled;
                    if ui
                        .checkbox(&mut audio_on, lang.play_chime_checkbox())
                        .changed()
                    {
                        config.audio_alert_enabled = audio_on;
                        config.save();
                    }

                    ui.horizontal(|ui| {
                        ui.add_space(20.0);
                        if ui.button(lang.test_chime_button()).clicked() {
                            audio::play_gentle_chime();
                        }
                    });

                    ui.add_space(4.0);

                    let mut visual_on = config.visual_alert_enabled;
                    if ui
                        .checkbox(&mut visual_on, lang.desktop_notifications_checkbox())
                        .changed()
                    {
                        config.visual_alert_enabled = visual_on;
                        config.save();
                    }

                    ui.add_space(4.0);

                    let mut full_chg = config.full_charge_alert_enabled;
                    if ui
                        .checkbox(&mut full_chg, lang.full_charge_alert_checkbox())
                        .changed()
                    {
                        config.full_charge_alert_enabled = full_chg;
                        config.save();
                    }
                });
            }
            SettingsTab::Customization => {
                // Theme Selection
                card_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(lang.heading_theme())
                            .strong()
                            .size(13.0),
                    );
                    ui.add_space(6.0);

                    let current_theme = config.theme_mode;
                    ui.radio_value(
                        &mut config.theme_mode,
                        crate::config::ThemeMode::LiquidGlass,
                        lang.theme_liquid_glass(),
                    );
                    ui.radio_value(
                        &mut config.theme_mode,
                        crate::config::ThemeMode::Dark,
                        lang.theme_dark(),
                    );
                    ui.radio_value(
                        &mut config.theme_mode,
                        crate::config::ThemeMode::Light,
                        lang.theme_light(),
                    );
                    ui.radio_value(
                        &mut config.theme_mode,
                        crate::config::ThemeMode::System,
                        lang.theme_system(),
                    );

                    if config.theme_mode != current_theme {
                        config.save();
                    }
                });

                ui.add_space(8.0);

                // Dock Appearance & Layout
                card_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(lang.heading_dock_appearance())
                            .strong()
                            .size(13.0),
                    );
                    ui.add_space(6.0);

                    let mut show_pct = config.show_percentage_text;
                    if ui
                        .checkbox(&mut show_pct, lang.show_percentage_checkbox())
                        .changed()
                    {
                        config.show_percentage_text = show_pct;
                        config.compact_mode = !show_pct;
                        config.save();
                    }
                });

                ui.add_space(8.0);

                // Dock Placement
                card_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(lang.heading_dock_placement())
                            .strong()
                            .size(13.0),
                    );
                    ui.add_space(6.0);

                    let current_mode = config.position_mode;
                    ui.radio_value(
                        &mut config.position_mode,
                        DockPositionMode::TaskbarLeftOfTray,
                        lang.placement_tray_clock(),
                    );
                    ui.radio_value(
                        &mut config.position_mode,
                        DockPositionMode::TaskbarRight,
                        lang.placement_far_right(),
                    );
                    ui.radio_value(
                        &mut config.position_mode,
                        DockPositionMode::TaskbarLeft,
                        lang.placement_left_side(),
                    );
                    ui.radio_value(
                        &mut config.position_mode,
                        DockPositionMode::Custom,
                        lang.placement_custom(),
                    );

                    if config.position_mode != current_mode {
                        config.save();
                    }

                    ui.add_space(4.0);
                    if ui.button(lang.reset_position_button()).clicked() {
                        config.position_mode = DockPositionMode::TaskbarLeftOfTray;
                        config.custom_pos_x = 0.0;
                        config.custom_pos_y = 0.0;
                        config.save();
                    }
                });

                ui.add_space(8.0);

                // Gaming & Fullscreen HUD Overlay
                card_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(lang.heading_game_overlay())
                            .strong()
                            .size(13.0),
                    );
                    ui.add_space(3.0);
                    ui.label(
                        egui::RichText::new(lang.game_overlay_hint())
                            .size(11.0)
                            .weak(),
                    );
                    ui.add_space(6.0);

                    let mut game_on = config.game_overlay_enabled;
                    if ui
                        .checkbox(&mut game_on, lang.game_overlay_enable_checkbox())
                        .changed()
                    {
                        config.game_overlay_enabled = game_on;
                        config.save();
                    }

                    ui.add_space(4.0);

                    let mut hk_on = config.game_overlay_hotkey_enabled;
                    if ui
                        .checkbox(&mut hk_on, lang.game_overlay_hotkey_checkbox())
                        .changed()
                    {
                        config.game_overlay_hotkey_enabled = hk_on;
                        config.save();
                    }

                    if config.game_overlay_hotkey_enabled {
                        ui.add_space(4.0);
                        let rec_id = ui.id().with("recording_overlay_hotkey");
                        let mut is_recording =
                            ui.data(|d| d.get_temp::<bool>(rec_id).unwrap_or(false));

                        if is_recording {
                            let mut captured: Option<crate::config::OverlayHotkey> = None;
                            let mut cancelled = false;
                            ui.input(|i| {
                                for ev in &i.events {
                                    if let egui::Event::Key {
                                        key,
                                        pressed: true,
                                        modifiers,
                                        ..
                                    } = ev
                                    {
                                        if *key == egui::Key::Escape {
                                            cancelled = true;
                                            break;
                                        }
                                        if let Some(vk) = egui_key_to_vk(*key) {
                                            let has_mod = modifiers.ctrl
                                                || modifiers.alt
                                                || modifiers.shift;
                                            if has_mod
                                                || crate::config::OverlayHotkey::is_standalone_allowed_vk(vk)
                                            {
                                                captured = Some(crate::config::OverlayHotkey {
                                                    ctrl: modifiers.ctrl,
                                                    alt: modifiers.alt,
                                                    shift: modifiers.shift,
                                                    vk_code: vk,
                                                });
                                                break;
                                            }
                                        }
                                    }
                                }
                            });

                            if let Some(new_hk) = captured {
                                config.game_overlay_hotkey = new_hk;
                                config.save();
                                is_recording = false;
                                ui.data_mut(|d| d.insert_temp(rec_id, false));
                            } else if cancelled {
                                is_recording = false;
                                ui.data_mut(|d| d.insert_temp(rec_id, false));
                            }
                        }

                        ui.horizontal(|ui| {
                            ui.add_space(18.0);
                            ui.label(
                                egui::RichText::new(lang.game_overlay_hotkey_label())
                                    .size(12.0)
                                    .strong(),
                            );

                            let btn_text = if is_recording {
                                lang.game_overlay_hotkey_recording().to_string()
                            } else {
                                config.game_overlay_hotkey.display_string()
                            };
                            let btn_fill = if is_recording {
                                egui::Color32::from_rgb(37, 99, 235)
                            } else {
                                egui::Color32::from_rgb(38, 42, 52)
                            };
                            let hk_btn = egui::Button::new(
                                egui::RichText::new(btn_text)
                                    .color(egui::Color32::WHITE)
                                    .strong()
                                    .size(11.5),
                            )
                            .fill(btn_fill)
                            .corner_radius(6.0);

                            if ui.add(hk_btn).clicked() {
                                let next = !is_recording;
                                ui.data_mut(|d| d.insert_temp(rec_id, next));
                            }

                            let def_hk = crate::config::OverlayHotkey::default();
                            if config.game_overlay_hotkey != def_hk
                                && !is_recording
                                && ui.button(lang.game_overlay_hotkey_reset()).clicked()
                            {
                                config.game_overlay_hotkey = def_hk;
                                config.save();
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.add_space(18.0);
                            ui.label(
                                egui::RichText::new(lang.game_overlay_hotkey_hint())
                                    .size(10.5)
                                    .weak(),
                            );
                        });
                    }

                    if config.game_overlay_enabled || config.game_overlay_hotkey_enabled {
                        ui.add_space(6.0);
                        ui.separator();
                        ui.add_space(6.0);

                        ui.label(
                            egui::RichText::new(lang.game_overlay_position_label())
                                .strong()
                                .size(12.0),
                        );
                        ui.add_space(4.0);

                        let mut corner = config.game_overlay_corner;
                        egui::ComboBox::from_id_salt("game_overlay_corner_combo")
                            .selected_text(match corner {
                                crate::config::OverlayCorner::TopLeft => lang.overlay_corner_top_left(),
                                crate::config::OverlayCorner::TopRight => lang.overlay_corner_top_right(),
                                crate::config::OverlayCorner::BottomLeft => lang.overlay_corner_bottom_left(),
                                crate::config::OverlayCorner::BottomRight => lang.overlay_corner_bottom_right(),
                                crate::config::OverlayCorner::Custom => lang.overlay_corner_custom(),
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut corner,
                                    crate::config::OverlayCorner::TopRight,
                                    lang.overlay_corner_top_right(),
                                );
                                ui.selectable_value(
                                    &mut corner,
                                    crate::config::OverlayCorner::TopLeft,
                                    lang.overlay_corner_top_left(),
                                );
                                ui.selectable_value(
                                    &mut corner,
                                    crate::config::OverlayCorner::BottomRight,
                                    lang.overlay_corner_bottom_right(),
                                );
                                ui.selectable_value(
                                    &mut corner,
                                    crate::config::OverlayCorner::BottomLeft,
                                    lang.overlay_corner_bottom_left(),
                                );
                                ui.selectable_value(
                                    &mut corner,
                                    crate::config::OverlayCorner::Custom,
                                    lang.overlay_corner_custom(),
                                );
                            });

                        if corner != config.game_overlay_corner {
                            config.game_overlay_corner = corner;
                            config.save();
                        }

                        ui.add_space(4.0);

                        if config.game_overlay_corner == crate::config::OverlayCorner::Custom {
                            ui.horizontal(|ui| {
                                ui.label(lang.overlay_custom_x_label());
                                let mut x_pct = config.game_overlay_custom_x_pct;
                                if ui
                                    .add(egui::Slider::new(&mut x_pct, 0.0..=100.0).suffix("%"))
                                    .changed()
                                {
                                    config.game_overlay_custom_x_pct = x_pct;
                                    config.save();
                                }
                            });
                            ui.horizontal(|ui| {
                                ui.label(lang.overlay_custom_y_label());
                                let mut y_pct = config.game_overlay_custom_y_pct;
                                if ui
                                    .add(egui::Slider::new(&mut y_pct, 0.0..=100.0).suffix("%"))
                                    .changed()
                                {
                                    config.game_overlay_custom_y_pct = y_pct;
                                    config.save();
                                }
                            });
                        } else {
                            ui.horizontal(|ui| {
                                ui.label(lang.overlay_margin_label());
                                let mut margin = config.game_overlay_margin;
                                if ui
                                    .add(egui::Slider::new(&mut margin, 0.0..=100.0).suffix("px"))
                                    .changed()
                                {
                                    config.game_overlay_margin = margin;
                                    config.save();
                                }
                            });
                        }

                        ui.add_space(4.0);

                        let mut click_through = config.game_overlay_click_through;
                        if ui
                            .checkbox(
                                &mut click_through,
                                lang.game_overlay_click_through_checkbox(),
                            )
                            .changed()
                        {
                            config.game_overlay_click_through = click_through;
                            config.save();
                        }

                        let mut always_on = config.game_overlay_always_on;
                        if ui
                            .checkbox(&mut always_on, lang.game_overlay_always_on_checkbox())
                            .changed()
                        {
                            config.game_overlay_always_on = always_on;
                            config.save();
                        }
                    }
                });

                ui.add_space(8.0);

                // Language
                card_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(lang.heading_language())
                            .strong()
                            .size(13.0),
                    );
                    ui.add_space(6.0);

                    let mut cur_lang = config.language;
                    egui::ComboBox::from_id_salt("settings_language_combobox")
                        .selected_text(cur_lang.display_name())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut cur_lang,
                                crate::i18n::Language::English,
                                crate::i18n::Language::English.display_name(),
                            );
                            ui.selectable_value(
                                &mut cur_lang,
                                crate::i18n::Language::Spanish,
                                crate::i18n::Language::Spanish.display_name(),
                            );
                        });
                    if cur_lang != config.language {
                        config.language = cur_lang;
                        config.save();
                    }
                });

                ui.add_space(8.0);

                // System & Updates
                card_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(lang.heading_updates())
                            .strong()
                            .size(13.0),
                    );
                    ui.add_space(6.0);

                    let mut start_win = config.start_with_windows;
                    if ui
                        .checkbox(&mut start_win, lang.start_with_windows_checkbox())
                        .changed()
                    {
                        config.start_with_windows = start_win;
                        let _ = set_run_at_startup(start_win);
                        config.save();
                    }

                    let mut auto_up = config.auto_update;
                    if ui
                        .checkbox(&mut auto_up, lang.auto_update_checkbox())
                        .changed()
                    {
                        config.auto_update = auto_up;
                        config.save();
                    }

                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        if ui.button(lang.check_updates_now_button()).clicked() {
                            crate::updater::check_for_updates_async(false);
                        }

                        match crate::updater::get_update_status() {
                            crate::updater::UpdateStatus::Idle => {}
                            crate::updater::UpdateStatus::Checking => {
                                ui.label(egui::RichText::new(lang.checking_github_label()).weak());
                            }
                            crate::updater::UpdateStatus::UpToDate => {
                                ui.label(
                                    egui::RichText::new(lang.up_to_date_label())
                                        .color(egui::Color32::from_rgb(34, 197, 94)),
                                );
                            }
                            crate::updater::UpdateStatus::UpdateAvailable {
                                version,
                                download_url,
                            } => {
                                ui.label(
                                    egui::RichText::new(lang.new_version_available_label(&version))
                                        .color(egui::Color32::from_rgb(56, 189, 248))
                                        .strong(),
                                );
                                if ui.button(lang.install_update_button()).clicked() {
                                    let v = version.clone();
                                    let u = download_url.clone();
                                    std::thread::spawn(move || {
                                        let _ = crate::updater::download_and_run_installer(&u, &v);
                                    });
                                }
                            }
                            crate::updater::UpdateStatus::Downloading { version } => {
                                ui.label(
                                    egui::RichText::new(lang.downloading_label(&version))
                                        .color(egui::Color32::from_rgb(56, 189, 248)),
                                );
                            }
                            crate::updater::UpdateStatus::Installing { version } => {
                                ui.label(
                                    egui::RichText::new(lang.installing_label(&version))
                                        .color(egui::Color32::from_rgb(34, 197, 94))
                                        .strong(),
                                );
                            }
                            crate::updater::UpdateStatus::Failed(err) => {
                                ui.label(
                                    egui::RichText::new(lang.update_failed_label(&err))
                                        .color(egui::Color32::from_rgb(239, 68, 68))
                                        .weak(),
                                );
                            }
                        }
                    });

                    ui.add_space(6.0);
                    ui.separator();
                    ui.add_space(6.0);

                    let diag_copied_id = ui.id().with("diag_copied_until");
                    let now_t = ui.input(|i| i.time);
                    let copied_until: f64 =
                        ui.data(|d| d.get_temp::<f64>(diag_copied_id).unwrap_or(0.0));

                    ui.horizontal(|ui| {
                        if ui.button(lang.copy_diagnostics_button()).clicked() {
                            let report =
                                crate::battery::diagnostics::build_hardware_diagnostics_report(
                                    devices,
                                );
                            ui.ctx().copy_text(report);
                            ui.data_mut(|d| d.insert_temp(diag_copied_id, now_t + 3.0));
                        }
                        if now_t < copied_until {
                            ui.label(
                                egui::RichText::new(lang.diagnostics_copied_label())
                                    .color(egui::Color32::from_rgb(34, 197, 94))
                                    .strong()
                                    .size(11.5),
                            );
                        }
                    });
                });
            }
        }

        ui.add_space(12.0);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(lang.close_settings_button()).clicked() {
                *open = false;
            }
        });

        if should_refresh {
            on_refresh();
        }
    }
}

fn egui_key_to_vk(key: egui::Key) -> Option<u16> {
    use egui::Key;
    match key {
        Key::A => Some(0x41),
        Key::B => Some(0x42),
        Key::C => Some(0x43),
        Key::D => Some(0x44),
        Key::E => Some(0x45),
        Key::F => Some(0x46),
        Key::G => Some(0x47),
        Key::H => Some(0x48),
        Key::I => Some(0x49),
        Key::J => Some(0x4A),
        Key::K => Some(0x4B),
        Key::L => Some(0x4C),
        Key::M => Some(0x4D),
        Key::N => Some(0x4E),
        Key::O => Some(0x4F),
        Key::P => Some(0x50),
        Key::Q => Some(0x51),
        Key::R => Some(0x52),
        Key::S => Some(0x53),
        Key::T => Some(0x54),
        Key::U => Some(0x55),
        Key::V => Some(0x56),
        Key::W => Some(0x57),
        Key::X => Some(0x58),
        Key::Y => Some(0x59),
        Key::Z => Some(0x5A),
        Key::Num0 => Some(0x30),
        Key::Num1 => Some(0x31),
        Key::Num2 => Some(0x32),
        Key::Num3 => Some(0x33),
        Key::Num4 => Some(0x34),
        Key::Num5 => Some(0x35),
        Key::Num6 => Some(0x36),
        Key::Num7 => Some(0x37),
        Key::Num8 => Some(0x38),
        Key::Num9 => Some(0x39),
        Key::F1 => Some(0x70),
        Key::F2 => Some(0x71),
        Key::F3 => Some(0x72),
        Key::F4 => Some(0x73),
        Key::F5 => Some(0x74),
        Key::F6 => Some(0x75),
        Key::F7 => Some(0x76),
        Key::F8 => Some(0x77),
        Key::F9 => Some(0x78),
        Key::F10 => Some(0x79),
        Key::F11 => Some(0x7A),
        Key::F12 => Some(0x7B),
        Key::Insert => Some(0x2D),
        Key::Home => Some(0x24),
        Key::End => Some(0x23),
        Key::PageUp => Some(0x21),
        Key::PageDown => Some(0x22),
        Key::Backtick => Some(0xC0),
        _ => None,
    }
}

/// Sets or removes Windows Run registry key
fn set_run_at_startup(enable: bool) -> Result<(), ()> {
    use std::os::windows::ffi::OsStrExt;
    use windows::core::PCWSTR;
    use windows::Win32::System::Registry::{
        RegCloseKey, RegCreateKeyW, RegDeleteValueW, RegSetValueExW, HKEY_CURRENT_USER, REG_SZ,
    };

    let subkey: Vec<u16> =
        std::ffi::OsStr::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
            .encode_wide()
            .chain(Some(0))
            .collect();
    let app_name: Vec<u16> = std::ffi::OsStr::new("BatteryDock")
        .encode_wide()
        .chain(Some(0))
        .collect();

    unsafe {
        let mut hkey = windows::Win32::System::Registry::HKEY::default();
        if RegCreateKeyW(HKEY_CURRENT_USER, PCWSTR(subkey.as_ptr()), &mut hkey).is_ok() {
            if enable {
                if let Ok(exe_path) = std::env::current_exe() {
                    let path_str = format!("\"{}\"", exe_path.to_string_lossy());
                    let path_wide: Vec<u16> = std::ffi::OsStr::new(&path_str)
                        .encode_wide()
                        .chain(Some(0))
                        .collect();
                    let _ = RegSetValueExW(
                        hkey,
                        PCWSTR(app_name.as_ptr()),
                        0,
                        REG_SZ,
                        Some(std::slice::from_raw_parts(
                            path_wide.as_ptr() as *const u8,
                            path_wide.len() * 2,
                        )),
                    );
                }
            } else {
                let _ = RegDeleteValueW(hkey, PCWSTR(app_name.as_ptr()));
            }
            let _ = RegCloseKey(hkey);
        }
    }
    Ok(())
}

pub struct SettingsApp {
    config: AppConfig,
    engine: crate::battery::engine::BatteryEngine,
    devices: Vec<DeviceBattery>,
    history: crate::battery::history::BatteryHistoryStore,
    is_open: bool,
    last_config_reload: std::time::Instant,
    last_history_reload: std::time::Instant,
    active_tab: SettingsTab,
}

impl SettingsApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let config = AppConfig::load();
        let engine = crate::battery::engine::BatteryEngine::start(std::time::Duration::from_secs(
            config.poll_interval_secs,
        ));
        let devices = engine
            .receiver()
            .recv_timeout(std::time::Duration::from_millis(350))
            .unwrap_or_default();
        let history = crate::battery::history::BatteryHistoryStore::load();
        Self {
            config,
            engine,
            devices,
            history,
            is_open: true,
            last_config_reload: std::time::Instant::now(),
            last_history_reload: std::time::Instant::now(),
            active_tab: SettingsTab::Devices,
        }
    }

    fn reload_external_config_if_due(&mut self) {
        let path = AppConfig::config_path();
        sync_external_config_if_due(&mut self.config, &mut self.last_config_reload, &path);
        if self.last_history_reload.elapsed() >= std::time::Duration::from_secs(5) {
            self.last_history_reload = std::time::Instant::now();
            self.history = crate::battery::history::BatteryHistoryStore::load();
        }
    }
}

fn sync_external_config_if_due(
    config: &mut AppConfig,
    last_reload: &mut std::time::Instant,
    path: &std::path::Path,
) -> bool {
    if last_reload.elapsed() >= std::time::Duration::from_millis(250) {
        *last_reload = std::time::Instant::now();
        if let Some(loaded) = AppConfig::try_load_from_path(path) {
            if loaded != *config {
                *config = loaded;
                return true;
            }
        }
    }
    false
}

impl eframe::App for SettingsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.reload_external_config_if_due();

        while let Ok(updated) = self.engine.receiver().try_recv() {
            self.devices = updated;
        }

        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::from_rgb(18, 20, 24);
        visuals.window_fill = egui::Color32::from_rgb(18, 20, 24);
        ctx.set_visuals(visuals);

        let engine_ref = self.engine.clone();
        egui::CentralPanel::default()
            .frame(
                egui::Frame::NONE
                    .fill(egui::Color32::from_rgb(18, 20, 24))
                    .inner_margin(egui::Margin::same(16)),
            )
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    SettingsWindow::render_contents(
                        ui,
                        &mut self.active_tab,
                        &mut self.is_open,
                        &mut self.config,
                        &self.devices,
                        &self.history,
                        move || engine_ref.trigger_refresh(),
                    );
                });
            });

        ctx.request_repaint_after(std::time::Duration::from_millis(250));

        if !self.is_open || ctx.input(|i| i.viewport().close_requested()) {
            std::process::exit(0);
        }
    }
}

pub fn run_standalone_settings() -> eframe::Result<()> {
    use windows::core::w;
    use windows::Win32::Foundation::{GetLastError, ERROR_ALREADY_EXISTS};
    use windows::Win32::System::Threading::CreateMutexW;
    use windows::Win32::UI::WindowsAndMessaging::{
        FindWindowW, SetForegroundWindow, ShowWindow, SW_RESTORE,
    };

    unsafe {
        let _mutex = CreateMutexW(None, true, w!("Global\\BatteryDockSettingsMutex"));
        if GetLastError() == ERROR_ALREADY_EXISTS {
            if let Ok(hwnd) = FindWindowW(None, w!("BatteryDock Settings")) {
                if !hwnd.0.is_null() {
                    let _ = ShowWindow(hwnd, SW_RESTORE);
                    let _ = SetForegroundWindow(hwnd);
                }
            }
            return Ok(());
        }
    }

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("BatteryDock Settings")
            .with_inner_size([500.0, 580.0])
            .with_min_inner_size([440.0, 480.0])
            .with_decorations(true)
            .with_resizable(true)
            .with_transparent(false),
        ..Default::default()
    };

    eframe::run_native(
        "BatteryDock Settings",
        native_options,
        Box::new(|cc| Ok(Box::new(SettingsApp::new(cc)))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_reloads_external_reset_position_without_overwriting() {
        let dir = std::env::temp_dir().join(format!(
            "batterydock_settings_reload_test_{}",
            std::process::id()
        ));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("config.toml");

        // Initial Settings state: Custom dragged position
        let mut settings_cfg = AppConfig {
            position_mode: DockPositionMode::Custom,
            custom_pos_x: 640.0,
            custom_pos_y: 420.0,
            ..AppConfig::default()
        };
        settings_cfg.save_to_path(&path);

        // External dock/tray action clicks "Reset to Taskbar Clock" and writes to disk
        let external_cfg = AppConfig {
            position_mode: DockPositionMode::TaskbarLeftOfTray,
            custom_pos_x: 0.0,
            custom_pos_y: 0.0,
            ..settings_cfg.clone()
        };
        external_cfg.save_to_path(&path);

        // Before 250ms elapsed, sync does not run
        let mut recent = std::time::Instant::now();
        assert!(!sync_external_config_if_due(
            &mut settings_cfg,
            &mut recent,
            &path
        ));
        assert_eq!(settings_cfg.position_mode, DockPositionMode::Custom);

        // Once due (>=250ms), Settings reloads the external reset cleanly
        let mut due = std::time::Instant::now() - std::time::Duration::from_millis(300);
        assert!(sync_external_config_if_due(
            &mut settings_cfg,
            &mut due,
            &path
        ));
        assert_eq!(
            settings_cfg.position_mode,
            DockPositionMode::TaskbarLeftOfTray
        );
        assert_eq!(settings_cfg.custom_pos_x, 0.0);
        assert_eq!(settings_cfg.custom_pos_y, 0.0);

        let _ = std::fs::remove_dir_all(&dir);
    }
}
