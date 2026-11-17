# Changelog

All notable changes to **BatteryDock** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.0] - 2026-11-17

BatteryDock 1.0.0 is our first major milestone release. It brings a redesigned taskbar pill and Fluent settings dashboard, a massive expansion of native wireless hardware support, an optional gaming HUD overlay with global hotkeys, 24-hour battery trend sparklines, and full localization in 21 languages.

### Added
- **Major Hardware Expansion**: Native battery detection for Razer HyperSpeed mice and headsets, SteelSeries Arctis and Aerox, Corsair wireless receivers, Glorious mice, HyperX headsets, Astro Gaming headsets, Nintendo Switch Pro Controller and Joy-Con, Sony DualShock 4 wireless adapters, and Windows WinRT Xbox controller precision polling.
- **Classic Bluetooth HFP Headset Support**: Added direct `DEVPKEY_Bluetooth_Battery` querying on present Windows PnP device nodes so Classic Bluetooth Hands-Free headsets (such as Sony WH-CH720N, WH-1000XM series, and JBL Tune 770NC) report accurate battery levels immediately and stay visible alongside USB dongles (fixes #1).
- **Optional Gaming and Fullscreen HUD Overlay**: Disabled by default. Enable it in Settings to keep a compact, click-through (`MousePassthrough`) battery readout in any screen corner or custom X/Y coordinate while gaming in borderless or fullscreen mode, complete with a live preview toggle.
- **Configurable Global Overlay Hotkey**: Toggle the gaming HUD overlay on or off mid-game from any app using a customizable global keyboard shortcut (default `Alt + B`).
- **Four Pill Themes**: Choose between Liquid Glass (ultra-transparent), Dark (frosted acrylic), Light (frosted white acrylic), or System (automatically follows your Windows Dark/Light theme).
- **21 Built-In Languages with System Auto-Detection**: Full native localization for English, Spanish, French, German, Portuguese (Brazil), Italian, Dutch, Polish, Czech, Swedish, Russian, Ukrainian, Turkish, Arabic, Hindi, Indonesian, Vietnamese, Japanese, Korean, Simplified Chinese, and Traditional Chinese. Automatically matches your Windows UI language on startup (falling back to English) with native Windows font fallbacks for CJK, Arabic, Devanagari, and Cyrillic scripts.
- **24-Hour Battery History Sparklines**: Persistent per-device battery trend charts inside each Settings card so you can see how fast a peripheral drains over time.
- **100% Full-Charge Alert**: Optional desktop notification when a charging peripheral reaches 100%, so you can unplug without overcharging.
- **Device Reordering and Per-Device Controls**: Reorder peripherals on the taskbar pill with up/down buttons, assign custom nicknames, set custom low-battery thresholds, mute alerts, or hide individual devices from the pill.
- **One-Click Anonymized Hardware Diagnostics**: Copy an anonymized snapshot of connected HID, Bluetooth, and PnP hardware directly from Settings to make device support requests and bug reports effortless.

### Improved
- **Minimalist Pill and Redesigned Settings Window**: Removed the gear icon from the taskbar pill (left-click the pill directly to open Settings), organized Settings into three clean tabs (Peripherals, Alerts & Audio, Customization), equalized bento card widths, and pinned the Close Settings button to a dedicated bottom footer.
- **Smarter Fullscreen Video Detection**: Automatically hides the taskbar pill during fullscreen browser videos (such as YouTube) and media players when the gaming HUD overlay is off.
- **Unified Display Options**: Removed the redundant compact mode toggle and intrusive hover tooltip overlay in favor of a cleaner pill with a single percentage text toggle.

### Fixed
- **Bluetooth Hands-Free Battery Detection (#1)**: Resolved an issue where Classic Bluetooth headsets (such as Sony WH-CH720N and JBL Tune 770NC) were missed or disappeared because Windows exposes their battery via `DEVPKEY_Bluetooth_Battery` on PnP nodes rather than `System.Devices.BatteryLife`.
- **Missing Glyph Rendering**: Replaced raw emoji characters in UI labels with clean native icons and typography to prevent missing-glyph ("tofu") boxes across Windows configurations.

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
