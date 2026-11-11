<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock Logo" width="160" style="border-radius: 24px;" />

# BatteryDock

**Eine schlanke, native Rust-Batterieanzeige direkt in der Windows 10/11-Taskleiste — ganz ohne Hersteller-Bloatware.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Neuesten Installer herunterladen](https://github.com/jorjanion/BatteryDock/releases/latest) · [Fehler melden](https://github.com/jorjanion/BatteryDock/issues) · [Gerät anfragen](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)

<br />

<sub>
<a href="../../README.md">English</a> ·
<a href="README.es.md">Español</a> ·
<a href="README.fr.md">Français</a> ·
<b>Deutsch</b> ·
<a href="README.pt-BR.md">Português</a> ·
<a href="README.it.md">Italiano</a> ·
<a href="README.nl.md">Nederlands</a> ·
<a href="README.pl.md">Polski</a> ·
<a href="README.sv.md">Svenska</a> ·
<a href="README.cs.md">Čeština</a> ·
<a href="README.ru.md">Русский</a> ·
<a href="README.uk.md">Українська</a> ·
<a href="README.tr.md">Türkçe</a> ·
<a href="README.ar.md">العربية</a> ·
<a href="README.hi.md">हिन्दी</a> ·
<a href="README.id.md">Bahasa Indonesia</a> ·
<a href="README.vi.md">Tiếng Việt</a> ·
<a href="README.ja.md">日本語</a> ·
<a href="README.ko.md">한국어</a> ·
<a href="README.zh-CN.md">简体中文</a> ·
<a href="README.zh-TW.md">繁體中文</a>
</sub>

</div>

---

## 📸 Vorschau

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="42%">
        <img src="../screenshots/dock-closeup.png" alt="Nahaufnahme der BatteryDock-Kapsel mit 92 % Akkustand" /><br /><br />
        <sub><b>Taskleisten-Kapsel</b> — Direkt in die Windows-Taskleiste integriert, mit Per-Monitor-DPI-Skalierung, Farbstufen und live ⚡ Ladeanzeige.</sub>
      </td>
      <td align="center" valign="middle" width="58%">
        <img src="../screenshots/settings.png" alt="BatteryDock Einstellungsfenster v0.1.11" width="340" /><br />
        <sub><b>Einstellungen</b> — Warnschwellen, Tonvorschau, Positionierung und Liste verbundener Geräte.</sub>
      </td>
    </tr>
  </table>
</div>

---

## ⚡ Warum BatteryDock?

Die meisten kabellosen Peripheriegeräte verstecken ihren Akkustand in überladenen Hersteller-Suiten mit Dutzenden Hintergrunddiensten. **BatteryDock** zeigt alles sauber in einer kompakten Taskleisten-Kapsel an:

- **Keine Bloatware:** Eine einzelne **~4 MB** große native Rust-Datei (kein Electron, keine Telemetrie, ~77 MB privater Speicher live unter Windows 11).
- **Direkte Hardware-Abfrage:** Liest **Logitech HID++ 2.0**, **Windows Bluetooth / BLE GATT**, **XInput** und **Sony DualSense / DualShock 4 HID** direkt aus — Logitech-Geräte funktionieren auch ohne gestartetes G HUB (G HUB IPC wird zusätzlich unterstützt).
- **Intelligente Vollbild-Regeln:** Blendet automatisch aus, sobald Vollbildspiele, Vollbildvideos oder eine ausgeblendete Taskleiste aktiv sind — und bleibt bei `Win`-Taste, Startmenü, Suche, Widgets oder Desktop-Klicks zuverlässig sichtbar.
- **Live-Ladesymbol:** Markiert ladende Geräte in **Cyan** mit einem **⚡ Blitz-Symbol**.
- **Zuverlässige Hintergrund-Warnungen:** Töne und Windows-Benachrichtigungen werden auch dann ausgelöst, wenn die Kapsel im Vollbild ausgeblendet ist.

---

## 📥 Download & Installation

Lade die neueste Version von der **[Releases-Seite](https://github.com/jorjanion/BatteryDock/releases/latest)** herunter:

1. Lade **`BatteryDock-Setup-v0.1.11.exe`** (~4,4 MB) herunter — oder `BatteryDock-Windows-Portable.zip` für die portable Nutzung.
2. Starte den Installer und aktiviere optional **"Start BatteryDock when Windows boots"**.
3. Die Kapsel erscheint neben dem System-Tray und der Uhr.

### 🛡️ Hinweis zu Windows SmartScreen
Da BatteryDock eine kostenlose Open-Source-App ohne teures EV-Code-Signing-Zertifikat ist, kann beim ersten Start die Meldung *"Der Computer wurde durch Windows geschützt"* erscheinen (**Weitere Informationen → Trotzdem ausführen**).
- Alle Releases werden öffentlich auf sauberen GitHub-Actions-Runnern in [`.github/workflows/release.yml`](../../.github/workflows/release.yml) gebaut.
- Du kannst den gesamten Rust-Code prüfen und das Programm in unter einer Minute [selbst kompilieren](#build-from-source).

---

## 🎧 Unterstützte Hardware

| Kategorie | Protokoll / Quelle | Beispiele & Hinweise |
| :--- | :--- | :--- |
| **Logitech Wireless** | Direktes HID++ 2.0 (`0x1004` / `0x1000`) & G HUB IPC | Lightspeed-Headsets (G733, G535, G935), Mäuse (G PRO, Superlight, G502), Tastaturen |
| **Bluetooth & BLE** | Windows WinRT PnP + BLE GATT (`0x180F` / `0x2A19`) | Kopfhörer (Sony WH/WF, Bose, AirPods), BT-Mäuse & -Tastaturen |
| **Xbox-Controller** | Windows `XInput` | Xbox Wireless Controller (Series X\|S, One, 360) inkl. Kabel-Ladeerkennung |
| **PlayStation** | Sony HID (`VID 0x054C`) USB & Bluetooth | DualSense & DualShock 4 (Multi-Controller-Support + Ladestatus) |

**Deine Marke fehlt (Razer, Corsair, SteelSeries, Keychron usw.)?**
👉 **[Geräte-Unterstützung anfragen](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** — Pull Requests für weitere HID-/Bluetooth-Profile sind ebenfalls willkommen!

---

## 🎯 Bedienung & Statusfarben

### Statusfarben
- **Grün:** Ausreichender Akkustand (über dem niedrigen Schwellenwert)
- **Gelb/Amber:** Niedriger Akkustand (konfigurierbar, Standard `20%`)
- **Rot (pulsierend):** Kritischer Akkustand (Standard `10%`)
- **Cyan + ⚡:** Gerät lädt gerade

### Interaktion
- **Linksklick** auf Kapsel oder **`⚙`** (bzw. Tray-Icon): Öffnet **BatteryDock Settings** (`BatteryDock.exe --settings`).
- **Rechtsklick** auf Kapsel oder Tray-Icon: Natives Kontextmenü (**Settings**, **Refresh Devices**, **Reset to Taskbar Clock**, **Restart BatteryDock**, **Exit BatteryDock**).
- **Klicken & Ziehen (> 5px):** Kapsel frei platzieren (auch auf Zweitmonitoren); Rechtsklick → **Reset to Taskbar Clock** setzt sie zurück.

---

## ⚙️ Konfiguration

Einstellungen werden unter `%APPDATA%\jorjanion\BatteryDock\config\config.toml` gespeichert:

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

<a id="build-from-source"></a>

## 🛠️ Aus dem Quellcode kompilieren

### Voraussetzungen
- [Rust Toolchain](https://rustup.rs/) (stable)
- Windows 10 oder Windows 11 (MSVC-Target)

### Befehle

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

# Formatierung, Clippy & Tests prüfen
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked

# Optimiertes Release-Binary bauen
cargo build --release --locked
```

Erzeugte EXE: `target\release\battery-dock.exe` (mit `--settings` / `-s` für das Einstellungsfenster).

---

## 📄 Lizenz

Veröffentlicht unter der **MIT-Lizenz**. Details siehe [LICENSE](../../LICENSE).

<div align="center">

**Wenn dir BatteryDock gefällt, lass dem Repository gerne einen ⭐ da!**

</div>
