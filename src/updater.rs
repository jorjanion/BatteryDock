#[cfg(windows)]
use std::ffi::c_void;
use std::fs::File;
use std::io::Write;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
#[cfg(windows)]
use windows::core::PCWSTR;
#[cfg(windows)]
use windows::Win32::Networking::WinHttp::{
    WinHttpCloseHandle, WinHttpConnect, WinHttpOpen, WinHttpOpenRequest, WinHttpQueryDataAvailable,
    WinHttpReadData, WinHttpReceiveResponse, WinHttpSendRequest, WinHttpSetOption,
    WINHTTP_ACCESS_TYPE_DEFAULT_PROXY, WINHTTP_FLAG_SECURE, WINHTTP_OPTION_REDIRECT_POLICY,
    WINHTTP_OPTION_REDIRECT_POLICY_ALWAYS,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateStatus {
    Idle,
    Checking,
    UpToDate,
    UpdateAvailable {
        version: String,
        download_url: String,
    },
    Downloading {
        version: String,
    },
    Installing {
        version: String,
    },
    Failed(String),
}

static CURRENT_STATUS: Mutex<Option<UpdateStatus>> = Mutex::new(None);
static IS_BUSY: AtomicBool = AtomicBool::new(false);

pub fn get_update_status() -> UpdateStatus {
    CURRENT_STATUS
        .lock()
        .unwrap()
        .clone()
        .unwrap_or(UpdateStatus::Idle)
}

fn set_status(status: UpdateStatus) {
    *CURRENT_STATUS.lock().unwrap() = Some(status);
}

/// Helper struct for release info parsed from GitHub JSON
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseInfo {
    pub tag_name: String,
    pub installer_url: Option<String>,
}

/// Parses a semver string like "v0.1.11" or "0.1.12" into (major, minor, patch)
pub fn parse_semver(v: &str) -> Option<(u32, u32, u32)> {
    let clean = v.trim().trim_start_matches('v').trim_start_matches('V');
    let parts: Vec<&str> = clean.split('.').collect();
    if parts.len() < 3 {
        return None;
    }
    let major = parts[0].parse::<u32>().ok()?;
    let minor = parts[1].parse::<u32>().ok()?;
    let patch = parts[2]
        .split(|c: char| !c.is_ascii_digit())
        .next()?
        .parse::<u32>()
        .ok()?;
    Some((major, minor, patch))
}

/// Compares two semver strings: returns true if target is strictly newer than current
pub fn is_newer_version(current: &str, target: &str) -> bool {
    match (parse_semver(current), parse_semver(target)) {
        (Some((c_maj, c_min, c_pat)), Some((t_maj, t_min, t_pat))) => {
            (t_maj, t_min, t_pat) > (c_maj, c_min, c_pat)
        }
        _ => false,
    }
}

/// Extracts tag_name and the installer asset download URL from GitHub Release JSON
pub fn parse_github_release_json(json: &str) -> Option<ReleaseInfo> {
    // Find "tag_name": "..."
    let tag_idx = json.find("\"tag_name\"")?;
    let colon_idx = json[tag_idx..].find(':')? + tag_idx;
    let quote_start = json[colon_idx..].find('"')? + colon_idx + 1;
    let quote_end = json[quote_start..].find('"')? + quote_start;
    let tag_name = json[quote_start..quote_end].trim().to_string();

    // Look for installer download url: "browser_download_url": "https://...BatteryDock-Setup-...exe"
    let mut installer_url = None;
    let mut search_from = 0;
    while let Some(pos) = json[search_from..].find("\"browser_download_url\"") {
        let abs_pos = search_from + pos;
        if let Some(col) = json[abs_pos..].find(':') {
            let col_pos = abs_pos + col;
            if let Some(q1) = json[col_pos..].find('"') {
                let start = col_pos + q1 + 1;
                if let Some(q2) = json[start..].find('"') {
                    let url = &json[start..start + q2];
                    #[cfg(windows)]
                    let is_match = url.contains("BatteryDock-Setup") && url.ends_with(".exe");
                    #[cfg(target_os = "macos")]
                    let is_match = url.contains("BatteryDock") && url.ends_with(".dmg");
                    #[cfg(not(any(windows, target_os = "macos")))]
                    let is_match = url.ends_with(".tar.gz") || url.ends_with(".zip");

                    if is_match {
                        installer_url = Some(url.to_string());
                        break;
                    }
                }
            }
        }
        search_from = abs_pos + 20;
    }

    Some(ReleaseInfo {
        tag_name,
        installer_url,
    })
}

#[cfg(windows)]
fn to_wide_null(s: &str) -> Vec<u16> {
    std::ffi::OsStr::new(s)
        .encode_wide()
        .chain(Some(0))
        .collect()
}

/// Performs a native HTTPS GET request using Windows WinHTTP
#[cfg(windows)]
pub fn winhttp_get(url: &str) -> Result<Vec<u8>, String> {
    let clean_url = url.trim();
    let without_scheme = clean_url
        .strip_prefix("https://")
        .ok_or_else(|| "Only HTTPS URLs are supported".to_string())?;

    let slash_idx = without_scheme.find('/').unwrap_or(without_scheme.len());
    let host = &without_scheme[..slash_idx];
    let path = if slash_idx < without_scheme.len() {
        &without_scheme[slash_idx..]
    } else {
        "/"
    };

    let user_agent = to_wide_null(&format!("BatteryDock/{}", env!("CARGO_PKG_VERSION")));
    let host_wide = to_wide_null(host);
    let path_wide = to_wide_null(path);
    let verb_wide = to_wide_null("GET");
    let headers_raw = "User-Agent: BatteryDock\r\nAccept: application/vnd.github.v3+json, application/octet-stream\r\n";
    let headers_wide: Vec<u16> = std::ffi::OsStr::new(headers_raw).encode_wide().collect();

    unsafe {
        let session = WinHttpOpen(
            PCWSTR(user_agent.as_ptr()),
            WINHTTP_ACCESS_TYPE_DEFAULT_PROXY,
            PCWSTR::null(),
            PCWSTR::null(),
            0,
        );
        if session.is_null() {
            return Err("Failed to initialize WinHttp session".to_string());
        }

        // Enable automatic redirect following (essential for GitHub Release asset CDN links)
        let redirect_policy = WINHTTP_OPTION_REDIRECT_POLICY_ALWAYS;
        let _ = WinHttpSetOption(
            Some(session as *const c_void),
            WINHTTP_OPTION_REDIRECT_POLICY,
            Some(&redirect_policy.to_ne_bytes()),
        );

        let connect = WinHttpConnect(session, PCWSTR(host_wide.as_ptr()), 443, 0);
        if connect.is_null() {
            let _ = WinHttpCloseHandle(session);
            return Err(format!("Failed to connect to host: {host}"));
        }

        let request = WinHttpOpenRequest(
            connect,
            PCWSTR(verb_wide.as_ptr()),
            PCWSTR(path_wide.as_ptr()),
            PCWSTR::null(),
            PCWSTR::null(),
            std::ptr::null(),
            WINHTTP_FLAG_SECURE,
        );
        if request.is_null() {
            let _ = WinHttpCloseHandle(connect);
            let _ = WinHttpCloseHandle(session);
            return Err("Failed to create HTTP request".to_string());
        }

        let send_res = WinHttpSendRequest(request, Some(&headers_wide), None, 0, 0, 0);
        if send_res.is_err() {
            let _ = WinHttpCloseHandle(request);
            let _ = WinHttpCloseHandle(connect);
            let _ = WinHttpCloseHandle(session);
            return Err("Failed to send HTTP request to GitHub".to_string());
        }

        if WinHttpReceiveResponse(request, std::ptr::null_mut()).is_err() {
            let _ = WinHttpCloseHandle(request);
            let _ = WinHttpCloseHandle(connect);
            let _ = WinHttpCloseHandle(session);
            return Err("Failed to receive HTTP response".to_string());
        }

        let mut body = Vec::new();
        let mut buf = [0u8; 8192];
        loop {
            let mut available = 0u32;
            if WinHttpQueryDataAvailable(request, &mut available).is_err() || available == 0 {
                break;
            }
            let to_read = available.min(buf.len() as u32);
            let mut bytes_read = 0u32;
            if WinHttpReadData(
                request,
                buf.as_mut_ptr() as *mut _,
                to_read,
                &mut bytes_read,
            )
            .is_err()
                || bytes_read == 0
            {
                break;
            }
            body.extend_from_slice(&buf[..bytes_read as usize]);
        }

        let _ = WinHttpCloseHandle(request);
        let _ = WinHttpCloseHandle(connect);
        let _ = WinHttpCloseHandle(session);

        Ok(body)
    }
}

#[cfg(windows)]
pub fn http_get(url: &str) -> Result<Vec<u8>, String> {
    winhttp_get(url)
}

#[cfg(not(windows))]
pub fn http_get(url: &str) -> Result<Vec<u8>, String> {
    let output = std::process::Command::new("curl")
        .args([
            "-sL",
            "-H",
            "User-Agent: BatteryDock",
            "-H",
            "Accept: application/vnd.github.v3+json, application/octet-stream",
            url,
        ])
        .output()
        .map_err(|e| format!("curl execution failed: {e}"))?;

    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(format!("curl error code: {:?}", output.status.code()))
    }
}

