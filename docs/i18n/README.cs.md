<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock Logo" width="148" style="border-radius: 24px;" />

# BatteryDock

**Stav baterie všech bezdrátových periferií v jedné čisté nativní liště pro Windows. Bez těžkého softwaru výrobců.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Stáhnout v1.0.0](https://github.com/jorjanion/BatteryDock/releases/latest) · [Nahlásit chybu](https://github.com/jorjanion/BatteryDock/issues/new) · [Požádat o podporu zařízení](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md) · [Změny](../../CHANGELOG.md)

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
<a href="README.sv.md">Svenska</a> ·
<b>Čeština</b> ·
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

Pokud používáte bezdrátovou myš jedné značky, sluchátka druhé a Bluetooth ovladač, kontrola baterie ve Windows často znamená mít spuštěné tři náročné aplikace na pozadí.

**BatteryDock** to nahrazuje jedinou ~4 MB nativní aplikací v Rustu přímo na hlavním panelu Windows 10/11. Žádný Electron, žádná telemetrie, žádné účty.

---

## Náhled

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-single.png" alt="BatteryDock single peripheral pill" /><br /><br />
        <sub><b>Jedno zařízení</b>: Ostrá vektorová ikona, barevný ukazatel a procenta na hlavním panelu.</sub>
      </td>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-multi.png" alt="BatteryDock multiple peripherals pill" /><br /><br />
        <sub><b>Více zařízení</b>: Změňte pořadí, přejmenujte nebo skryjte vybraná zařízení.</sub>
      </td>
    </tr>
  </table>
  <br />
  <table>
    <tr>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-peripherals.jpg" alt="BatteryDock Peripherals dashboard v1.0.0" /><br />
        <sub><b>Periferie</b>: Odhad výdrže, 24h graf baterie, vlastní názvy a upozornění pro jednotlivá zařízení.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-alerts.jpg" alt="BatteryDock Alerts and Audio settings v1.0.0" /><br />
        <sub><b>Upozornění a zvuk</b>: Oznámení při nízké, kritické i 100% plné baterii se zvukovým testem.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-customization.jpg" alt="BatteryDock Customization settings v1.0.0" /><br />
        <sub><b>Přizpůsobení</b>: Motivy Liquid Glass / Tmavý / Světlý / Systémový, herní HUD, klávesová zkratka a 21 jazyků.</sub>
      </td>
    </tr>
  </table>
</div>

---

## Funkce

### Minimalistický ukazatel na hlavním panelu
- **4 motivy:** **Liquid Glass** (průhledný), **Tmavý**, **Světlý** nebo **Systémový** (automaticky podle Windows).
- **Čisté ovládání:** Levý klik otevře nastavení, pravý rychlou nabídku a tažením jej přesunete na jakýkoli monitor.
- **Chytré chování:** Zůstává nad hlavním panelem i při otevření Startu či Hledání a automaticky se skryje při videu či hře na celou obrazovku.
- **Vlastní rozvržení:** Přesouvejte pořadí jedním kliknutím, přejmenujte či skryjte zařízení nebo zapněte/vypněte procenta.

### Volitelný herní překryv (HUD)
- **Ve výchozím stavu vypnutý**, aby nerušil při sledování filmů a YouTube.
- Po zapnutí se zobrazí v rohu obrazovky při hraní v bezokrajovém režimu celé obrazovky.
- **Kdekoli na obrazovce:** 4 rohy s nastavitelným okrajem nebo vlastní pozice X/Y (0% až 100%) s živým náhledem.
- **Click-through:** Kliknutí myší procházejí přímo do hry.
- **Globální zkratka:** Nastavte libovolnou kombinaci (výchozí `Alt + B`) pro okamžité skrytí nebo zobrazení bez Alt+Tab.

### Chytrá telemetrie a upozornění
- **24hodinová historie:** Každá karta obsahuje lehký lokální graf nabíjení a vybíjení.
- **Výdrž a režim spánku:** Odhaduje zbývající čas nebo dobu do plného nabití a detekuje neaktivní zařízení.
- **Upozornění na nízkou, kritickou a 100% baterii:** Globální i vlastní prahy, ztlumení, jemný zvuk a oznámení Windows.

### 21 vestavěných jazyků
- Při prvním spuštění automaticky rozpozná jazyk Windows (jinak použije angličtinu).
- Na vyžádání načítá nativní písma Windows pro ostré vykreslení všech abeced.

---

## Podporovaný hardware

BatteryDock komunikuje s hardwarem přímo přes HID, WinRT a Bluetooth bez softwaru výrobců.

| Kategorie | Protokol / Metoda | Podporované příklady |
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

## Stažení a instalace

Stáhněte nejnovější verzi na **[stránce Releases](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Stáhněte **`BatteryDock-Setup-v1.0.0.exe`** (instalátor) nebo **`BatteryDock-Windows-Portable.zip`** (přenosná verze).
2. Spusťte aplikaci a volitelně zapněte spuštění při startu Windows v nastavení.
3. Panel se zobrazí vedle hodin s automatickými aktualizacemi z GitHubu.

### Poznámka k Windows SmartScreen
Protože je BatteryDock bezplatný open-source projekt bez placeného certifikátu, SmartScreen může při prvním spuštění zobrazit varování. Klikněte na **Další informace** a **Přesto spustit**. Všechny soubory jsou sestavovány veřejně v [`.github/workflows/release.yml`](../../.github/workflows/release.yml).

---

## Konfigurace

Nastavení je uloženo v `%APPDATA%\jorjanion\BatteryDock\config\config.toml` (historie v `history.toml`) a změny platí ihned.

---

## Sestavení ze zdrojového kódu

Požadavky: Windows 10/11 a [Rust](https://rustup.rs/) (stabilní MSVC).

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock
cargo test --locked
cargo build --release --locked
.\target\release\battery-dock.exe
```

---

## Přispívání, Issues a Pull Requesty

BatteryDock je open source řízený komunitou. Hlášení chyb, překlady, návrhy zařízení i PR jsou vždy vítány.

- **Vaše zařízení se nezobrazuje?** Otevřete **[žádost o podporu zařízení](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)**. Klikněte na **Nastavení > Přizpůsobení > Kopírovat diagnostiku hardwaru** a vložte anonymizovaný výpis.
- **Chyba nebo nápad?** Otevřete **[Issue](https://github.com/jorjanion/BatteryDock/issues)** s verzí Windows a popisem.
- **Chcete přidat kód?** Pull Requesty jsou vítány. Před odesláním spusťte `cargo test --locked` a `cargo clippy --locked`.

---

## Licence

Licence MIT. Viz [LICENSE](../../LICENSE).
