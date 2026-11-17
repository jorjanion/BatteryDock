<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock Logo" width="148" style="border-radius: 24px;" />

# BatteryDock

**Poziom baterii wszystkich urządzeń bezprzewodowych w jednym lekkim pasku w Windows. Bez ciężkich aplikacji producentów.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Pobierz v1.0.0](https://github.com/jorjanion/BatteryDock/releases/latest) · [Zgłoś błąd](https://github.com/jorjanion/BatteryDock/issues/new) · [Poproś o wsparcie urządzenia](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md) · [Zmiany](../../CHANGELOG.md)

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

Jeśli używasz bezprzewodowej myszy jednej marki, słuchawek innej i kontrolera Bluetooth, sprawdzanie baterii w Windows wymaga zwykle uruchamiania kilku ciężkich aplikacji w tle.

**BatteryDock** zastępuje je jedną natywną aplikacją w Rust (~4 MB) na pasku zadań Windows 10/11. Bez Electrona, bez telemetrii i bez kont.

---

## Podgląd

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-single.png" alt="BatteryDock single peripheral pill" /><br /><br />
        <sub><b>Jedno urządzenie</b>: Wyraźna ikona wektorowa, kolorowy pasek baterii i procent na pasku zadań.</sub>
      </td>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-multi.png" alt="BatteryDock multiple peripherals pill" /><br /><br />
        <sub><b>Wiele urządzeń</b>: Zmieniaj kolejność, nadawaj własne nazwy lub ukrywaj wybrane urządzenia.</sub>
      </td>
    </tr>
  </table>
  <br />
  <table>
    <tr>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-peripherals.jpg" alt="BatteryDock Peripherals dashboard v1.0.0" /><br />
        <sub><b>Urządzenia</b>: Szacowany czas pracy, wykres 24h, własne nazwy i progi alertów dla każdego sprzętu.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-alerts.jpg" alt="BatteryDock Alerts and Audio settings v1.0.0" /><br />
        <sub><b>Alerty i dźwięk</b>: Powiadomienia o niskim, krytycznym i 100% poziomie naładowania z podglądem dźwięku.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-customization.jpg" alt="BatteryDock Customization settings v1.0.0" /><br />
        <sub><b>Personalizacja</b>: Motywy Liquid Glass / Ciemny / Jasny / Systemowy, nakładka HUD do gier, skrót i 21 języków.</sub>
      </td>
    </tr>
  </table>
</div>

---

## Funkcje

### Minimalistyczna pastylka na pasku zadań
- **4 motywy:** **Liquid Glass** (przezroczysty), **Ciemny**, **Jasny** lub **Systemowy** (dopasowany do Windows).
- **Prosta obsługa:** Lewy klik otwiera ustawienia, prawy szybkie menu, a przeciągnięcie pozwala ustawić pasek na dowolnym monitorze.
- **Inteligentne zachowanie:** Pozostaje nad paskiem zadań nawet przy otwartym menu Start czy Wyszukiwaniu i automatycznie znika na pełnym ekranie.
- **Własny układ:** Zmieniaj kolejność urządzeń jednym kliknięciem, dodawaj nazwy, ukrywaj sprzęt lub przełączaj wyświetlanie procentów.

### Opcjonalna nakładka w grach (HUD)
- **Domyślnie wyłączona**, aby nie przeszkadzać podczas oglądania filmów na pełnym ekranie.
- Po włączeniu pojawia się w rogu ekranu podczas gier w trybie pełnoekranowym bez ramki.
- **Dowolne położenie:** 4 narożniki z regulowanym marginesem lub własna pozycja X/Y (0% do 100%) z podglądem na żywo.
- **Tryb Click-through:** Kliknięcia myszy przechodzą prosto do gry.
- **Globalny skrót klawiszowy:** Ustaw dowolną kombinację (domyślnie `Alt + B`), aby przełączać HUD w trakcie gry bez Alt+Tab.

### Inteligentna telemetria i alerty
- **Wykres z 24 godzin:** Każda karta urządzenia pokazuje lokalny wykres ładowania i rozładowywania.
- **Czas pracy i uśpienie:** Szacuje pozostały czas działania lub ładowania oraz wykrywa tryb uśpienia.
- **Alerty niskiego, krytycznego i 100% poziomu:** Progi globalne lub indywidualne, wyciszenie, delikatny dźwięk i powiadomienia Windows.

### 21 wbudowanych języków
- Automatycznie wykrywa język systemu Windows przy pierwszym uruchomieniu (z domyślnym angielskim dla nieobsługiwanych).
- Wczytuje natywne czcionki Windows na żądanie, zapewniając idealne wyświetlanie każdego alfabetu.

---

## Obsługiwany sprzęt

BatteryDock komunikuje się ze sprzętem bezpośrednio przez HID, WinRT i Bluetooth, bez oprogramowania producentów.

| Kategoria | Protokół / Metoda | Obsługiwane przykłady |
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

## Pobieranie i instalacja

Pobierz najnowszą wersję ze **[strony Releases](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Pobierz **`BatteryDock-Setup-v1.0.0.exe`** (instalator) lub **`BatteryDock-Windows-Portable.zip`** (wersja przenośna).
2. Uruchom aplikację i opcjonalnie włącz autostart z Windows w ustawieniach.
3. Pasek pojawi się obok zegara, a wbudowany mechanizm sprawdzi aktualizacje z GitHuba.

### Informacja o Windows SmartScreen
Ponieważ BatteryDock jest darmowym projektem open source bez płatnego certyfikatu, SmartScreen może wyświetlić ostrzeżenie przy pierwszym uruchomieniu. Kliknij **Więcej informacji**, a następnie **Uruchom mimo to**. Wszystkie pliki są budowane publicznie w [`.github/workflows/release.yml`](../../.github/workflows/release.yml).

---

## Konfiguracja

Ustawienia zapisywane są w `%APPDATA%\jorjanion\BatteryDock\config\config.toml` (historia w `history.toml`) i działają od razu bez restartu.

---

## Budowanie ze źródeł

Wymagania: Windows 10/11 i [Rust](https://rustup.rs/) (stabilny MSVC).

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock
cargo test --locked
cargo build --release --locked
.\target\release\battery-dock.exe
```

---

## Wkład, Issues i Pull Requesty

BatteryDock to projekt open source tworzony ze społecznością. Zgłoszenia błędów, tłumaczenia, prośby o sprzęt i PR są mile widziane.

- **Brakuje Twojego urządzenia?** Otwórz **[zgłoszenie urządzenia](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)**. Kliknij **Ustawienia > Personalizacja > Kopiuj diagnostykę sprzętu** i wklej zanonimizowany raport do zgłoszenia.
- **Błąd lub pomysł?** Otwórz **[Issue](https://github.com/jorjanion/BatteryDock/issues)** z wersją Windows i opisem.
- **Chcesz dodać kod?** Pull Requesty są mile widziane. Uruchom `cargo test --locked` i `cargo clippy --locked` przed wysłaniem.

---

## Licencja

Licencja MIT. Zobacz [LICENSE](../../LICENSE).
