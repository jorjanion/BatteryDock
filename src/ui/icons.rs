use crate::battery::DeviceType;
use egui::{Color32, Pos2, Rect, Stroke, StrokeKind, Vec2};

pub struct DevicePainter;

impl DevicePainter {
    /// Draws a crisp, vector-rendered device icon
    pub fn paint_device_icon(
        painter: &egui::Painter,
        rect: Rect,
        device_type: DeviceType,
        color: Color32,
    ) {
        let center = rect.center();
        let size = rect.size();
        let stroke = Stroke::new(1.5_f32, color);

        match device_type {
            DeviceType::Headset => {
                // Headband arc
                let arc_radius = size.x * 0.35;
                let arc_center = Pos2::new(center.x, center.y - 1.0);
                let mut points = Vec::new();
                for i in 0..=12 {
                    let angle = std::f32::consts::PI + (std::f32::consts::PI * i as f32 / 12.0);
                    points.push(Pos2::new(
                        arc_center.x + arc_radius * angle.cos(),
                        arc_center.y + arc_radius * 0.7 * angle.sin(),
                    ));
                }
                painter.add(egui::Shape::line(points, stroke));

                // Left ear cup
                let cup_w = 3.5;
                let cup_h = 7.0;
                let left_cup = Rect::from_center_size(
                    Pos2::new(center.x - arc_radius, center.y + 1.0),
                    Vec2::new(cup_w, cup_h),
                );
                painter.rect_filled(left_cup, 2.0, color);

                // Right ear cup
                let right_cup = Rect::from_center_size(
                    Pos2::new(center.x + arc_radius, center.y + 1.0),
                    Vec2::new(cup_w, cup_h),
                );
                painter.rect_filled(right_cup, 2.0, color);
            }
            DeviceType::Mouse => {
                // Mouse body
                let mouse_rect = Rect::from_center_size(center, Vec2::new(10.0, 15.0));
                painter.rect_stroke(mouse_rect, 5.0, stroke, StrokeKind::Inside);

                // Mouse wheel / divider line
                painter.line_segment(
                    [
                        Pos2::new(center.x, center.y - 6.0),
                        Pos2::new(center.x, center.y - 1.0),
                    ],
                    Stroke::new(1.2_f32, color),
                );
            }
            DeviceType::Keyboard => {
                let kb_rect = Rect::from_center_size(center, Vec2::new(17.0, 11.0));
                painter.rect_stroke(kb_rect, 2.5, stroke, StrokeKind::Inside);

                // Key dots
                for dx in [-4.0, 0.0, 4.0] {
                    for dy in [-2.0, 2.0] {
                        painter.circle_filled(Pos2::new(center.x + dx, center.y + dy), 0.8, color);
                    }
                }
            }
            DeviceType::Gamepad => {
                // Gamepad body
                let pad_rect = Rect::from_center_size(center, Vec2::new(16.0, 11.0));
                painter.rect_stroke(pad_rect, 4.0, stroke, StrokeKind::Inside);

                // D-pad cross on left
                painter.circle_filled(Pos2::new(center.x - 4.5, center.y), 1.6, color);

                // Action buttons on right
                painter.circle_filled(Pos2::new(center.x + 4.5, center.y), 1.4, color);
            }
            DeviceType::VRHeadset => {
                // Visor shape
                let vr_rect = Rect::from_center_size(center, Vec2::new(16.0, 9.0));
                painter.rect_filled(vr_rect, 4.0, color);

                // Lens bridge cutout
                let bridge = Rect::from_center_size(
                    Pos2::new(center.x, center.y + 3.0),
                    Vec2::new(4.0, 3.0),
                );
                painter.rect_filled(bridge, 1.0, Color32::from_black_alpha(200));
            }
            DeviceType::Wheel => {
                // Steering wheel circle
                painter.circle_stroke(center, 7.0, stroke);
                // Center hub
                painter.circle_filled(center, 2.0, color);
                // Horizontal spokes
                painter.line_segment(
                    [
                        Pos2::new(center.x - 7.0, center.y),
                        Pos2::new(center.x + 7.0, center.y),
                    ],
                    stroke,
                );
            }
            DeviceType::Camera => {
                let body = Rect::from_center_size(center, Vec2::new(15.0, 10.0));
                painter.rect_stroke(body, 2.0, stroke, StrokeKind::Inside);
                painter.circle_stroke(center, 3.0, stroke);
            }
            DeviceType::Unknown => {
                let rect = Rect::from_center_size(center, Vec2::new(12.0, 12.0));
                painter.rect_stroke(rect, 3.0, stroke, StrokeKind::Inside);
                painter.circle_filled(center, 1.5, color);
            }
        }
    }

