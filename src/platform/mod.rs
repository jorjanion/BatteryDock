#[cfg(windows)]
pub mod windows;
#[cfg(windows)]
pub use windows::*;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(not(any(windows, target_os = "macos")))]
pub mod fallback {
    pub struct SingleInstanceGuard;
    pub fn acquire_single_instance(_name: &str) -> (bool, Option<SingleInstanceGuard>) {
        (true, Some(SingleInstanceGuard))
    }
    pub fn set_autostart(_enable: bool) -> Result<(), ()> {
        Ok(())
    }
    pub fn is_autostart_enabled() -> bool {
        false
    }
    pub fn play_sound_chime() {}
    pub fn show_system_notification(_title: &str, _message: &str) {}
    pub fn restart_app() {
        std::process::exit(0);
    }
}
#[cfg(not(any(windows, target_os = "macos")))]
pub use fallback::*;
