#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/icon.ico");
    res.set("ProductName", "BatteryDock");
    res.set("FileDescription", "BatteryDock - Windows Peripheral Battery Monitor");
    res.set("LegalCopyright", "Copyright (c) 2026 jorjanion");
    let _ = res.compile();
}

#[cfg(not(windows))]
fn main() {}
