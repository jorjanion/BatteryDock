use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Shell::{
    Shell_NotifyIconW, NIF_INFO, NIIF_INFO, NIIF_WARNING, NIM_MODIFY, NOTIFYICONDATAW,
};

pub fn show_toast(title: &str, message: &str) {
    let title = title.to_string();
    let message = message.to_string();

    std::thread::spawn(move || {
        let mut nid = NOTIFYICONDATAW::default();
        nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
        nid.hWnd = HWND(std::ptr::null_mut());
        nid.uFlags = NIF_INFO;
        nid.dwInfoFlags = if title.contains("⚠️") || title.to_lowercase().contains("critical") {
            NIIF_WARNING
        } else {
            NIIF_INFO
        };

        // Convert title to UTF-16
        let title_wide: Vec<u16> = OsStr::new(&title).encode_wide().chain(Some(0)).collect();
        let msg_wide: Vec<u16> = OsStr::new(&message).encode_wide().chain(Some(0)).collect();

        let title_len = title_wide.len().min(nid.szInfoTitle.len());
        nid.szInfoTitle[..title_len].copy_from_slice(&title_wide[..title_len]);

        let msg_len = msg_wide.len().min(nid.szInfo.len());
        nid.szInfo[..msg_len].copy_from_slice(&msg_wide[..msg_len]);

        unsafe {
            // Send notification
            let _ = Shell_NotifyIconW(NIM_MODIFY, &nid);
        }
    });
}
