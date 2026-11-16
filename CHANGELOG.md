# Changelog

All notable changes to **BatteryDock** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.12] - 2026-11-16

### 🚀 Added
- **Automatic & Manual GitHub Updates**: BatteryDock now checks for new GitHub releases automatically in the background, with a "Check for updates" button in Settings and a toggle to enable/disable auto-updating.
- **Multilingual Support (Spanish & English)**: Complete Spanish localization for the Settings window, context menu, tooltips, and desktop notifications, with an in-app language selector.
- **Empty State Indicator**: When no wireless devices or gamepads are connected, the pill displays a clear *"No devices connected"* (*"No hay dispositivos conectados"*) prompt instead of disappearing or displaying a confusing placeholder.

### ⚡ Performance & Polish
- **Faster Default Polling (4 Seconds)**: Decreased default polling interval from 15s to 4s (`poll_interval_secs = 4`), delivering near-instant detection when plugging in or connecting peripherals with virtually 0.0% CPU overhead.

### 🐛 Fixed
- **Windows 11 Search & Taskbar Occlusion Bug**: Fixed an issue where opening the Start Menu, Windows Search (`Win + S`), or clicking the taskbar would cause BatteryDock to be occluded and hidden behind `Shell_TrayWnd`. Implemented Win32 owned-window binding (`GWLP_HWNDPARENT` set to `Shell_TrayWnd`) so the DWM compositing manager guarantees BatteryDock is always drawn in front of the taskbar.
- **Z-Order Self-Healing Engine**: Replaced destructive `HWND_NOTOPMOST` loops with an upward-scanning `GW_HWNDPREV` traversal to detect and instantly recover Z-order if shell flyouts or settings windows temporarily take focus.
- **Settings Launch Reliability**: Switched settings window launcher from shell verb execution to direct process spawning with automatic foreground activation if already running.

---

## [0.1.11] - 2026-11-11

### 🐛 Fixed
- **WinRT Bluetooth Queries**: Fixed device enumeration syntax for Bluetooth battery and connection state properties.
- **Background Alerts**: Guaranteed low and critical battery alerts fire even when the pill is temporarily hidden in fullscreen games.
- **DualSense HID**: Enhanced Sony DualSense battery and charging reporting over both USB and Bluetooth.

---

## [0.1.10] - 2026-11-10

### 🚀 Initial Public Release
- Ultra-lightweight native Rust battery capsule on the Windows taskbar.
- Direct hardware reading for Logitech HID++ 2.0, Windows Bluetooth / BLE GATT, XInput, and Sony DualSense.
- Per-Monitor DPI scaling and customizable audio/visual battery alerts.
