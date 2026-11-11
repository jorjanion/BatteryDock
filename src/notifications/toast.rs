use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{GetLastError, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Shell::{
    Shell_NotifyIconW, NIF_ICON, NIF_INFO, NIF_TIP, NIIF_INFO, NIIF_NOSOUND, NIIF_WARNING, NIM_ADD,
    NIM_DELETE, NOTIFYICONDATAW,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, LoadIconW, PeekMessageW,
    RegisterClassExW, TranslateMessage, IDI_INFORMATION, IDI_WARNING, MSG, PM_REMOVE, WNDCLASSEXW,
    WS_EX_TOOLWINDOW, WS_OVERLAPPED,
};

static NEXT_TOAST_ID: AtomicU32 = AtomicU32::new(100);
static TOAST_CLASS_ATOM: OnceLock<Result<u16, u32>> = OnceLock::new();

unsafe extern "system" fn toast_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    DefWindowProcW(hwnd, msg, wparam, lparam)
}

fn ensure_toast_window_class() -> Result<u16, u32> {
    *TOAST_CLASS_ATOM.get_or_init(|| unsafe {
        let hinstance = GetModuleHandleW(None).map_err(|e| e.code().0 as u32)?;
        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            lpfnWndProc: Some(toast_wnd_proc),
            hInstance: hinstance.into(),
            lpszClassName: w!("BatteryDockToastHelper"),
            ..Default::default()
        };
        let atom = RegisterClassExW(&wc);
        if atom == 0 {
            Err(GetLastError().0)
        } else {
            Ok(atom)
        }
    })
}

/// Copies `text` into a fixed UTF-16 Win32 buffer, always guaranteeing a trailing NUL (`0`)
/// and never leaving a dangling UTF-16 high surrogate at the truncation boundary.
pub fn copy_wide_truncated(dst: &mut [u16], text: &str) {
    dst.fill(0);
    if dst.is_empty() {
        return;
    }
    let max_units = dst.len() - 1;
    let mut wide: Vec<u16> = OsStr::new(text).encode_wide().take(max_units).collect();
    if wide.len() == max_units {
        if let Some(&last) = wide.last() {
            if (0xD800..=0xDBFF).contains(&last) {
                wide.pop();
            }
        }
    }
    let n = wide.len();
    dst[..n].copy_from_slice(&wide);
    dst[n] = 0;
}

