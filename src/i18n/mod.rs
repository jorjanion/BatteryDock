mod europe;
mod slavic_asia;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    Spanish,
    Arabic,
    Czech,
    German,
    French,
    Hindi,
    Indonesian,
    Italian,
    Japanese,
    Korean,
    Dutch,
    Polish,
    PortugueseBR,
    Russian,
    Swedish,
    Turkish,
    Ukrainian,
    Vietnamese,
    ChineseSimplified,
    ChineseTraditional,
}

impl Default for Language {
    fn default() -> Self {
        Self::detect_system_language()
    }
}

pub(crate) struct Strings {
    pub display_name: &'static str,
    pub no_devices_connected: &'static str,
    pub empty_dock_text_width: f32,
    pub header_subtitle: &'static str,
    pub heading_alert_thresholds: &'static str,
    pub low_battery_warning: &'static str,
    pub critical_battery_warning: &'static str,
    pub heading_notifications: &'static str,
    pub play_chime_checkbox: &'static str,
    pub test_chime_button: &'static str,
    pub desktop_notifications_checkbox: &'static str,
    pub full_charge_alert_checkbox: &'static str,
    pub toast_full_charge_prefix: &'static str,
    pub toast_full_charge_body: &'static str,
    pub heading_dock_placement: &'static str,
    pub placement_tray_clock: &'static str,
    pub placement_far_right: &'static str,
    pub placement_left_side: &'static str,
    pub placement_custom: &'static str,
    pub placement_all_monitors_checkbox: &'static str,
    pub reset_position_button: &'static str,
    pub connected_devices_prefix: &'static str,
    pub refresh_now_button: &'static str,
    pub no_devices_detected_label: &'static str,
    pub no_devices_connect_hint: &'static str,
    pub charging_suffix: &'static str,
    pub show_percentage_checkbox: &'static str,
    pub start_with_windows_checkbox: &'static str,
    pub heading_updates: &'static str,
    pub auto_update_checkbox: &'static str,
    pub check_updates_now_button: &'static str,
    pub checking_github_label: &'static str,
    pub up_to_date_label: &'static str,
    pub new_version_prefix: &'static str,
    pub new_version_suffix: &'static str,
    pub install_update_button: &'static str,
    pub downloading_prefix: &'static str,
    pub installing_prefix: &'static str,
    pub update_failed_prefix: &'static str,
    pub heading_language: &'static str,
    pub system_language_detected_prefix: &'static str,
    pub system_language_unsupported_suffix: &'static str,
    pub use_system_language_button: &'static str,
    pub close_settings_button: &'static str,
    pub context_menu_settings: &'static str,
    pub context_menu_refresh: &'static str,
    pub context_menu_reset: &'static str,
    pub context_menu_restart: &'static str,
    pub context_menu_exit: &'static str,
    pub dt_headset: &'static str,
    pub dt_mouse: &'static str,
    pub dt_keyboard: &'static str,
    pub dt_gamepad: &'static str,
    pub dt_vr: &'static str,
    pub dt_wheel: &'static str,
    pub dt_camera: &'static str,
    pub dt_unknown: &'static str,
    pub status_charging: &'static str,
    pub status_asleep: &'static str,
    pub status_active: &'static str,
    pub label_protocol: &'static str,
    pub label_status: &'static str,
    pub label_full_charge_in: &'static str,
    pub label_est_autonomy: &'static str,
    pub time_fully_charged: &'static str,
    pub time_to_full_suffix: &'static str,
    pub time_remaining_suffix: &'static str,
    pub heading_per_device_settings: &'static str,
    pub nickname_label: &'static str,
    pub label_custom_low_alert: &'static str,
    pub label_mute_alerts: &'static str,
    pub label_hide_from_dock: &'static str,
    pub label_use_default_threshold: &'static str,
    pub tab_devices: &'static str,
    pub tab_alerts: &'static str,
    pub tab_customization: &'static str,
    pub compact_mode_checkbox: &'static str,
    pub status_nominal_prefix: &'static str,
    pub status_nominal_suffix: &'static str,
    pub heading_theme: &'static str,
    pub heading_dock_appearance: &'static str,
    pub theme_system: &'static str,
    pub theme_liquid_glass: &'static str,
    pub theme_dark: &'static str,
    pub theme_light: &'static str,
    pub heading_game_overlay: &'static str,
    pub game_overlay_enable_checkbox: &'static str,
    pub game_overlay_hint: &'static str,
    pub game_overlay_position_label: &'static str,
    pub overlay_corner_top_left: &'static str,
    pub overlay_corner_top_right: &'static str,
    pub overlay_corner_bottom_left: &'static str,
    pub overlay_corner_bottom_right: &'static str,
    pub overlay_corner_custom: &'static str,
    pub overlay_margin_label: &'static str,
    pub overlay_custom_x_label: &'static str,
    pub overlay_custom_y_label: &'static str,
    pub game_overlay_click_through_checkbox: &'static str,
    pub game_overlay_always_on_checkbox: &'static str,
    pub move_device_up_tooltip: &'static str,
    pub move_device_down_tooltip: &'static str,
    pub battery_history_label: &'static str,
    pub battery_history_collecting: &'static str,
    pub copy_diagnostics_button: &'static str,
    pub diagnostics_copied_label: &'static str,
    pub game_overlay_hotkey_checkbox: &'static str,
    pub game_overlay_hotkey_label: &'static str,
    pub game_overlay_hotkey_recording: &'static str,
    pub game_overlay_hotkey_hint: &'static str,
    pub game_overlay_hotkey_reset: &'static str,
}

