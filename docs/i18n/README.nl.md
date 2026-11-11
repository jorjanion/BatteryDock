<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock Logo" width="160" style="border-radius: 24px;" />

# BatteryDock

**Een strakke, native Rust-batterijindicator rechtstreeks in de Windows 10/11-taakbalk — zonder zware fabrikantensoftware.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Nieuwste installer downloaden](https://github.com/jorjanion/BatteryDock/releases/latest) · [Bug melden](https://github.com/jorjanion/BatteryDock/issues) · [Apparaat aanvragen](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)

<br />

<sub>
<a href="../../README.md">English</a> ·
<a href="README.es.md">Español</a> ·
<a href="README.fr.md">Français</a> ·
<a href="README.de.md">Deutsch</a> ·
<a href="README.pt-BR.md">Português</a> ·
<a href="README.it.md">Italiano</a> ·
<b>Nederlands</b> ·
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

## 📸 Voorbeeld

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="42%">
        <img src="../screenshots/dock-closeup.png" alt="Close-up van BatteryDock-capsule met koptelefoon op 92%" /><br /><br />
        <sub><b>Taakbalkcapsule</b> — Direct in de Windows-taakbalk geïntegreerd met DPI-schaling per monitor, kleurcodering en live ⚡ oplaadicoon.</sub>
      </td>
      <td align="center" valign="middle" width="58%">
        <img src="../screenshots/settings.png" alt="BatteryDock Instellingenvenster v0.1.11" width="340" /><br />
        <sub><b>Zelfstandige instellingen</b> — Waarschuwingsdrempels, geluids voorbeeld, posities en lijst met verbonden apparaten.</sub>
      </td>
    </tr>
  </table>
</div>

---

## ⚡ Waarom BatteryDock?

Veel draadloze randapparatuur verbergt het batterijpercentage in logge softwaresuites met tientallen achtergrondservices. **BatteryDock** zet al je apparaten in één strakke taakbalkcapsule:

- **Geen bloatware:** Eén enkel **~4 MB** native Rust-bestand (geen Electron, geen telemetrie, ~77 MB privé-geheugen live gemeten op Windows 11).
- **Directe hardware-uitlezing:** Leest rechtstreeks **Logitech HID++ 2.0**, **Windows Bluetooth / BLE GATT**, **XInput** en **Sony DualSense / DualShock 4 HID** uit — Logitech-apparaten werken ook zonder G HUB (G HUB IPC wordt ondersteund als het draait).
- **Slimme fullscreen-regels:** Verbergt automatisch bij games of video's op volledig scherm en automatisch verbergende taakbalken, en blijft stabiel zichtbaar bij de `Win`-toets, Start, Zoeken, Widgets of bureaubladkliks.
- **Live oplaadindicator:** Kleurt apparaten **cyaan** met een **⚡ bliksem-icoon** zodra ze opladen.
- **Achtergrondmeldingen werken altijd:** Geluidssignalen en bureaubladmeldingen blijven werken vanuit de achtergrondthread, zelfs tijdens fullscreen-games.

---

## 📥 Downloaden en installeren

Download de nieuwste release van de **[Releases-pagina](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Download **`BatteryDock-Setup-v0.1.11.exe`** (~4,4 MB) — of `BatteryDock-Windows-Portable.zip` voor gebruik zonder installatie.
2. Start de installer en vink optioneel **"Start BatteryDock when Windows boots"** aan.
3. De capsule verschijnt direct naast het systeemvak en de klok.

### 🛡️ Windows SmartScreen-melding
Omdat BatteryDock gratis open-source is zonder betaald EV-certificaat, kan Windows SmartScreen *"Windows heeft uw pc beschermd"* tonen (**Meer informatie → Toch uitvoeren**).
- Elke release wordt openbaar gecompileerd op GitHub Actions-runners in [`.github/workflows/release.yml`](../../.github/workflows/release.yml).
- Je kunt ook alle code inzien en binnen een minuut [zelf compileren](#build-from-source).

---

## 🎧 Ondersteunde hardware

| Categorie | Protocol / Bron | Voorbeelden en opmerkingen |
| :--- | :--- | :--- |
| **Logitech Draadloos** | Direct HID++ 2.0 (`0x1004` / `0x1000`) & G HUB IPC | Lightspeed-headsets (G733, G535, G935), muizen (G PRO, Superlight, G502), toetsenborden |
| **Bluetooth & BLE** | Windows WinRT PnP + BLE GATT (`0x180F` / `0x2A19`) | Koptelefoons (Sony WH/WF, Bose, AirPods), BT-muizen & -toetsenborden |
| **Xbox-controllers** | Windows `XInput` | Xbox Wireless Controllers (Series X\|S, One, 360) incl. bedraad opladen |
| **PlayStation** | Sony HID (`VID 0x054C`) USB & Bluetooth | DualSense & DualShock 4 (meerdere controllers + oplaadstatus) |

**Mist jouw merk (Razer, Corsair, SteelSeries, Keychron, enz.)?**
👉 **[Vraag apparaatondersteuning aan](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** — Pull Requests zijn ook welkom!

---

## 🎯 Bediening en indicatoren

### Statuskleuren
- **Groen:** Gezond batterijniveau (boven lage drempel)
- **Amber:** Lage batterij (instelbaar, standaard `20%`)
- **Rood (pulserend):** Kritiek niveau (standaard `10%`)
- **Cyaan + ⚡:** Apparaat laadt op

### Interactie
- **Linkermuisklik** op capsule of **`⚙`** (of tray-icoon): Opent **BatteryDock Settings** (`BatteryDock.exe --settings`).
- **Rechtermuisklik** op capsule of tray-icoon: Opent native menu (**Settings**, **Refresh Devices**, **Reset to Taskbar Clock**, **Restart BatteryDock**, **Exit BatteryDock**).
- **Slepen (> 5px):** Verplaats overal (ook op andere schermen); rechtermuisklik → **Reset to Taskbar Clock** zet hem terug.

---

## ⚙️ Configuratie

Instellingen worden opgeslagen in `%APPDATA%\jorjanion\BatteryDock\config\config.toml`:

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

## 🛠️ Bouwen vanuit broncode

### Vereisten
- [Rust-toolchain](https://rustup.rs/) (stable)
- Windows 10 of Windows 11 (MSVC-target)

### Commando's

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

# Opmaak, clippy en tests controleren
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked

# Geoptimaliseerde release bouwen
cargo build --release --locked
```

Uitvoerbestand: `target\release\battery-dock.exe` (gebruik `--settings` / `-s` om direct instellingen te openen).

---

## 📄 Licentie

Uitgebracht onder de **MIT-licentie**. Zie [LICENSE](../../LICENSE).

<div align="center">

**Helpt BatteryDock je? Geef de repository gerust een ⭐!**

</div>
