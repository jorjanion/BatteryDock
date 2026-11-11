<div align="center">

<img src="assets/logo.png" alt="BatteryDock Logo" width="160" style="border-radius: 24px;" />

# BatteryDock

**A sleek, native Rust battery overlay embedded directly into your Windows 10/11 taskbar — zero vendor suite bloat.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Download Latest Installer](https://github.com/jorjanion/BatteryDock/releases/latest) · [Report a Bug](https://github.com/jorjanion/BatteryDock/issues) · [Request Device Support](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)

</div>

---

## 📸 Preview

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="42%">
        <img src="docs/screenshots/dock-closeup.png" alt="BatteryDock capsule closeup showing headset battery at 92%" /><br /><br />
        <sub><b>Taskbar Capsule</b> — Embedded directly on the Windows taskbar with crisp Per-Monitor DPI scaling, color-coded battery levels, and live ⚡ charging badge.</sub>
      </td>
      <td align="center" valign="middle" width="58%">
        <img src="docs/screenshots/settings.png" alt="BatteryDock Settings window v0.1.11" width="340" /><br />
        <sub><b>Standalone Settings</b> — Threshold sliders, chime preview, placement modes, and connected device list.</sub>
      </td>
    </tr>
  </table>
</div>

---

## ⚡ Why BatteryDock?

Most wireless peripherals lock your battery percentage inside bloated manufacturer suites running dozens of background services. **BatteryDock** puts all your devices in one clean taskbar pill:

- **Zero Vendor Bloat:** A single **~4 MB** native Rust binary (zero Electron, zero telemetry, ~77 MB private memory observed live on Windows 11).
- **Direct Hardware Reading:** Queries **Logitech HID++ 2.0**, **Windows Bluetooth / BLE GATT**, **XInput**, and **Sony DualSense / DualShock 4 HID** directly — Logitech devices can report without even opening G HUB (and also read via G HUB IPC when running).
- **Smart Fullscreen & Taskbar Rules:** Automatically hides when fullscreen games, fullscreen videos, or auto-hidden taskbars cover the bar — and stays rock-solid when pressing the `Win` key, opening Start/Search/Widgets, or clicking the desktop.
- **Live Charging Indicator:** Highlights charging devices in **cyan** with a **⚡ lightning bolt badge** so you know at a glance when peripherals are plugged in.
- **Background Alerts That Actually Fire:** Low and critical battery chimes & desktop notifications continue working from the background polling engine even while the pill is hidden in a fullscreen game.

---

## 📥 Download & Installation

Grab the latest release from the **[Releases Page](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Download **`BatteryDock-Setup-v0.1.11.exe`** (~4.4 MB) — or `BatteryDock-Windows-Portable.zip` if you prefer zero installation.
2. Run the installer (English & Spanish supported) and optionally check **"Start BatteryDock when Windows boots"**.
3. The pill will appear on your taskbar next to the clock tray.

### 🛡️ Windows SmartScreen Note
Because BatteryDock is a free, community open-source app without a paid EV code-signing certificate yet, Windows SmartScreen may show *"Windows protected your PC"* on first launch (click **More info → Run anyway**).
- Every release executable and installer is compiled publicly on clean GitHub-hosted Windows runners in [`.github/workflows/release.yml`](.github/workflows/release.yml).
- Want total peace of mind? You can audit every line of Rust code and [compile it yourself](#-building-from-source) in under a minute.

---

## 🎧 Supported Hardware

| Category | Protocol / Source | Examples & Notes |
| :--- | :--- | :--- |
| **Logitech Wireless** | Direct HID++ 2.0 (`0x1004` / `0x1000`) & G HUB IPC | Lightspeed headsets (G733, G535, G935), mice (G PRO, Superlight, G502), keyboards |
| **Bluetooth & BLE** | Windows WinRT PnP + BLE GATT (`0x180F` / `0x2A19`) | Headphones (Sony WH/WF, Bose, AirPods), BT mice & keyboards |
| **Xbox Controllers** | Windows `XInput` | Xbox Wireless Controllers (Series X\|S, One, 360) including wired charging detection |
| **PlayStation** | Sony HID (`VID 0x054C`) USB & Bluetooth | DualSense & DualShock 4 (supports multiple controllers + charging status) |

**Missing your brand (Razer, Corsair, SteelSeries, Keychron, etc.)?**
👉 **[Open a Device Support Request](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** — adding HID/Bluetooth profiles is welcome via PRs too!

---

## 🎯 Controls & Visual Indicators

### Status Colors
- **Green:** Healthy battery (above low threshold)
- **Amber:** Low battery (configurable, default `20%`)
- **Red (Pulsing):** Critical battery (configurable, default `10%`)
- **Cyan + ⚡:** Device currently charging

### Interaction
- **Left-click** pill or **`⚙`** gear (or click the tray icon): Open **BatteryDock Settings** (`BatteryDock.exe --settings`).
- **Right-click** anywhere on the pill or tray icon: Open native menu (**Settings**, **Refresh Devices**, **Reset to Taskbar Clock**, **Restart BatteryDock**, **Exit BatteryDock**).
- **Click & drag (> 5px):** Move the pill anywhere (including secondary monitors); right-click → **Reset to Taskbar Clock** to snap back.

---

## ⚙️ Configuration

Settings persist automatically at `%APPDATA%\jorjanion\BatteryDock\config\config.toml`:

```toml
low_battery_threshold = 20
critical_battery_threshold = 10
audio_alert_enabled = true
visual_alert_enabled = true
position_mode = "TaskbarLeftOfTray" # TaskbarLeftOfTray | TaskbarRight | TaskbarLeft | Custom
custom_pos_x = 0.0
custom_pos_y = 0.0
snap_to_taskbar = true
start_with_windows = false
poll_interval_secs = 15
show_percentage_text = true
compact_mode = false
```

---

## 🛠️ Building from Source

### Prerequisites
- [Rust toolchain](https://rustup.rs/) (stable)
- Windows 10 or Windows 11 (MSVC target)

### Commands

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

# Verify formatting, lints, and tests
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked

# Build optimized release binary
cargo build --release --locked
```

Standalone executable output: `target\release\battery-dock.exe` (pass `--settings` / `-s` to open settings directly).

---

## 📄 License

Distributed under the **MIT License**. See [LICENSE](LICENSE) for details.

<div align="center">

**If BatteryDock helps keep your desk clean and your headset charged, consider giving the repo a ⭐!**

</div>