    /// Draws a modern, rounded battery bar with percentage fill and charging icon
    pub fn paint_battery_meter(
        painter: &egui::Painter,
        rect: Rect,
        percentage: u8,
        is_charging: bool,
        low_threshold: u8,
        crit_threshold: u8,
        pulse_alpha: f32,
    ) {
        let pct = percentage.min(100);

        // Determine battery fill color
        let fill_color = if is_charging {
            Color32::from_rgb(56, 189, 248) // Electric Sky Blue
        } else if pct <= crit_threshold {
            let alpha = (200.0 + 55.0 * pulse_alpha) as u8;
            Color32::from_rgba_premultiplied(239, 68, 68, alpha) // Red pulse
        } else if pct <= low_threshold {
            Color32::from_rgb(245, 158, 11) // Amber
        } else if pct > 50 {
            Color32::from_rgb(34, 197, 94) // Mint Emerald
        } else {
            Color32::from_rgb(132, 204, 22) // Lime
        };

        // Battery outer shell
        let shell_rect =
            Rect::from_min_size(rect.min, Vec2::new(rect.width() - 2.5, rect.height()));
        let border_stroke = Stroke::new(1.0_f32, Color32::from_white_alpha(70));
        let bg_fill = Color32::from_black_alpha(100);
        painter.rect_filled(shell_rect, 2.5, bg_fill);
        painter.rect_stroke(shell_rect, 2.5, border_stroke, StrokeKind::Inside);

        // Battery terminal tip (+ side)
        let tip_rect = Rect::from_min_size(
            Pos2::new(shell_rect.max.x + 0.5, shell_rect.center().y - 2.5),
            Vec2::new(1.5, 5.0),
        );
        painter.rect_filled(tip_rect, 0.8, Color32::from_white_alpha(90));

        // Inner battery level fill
        let padding = 1.5;
        let max_fill_w = (shell_rect.width() - padding * 2.0).max(0.0);
        let fill_w = max_fill_w * (pct as f32 / 100.0);

        if fill_w > 0.5 {
            let fill_rect = Rect::from_min_size(
                Pos2::new(shell_rect.min.x + padding, shell_rect.min.y + padding),
                Vec2::new(fill_w, shell_rect.height() - padding * 2.0),
            );
            painter.rect_filled(fill_rect, 1.5, fill_color);
        }

        // Crisp vector lightning bolt icon inside meter if charging
        if is_charging {
            let center = shell_rect.center();
            let bolt_color = Color32::WHITE;
            let outline = Color32::from_black_alpha(150);
            Self::draw_bolt_triangles(painter, center, 2.6, 4.4, outline);
            Self::draw_bolt_triangles(painter, center, 2.0, 3.8, bolt_color);
        }
    }

    fn draw_bolt_triangles(
        painter: &egui::Painter,
        center: Pos2,
        half_w: f32,
        half_h: f32,
        color: Color32,
    ) {
        let top_tri = vec![
            Pos2::new(center.x + half_w * 0.30, center.y - half_h),
            Pos2::new(center.x - half_w, center.y + half_h * 0.14),
            Pos2::new(center.x + half_w * 0.14, center.y + half_h * 0.14),
        ];
        let bot_tri = vec![
            Pos2::new(center.x + half_w, center.y - half_h * 0.14),
            Pos2::new(center.x - half_w * 0.14, center.y - half_h * 0.14),
            Pos2::new(center.x - half_w * 0.30, center.y + half_h),
        ];
        painter.add(egui::Shape::convex_polygon(top_tri, color, Stroke::NONE));
        painter.add(egui::Shape::convex_polygon(bot_tri, color, Stroke::NONE));
    }

    /// Draws an unmistakable external charging lightning badge next to a charging peripheral
    pub fn paint_charging_icon(painter: &egui::Painter, rect: Rect, pulse_alpha: f32) {
        let center = rect.center();
        let half_w = (rect.width() * 0.42).max(2.5);
        let half_h = (rect.height() * 0.46).max(4.0);
        let gold = Color32::from_rgb(250, (196.0 + 20.0 * pulse_alpha) as u8, 21);
        Self::draw_bolt_triangles(
            painter,
            center,
            half_w + 0.7,
            half_h + 0.7,
            Color32::from_black_alpha(180),
        );
        Self::draw_bolt_triangles(painter, center, half_w, half_h, gold);
    }

    /// Draws a clean vector settings gear button icon
    pub fn paint_settings_gear(painter: &egui::Painter, rect: Rect, color: Color32) {
        let center = rect.center();
        let outer_r = (rect.width().min(rect.height()) * 0.42).max(3.5);
        let inner_r = outer_r * 0.62;
        let hole_r = outer_r * 0.28;

        for i in 0..6 {
            let angle = (i as f32) * std::f32::consts::FRAC_PI_3;
            let dx = angle.cos();
            let dy = angle.sin();
            painter.line_segment(
                [
                    Pos2::new(
                        center.x + dx * inner_r * 0.75,
                        center.y + dy * inner_r * 0.75,
                    ),
                    Pos2::new(center.x + dx * outer_r, center.y + dy * outer_r),
                ],
                Stroke::new(1.5_f32, color),
            );
        }
        painter.circle_stroke(center, inner_r, Stroke::new(1.4_f32, color));
        painter.circle_filled(
            center,
            hole_r,
            Color32::from_rgba_premultiplied(18, 20, 24, 215),
        );
    }
}