impl Language {
    pub const ALL: &'static [Language] = &[
        Language::English,
        Language::Spanish,
        Language::Arabic,
        Language::Czech,
        Language::German,
        Language::French,
        Language::Hindi,
        Language::Indonesian,
        Language::Italian,
        Language::Japanese,
        Language::Korean,
        Language::Dutch,
        Language::Polish,
        Language::PortugueseBR,
        Language::Russian,
        Language::Swedish,
        Language::Turkish,
        Language::Ukrainian,
        Language::Vietnamese,
        Language::ChineseSimplified,
        Language::ChineseTraditional,
    ];

    /// Maps a Windows `LANGID` (from `GetUserDefaultUILanguage()`) to a supported `Language`,
    /// or `None` if Windows is set to a language we don't yet translate.
    pub fn from_windows_lang_id(lang_id: u16) -> Option<Self> {
        let primary = lang_id & 0x03FF;
        let sublang = (lang_id >> 10) & 0x3F;
        match primary {
            0x01 => Some(Language::Arabic),
            0x04 => match sublang {
                0x01 | 0x03 | 0x05 => Some(Language::ChineseTraditional),
                _ => Some(Language::ChineseSimplified),
            },
            0x05 => Some(Language::Czech),
            0x07 => Some(Language::German),
            0x09 => Some(Language::English),
            0x0A => Some(Language::Spanish),
            0x0C => Some(Language::French),
            0x10 => Some(Language::Italian),
            0x11 => Some(Language::Japanese),
            0x12 => Some(Language::Korean),
            0x13 => Some(Language::Dutch),
            0x15 => Some(Language::Polish),
            0x16 => Some(Language::PortugueseBR),
            0x19 => Some(Language::Russian),
            0x1D => Some(Language::Swedish),
            0x1F => Some(Language::Turkish),
            0x21 => Some(Language::Indonesian),
            0x22 => Some(Language::Ukrainian),
            0x2A => Some(Language::Vietnamese),
            0x39 => Some(Language::Hindi),
            _ => None,
        }
    }

    /// Queries Windows UI language; returns `(Some(lang), lang_id)` if supported or `(None, lang_id)` otherwise.
    pub fn detect_system_language_status() -> (Option<Self>, u16) {
        #[cfg(windows)]
        {
            extern "system" {
                fn GetUserDefaultUILanguage() -> u16;
            }
            let lang_id = unsafe { GetUserDefaultUILanguage() };
            (Self::from_windows_lang_id(lang_id), lang_id)
        }
        #[cfg(not(windows))]
        {
            (Some(Language::English), 0x0409)
        }
    }

    /// Detects Windows system UI language and selects it automatically if supported; otherwise defaults to English.
    pub fn detect_system_language() -> Self {
        Self::detect_system_language_status()
            .0
            .unwrap_or(Language::English)
    }

    fn strings(&self) -> &'static Strings {
        match self {
            Language::English => &europe::EN,
            Language::Spanish => &europe::ES,
            Language::Czech => &europe::CS,
            Language::German => &europe::DE,
            Language::French => &europe::FR,
            Language::Italian => &europe::IT,
            Language::Dutch => &europe::NL,
            Language::Polish => &europe::PL,
            Language::PortugueseBR => &europe::PT_BR,
            Language::Swedish => &europe::SV,
            Language::Turkish => &europe::TR,
            Language::Russian => &slavic_asia::RU,
            Language::Ukrainian => &slavic_asia::UK,
            Language::Arabic => &slavic_asia::AR,
            Language::Hindi => &slavic_asia::HI,
            Language::Indonesian => &slavic_asia::ID,
            Language::Vietnamese => &slavic_asia::VI,
            Language::Japanese => &slavic_asia::JA,
            Language::Korean => &slavic_asia::KO,
            Language::ChineseSimplified => &slavic_asia::ZH_CN,
            Language::ChineseTraditional => &slavic_asia::ZH_TW,
        }
    }

    pub fn display_name(&self) -> &'static str {
        self.strings().display_name
    }

    pub fn no_devices_connected(&self) -> &'static str {
        self.strings().no_devices_connected
    }

    pub fn empty_dock_text_width(&self) -> f32 {
        self.strings().empty_dock_text_width
    }

    pub fn header_subtitle(&self) -> &'static str {
        self.strings().header_subtitle
    }

    pub fn heading_alert_thresholds(&self) -> &'static str {
        self.strings().heading_alert_thresholds
    }

    pub fn low_battery_warning(&self) -> &'static str {
        self.strings().low_battery_warning
    }

    pub fn critical_battery_warning(&self) -> &'static str {
        self.strings().critical_battery_warning
    }

    pub fn heading_notifications(&self) -> &'static str {
        self.strings().heading_notifications
    }

    pub fn play_chime_checkbox(&self) -> &'static str {
        self.strings().play_chime_checkbox
    }

    pub fn test_chime_button(&self) -> &'static str {
        self.strings().test_chime_button
    }

    pub fn desktop_notifications_checkbox(&self) -> &'static str {
        self.strings().desktop_notifications_checkbox
    }

    pub fn full_charge_alert_checkbox(&self) -> &'static str {
        self.strings().full_charge_alert_checkbox
    }

    pub fn toast_full_charge_title(&self, name: &str) -> String {
        format!("{}: {name}", self.strings().toast_full_charge_prefix)
    }

    pub fn toast_full_charge_body(&self) -> &'static str {
        self.strings().toast_full_charge_body
    }

    pub fn heading_dock_placement(&self) -> &'static str {
        self.strings().heading_dock_placement
    }

    pub fn placement_tray_clock(&self) -> &'static str {
        self.strings().placement_tray_clock
    }

    pub fn placement_far_right(&self) -> &'static str {
        self.strings().placement_far_right
    }

    pub fn placement_left_side(&self) -> &'static str {
        self.strings().placement_left_side
    }

    pub fn placement_custom(&self) -> &'static str {
        self.strings().placement_custom
    }

    pub fn placement_all_monitors_checkbox(&self) -> &'static str {
        self.strings().placement_all_monitors_checkbox
    }

    pub fn reset_position_button(&self) -> &'static str {
        self.strings().reset_position_button
    }

    #[allow(dead_code)]
    pub fn heading_connected_devices(&self, count: usize) -> String {
        format!("{} ({count})", self.strings().connected_devices_prefix)
    }

    pub fn refresh_now_button(&self) -> &'static str {
        self.strings().refresh_now_button
    }

    pub fn no_devices_detected_label(&self) -> &'static str {
        self.strings().no_devices_detected_label
    }

    pub fn no_devices_connect_hint(&self) -> &'static str {
        self.strings().no_devices_connect_hint
    }

    pub fn charging_suffix(&self) -> &'static str {
        self.strings().charging_suffix
    }

    pub fn show_percentage_checkbox(&self) -> &'static str {
        self.strings().show_percentage_checkbox
    }

    pub fn start_with_windows_checkbox(&self) -> &'static str {
        self.strings().start_with_windows_checkbox
    }

    pub fn heading_updates(&self) -> &'static str {
        self.strings().heading_updates
    }

    pub fn auto_update_checkbox(&self) -> &'static str {
        self.strings().auto_update_checkbox
    }

    pub fn check_updates_now_button(&self) -> &'static str {
        self.strings().check_updates_now_button
    }

    pub fn checking_github_label(&self) -> &'static str {
        self.strings().checking_github_label
    }

    pub fn up_to_date_label(&self) -> &'static str {
        self.strings().up_to_date_label
    }

    pub fn new_version_available_label(&self, version: &str) -> String {
        let s = self.strings();
        format!("{}{version}{}", s.new_version_prefix, s.new_version_suffix)
    }

    pub fn install_update_button(&self) -> &'static str {
        self.strings().install_update_button
    }

    pub fn downloading_label(&self, version: &str) -> String {
        format!("{}{version}...", self.strings().downloading_prefix)
    }

    pub fn installing_label(&self, version: &str) -> String {
        format!("{}{version}...", self.strings().installing_prefix)
    }

    pub fn update_failed_label(&self, err: &str) -> String {
        format!("{}: {err}", self.strings().update_failed_prefix)
    }

    pub fn heading_language(&self) -> &'static str {
        self.strings().heading_language
    }

    pub fn detected_system_language_label(&self, detected: Option<Language>) -> String {
        let s = self.strings();
        match detected {
            Some(lang) => format!("{}: {}", s.system_language_detected_prefix, lang.display_name()),
            None => format!(
                "{}: {}",
                s.system_language_detected_prefix, s.system_language_unsupported_suffix
            ),
        }
    }

    pub fn use_system_language_button(&self) -> &'static str {
        self.strings().use_system_language_button
    }

    pub fn close_settings_button(&self) -> &'static str {
        self.strings().close_settings_button
    }

    pub fn context_menu_settings(&self) -> &'static str {
        self.strings().context_menu_settings
    }

    pub fn context_menu_refresh(&self) -> &'static str {
        self.strings().context_menu_refresh
    }

    pub fn context_menu_reset(&self) -> &'static str {
        self.strings().context_menu_reset
    }

    pub fn context_menu_restart(&self) -> &'static str {
        self.strings().context_menu_restart
    }

    pub fn context_menu_exit(&self) -> &'static str {
        self.strings().context_menu_exit
    }

    #[allow(dead_code)]
    pub fn device_type_name(&self, dt: &crate::battery::DeviceType) -> &'static str {
        let s = self.strings();
        match dt {
            crate::battery::DeviceType::Headset => s.dt_headset,
            crate::battery::DeviceType::Mouse => s.dt_mouse,
            crate::battery::DeviceType::Keyboard => s.dt_keyboard,
            crate::battery::DeviceType::Gamepad => s.dt_gamepad,
            crate::battery::DeviceType::VRHeadset => s.dt_vr,
            crate::battery::DeviceType::Wheel => s.dt_wheel,
            crate::battery::DeviceType::Camera => s.dt_camera,
            crate::battery::DeviceType::Unknown => s.dt_unknown,
        }
    }

    pub fn status_charging(&self) -> &'static str {
        self.strings().status_charging
    }

    pub fn status_asleep(&self) -> &'static str {
        self.strings().status_asleep
    }

    pub fn status_active(&self) -> &'static str {
        self.strings().status_active
    }

    #[allow(dead_code)]
    pub fn label_protocol(&self) -> &'static str {
        self.strings().label_protocol
    }

    #[allow(dead_code)]
    pub fn label_status(&self) -> &'static str {
        self.strings().label_status
    }

    #[allow(dead_code)]
    pub fn label_estimated_time(&self, is_charging: bool) -> &'static str {
        let s = self.strings();
        if is_charging {
            s.label_full_charge_in
        } else {
            s.label_est_autonomy
        }
    }

    pub fn format_time_left(&self, minutes: u32, is_charging: bool) -> String {
        let s = self.strings();
        if is_charging {
            if minutes == 0 {
                s.time_fully_charged.to_string()
            } else if minutes < 60 {
                format!("~{minutes}m {}", s.time_to_full_suffix)
            } else {
                format!("~{}h {}m {}", minutes / 60, minutes % 60, s.time_to_full_suffix)
            }
        } else if minutes < 60 {
            format!("~{minutes}m {}", s.time_remaining_suffix)
        } else {
            format!(
                "~{}h {}m {}",
                minutes / 60,
                minutes % 60,
                s.time_remaining_suffix
            )
        }
    }

    pub fn heading_per_device_settings(&self) -> &'static str {
        self.strings().heading_per_device_settings
    }

    pub fn nickname_label(&self) -> &'static str {
        self.strings().nickname_label
    }

    pub fn label_custom_low_alert(&self) -> &'static str {
        self.strings().label_custom_low_alert
    }

    pub fn label_mute_alerts(&self) -> &'static str {
        self.strings().label_mute_alerts
    }

    pub fn label_hide_from_dock(&self) -> &'static str {
        self.strings().label_hide_from_dock
    }

    pub fn label_use_default_threshold(&self) -> &'static str {
        self.strings().label_use_default_threshold
    }

    pub fn tab_devices(&self) -> &'static str {
        self.strings().tab_devices
    }

    pub fn tab_alerts(&self) -> &'static str {
        self.strings().tab_alerts
    }

    pub fn tab_customization(&self) -> &'static str {
        self.strings().tab_customization
    }

    #[allow(dead_code)]
    pub fn compact_mode_checkbox(&self) -> &'static str {
        self.strings().compact_mode_checkbox
    }

    pub fn status_nominal(&self, count: usize) -> String {
        let s = self.strings();
        format!("{} • {count} {}", s.status_nominal_prefix, s.status_nominal_suffix)
    }

    pub fn heading_theme(&self) -> &'static str {
        self.strings().heading_theme
    }

    pub fn heading_dock_appearance(&self) -> &'static str {
        self.strings().heading_dock_appearance
    }

    pub fn theme_system(&self) -> &'static str {
        self.strings().theme_system
    }

    pub fn theme_liquid_glass(&self) -> &'static str {
        self.strings().theme_liquid_glass
    }

    pub fn theme_dark(&self) -> &'static str {
        self.strings().theme_dark
    }

    pub fn theme_light(&self) -> &'static str {
        self.strings().theme_light
    }

    pub fn heading_game_overlay(&self) -> &'static str {
        self.strings().heading_game_overlay
    }

    pub fn game_overlay_enable_checkbox(&self) -> &'static str {
        self.strings().game_overlay_enable_checkbox
    }

    pub fn game_overlay_hint(&self) -> &'static str {
        self.strings().game_overlay_hint
    }

    pub fn game_overlay_position_label(&self) -> &'static str {
        self.strings().game_overlay_position_label
    }

    pub fn overlay_corner_top_left(&self) -> &'static str {
        self.strings().overlay_corner_top_left
    }

    pub fn overlay_corner_top_right(&self) -> &'static str {
        self.strings().overlay_corner_top_right
    }

    pub fn overlay_corner_bottom_left(&self) -> &'static str {
        self.strings().overlay_corner_bottom_left
    }

    pub fn overlay_corner_bottom_right(&self) -> &'static str {
        self.strings().overlay_corner_bottom_right
    }

    pub fn overlay_corner_custom(&self) -> &'static str {
        self.strings().overlay_corner_custom
    }

    pub fn overlay_margin_label(&self) -> &'static str {
        self.strings().overlay_margin_label
    }

    pub fn overlay_custom_x_label(&self) -> &'static str {
        self.strings().overlay_custom_x_label
    }

    pub fn overlay_custom_y_label(&self) -> &'static str {
        self.strings().overlay_custom_y_label
    }

    pub fn game_overlay_click_through_checkbox(&self) -> &'static str {
        self.strings().game_overlay_click_through_checkbox
    }

    pub fn game_overlay_always_on_checkbox(&self) -> &'static str {
        self.strings().game_overlay_always_on_checkbox
    }

    pub fn move_device_up_tooltip(&self) -> &'static str {
        self.strings().move_device_up_tooltip
    }

    pub fn move_device_down_tooltip(&self) -> &'static str {
        self.strings().move_device_down_tooltip
    }

    pub fn battery_history_label(&self) -> &'static str {
        self.strings().battery_history_label
    }

    pub fn battery_history_collecting(&self) -> &'static str {
        self.strings().battery_history_collecting
    }

    pub fn copy_diagnostics_button(&self) -> &'static str {
        self.strings().copy_diagnostics_button
    }

    pub fn diagnostics_copied_label(&self) -> &'static str {
        self.strings().diagnostics_copied_label
    }

    pub fn game_overlay_hotkey_checkbox(&self) -> &'static str {
        self.strings().game_overlay_hotkey_checkbox
    }

    pub fn game_overlay_hotkey_label(&self) -> &'static str {
        self.strings().game_overlay_hotkey_label
    }

    pub fn game_overlay_hotkey_recording(&self) -> &'static str {
        self.strings().game_overlay_hotkey_recording
    }

    pub fn game_overlay_hotkey_hint(&self) -> &'static str {
        self.strings().game_overlay_hotkey_hint
    }

    pub fn game_overlay_hotkey_reset(&self) -> &'static str {
        self.strings().game_overlay_hotkey_reset
    }
}

