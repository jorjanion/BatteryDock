<div align="center">

<img src="assets/logo.png" alt="BatteryDock Logo" width="180" style="border-radius: 24px;" />

# BatteryDock

**Ultra-lightweight Windows taskbar dock for peripheral battery levels with custom alerts.**

[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Made%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

</div>

---

## ⚡ Overview

**BatteryDock** is a standalone open-source Windows utility built in Rust that displays a sleek, minimalist status dock directly **on your Windows taskbar** showing real-time battery levels for all connected wireless peripherals (headphones, mice, keyboards, gamepads, VR headsets, and more).

Unlike heavy manufacturer suites, BatteryDock runs with **zero bloat** (~15 MB RAM, 0% idle CPU) and directly communicates with your hardware without requiring proprietary vendor software running in the background.

---

## 📥 Download & Installation

Get the latest installer from the [Releases Page](https://github.com/jorjanion/BatteryDock/releases/latest):

1. **[Download BatteryDock-Setup-v0.1.6.exe (Latest)](https://github.com/jorjanion/BatteryDock/releases/download/v0.1.6/BatteryDock-Setup-v0.1.6.exe)**
2. Run the installer:
   - Select your language (English or Spanish).
   - Check **"Start BatteryDock automatically when Windows starts"** if you want it to run seamlessly in the background on startup.
   - Choose whether you want a Desktop shortcut.
3. Click **Install** and **Finish** to launch BatteryDock.

*(A portable zero-install `.zip` is also available on the releases page).*

---

## ✨ Features

- 🔋 **Taskbar Integrated:** Embedded directly on the Windows taskbar as a modern frosted acrylic capsule.
- 🎯 **Flexible Placement:** Draggable anywhere on screen, with one-click presets (left of the clock/system tray, taskbar left, taskbar right).
- 🎧 **Broad Hardware Support:**
  - **Bluetooth LE Peripherals:** Headphones (Sony WH/WF, Bose, AirPods, Sennheiser), mice, and keyboards via Windows Bluetooth GATT Battery Service.
  - **Logitech Peripherals:** Lightspeed wireless dongles, headsets (such as G733, G535, G935), mice (G Pro, Superlight, G502), and keyboards. Supports direct HID++ and local IPC integration.
  - **Gamepads & Controllers:** Xbox Wireless Controllers (Series X/S, One, 360 via XInput) and PlayStation DualSense / DualShock 4 (via HID).
- 🔔 **Smart Alerts:** Configurable low battery and critical battery thresholds with gentle, non-startling audio chimes and native desktop notifications.
- 🛠️ **System Tray Companion:** Quick access menu to refresh devices, customize settings, or launch on Windows startup.
- 🚀 **Extreme Efficiency:** Written from scratch in Rust using reactive rendering for minimal resource consumption.

---

## 📸 Placement & Visuals

The dock automatically matches taskbar height and scales crisply across all display resolutions and DPI scalings:

```
[ 🎧 66% ]  [ 🖱️ 85% ]  [ 🎮 92% ]  ⚙
```

- **Green:** Healthy battery (> 50%)
- **Amber:** Low battery (configurable threshold, default 20%)
- **Red (Pulsing):** Critical battery (default 10%)
- **Cyan + ⚡:** Currently charging

---

## 🛠️ Building from Source

### Prerequisites
- [Rust toolchain](https://rustup.rs/) (1.75+ recommended)
- Windows 10 or Windows 11

### Build Steps

```bash
# Clone the repository
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

# Run automated tests
cargo test

# Build optimized release binary
cargo build --release
```

The resulting standalone executable will be located at:
```
target/release/battery-dock.exe
```

---

## ⚙️ Configuration

Settings are saved in `%APPDATA%\BatteryDock\config.toml`:

```toml
low_battery_threshold = 20
critical_battery_threshold = 10
audio_alert_enabled = true
visual_alert_enabled = true
position_mode = "TaskbarLeftOfTray"
snap_to_taskbar = true
start_with_windows = false
poll_interval_secs = 15
show_percentage_text = true
```

You can adjust all options via the GUI settings window (double-click the dock or click the ⚙ icon).

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
