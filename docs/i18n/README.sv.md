<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock Logo" width="148" style="border-radius: 24px;" />

# BatteryDock

**Alla dina trådlösa enheters batterinivåer i en ren, inbyggd kapsel i Windows aktivitetsfält. Ingen bloatware.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Ladda ner v1.0.0](https://github.com/jorjanion/BatteryDock/releases/latest) · [Rapportera bugg](https://github.com/jorjanion/BatteryDock/issues/new) · [Begär enhetsstöd](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md) · [Ändringslogg](../../CHANGELOG.md)

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

Använder du trådlös mus från ett märke, headset från ett annat och en Bluetooth-handkontroll behöver du ofta flera tunga bakgrundsprogram bara för att se batteriet i Windows.

**BatteryDock** ersätter allt med en enda ~4 MB native Rust-app direkt i aktivitetsfältet i Windows 10/11. Inget Electron, ingen telemetri och inga konton.

---

## Förhandsvisning

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-single.png" alt="BatteryDock single peripheral pill" /><br /><br />
        <sub><b>En enhet</b>: Skarp vektorikon, färgkodad batterimätare och procent i aktivitetsfältet.</sub>
      </td>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-multi.png" alt="BatteryDock multiple peripherals pill" /><br /><br />
        <sub><b>Flera enheter</b>: Ändra ordning, döp om eller dölj enheter.</sub>
      </td>
    </tr>
  </table>
  <br />
  <table>
    <tr>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-peripherals.jpg" alt="BatteryDock Peripherals dashboard v1.0.0" /><br />
        <sub><b>Enheter</b>: Beräknad batteritid, 24-timmars graf, smeknamn och enhetsvarningar.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-alerts.jpg" alt="BatteryDock Alerts and Audio settings v1.0.0" /><br />
        <sub><b>Varningar & Ljud</b>: Aviseringar vid låg, kritisk och 100% full laddning med ljudtest.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-customization.jpg" alt="BatteryDock Customization settings v1.0.0" /><br />
        <sub><b>Anpassning</b>: Liquid Glass / Mörk / Ljus / System, spel-HUD, snabbtangent och 21 språk.</sub>
      </td>
    </tr>
  </table>
</div>

---

## Funktioner

### Minimalistisk kapsel i aktivitetsfältet
- **4 teman:** Välj **Liquid Glass** (ultratransparent), **Mörk**, **Ljus** eller **System** (följer Windows-temat).
- **Enkelt och rent:** Vänsterklick öppnar Inställningar, högerklick öppnar snabbmenyn, eller dra kapseln till valfri skärm.
- **Smart hantering:** Ligger stadigt ovanför aktivitetsfältet även med Start eller Sök öppet och döljs automatiskt vid helskärm.
- **Anpassad layout:** Ordna om enheter med ett klick, ge smeknamn, dölj enheter eller slå av/på procenttext.

### Valfritt spel- och helskärmsöverlägg (HUD)
- **Avstängt som standard** så helskärmsvideor aldrig störs.
- När det är aktivt visas batteriet i ett hörn under kantlösa helskärmsspel.
- **Placera var du vill:** 4 hörn med justerbar marginal eller fri X/Y-position (0% till 100%) med liveförhandsvisning.
- **Click-through:** Musklick går rakt igenom HUD:en till spelet.
- **Global genväg:** Välj valfri tangentkombination (standard `Alt + B`) för att visa/dölja direkt i spelet.

### Smart telemetri & aviseringar
- **24-timmars batterihistorik:** Varje enhetskort visar en lätt lokal kurva.
- **Batteritid & viloläge:** Uppskattar återstående tid eller tid till full laddning och känner av viloläge.
- **Låg, kritisk & 100%-varning:** Globala eller individuella gränser, ljudlös per enhet, mjukt ljud och Windows-aviseringar.

### 21 inbyggda språk
- Identifierar automatiskt ditt Windows-språk vid första start (engelska som reserv).
- Laddar inbyggda Windows-teckensnitt vid behov för knivskarp text på alla skriftsystem.

---

## Hårdvara som stöds

BatteryDock läser direkt från hårdvaran via HID, WinRT och Bluetooth utan tillverkarprogram.

| Kategori | Protokoll / Metod | Exempel som stöds |
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

## Nedladdning & Installation

Hämta senaste versionen på **[Releases-sidan](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Ladda ner **`BatteryDock-Setup-v1.0.0.exe`** (installerare) eller **`BatteryDock-Windows-Portable.zip`** (portabel).
2. Starta appen och aktivera gärna autostart med Windows i inställningarna.
3. Kapseln visas bredvid klockan med inbyggda GitHub-uppdateringar.

### Om Windows SmartScreen
Eftersom BatteryDock är gratis öppen källkod utan betalt certifikat kan SmartScreen visa en varning första gången. Klicka på **Mer information** och **Kör ändå**. Alla byggen skapas öppet i [`.github/workflows/release.yml`](../../.github/workflows/release.yml).

---

## Konfiguration

Inställningar sparas i `%APPDATA%\jorjanion\BatteryDock\config\config.toml` (historik i `history.toml`) och uppdateras direkt.

---

## Bygg från källkod

Krav: Windows 10/11 och [Rust](https://rustup.rs/) (stabil MSVC-toolchain).

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock
cargo test --locked
cargo build --release --locked
.\target\release\battery-dock.exe
```

---

## Bidra, Issues & Pull Requests

BatteryDock är öppen källkod och community-drivet. Buggrapporter, översättningar, enhetsönskemål och PR:er är alltid välkomna.

- **Saknas din enhet?** Öppna en **[enhetsförfrågan](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)**. Klicka på **Inställningar > Anpassning > Kopiera hårdvarudiagnostik** och klistra in den anonymiserade rapporten.
- **Bugg eller idé?** Öppna ett **[Issue](https://github.com/jorjanion/BatteryDock/issues)** med Windows-version och steg.
- **Vill du bidra med kod?** Pull Requests är mycket välkomna. Kör `cargo test --locked` och `cargo clippy --locked` innan du skickar.

---

## Licens

MIT-licens. Se [LICENSE](../../LICENSE).