/// Loads Windows native system fallback fonts into `egui` so every script
/// (Latin, Cyrillic, Arabic, Devanagari/Hindi, Japanese, Korean, Chinese) renders crisply
/// with zero missing-glyph tofu boxes.
pub fn configure_egui_fonts(ctx: &egui::Context, lang: Language, load_all_scripts: bool) {
    let mut fonts = egui::FontDefinitions::default();

    let mut try_add_font = |name: &str, candidates: &[&str]| {
        for path in candidates {
            if let Ok(bytes) = std::fs::read(path) {
                fonts
                    .font_data
                    .insert(name.to_owned(), std::sync::Arc::new(egui::FontData::from_owned(bytes)));
                if let Some(fam) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
                    fam.push(name.to_owned());
                }
                break;
            }
        }
    };

    // Segoe UI covers Latin Extended, Cyrillic (RU/UK), Turkish, Polish, Czech, Vietnamese, Arabic
    try_add_font("win_segoeui", &[r"C:\Windows\Fonts\segoeui.ttf"]);

    if load_all_scripts || lang == Language::Hindi {
        try_add_font(
            "win_nirmala",
            &[r"C:\Windows\Fonts\Nirmala.ttc", r"C:\Windows\Fonts\Nirmala.ttf"],
        );
    }
    if load_all_scripts || lang == Language::Japanese {
        try_add_font(
            "win_jp",
            &[r"C:\Windows\Fonts\YuGothR.ttc", r"C:\Windows\Fonts\msgothic.ttc"],
        );
    }
    if load_all_scripts || lang == Language::Korean {
        try_add_font("win_kr", &[r"C:\Windows\Fonts\malgun.ttf"]);
    }
    if load_all_scripts || lang == Language::ChineseSimplified {
        try_add_font("win_zh_cn", &[r"C:\Windows\Fonts\msyh.ttc"]);
    }
    if load_all_scripts || lang == Language::ChineseTraditional {
        try_add_font("win_zh_tw", &[r"C:\Windows\Fonts\msjh.ttc"]);
    }

    ctx.set_fonts(fonts);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_translations() {
        let en = Language::English;
        let es = Language::Spanish;

        assert_eq!(en.display_name(), "English");
        assert_eq!(es.display_name(), "Español");

        assert_eq!(en.no_devices_connected(), "No devices connected");
        assert_eq!(es.no_devices_connected(), "Sin dispositivos conectados");

        assert!(en.empty_dock_text_width() > 100.0);
        assert!(es.empty_dock_text_width() > en.empty_dock_text_width());

        assert_eq!(en.heading_connected_devices(2), "Connected Devices (2)");
        assert_eq!(
            es.heading_connected_devices(2),
            "Dispositivos conectados (2)"
        );

        // Verify all 21 languages have valid non-empty strings and positive dock widths
        assert_eq!(Language::ALL.len(), 21);
        for &lang in Language::ALL {
            assert!(!lang.display_name().is_empty());
            assert!(!lang.no_devices_connected().is_empty());
            assert!(lang.empty_dock_text_width() >= 90.0);
            assert!(!lang.tab_devices().is_empty());
            assert!(!lang.heading_game_overlay().is_empty());
        }
    }

    #[test]
    fn test_windows_lang_id_detection_and_english_fallback() {
        // Russian (ru-RU = 0x0419)
        assert_eq!(Language::from_windows_lang_id(0x0419), Some(Language::Russian));
        // Spanish Spain (es-ES = 0x0C0A) and Mexico (es-MX = 0x080A)
        assert_eq!(Language::from_windows_lang_id(0x0C0A), Some(Language::Spanish));
        assert_eq!(Language::from_windows_lang_id(0x080A), Some(Language::Spanish));
        // Chinese Simplified (zh-CN = 0x0804) vs Traditional (zh-TW = 0x0404, zh-HK = 0x0C04)
        assert_eq!(
            Language::from_windows_lang_id(0x0804),
            Some(Language::ChineseSimplified)
        );
        assert_eq!(
            Language::from_windows_lang_id(0x0404),
            Some(Language::ChineseTraditional)
        );
        assert_eq!(
            Language::from_windows_lang_id(0x0C04),
            Some(Language::ChineseTraditional)
        );
        // Japanese, Korean, Arabic, Hindi, Ukrainian, German, French, Portuguese-BR
        assert_eq!(Language::from_windows_lang_id(0x0411), Some(Language::Japanese));
        assert_eq!(Language::from_windows_lang_id(0x0412), Some(Language::Korean));
        assert_eq!(Language::from_windows_lang_id(0x0401), Some(Language::Arabic));
        assert_eq!(Language::from_windows_lang_id(0x0439), Some(Language::Hindi));
        assert_eq!(Language::from_windows_lang_id(0x0422), Some(Language::Ukrainian));
        assert_eq!(Language::from_windows_lang_id(0x0407), Some(Language::German));
        assert_eq!(Language::from_windows_lang_id(0x040C), Some(Language::French));
        assert_eq!(
            Language::from_windows_lang_id(0x0416),
            Some(Language::PortugueseBR)
        );
        // Unsupported language (e.g., Greek 0x0408, Finnish 0x040B) -> None (defaults to English)
        assert_eq!(Language::from_windows_lang_id(0x0408), None);
        assert_eq!(Language::from_windows_lang_id(0x040B), None);
    }
}
