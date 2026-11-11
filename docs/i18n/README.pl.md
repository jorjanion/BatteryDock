<div align="center">

<img src="../../assets/logo.png" alt="Logo BatteryDock" width="160" style="border-radius: 24px;" />

# BatteryDock

**Lekki, natywny wskaźnik baterii w Rust osadzony bezpośrednio na pasku zadań Windows 10/11 — bez ciężkiego oprogramowania producentów.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Pobierz najnowszy instalator](https://github.com/jorjanion/BatteryDock/releases/latest) · [Zgłoś błąd](https://github.com/jorjanion/BatteryDock/issues) · [Poproś o wsparcie urządzenia](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)

<br />

<sub>
<a href="../../README.md">English</a> ·
<a href="README.es.md">Español</a> ·
<a href="README.fr.md">Français</a> ·
<a href="README.de.md">Deutsch</a> ·
<a href="README.pt-BR.md">Português</a> ·
<a href="README.it.md">Italiano</a> ·
<a href="README.nl.md">Nederlands</a> ·
<b>Polski</b> ·
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

## 📸 Podgląd

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="42%">
        <img src="../screenshots/dock-closeup.png" alt="Zbliżenie kapsułki BatteryDock pokazującej słuchawki 92%" /><br /><br />
        <sub><b>Kapsułka na pasku zadań</b> — Osadzona w pasku Windows ze skalowaniem DPI dla każdego monitora, kolorami poziomów i ikoną ładowania ⚡.</sub>
      </td>
      <td align="center" valign="middle" width="58%">
        <img src="../screenshots/settings.png" alt="Okno ustawień BatteryDock v0.1.11" width="340" /><br />
        <sub><b>Osobne ustawienia</b> — Progi alertów, test dźwięku, tryby pozycjonowania i lista podłączonych urządzeń.</sub>
      </td>
    </tr>
  </table>
</div>

---

## ⚡ Dlaczego BatteryDock?

Większość urządzeń bezprzewodowych ukrywa poziom baterii w ciężkich pakietach producentów uruchamiających dziesiątki usług w tle. **BatteryDock** umieszcza wszystko w jednej minimalistycznej pigułce na pasku zadań:

- **Zero bloatware'u:** Pojedyncza natywna binarka Rust **~4 MB** (bez Electrona, bez telemetrii, ~77 MB pamięci prywatnej w Windows 11).
- **Bezpośredni odczyt sprzętu:** Odpytuje **Logitech HID++ 2.0**, **Windows Bluetooth / BLE GATT**, **XInput** oraz **Sony DualSense / DualShock 4 HID** — urządzenia Logitech działają nawet bez włączonego G HUB (a jeśli G HUB działa, wspiera też G HUB IPC).
- **Inteligentny tryb pełnoekranowy:** Ukrywa się automatycznie w grach pełnoekranowych, wideo i przy ukrywaniu paska zadań, ale nie znika po naciśnięciu klawisza `Win`, otwarciu Startu/Widgetów ani kliknięciu pulpitu.
- **Wskaźnik ładowania na żywo:** Podświetla ładowane urządzenie na **cyjanowo** z ikoną **⚡**.
- **Powiadomienia działające w tle:** Dźwięk i powiadomienia systemowe działają także wtedy, gdy pigułka jest ukryta podczas gry.

---

## 📥 Pobieranie i instalacja

Pobierz najnowszą wersję ze strony **[Releases](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Pobierz **`BatteryDock-Setup-v0.1.11.exe`** (~4,4 MB) — lub `BatteryDock-Windows-Portable.zip` w wersji przenośnej.
2. Uruchom instalator i opcjonalnie zaznacz **"Start BatteryDock when Windows boots"**.
3. Kapsułka pojawi się na pasku zadań obok zegara.

### 🛡️ Informacja o Windows SmartScreen
Ponieważ BatteryDock jest darmowym projektem open-source bez płatnego certyfikatu EV, Windows SmartScreen może wyświetlić komunikat *"System Windows ochronił ten komputer"* (**Więcej informacji → Uruchom mimo to**).
- Wszystkie wydania są publicznie kompilowane na runnerach GitHub Actions w [`.github/workflows/release.yml`](../../.github/workflows/release.yml).
- Możesz przejrzeć cały kod w Rust i [skompilować go samodzielnie](#build-from-source) w mniej niż minutę.

---

## 🎧 Obsługiwany sprzęt

| Kategoria | Protokół / Źródło | Przykłady i uwagi |
| :--- | :--- | :--- |
| **Logitech bezprzewodowe** | Bezpośredni HID++ 2.0 (`0x1004` / `0x1000`) & G HUB IPC | Słuchawki Lightspeed (G733, G535, G935), myszy (G PRO, Superlight, G502), klawiatury |
| **Bluetooth & BLE** | Windows WinRT PnP + BLE GATT (`0x180F` / `0x2A19`) | Słuchawki (Sony WH/WF, Bose, AirPods), myszy i klawiatury BT |
| **Kontrolery Xbox** | Windows `XInput` | Kontrolery Xbox Wireless (Series X\|S, One, 360) z wykrywaniem ładowania po kablu |
| **PlayStation** | Sony HID (`VID 0x054C`) USB & Bluetooth | DualSense i DualShock 4 (wiele padów + status ładowania) |

**Brakuje Twojej marki (Razer, Corsair, SteelSeries, Keychron itd.)?**
👉 **[Zgłoś prośbę o wsparcie urządzenia](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** — Pull Requesty również mile widziane!

---

## 🎯 Sterowanie i kolory

### Kolory statusu
- **Zielony:** Dobry poziom baterii (powyżej dolnego progu)
- **Bursztynowy:** Niski poziom (domyślnie `20%`)
- **Czerwony (pulsujący):** Poziom krytyczny (domyślnie `10%`)
- **Cyjan + ⚡:** Urządzenie ładuje się

### Obsługa
- **Lewy klik** w kapsułkę lub **`⚙`** (albo ikonę w zasobniku): Otwiera **BatteryDock Settings** (`BatteryDock.exe --settings`).
- **Prawy klik** na kapsułce lub w zasobniku: Natywne menu (**Settings**, **Refresh Devices**, **Reset to Taskbar Clock**, **Restart BatteryDock**, **Exit BatteryDock**).
- **Przeciąganie (> 5px):** Przenieś w dowolne miejsce (także na inne monitory); prawy klik → **Reset to Taskbar Clock** przywraca przy zegarze.

---

## ⚙️ Konfiguracja

Ustawienia zapisują się w `%APPDATA%\jorjanion\BatteryDock\config\config.toml`:

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

## 🛠️ Kompilacja ze źródeł

### Wymagania
- [Rust toolchain](https://rustup.rs/) (stable)
- Windows 10 lub Windows 11 (target MSVC)

### Polecenia

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

# Sprawdź formatowanie, clippy i testy
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked

# Zbuduj zoptymalizowaną wersję release
cargo build --release --locked
```

Plik wynikowy: `target\release\battery-dock.exe` (argument `--settings` / `-s` otwiera bezpośrednio ustawienia).

---

## 📄 Licencja

Udostępniono na **Licencji MIT**. Zobacz [LICENSE](../../LICENSE).

<div align="center">

**Jeśli BatteryDock Ci się podoba, zostaw proszę ⭐ w repozytorium!**

</div>
