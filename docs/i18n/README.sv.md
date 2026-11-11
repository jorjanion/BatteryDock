<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock Logotyp" width="160" style="border-radius: 24px;" />

# BatteryDock

**En stilren, inbyggd batteriindikator i Rust direkt i Windows 10/11-aktivitetsfältet — utan tung tillverkarprogramvara.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Ladda ner senaste installationsprogrammet](https://github.com/jorjanion/BatteryDock/releases/latest) · [Rapportera bugg](https://github.com/jorjanion/BatteryDock/issues) · [Efterfråga enhetsstöd](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)

<br />

<sub>
<a href="../../README.md">English</a> ·
<a href="README.es.md">Español</a> ·
<a href="README.fr.md">Français</a> ·
<a href="README.de.md">Deutsch</a> ·
<a href="README.pt-BR.md">Português</a> ·
<a href="README.it.md">Italiano</a> ·
<a href="README.nl.md">Nederlands</a> ·
<a href="README.pl.md">Polski</a> ·
<b>Svenska</b> ·
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

## 📸 Förhandsvisning

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="42%">
        <img src="../screenshots/dock-closeup.png" alt="Närbild på BatteryDock-kapsel med 92% headsetbatteri" /><br /><br />
        <sub><b>Aktivitetsfältskapsel</b> — Integrerad direkt i Windows aktivitetsfält med Per-Monitor DPI-skalning, statusfärger och ⚡ laddningsikon.</sub>
      </td>
      <td align="center" valign="middle" width="58%">
        <img src="../screenshots/settings.png" alt="Inställningsfönster för BatteryDock v0.1.11" width="340" /><br />
        <sub><b>Fristående inställningar</b> — Varningsgränser, ljudtest, placering och anslutna enheter.</sub>
      </td>
    </tr>
  </table>
</div>

---

## ⚡ Varför BatteryDock?

De flesta trådlösa tillbehör låser batterinivån i tunga tillverkarprogram med mängder av bakgrundstjänster. **BatteryDock** visar allt i en ren kapsel i aktivitetsfältet:

- **Ingen bloatware:** En enda **~4 MB** native Rust-binär (ingen Electron, ingen telemetri, ~77 MB privat minne uppmätt live på Windows 11).
- **Direkt hårdvaruläsning:** Läser **Logitech HID++ 2.0**, **Windows Bluetooth / BLE GATT**, **XInput** och **Sony DualSense / DualShock 4 HID** direkt — Logitech fungerar även utan att öppna G HUB.
- **Smart helskärmshantering:** Döljs automatiskt i helskärmsspel, helskärmsvideo eller när aktivitetsfältet är auto-dolt, men förblir synlig när du trycker på `Win`-tangenten, öppnar Start/Widgets eller klickar på skrivbordet.
- **Live-laddningsindikator:** Visar enheter i **cyan** med **⚡ blixtsymbol** under laddning.
- **Pålitliga bakgrundsvarningar:** Ljudsignaler och skrivbordsnotiser triggas från bakgrundstråden även när kapseln är dold i helskärm.

---

## 📥 Nedladdning & Installation

Hämta senaste versionen från **[Releases-sidan](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Ladda ner **`BatteryDock-Setup-v0.1.11.exe`** (~4,4 MB) — eller `BatteryDock-Windows-Portable.zip` för portabel användning.
2. Kör installationsprogrammet och bocka valfritt i **"Start BatteryDock when Windows boots"**.
3. Kapseln dyker upp bredvid klockan i aktivitetsfältet.

### 🛡️ Information om Windows SmartScreen
Eftersom BatteryDock är gratis öppen källkod utan dyrt EV-kodsigneringscertifikat kan SmartScreen visa *"Windows skyddade din dator"* första gången (**Mer information → Kör ändå**).
- Varje release byggs publikt på GitHub Actions-runners via [`.github/workflows/release.yml`](../../.github/workflows/release.yml).
- Du kan granska hela Rust-koden och [bygga själv](#build-from-source) på under en minut.

---

## 🎧 Stödd hårdvara

| Kategori | Protokoll / Källa | Exempel & anteckningar |
| :--- | :--- | :--- |
| **Logitech Trådlöst** | Direkt HID++ 2.0 (`0x1004` / `0x1000`) & G HUB IPC | Lightspeed-headset (G733, G535, G935), möss (G PRO, Superlight, G502), tangentbord |
| **Bluetooth & BLE** | Windows WinRT PnP + BLE GATT (`0x180F` / `0x2A19`) | Hörlurar (Sony WH/WF, Bose, AirPods), BT-möss & tangentbord |
| **Xbox-handkontroller** | Windows `XInput` | Xbox Wireless Controllers (Series X\|S, One, 360) inkl. trådad laddningsdetektering |
| **PlayStation** | Sony HID (`VID 0x054C`) USB & Bluetooth | DualSense & DualShock 4 (flera kontroller + laddningsstatus) |

**Saknas ditt märke (Razer, Corsair, SteelSeries, Keychron osv.)?**
👉 **[Begär enhetsstöd](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** — Pull Requests välkomnas också!

---

## 🎯 Kontroller & Indikatorer

### Statusfärger
- **Grön:** Bra batteri (över låg gräns)
- **Gul/Amber:** Låg batterinivå (standard `20%`)
- **Röd (pulserande):** Kritisk nivå (standard `10%`)
- **Cyan + ⚡:** Enheten laddas nu

### Interaktion
- **Vänsterklicka** på kapseln eller **`⚙`** (eller ikonen i meddelandefältet): Öppnar **BatteryDock Settings** (`BatteryDock.exe --settings`).
- **Högerklicka** på kapseln eller systemfältsikonen: Öppnar meny (**Settings**, **Refresh Devices**, **Reset to Taskbar Clock**, **Restart BatteryDock**, **Exit BatteryDock**).
- **Klicka & dra (> 5px):** Flytta fritt (även till andra skärmar); högerklick → **Reset to Taskbar Clock** återställer till klockan.

---

## ⚙️ Konfiguration

Inställningar sparas i `%APPDATA%\jorjanion\BatteryDock\config\config.toml`:

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

## 🛠️ Bygg från källkod

### Förutsättningar
- [Rust-verktygskedja](https://rustup.rs/) (stable)
- Windows 10 eller Windows 11 (MSVC)

### Kommandon

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

# Kontrollera formatering, clippy och tester
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked

# Bygg optimerad release-binär
cargo build --release --locked
```

Körbar fil skapas i `target\release\battery-dock.exe` (använd `--settings` / `-s` för inställningar direkt).

---

## 📄 Licens

Släppt under **MIT-licensen**. Se [LICENSE](../../LICENSE).

<div align="center">

**Gillar du BatteryDock? Ge gärna repot en ⭐!**

</div>