/// Queries GitHub releases API for latest release info
pub fn fetch_latest_release() -> Result<ReleaseInfo, String> {
    let api_url = "https://api.github.com/repos/jorjanion/BatteryDock/releases/latest";
    let bytes = http_get(api_url)?;
    let text = String::from_utf8(bytes).map_err(|e| format!("Invalid UTF-8 in response: {e}"))?;
    parse_github_release_json(&text)
        .ok_or_else(|| "Could not parse release info from GitHub".to_string())
}

/// Downloads the installer to a temporary location and launches it
pub fn download_and_run_installer(installer_url: &str, version: &str) -> Result<(), String> {
    set_status(UpdateStatus::Downloading {
        version: version.to_string(),
    });

    let bytes = http_get(installer_url)?;
    if bytes.len() < 100_000 {
        return Err("Downloaded installer is too small or corrupt".to_string());
    }

    let temp_dir = std::env::temp_dir();
    #[cfg(windows)]
    let installer_path: PathBuf = temp_dir.join(format!("BatteryDock-Setup-{version}.exe"));
    #[cfg(not(windows))]
    let installer_path: PathBuf = temp_dir.join(format!("BatteryDock-{version}.dmg"));

    let mut file = File::create(&installer_path)
        .map_err(|e| format!("Failed to create temporary installer file: {e}"))?;
    file.write_all(&bytes)
        .map_err(|e| format!("Failed to write installer data: {e}"))?;
    drop(file);

    set_status(UpdateStatus::Installing {
        version: version.to_string(),
    });

    #[cfg(windows)]
    let res = std::process::Command::new(&installer_path)
        .args(["/SILENT", "/CLOSEAPPLICATIONS", "/RESTARTAPPLICATIONS"])
        .spawn();

    #[cfg(target_os = "macos")]
    let res = std::process::Command::new("open")
        .arg(&installer_path)
        .spawn();

    #[cfg(not(any(windows, target_os = "macos")))]
    let res: Result<std::process::Child, std::io::Error> = Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Auto-install not supported on this platform",
    ));

    match res {
        Ok(_) => {
            // Give installer a split second to take over, then terminate our process so files can be updated
            std::thread::sleep(std::time::Duration::from_millis(300));
            std::process::exit(0);
        }
        Err(e) => Err(format!("Failed to launch installer: {e}")),
    }
}

