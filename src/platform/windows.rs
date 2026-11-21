use std::os::windows::ffi::OsStrExt;
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS, HANDLE, HWND};
use windows::Win32::System::Registry::{
    RegCloseKey, RegCreateKeyW, RegDeleteValueW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW,
    HKEY_CURRENT_USER, KEY_READ, REG_SZ,
};
use windows::Win32::System::Threading::CreateMutexW;
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, IsWindow, SetForegroundWindow, ShowWindow, SW_RESTORE, SW_SHOW,
};

pub struct SingleInstanceGuard {
    handle: HANDLE,
}

impl Drop for SingleInstanceGuard {
    fn drop(&mut self) {
        if !self.handle.0.is_null() {
            unsafe {
                let _ = CloseHandle(self.handle);
            }
        }
    }
}

unsafe impl Send for SingleInstanceGuard {}
unsafe impl Sync for SingleInstanceGuard {}

pub fn acquire_single_instance(name: &str) -> (bool, Option<SingleInstanceGuard>) {
    let name_wide: Vec<u16> = std::ffi::OsStr::new(name)
        .encode_wide()
        .chain(Some(0))
        .collect();

    unsafe {
        let res = CreateMutexW(None, true, PCWSTR(name_wide.as_ptr()));
        let already_exists = GetLastError() == ERROR_ALREADY_EXISTS;

        match res {
            Ok(handle) => {
                if already_exists {
                    let _ = CloseHandle(handle);
                    (false, None)
                } else {
                    (true, Some(SingleInstanceGuard { handle }))
                }
            }
            Err(_) => (false, None),
        }
    }
}

pub fn set_autostart(enable: bool) -> Result<(), ()> {
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

pub fn is_autostart_enabled() -> bool {
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
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(subkey.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        )
        .is_ok()
        {
            let mut val_type = REG_SZ;
            let mut data_len = 0u32;
            let query_res = RegQueryValueExW(
                hkey,
                PCWSTR(app_name.as_ptr()),
                None,
                Some(&mut val_type),
                None,
                Some(&mut data_len),
            );
            let _ = RegCloseKey(hkey);
            query_res.is_ok() && data_len > 0
        } else {
            false
        }
    }
}

pub fn open_settings_window() {
    unsafe {
        if let Ok(existing) = FindWindowW(None, w!("BatteryDock Settings")) {
            if !existing.0.is_null() && IsWindow(existing).as_bool() {
                let _ = ShowWindow(existing, SW_RESTORE);
                let _ = SetForegroundWindow(existing);
                return;
            }
        }

        if let Ok(exe) = std::env::current_exe() {
            let exe_str = exe.to_string_lossy().to_string();
            let exe_hstring = windows::core::HSTRING::from(exe_str);
            let open_hstring = windows::core::HSTRING::from("open");
            let param_hstring = windows::core::HSTRING::from("--settings");
            let _ = windows::Win32::UI::Shell::ShellExecuteW(
                HWND(std::ptr::null_mut()),
                PCWSTR(open_hstring.as_ptr()),
                PCWSTR(exe_hstring.as_ptr()),
                PCWSTR(param_hstring.as_ptr()),
                PCWSTR(std::ptr::null()),
                SW_SHOW,
            );
        }
    }
}

pub fn restart_app() {
    if let Ok(exe) = std::env::current_exe() {
        let exe_str = exe.to_string_lossy().to_string();
        let exe_hstring = windows::core::HSTRING::from(exe_str);
        let open_hstring = windows::core::HSTRING::from("open");
        unsafe {
            let _ = windows::Win32::UI::Shell::ShellExecuteW(
                HWND(std::ptr::null_mut()),
                PCWSTR(open_hstring.as_ptr()),
                PCWSTR(exe_hstring.as_ptr()),
                PCWSTR(std::ptr::null()),
                PCWSTR(std::ptr::null()),
                SW_SHOW,
            );
        }
    }
    std::process::exit(0);
}
