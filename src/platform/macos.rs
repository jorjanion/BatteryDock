use std::fs::{self, File};
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;
use std::process::Command;

pub struct SingleInstanceGuard {
    _file: File,
    _path: PathBuf,
}

impl Drop for SingleInstanceGuard {
    fn drop(&mut self) {
        unsafe {
            libc::flock(self._file.as_raw_fd(), libc::LOCK_UN);
        }
    }
}

pub fn get_macos_app_support_dir() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home)
            .join("Library")
            .join("Application Support")
            .join("BatteryDock")
    } else {
        PathBuf::from("/tmp/BatteryDock")
    }
}

pub fn acquire_single_instance(name: &str) -> (bool, Option<SingleInstanceGuard>) {
    let dir = get_macos_app_support_dir();
    let _ = fs::create_dir_all(&dir);
    let lock_path = dir.join(format!("{}.lock", name.replace('\\', "_").replace('/', "_")));

    let file = match fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
    {
        Ok(f) => f,
        Err(_) => return (false, None),
    };

    let fd = file.as_raw_fd();
    let ret = unsafe { libc::flock(fd, libc::LOCK_EX | libc::LOCK_NB) };
    if ret == 0 {
        (
            true,
            Some(SingleInstanceGuard {
                _file: file,
                _path: lock_path,
            }),
        )
    } else {
        (false, None)
    }
}

fn get_launch_agent_path() -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    Some(
        PathBuf::from(home)
            .join("Library")
            .join("LaunchAgents")
            .join("com.battery-dock.plist"),
    )
}

pub fn set_autostart(enable: bool) -> Result<(), ()> {
    let plist_path = match get_launch_agent_path() {
        Some(p) => p,
        None => return Err(()),
    };

    if enable {
        if let Some(parent) = plist_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let current_exe = match std::env::current_exe() {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(_) => return Err(()),
        };

        let plist_content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.battery-dock</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>ProcessType</key>
    <string>Interactive</string>
</dict>
</plist>"#,
            current_exe
        );

        fs::write(&plist_path, plist_content).map_err(|_| ())?;
    } else if plist_path.exists() {
        let _ = fs::remove_file(&plist_path);
    }

    Ok(())
}

pub fn is_autostart_enabled() -> bool {
    get_launch_agent_path().map_or(false, |p| p.exists())
}

pub fn play_sound_chime() {
    // Try pleasant system ping/tink sound on macOS asynchronously
    let _ = Command::new("/usr/bin/afplay")
        .arg("/System/Library/Sounds/Tink.aiff")
        .spawn();
}

pub fn show_system_notification(title: &str, message: &str) {
    let clean_title = title.replace('"', "\\\"");
    let clean_msg = message.replace('"', "\\\"");
    let script = format!(
        "display notification \"{}\" with title \"{}\"",
        clean_msg, clean_title
    );
    let _ = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .spawn();
}

pub fn open_settings_window() {
    if let Ok(exe) = std::env::current_exe() {
        let _ = Command::new(exe).arg("--settings").spawn();
    }
}

pub fn restart_app() {
    if let Ok(exe) = std::env::current_exe() {
        let _ = Command::new(exe).spawn();
    }
    std::process::exit(0);
}
