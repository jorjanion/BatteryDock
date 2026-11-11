<div align="center">

<img src="../../assets/logo.png" alt="Logo BatteryDock" width="160" style="border-radius: 24px;" />

# BatteryDock

**Čistý nativní indikátor baterie v Rustu integrovaný přímo do hlavního panelu Windows 10/11 — bez těžkopádného softwaru výrobců.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Stáhnout nejnovější instalátor](https://github.com/jorjanion/BatteryDock/releases/latest) · [Nahlásit chybu](https://github.com/jorjanion/BatteryDock/issues) · [Požádat o podporu zařízení](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)

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

## 📸 Náhled

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="42%">
        <img src="../screenshots/dock-closeup.png" alt="Detail kapsle BatteryDock se sluchátky na 92%" /><br /><br />
        <sub><b>Kapsle na hlavním panelu</b> — Přímo v hlavním panelu Windows s DPI škálováním pro každý monitor, barevnými úrovněmi a ikonou nabíjení ⚡.</sub>
      </td>
      <td align="center" valign="middle" width="58%">
        <img src="../screenshots/settings.png" alt="Okno nastavení BatteryDock v0.1.11" width="340" /><br />
        <sub><b>Samostatné nastavení</b> — Prahy upozornění, ukázka zvuku, umístění a seznam připojených zařízení.</sub>
      </td>
    </tr>
  </table>
</div>

---

## ⚡ Proč BatteryDock?

Většina bezdrátových periferií uzamyká stav baterie do objemných balíků výrobců s desítkami služeb na pozadí. **BatteryDock** vše přehledně spojí do jediné kapsle na hlavním panelu:

- **Bez bloatwaru:** Jediná nativní binárka v Rustu o velikosti **~4 MB** (žádný Electron, žádná telemetrie, ~77 MB soukromé paměti naměřených živě ve Windows 11).
- **Přímé čtení hardwaru:** Čte přímo **Logitech HID++ 2.0**, **Windows Bluetooth / BLE GATT**, **XInput** a **Sony DualSense / DualShock 4 HID** — zařízení Logitech hlásí baterii i bez otevřeného G HUB.
- **Chytrá pravidla celé obrazovky:** Automaticky se skryje při hrách a videích na celou obrazovku nebo při automatickém skrytí panelu, ale zůstává viditelný po stisku klávesy `Win`, otevření Startu/Widgetů i kliknutí na plochu.
- **Živý indikátor nabíjení:** Zvýrazní nabíjené zařízení **azurově** ikonou blesku **⚡**.
- **Spolehlivá upozornění na pozadí:** Zvukové signály i systémová oznámení fungují, i když je kapsle během hry skrytá.

---

## 📥 Stažení a instalace

Stáhněte nejnovější verzi ze **[stránky Releases](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Stáhněte **`BatteryDock-Setup-v0.1.11.exe`** (~4,4 MB) — nebo `BatteryDock-Windows-Portable.zip` pro přenosnou verzi.
2. Spusťte instalátor a případně zaškrtněte **"Start BatteryDock when Windows boots"**.
3. Kapsle se zobrazí vedle hodin na hlavním panelu.

### 🛡️ Poznámka k Windows SmartScreen
Protože je BatteryDock bezplatný open-source projekt bez placeného certifikátu EV, může SmartScreen při prvním spuštění zobrazit *"Systém Windows ochránil váš počítač"* (**Další informace → Přesto spustit**).
- Každé vydání se veřejně kompiluje v GitHub Actions podle [`.github/workflows/release.yml`](../../.github/workflows/release.yml).
- Kód si můžete zkontrolovat a [sestavit sami](#build-from-source) za méně než minutu.

---

## 🎧 Podporovaný hardware

| Kategorie | Protokol / Zdroj | Příklady a poznámky |
| :--- | :--- | :--- |
| **Bezdrátový Logitech** | Přímý HID++ 2.0 (`0x1004` / `0x1000`) & G HUB IPC | Sluchátka Lightspeed (G733, G535, G935), myši (G PRO, Superlight, G502), klávesnice |
| **Bluetooth & BLE** | Windows WinRT PnP + BLE GATT (`0x180F` / `0x2A19`) | Sluchátka (Sony WH/WF, Bose, AirPods), BT myši a klávesnice |
| **Ovladače Xbox** | Windows `XInput` | Xbox Wireless Controllers (Series X\|S, One, 360) včetně nabíjení kabelem |
| **PlayStation** | Sony HID (`VID 0x054C`) USB & Bluetooth | DualSense a DualShock 4 (více ovladačů + stav nabíjení) |

**Chybí vaše značka (Razer, Corsair, SteelSeries, Keychron atd.)?**
👉 **[Požádejte o podporu zařízení](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** — Pull Requesty jsou také vítány!

---

## 🎯 Ovládání a indikátory

### Barevné stavy
- **Zelená:** Dostatečná baterie (nad nízkým prahem)
- **Oranžová:** Nízká baterie (výchozí `20%`)
- **Červená (pulzující):** Kritický stav (výchozí `10%`)
- **Azurová + ⚡:** Právě se nabíjí

### Ovládání
- **Levé kliknutí** na kapsli nebo **`⚙`** (či ikonu v liště): Otevře **BatteryDock Settings** (`BatteryDock.exe --settings`).
- **Pravé kliknutí** na kapsli nebo ikonu v liště: Nativní menu (**Settings**, **Refresh Devices**, **Reset to Taskbar Clock**, **Restart BatteryDock**, **Exit BatteryDock**).
- **Tažení myší (> 5px):** Přesuňte kamkoli (i na jiné monitory); pravé kliknutí → **Reset to Taskbar Clock** jej vrátí k hodinám.

---

## ⚙️ Konfigurace

Nastavení se ukládají do `%APPDATA%\jorjanion\BatteryDock\config\config.toml`:

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

## 🛠️ Sestavení ze zdrojového kódu

### Požadavky
- [Rust toolchain](https://rustup.rs/) (stable)
- Windows 10 nebo Windows 11 (MSVC)

### Příkazy

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

# Ověření formátování, clippy a testů
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked

# Sestavení optimalizované release verze
cargo build --release --locked
```

Výsledný soubor: `target\release\battery-dock.exe` (parametr `--settings` / `-s` otevře přímo nastavení).

---

## 📄 Licence

Šířeno pod **licencí MIT**. Podrobnosti v [LICENSE](../../LICENSE).

<div align="center">

**Pokud se vám BatteryDock líbí, zvažte přidání ⭐ repozitáři!**

</div>