/// Checks for updates asynchronously in a background thread.
/// If `auto_install` is true and a newer version is found, downloads and installs it automatically.
pub fn check_for_updates_async(auto_install: bool) {
    if IS_BUSY.swap(true, Ordering::SeqCst) {
        return;
    }

    set_status(UpdateStatus::Checking);

    std::thread::spawn(move || {
        let current_version = env!("CARGO_PKG_VERSION");
        match fetch_latest_release() {
            Ok(rel) => {
                if is_newer_version(current_version, &rel.tag_name) {
                    if let Some(installer_url) = rel.installer_url {
                        if auto_install {
                            if let Err(e) =
                                download_and_run_installer(&installer_url, &rel.tag_name)
                            {
                                set_status(UpdateStatus::Failed(e));
                            }
                        } else {
                            set_status(UpdateStatus::UpdateAvailable {
                                version: rel.tag_name,
                                download_url: installer_url,
                            });
                        }
                    } else {
                        #[cfg(windows)]
                        let no_asset_msg = "No Windows installer (.exe) found in latest release assets";
                        #[cfg(target_os = "macos")]
                        let no_asset_msg = "No macOS disk image (.dmg) found in latest release assets";
                        #[cfg(not(any(windows, target_os = "macos")))]
                        let no_asset_msg = "No installer package found in latest release assets";

                        set_status(UpdateStatus::Failed(no_asset_msg.to_string()));
                    }
                } else {
                    set_status(UpdateStatus::UpToDate);
                }
            }
            Err(e) => {
                set_status(UpdateStatus::Failed(e));
            }
        }

        IS_BUSY.store(false, Ordering::SeqCst);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_semver() {
        assert_eq!(parse_semver("0.1.11"), Some((0, 1, 11)));
        assert_eq!(parse_semver("v0.1.11"), Some((0, 1, 11)));
        assert_eq!(parse_semver("v1.2.3-alpha"), Some((1, 2, 3)));
        assert_eq!(parse_semver("invalid"), None);
    }

    #[test]
    fn test_is_newer_version() {
        assert!(is_newer_version("0.1.11", "0.1.12"));
        assert!(is_newer_version("0.1.11", "v0.2.0"));
        assert!(is_newer_version("0.1.11", "1.0.0"));
        assert!(!is_newer_version("0.1.11", "0.1.11"));
        assert!(!is_newer_version("0.1.11", "0.1.10"));
        assert!(!is_newer_version("0.1.11", "0.0.9"));
    }

    #[test]
    fn test_parse_github_release_json() {
        let sample = r#"{
            "tag_name": "v0.1.12",
            "name": "Release v0.1.12",
            "assets": [
                {
                    "name": "BatteryDock-Windows-Portable.zip",
                    "browser_download_url": "https://github.com/jorjanion/BatteryDock/releases/download/v0.1.12/BatteryDock-Windows-Portable.zip"
                },
                {
                    "name": "BatteryDock-Setup-v0.1.12.exe",
                    "browser_download_url": "https://github.com/jorjanion/BatteryDock/releases/download/v0.1.12/BatteryDock-Setup-v0.1.12.exe"
                }
            ]
        }"#;

        let info = parse_github_release_json(sample).expect("Should parse");
        assert_eq!(info.tag_name, "v0.1.12");
        assert_eq!(
            info.installer_url,
            Some("https://github.com/jorjanion/BatteryDock/releases/download/v0.1.12/BatteryDock-Setup-v0.1.12.exe".to_string())
        );
    }
}
