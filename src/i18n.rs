use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    Spanish,
}

impl Default for Language {
    fn default() -> Self {
        Self::detect_system_language()
    }
}

impl Language {
    /// Detects Windows system default UI language.
    /// If primary language ID is 0x0A (Spanish), defaults to Spanish, otherwise English.
    pub fn detect_system_language() -> Self {
        #[cfg(windows)]
        {
            extern "system" {
                fn GetUserDefaultUILanguage() -> u16;
            }
            let lang_id = unsafe { GetUserDefaultUILanguage() };
            let primary_lang = lang_id & 0x03FF;
            if primary_lang == 0x0A {
                return Language::Spanish;
            }
        }
        Language::English
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Spanish => "Español",
        }
    }

    pub fn no_devices_connected(&self) -> &'static str {
        match self {
            Language::English => "No devices connected",
            Language::Spanish => "Sin dispositivos conectados",
        }
    }

    pub fn empty_dock_text_width(&self) -> f32 {
        match self {
            Language::English => 120.0,
            Language::Spanish => 152.0,
        }
    }

    pub fn header_subtitle(&self) -> &'static str {
        match self {
            Language::English => "Ultra-lightweight peripheral battery overlay for Windows",
            Language::Spanish => "Indicador ultra ligero de batería para periféricos en Windows",
        }
    }

    pub fn heading_alert_thresholds(&self) -> &'static str {
        match self {
            Language::English => "⚡ Battery Alert Thresholds",
            Language::Spanish => "⚡ Umbrales de alerta de batería",
        }
    }

    pub fn low_battery_warning(&self) -> &'static str {
        match self {
            Language::English => "Low Battery Warning:",
            Language::Spanish => "Aviso de batería baja:",
        }
    }

    pub fn critical_battery_warning(&self) -> &'static str {
        match self {
            Language::English => "Critical Battery Warning:",
            Language::Spanish => "Aviso de batería crítica:",
        }
    }

    pub fn heading_notifications(&self) -> &'static str {
        match self {
            Language::English => "🔔 Notifications & Sound",
            Language::Spanish => "🔔 Notificaciones y sonido",
        }
    }

    pub fn play_chime_checkbox(&self) -> &'static str {
        match self {
            Language::English => "Play gentle chime on low battery",
            Language::Spanish => "Reproducir aviso sonoro con batería baja",
        }
    }

    pub fn test_chime_button(&self) -> &'static str {
        match self {
            Language::English => "▶ Test Chime Sound",
            Language::Spanish => "▶ Probar sonido",
        }
    }

    pub fn desktop_notifications_checkbox(&self) -> &'static str {
        match self {
            Language::English => "Show desktop notifications",
            Language::Spanish => "Mostrar notificaciones de escritorio",
        }
    }

    pub fn heading_dock_placement(&self) -> &'static str {
        match self {
            Language::English => "📌 Dock Placement",
            Language::Spanish => "📌 Posición de la pastilla",
        }
    }

    pub fn placement_tray_clock(&self) -> &'static str {
        match self {
            Language::English => "Taskbar: Next to System Tray (Clock)",
            Language::Spanish => "Barra de tareas: Junto al reloj",
        }
    }

    pub fn placement_far_right(&self) -> &'static str {
        match self {
            Language::English => "Taskbar: Far Right",
            Language::Spanish => "Barra de tareas: Extremo derecho",
        }
    }

    pub fn placement_left_side(&self) -> &'static str {
        match self {
            Language::English => "Taskbar: Left Side",
            Language::Spanish => "Barra de tareas: Lado izquierdo",
        }
    }

    pub fn placement_custom(&self) -> &'static str {
        match self {
            Language::English => "Custom (Click & drag the dock anywhere)",
            Language::Spanish => "Personalizada (haz clic y arrastra la pastilla)",
        }
    }

    pub fn reset_position_button(&self) -> &'static str {
        match self {
            Language::English => "📍 Reset Dock Position to Clock",
            Language::Spanish => "📍 Restablecer posición al reloj",
        }
    }

    pub fn heading_connected_devices(&self, count: usize) -> String {
        match self {
            Language::English => format!("🎧 Connected Devices ({count})"),
            Language::Spanish => format!("🎧 Dispositivos conectados ({count})"),
        }
    }

    pub fn refresh_now_button(&self) -> &'static str {
        match self {
            Language::English => "🔄 Refresh Now",
            Language::Spanish => "🔄 Actualizar ahora",
        }
    }

    pub fn no_devices_detected_label(&self) -> &'static str {
        match self {
            Language::English => "No active wireless peripherals detected.",
            Language::Spanish => "No se han detectado periféricos inalámbricos activos.",
        }
    }

    pub fn charging_suffix(&self) -> &'static str {
        match self {
            Language::English => " (Charging)",
            Language::Spanish => " (Cargando)",
        }
    }

    pub fn show_percentage_checkbox(&self) -> &'static str {
        match self {
            Language::English => "Show battery percentage text on dock",
            Language::Spanish => "Mostrar porcentaje de batería en la pastilla",
        }
    }

    pub fn start_with_windows_checkbox(&self) -> &'static str {
        match self {
            Language::English => "Start BatteryDock when Windows boots",
            Language::Spanish => "Iniciar BatteryDock con Windows",
        }
    }

    pub fn heading_updates(&self) -> &'static str {
        match self {
            Language::English => "🔄 Application Updates",
            Language::Spanish => "🔄 Actualizaciones de la aplicación",
        }
    }

    pub fn auto_update_checkbox(&self) -> &'static str {
        match self {
            Language::English => "Check for and install updates automatically",
            Language::Spanish => "Buscar e instalar actualizaciones automáticamente",
        }
    }

    pub fn check_updates_now_button(&self) -> &'static str {
        match self {
            Language::English => "🔍 Check for Updates Now",
            Language::Spanish => "🔍 Buscar actualizaciones ahora",
        }
    }

    pub fn checking_github_label(&self) -> &'static str {
        match self {
            Language::English => "Checking GitHub...",
            Language::Spanish => "Comprobando GitHub...",
        }
    }

    pub fn up_to_date_label(&self) -> &'static str {
        match self {
            Language::English => "BatteryDock is up to date!",
            Language::Spanish => "¡BatteryDock está actualizado!",
        }
    }

    pub fn new_version_available_label(&self, version: &str) -> String {
        match self {
            Language::English => format!("New version {version} available!"),
            Language::Spanish => format!("¡Nueva versión {version} disponible!"),
        }
    }

    pub fn install_update_button(&self) -> &'static str {
        match self {
            Language::English => "📥 Install Update",
            Language::Spanish => "📥 Instalar actualización",
        }
    }

    pub fn downloading_label(&self, version: &str) -> String {
        match self {
            Language::English => format!("Downloading {version}..."),
            Language::Spanish => format!("Descargando {version}..."),
        }
    }

    pub fn installing_label(&self, version: &str) -> String {
        match self {
            Language::English => format!("Installing {version}..."),
            Language::Spanish => format!("Instalando {version}..."),
        }
    }

    pub fn update_failed_label(&self, err: &str) -> String {
        match self {
            Language::English => format!("Update check failed: {err}"),
            Language::Spanish => format!("Error al buscar actualizaciones: {err}"),
        }
    }

    pub fn heading_language(&self) -> &'static str {
        match self {
            Language::English => "🌐 Interface Language",
            Language::Spanish => "🌐 Idioma de la interfaz",
        }
    }

    pub fn close_settings_button(&self) -> &'static str {
        match self {
            Language::English => "Close Settings",
            Language::Spanish => "Cerrar ajustes",
        }
    }

    pub fn context_menu_settings(&self) -> &'static str {
        match self {
            Language::English => "Settings",
            Language::Spanish => "Ajustes",
        }
    }

    pub fn context_menu_refresh(&self) -> &'static str {
        match self {
            Language::English => "Refresh Devices",
            Language::Spanish => "Actualizar dispositivos",
        }
    }

    pub fn context_menu_reset(&self) -> &'static str {
        match self {
            Language::English => "Reset to Taskbar Clock",
            Language::Spanish => "Restablecer posición al reloj",
        }
    }

    pub fn context_menu_restart(&self) -> &'static str {
        match self {
            Language::English => "Restart BatteryDock",
            Language::Spanish => "Reiniciar BatteryDock",
        }
    }

    pub fn context_menu_exit(&self) -> &'static str {
        match self {
            Language::English => "Exit BatteryDock",
            Language::Spanish => "Salir de BatteryDock",
        }
    }

    pub fn device_type_name(&self, dt: &crate::battery::DeviceType) -> &'static str {
        match self {
            Language::English => dt.display_name(),
            Language::Spanish => match dt {
                crate::battery::DeviceType::Headset => "Auriculares",
                crate::battery::DeviceType::Mouse => "Ratón",
                crate::battery::DeviceType::Keyboard => "Teclado",
                crate::battery::DeviceType::Gamepad => "Mando",
                crate::battery::DeviceType::VRHeadset => "Visor VR",
                crate::battery::DeviceType::Wheel => "Volante",
                crate::battery::DeviceType::Camera => "Cámara",
                crate::battery::DeviceType::Unknown => "Dispositivo",
            },
        }
    }

    pub fn status_charging(&self) -> &'static str {
        match self {
            Language::English => "Charging",
            Language::Spanish => "Cargando",
        }
    }

    pub fn status_asleep(&self) -> &'static str {
        match self {
            Language::English => "Sleep Mode",
            Language::Spanish => "En reposo",
        }
    }

    pub fn status_active(&self) -> &'static str {
        match self {
            Language::English => "Active",
            Language::Spanish => "En uso",
        }
    }

    pub fn label_protocol(&self) -> &'static str {
        match self {
            Language::English => "Protocol:",
            Language::Spanish => "Protocolo:",
        }
    }

    pub fn label_status(&self) -> &'static str {
        match self {
            Language::English => "Status:",
            Language::Spanish => "Estado:",
        }
    }

    pub fn label_estimated_time(&self, is_charging: bool) -> &'static str {
        match self {
            Language::English => {
                if is_charging {
                    "Full Charge in:"
                } else {
                    "Est. Autonomy:"
                }
            }
            Language::Spanish => {
                if is_charging {
                    "Carga completa en:"
                } else {
                    "Autonomía est.:"
                }
            }
        }
    }

    pub fn format_time_left(&self, minutes: u32, is_charging: bool) -> String {
        match self {
            Language::English => {
                if is_charging {
                    if minutes == 0 {
                        "Fully charged".to_string()
                    } else if minutes < 60 {
                        format!("~{minutes}m to full")
                    } else {
                        format!("~{}h {}m to full", minutes / 60, minutes % 60)
                    }
                } else if minutes < 60 {
                    format!("~{minutes}m remaining")
                } else {
                    format!("~{}h {}m remaining", minutes / 60, minutes % 60)
                }
            }
            Language::Spanish => {
                if is_charging {
                    if minutes == 0 {
                        "Completamente cargado".to_string()
                    } else if minutes < 60 {
                        format!("~{minutes}m para completar")
                    } else {
                        format!("~{}h {}m para completar", minutes / 60, minutes % 60)
                    }
                } else if minutes < 60 {
                    format!("~{minutes}m restantes")
                } else {
                    format!("~{}h {}m restantes", minutes / 60, minutes % 60)
                }
            }
        }
    }

    pub fn heading_per_device_settings(&self) -> &'static str {
        match self {
            Language::English => "🎛️ Per-Device Settings & Alerts",
            Language::Spanish => "🎛️ Ajustes y alertas por dispositivo",
        }
    }

    pub fn label_custom_low_alert(&self) -> &'static str {
        match self {
            Language::English => "Custom Low Alert:",
            Language::Spanish => "Alerta baja personalizada:",
        }
    }

    pub fn label_mute_alerts(&self) -> &'static str {
        match self {
            Language::English => "Mute alerts for this device",
            Language::Spanish => "Silenciar avisos para este dispositivo",
        }
    }

    pub fn label_hide_from_dock(&self) -> &'static str {
        match self {
            Language::English => "Hide from dock pill",
            Language::Spanish => "Ocultar en la pastilla",
        }
    }

    pub fn label_use_default_threshold(&self) -> &'static str {
        match self {
            Language::English => "Default",
            Language::Spanish => "Predeterminado",
        }
    }
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

        assert_eq!(en.heading_connected_devices(2), "🎧 Connected Devices (2)");
        assert_eq!(
            es.heading_connected_devices(2),
            "🎧 Dispositivos conectados (2)"
        );
    }
}
