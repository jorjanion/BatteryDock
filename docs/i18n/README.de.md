<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock Logo" width="148" style="border-radius: 24px;" />

# BatteryDock

**Alle Akkustände deiner kabellosen Geräte in einer nativen Windows-Taskleisten-Leiste. Ohne Hersteller-Bloatware.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[v1.0.0 herunterladen](https://github.com/jorjanion/BatteryDock/releases/latest) · [Fehler melden](https://github.com/jorjanion/BatteryDock/issues/new) · [Gerät anfragen](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md) · [Changelog](../../CHANGELOG.md)

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

Wer eine kabellose Maus einer Marke, ein Headset einer anderen und einen Bluetooth-Controller nutzt, braucht unter Windows oft mehrere schwere Hintergrundprogramme, nur um den Akkustand zu sehen.

**BatteryDock** ersetzt all das durch eine einzige ~4 MB große native Rust-App direkt in deiner Windows 10/11 Taskleiste. Kein Electron, keine Telemetrie, kein Konto.

---

## Vorschau

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-single.png" alt="BatteryDock single peripheral pill" /><br /><br />
        <sub><b>Einzelnes Gerät</b>: Scharfes Vektor-Icon, farbiger Akkubalken und Prozentzahl in der Taskleiste.</sub>
      </td>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-multi.png" alt="BatteryDock multiple peripherals pill" /><br /><br />
        <sub><b>Mehrere Geräte</b>: Reihenfolge ändern, umbenennen oder einzelne Geräte ausblenden.</sub>
      </td>
    </tr>
  </table>
  <br />
  <table>
    <tr>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-peripherals.jpg" alt="BatteryDock Peripherals dashboard v1.0.0" /><br />
        <sub><b>Geräte</b>: Geschätzte Laufzeit, 24h-Akkuverlauf, eigene Namen und gerätespezifische Warnungen.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-alerts.jpg" alt="BatteryDock Alerts and Audio settings v1.0.0" /><br />
        <sub><b>Warnungen & Ton</b>: Warnungen bei niedrigem, kritischem und 100% vollem Akku samt Tonvorschau.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-customization.jpg" alt="BatteryDock Customization settings v1.0.0" /><br />
        <sub><b>Anpassung</b>: Liquid Glass / Dunkel / Hell / System-Design, Gaming-HUD, Hotkey und 21 Sprachen.</sub>
      </td>
    </tr>
  </table>
</div>

---

## Funktionen

### Minimalistische Taskleisten-Leiste
- **4 Designs:** Wähle zwischen **Liquid Glass** (ultratransparent), **Dunkel**, **Hell** oder **System** (folgt deinem Windows-Design).
- **Aufgeräumt:** Linksklick öffnet die Einstellungen, Rechtsklick das Schnellmenü, oder per Drag-and-Drop auf jedem Monitor platzieren.
- **Cleveres Taskleisten-Verhalten:** Bleibt auch über Start, Suche und Widgets sichtbar und blendet sich bei Vollbildvideos und Spielen automatisch aus.
- **Flexibles Layout:** Geräte per Klick neu anordnen, umbenennen, ausblenden oder Prozentzahl ein-/ausschalten.

### Optionales Gaming- & Vollbild-Overlay (HUD)
- **Standardmäßig deaktiviert**, damit Vollbildvideos komplett ungestört bleiben.
- Aktiviert springt BatteryDock bei randlosen Vollbildspielen automatisch in eine Bildschirmecke.
- **Überall platzierbar:** 4 Ecken mit einstellbarem Randabstand oder freie X/Y-Position (0% bis 100%) mit Live-Vorschau.
- **Click-Through:** Mausklicks gehen direkt durch das HUD ins Spiel.
- **Globaler Hotkey:** Lege eine beliebige Tastenkombination fest (Standard `Alt + B`), um das Overlay im Spiel umzuschalten.

### Intelligente Telemetrie & Warnungen
- **24-Stunden-Akkuverlauf:** Jede Gerätekarte zeigt eine kompakte lokale Verlaufskurve.
- **Laufzeit & Ruhezustand:** Schätzt verbleibende Laufzeit oder Ladezeit und erkennt inaktive Geräte.
- **Niedrig-, Kritisch- & 100%-Warnung:** Globale oder gerätespezifische Schwellenwerte, Stummschaltung, sanfter Ton und Windows-Benachrichtigungen.

### 21 Sprachen integriert
- Erkennt beim ersten Start automatisch deine Windows-Sprache (mit Englisch als Standard-Fallback).
- Lädt bei Bedarf native Windows-Systemschriften für alle Schriftsysteme.

---

## Unterstützte Hardware

BatteryDock liest Geräte direkt über HID, WinRT und Bluetooth aus, ganz ohne Hersteller-Software.

| Kategorie | Protokoll / Methode | Unterstützte Beispiele |
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

## Download & Installation

Hol dir die neueste Version auf der **[Releases-Seite](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Lade **`BatteryDock-Setup-v1.0.0.exe`** (Installer) oder **`BatteryDock-Windows-Portable.zip`** (portabel) herunter.
2. Starte die App und aktiviere optional den Windows-Autostart in den Einstellungen.
3. BatteryDock erscheint neben der Uhr. Integrierte GitHub-Auto-Updates halten dich aktuell.

### Hinweis zu Windows SmartScreen
Da BatteryDock ein kostenloses Open-Source-Projekt ohne kostenpflichtiges Zertifikat ist, zeigt SmartScreen beim ersten Start eventuell eine Warnung. Klicke auf **Weitere Informationen** und **Trotzdem ausführen**. Alle Builds entstehen öffentlich in [`.github/workflows/release.yml`](../../.github/workflows/release.yml).

---

## Konfiguration

Einstellungen liegen als TOML-Datei unter `%APPDATA%\jorjanion\BatteryDock\config\config.toml` (Verlauf in `history.toml`). Änderungen wirken sofort ohne Neustart.

---

## Aus dem Quellcode bauen

Voraussetzungen: Windows 10/11 und [Rust](https://rustup.rs/) (stabile MSVC-Toolchain).

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock
cargo test --locked
cargo build --release --locked
.\target\release\battery-dock.exe
```

---

## Mitwirken, Issues & Pull Requests

BatteryDock ist Open Source und lebt von der Community. Fehlerberichte, Übersetzungen, Gerätewünsche und PRs sind herzlich willkommen.

- **Gerät fehlt?** Öffne eine **[Geräteanfrage](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)**. Klicke in **Einstellungen > Anpassung > Hardware-Diagnose kopieren** und füge den anonymisierten Bericht direkt ein.
- **Bug oder Idee?** Öffne ein **[Issue](https://github.com/jorjanion/BatteryDock/issues)** mit Windows-Version und Beschreibung.
- **Code beitragen?** Pull Requests sind sehr willkommen. Führe `cargo test --locked` und `cargo clippy --locked` aus, bevor du deinen PR erstellst.

---

## Lizenz

MIT-Lizenz. Details in [LICENSE](../../LICENSE).
