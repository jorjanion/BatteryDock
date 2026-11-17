<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock Logo" width="148" style="border-radius: 24px;" />

# BatteryDock

**Alle batterijniveaus van je draadloze randapparatuur in één strakke native Windows-taakbalkpil. Zonder bloatware.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Download v1.0.0](https://github.com/jorjanion/BatteryDock/releases/latest) · [Bug melden](https://github.com/jorjanion/BatteryDock/issues/new) · [Apparaat aanvragen](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md) · [Changelog](../../CHANGELOG.md)

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

Als je een draadloze muis, headset en Bluetooth-controller van verschillende merken gebruikt, heb je op Windows vaak meerdere zware achtergrondapps nodig om de batterij te zien.

**BatteryDock** vervangt dat door één ~4 MB native Rust-app op je Windows 10/11 taakbalk. Geen Electron, geen telemetrie, geen accounts.

---

## Voorbeeld

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-single.png" alt="BatteryDock single peripheral pill" /><br /><br />
        <sub><b>Eén apparaat</b>: Scherp vectoricoon, gekleurde batterijbalk en percentage direct op de taakbalk.</sub>
      </td>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-multi.png" alt="BatteryDock multiple peripherals pill" /><br /><br />
        <sub><b>Meerdere apparaten</b>: Verander de volgorde, geef namen of verberg apparaten.</sub>
      </td>
    </tr>
  </table>
  <br />
  <table>
    <tr>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-peripherals.jpg" alt="BatteryDock Peripherals dashboard v1.0.0" /><br />
        <sub><b>Apparaten</b>: Geschatte batterijduur, 24-uurs grafiek, bijnamen en meldingen per apparaat.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-alerts.jpg" alt="BatteryDock Alerts and Audio settings v1.0.0" /><br />
        <sub><b>Meldingen & Geluid</b>: Meldingen bij laag, kritiek en 100% vol niveau met geluidstest.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-customization.jpg" alt="BatteryDock Customization settings v1.0.0" /><br />
        <sub><b>Aanpassing</b>: Liquid Glass / Donker / Licht / Systeem thema, Game HUD, sneltoets en 21 talen.</sub>
      </td>
    </tr>
  </table>
</div>

---

## Functies

### Minimalistische taakbalkpil
- **4 thema's:** Kies **Liquid Glass** (ultratransparant), **Donker**, **Licht** of **Systeem** (volgt Windows).
- **Geen rommel:** Linksklik opent Instellingen, rechtsklik opent het snelmenu, of sleep naar elke monitor.
- **Slim taakbalkgedrag:** Blijft zichtbaar boven Start en Zoeken en verbergt automatisch bij volledig scherm.
- **Eigen indeling:** Verander de volgorde met één klik, hernoem of verberg apparaten en schakel het percentage in of uit.

### Optionele Game- & Volledig scherm-overlay (HUD)
- **Standaard uitgeschakeld** zodat films en YouTube ongestoord blijven.
- Ingeschakeld verschijnt het batterijniveau in een hoek tijdens randloze games op volledig scherm.
- **Overal te plaatsen:** 4 hoeken met instelbare marge of vrije X/Y-positie (0% tot 100%) met live voorbeeld.
- **Click-through modus:** Muiskliks gaan rechtstreeks door de HUD naar je game.
- **Globale sneltoets:** Stel elke toetscombinatie in (standaard `Alt + B`) om de HUD direct te schakelen.

### Slimme telemetrie & meldingen
- **24-uurs batterijgeschiedenis:** Elke kaart toont een lokale grafiek van laden en ontladen.
- **Batterijduur & slaapstand:** Schat de resterende tijd en herkent apparaten in slaapstand.
- **Laag, kritiek en 100% vol meldingen:** Drempels per apparaat, dempen, zacht geluid en Windows-meldingen.

### 21 talen ingebouwd
- Herkent automatisch je Windows-taal bij eerste start (met Engels als standaard fallback).
- Laadt native Windows-lettertypen op aanvraag voor scherpe weergave van elk schrift.

---

## Ondersteunde hardware

BatteryDock praat direct met je hardware via HID, WinRT en Bluetooth zonder fabrikantsoftware.

| Categorie | Protocol / Methode | Ondersteunde voorbeelden |
| :--- | :--- | :--- |
| **Logitech Wireless** | Direct HID++ 2.0 & G HUB IPC | Lightspeed headsets (G733, G535, G935, G435), mice (G PRO, Superlight, G502, MX Master), keyboards (G915) |
| **Razer Wireless** | Native Razer HyperSpeed HID | Viper, DeathAdder, Basilisk, Naga, Cobra, BlackWidow, Barracuda, BlackShark |
| **SteelSeries, Corsair & Glorious** | Native USB-HID | Arctis / Aerox / Rival, Corsair wireless mice & headsets, Glorious Model O / D |
| **HyperX & Astro Gaming** | Native Wireless Audio HID | HyperX Cloud II / III / Alpha / Flight / Stinger Wireless, Astro A50 / A30 |
| **Bluetooth Classic & BLE** | Windows PnP (`DEVPKEY_Bluetooth_Battery`) & BLE GATT | Sony WH/WF series (WH-1000XM4/XM5, WH-CH720N), JBL (Tune 770NC, Live), Bose, Sennheiser, AirPods, BT mice & keyboards |
| **Xbox Controllers** | XInput & Windows Gaming Input (WinRT) | Xbox Series X/S, Xbox One, Xbox Elite Series 2 |
| **PlayStation Controllers** | Direct Sony USB & Bluetooth HID | DualSense, DualSense Edge, DualShock 4 (USB, BT, & Wireless Adapter) |
| **Nintendo Controllers** | Direct Nintendo HID | Switch Pro Controller, Joy-Con L/R |

---

## Download & Installatie

Download de nieuwste versie op de **[Releases-pagina](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Download **`BatteryDock-Setup-v1.0.0.exe`** (installer) of **`BatteryDock-Windows-Portable.zip`** (portable).
2. Start de app en schakel optioneel **BatteryDock starten met Windows** in.
3. De pil verschijnt naast de klok met automatische GitHub-updates.

### Windows SmartScreen-melding
Omdat BatteryDock gratis en open source is zonder betaald certificaat, kan SmartScreen een melding tonen. Klik op **Meer informatie** en **Toch uitvoeren**. Alle builds worden openbaar gemaakt via [`.github/workflows/release.yml`](../../.github/workflows/release.yml).

---

## Configuratie

Instellingen staan in `%APPDATA%\jorjanion\BatteryDock\config\config.toml` (geschiedenis in `history.toml`) en worden direct toegepast.

---

## Zelf bouwen

Vereisten: Windows 10/11 en [Rust](https://rustup.rs/) (stabiele MSVC-toolchain).

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock
cargo test --locked
cargo build --release --locked
.\target\release\battery-dock.exe
```

---

## Bijdragen, Issues & Pull Requests

BatteryDock is open source en community-gedreven. Bugmeldingen, vertalingen, apparaataanvragen en PR's zijn altijd welkom.

- **Apparaat niet zichtbaar?** Open een **[apparaataanvraag](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)**. Gebruik **Instellingen > Aanpassing > Hardware-diagnose kopiëren** en plak het geanonimiseerde rapport in je issue.
- **Bug of idee?** Open een **[Issue](https://github.com/jorjanion/BatteryDock/issues)** met je Windows-versie en stappen.
- **Code bijdragen?** Pull Requests zijn welkom. Draai `cargo test --locked` en `cargo clippy --locked` voordat je een PR opent.

---

## Licentie

MIT-licentie. Zie [LICENSE](../../LICENSE).