/// Displays a desktop notification via a valid hidden helper `HWND`, `NIM_ADD`, brief hold
/// while pumping messages, and clean `NIM_DELETE` + `DestroyWindow`.
pub fn show_toast_with_hold(
    title: &str,
    message: &str,
    hold_duration: Duration,
) -> Result<(), u32> {
    let _atom = ensure_toast_window_class()?;
    let is_critical = title.contains("⚠️") || title.to_ascii_lowercase().contains("critical");

    unsafe {
        let hinstance = GetModuleHandleW(None).map_err(|e| e.code().0 as u32)?;
        // Must be an actual hidden top-level window (never NULL and never HWND_MESSAGE)
        // for Shell_NotifyIconW(NIM_ADD, ...) to register and display NIF_INFO toasts reliably.
        let hwnd = CreateWindowExW(
            WS_EX_TOOLWINDOW,
            w!("BatteryDockToastHelper"),
            w!("BatteryDock Notification"),
            WS_OVERLAPPED,
            0,
            0,
            0,
            0,
            None,
            None,
            hinstance,
            None,
        )
        .map_err(|_| GetLastError().0)?;

        if hwnd.0.is_null() {
            return Err(GetLastError().0);
        }

        let icon = LoadIconW(hinstance, PCWSTR(std::ptr::without_provenance(1)))
            .or_else(|_| {
                LoadIconW(
                    None,
                    if is_critical {
                        IDI_WARNING
                    } else {
                        IDI_INFORMATION
                    },
                )
            })
            .unwrap_or_default();

        let uid = NEXT_TOAST_ID.fetch_add(1, Ordering::Relaxed);
        let base_flag = if is_critical { NIIF_WARNING } else { NIIF_INFO };

        let mut nid = NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: uid,
            uFlags: NIF_ICON | NIF_TIP | NIF_INFO,
            hIcon: icon,
            dwInfoFlags: base_flag | NIIF_NOSOUND,
            ..Default::default()
        };

        copy_wide_truncated(&mut nid.szTip, "BatteryDock");
        copy_wide_truncated(&mut nid.szInfoTitle, title);
        copy_wide_truncated(&mut nid.szInfo, message);

        let added = Shell_NotifyIconW(NIM_ADD, &nid).as_bool();
        if !added {
            let err = GetLastError().0;
            let _ = DestroyWindow(hwnd);
            return Err(if err == 0 { 1 } else { err });
        }

        // Pump window messages gently while Windows displays/queues the toast
        let deadline = Instant::now() + hold_duration;
        let mut msg = MSG::default();
        while Instant::now() < deadline {
            while PeekMessageW(&mut msg, hwnd, 0, 0, PM_REMOVE).as_bool() {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                break;
            }
            std::thread::sleep(remaining.min(Duration::from_millis(50)));
        }

        let _ = Shell_NotifyIconW(NIM_DELETE, &nid);
        let _ = DestroyWindow(hwnd);
        let _ = (WPARAM(0), LPARAM(0));
        let _ = HWND(std::ptr::null_mut());
        Ok(())
    }
}

pub fn show_toast(title: &str, message: &str) {
    let title = title.to_string();
    let message = message.to_string();

    std::thread::spawn(move || {
        let _ = show_toast_with_hold(&title, &message, Duration::from_secs(7));
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows::Win32::UI::WindowsAndMessaging::FindWindowW;

    #[test]
    fn test_copy_wide_truncated_guarantees_null_termination_and_valid_utf16() {
        let mut buf = [0xFFFFu16; 64];
        let long_title = format!("⚠️ {}", "A".repeat(100));
        copy_wide_truncated(&mut buf, &long_title);
        assert_eq!(buf[63], 0, "Buffer must always end with NUL terminator");
        let nul_pos = buf.iter().position(|&c| c == 0).unwrap();
        assert!(nul_pos <= 63);
        assert!(String::from_utf16(&buf[..nul_pos]).is_ok());

        // Boundary split directly inside a surrogate-pair emoji (🔋 = 2 UTF-16 code units)
        let mut small = [0xFFFFu16; 4]; // max 3 code units + NUL
        copy_wide_truncated(&mut small, "AB🔋C");
        assert_eq!(small[3], 0);
        let nul = small.iter().position(|&c| c == 0).unwrap();
        let decoded = String::from_utf16(&small[..nul]).expect("Must not split surrogate pair");
        assert_eq!(decoded, "AB");
    }

    #[test]
    fn test_show_toast_with_hold_creates_valid_window_and_never_returns_203() {
        let has_tray = unsafe {
            FindWindowW(w!("Shell_TrayWnd"), None)
                .map(|h| !h.0.is_null())
                .unwrap_or(false)
        };

        let res = show_toast_with_hold(
            "BatteryDock Test",
            "Toast helper window verification",
            Duration::from_millis(50),
        );

        if has_tray {
            assert_eq!(
                res,
                Ok(()),
                "Shell_NotifyIconW(NIM_ADD) must succeed when Shell_TrayWnd is active"
            );
        } else {
            // Even in headless/isolated desktops without Explorer Shell_TrayWnd,
            // it must NOT fail with error 203 (ERROR_ENVVAR_NOT_FOUND from invalid NIM_MODIFY).
            assert_ne!(
                res,
                Err(203),
                "Must never fail with Win32 error 203 (broken NIM_MODIFY)"
            );
        }
    }
}
