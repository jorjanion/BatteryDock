<div align="center">

<img src="../../assets/logo.png" alt="Логотип BatteryDock" width="160" style="border-radius: 24px;" />

# BatteryDock

**Мінімалістичний нативний індикатор батареї на Rust прямо на панелі завдань Windows 10/11 — без важкого ПЗ виробників.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Завантажити інсталятор](https://github.com/jorjanion/BatteryDock/releases/latest) · [Повідомити про помилку](https://github.com/jorjanion/BatteryDock/issues) · [Запросити підтримку пристрою](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)

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
<a href="README.cs.md">Čeština</a> ·
<a href="README.ru.md">Русский</a> ·
<b>Українська</b> ·
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

## 📸 Попередній перегляд

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="42%">
        <img src="../screenshots/dock-closeup.png" alt="Капсула BatteryDock із зарядом гарнітури 92%" /><br /><br />
        <sub><b>Капсула на панелі завдань</b> — Вбудована безпосередньо в панель завдань Windows із Per-Monitor DPI-масштабуванням, кольорами рівнів та іконкою зарядки ⚡.</sub>
      </td>
      <td align="center" valign="middle" width="58%">
        <img src="../screenshots/settings.png" alt="Вікно налаштувань BatteryDock v0.1.11" width="340" /><br />
        <sub><b>Окремі налаштування</b> — Пороги сповіщень, перевірка звуку, режими позиції та список підключених пристроїв.</sub>
      </td>
    </tr>
  </table>
</div>

---

## ⚡ Чому BatteryDock?

Більшість бездротових пристроїв ховають відсоток батареї у важких фірмових пакетах із десятками фонових сервісів. **BatteryDock** показує все в акуратній капсулі на панелі завдань:

- **Жодного bloatware:** Єдиний нативний файл на Rust розміром **~4 МБ** (без Electron і телеметрії, ~77 МБ приватної пам'яті у Windows 11).
- **Пряме читання обладнання:** Опитує **Logitech HID++ 2.0**, **Windows Bluetooth / BLE GATT**, **XInput** та **Sony DualSense / DualShock 4 HID** напряму — пристрої Logitech працюють навіть без відкритого G HUB (G HUB IPC також підтримується).
- **Розумний повноекранний режим:** Автоматично ховається у повноекранних іграх, відео або при автоприховуванні панелі, але не зникає при натисканні `Win`, відкритті Пуску/Віджетів чи кліку по робочому столу.
- **Індикатор заряджання наживо:** Підсвічує пристрій **блакитним** із **блискавкою ⚡**.
- **Надійні фонові сповіщення:** Звук та сповіщення Windows спрацьовують із фонового рушія навіть коли капсула прихована у грі.

---

## 📥 Завантаження та встановлення

Завантажте останню версію зі **[сторінки Releases](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Завантажте **`BatteryDock-Setup-v0.1.11.exe`** (~4,4 МБ) — або `BatteryDock-Windows-Portable.zip` для портативної версії.
2. Запустіть інсталятор і за бажанням увімкніть **"Start BatteryDock when Windows boots"**.
3. Капсула з'явиться на панелі завдань біля годинника.

### 🛡️ Примітка щодо Windows SmartScreen
Оскільки BatteryDock — безкоштовний open-source проєкт без платного EV-сертифіката підпису, SmartScreen може показати *"Windows захистила ваш ПК"* (**Докладніше → Виконати в будь-якому разі**).
- Кожен реліз відкрито збирається на серверах GitHub Actions у [`.github/workflows/release.yml`](../../.github/workflows/release.yml).
- Ви можете переглянути код на Rust і [зібрати програму самостійно](#build-from-source) менше ніж за хвилину.

---

## 🎧 Підтримувані пристрої

| Категорія | Протокол / Джерело | Приклади та примітки |
| :--- | :--- | :--- |
| **Logitech бездротові** | Прямий HID++ 2.0 (`0x1004` / `0x1000`) & G HUB IPC | Гарнітури Lightspeed (G733, G535, G935), миші (G PRO, Superlight, G502), клавіатури |
| **Bluetooth & BLE** | Windows WinRT PnP + BLE GATT (`0x180F` / `0x2A19`) | Навушники (Sony WH/WF, Bose, AirPods), BT-миші й клавіатури |
| **Геймпади Xbox** | Windows `XInput` | Xbox Wireless Controllers (Series X\|S, One, 360) включно із зарядкою по кабелю |
| **PlayStation** | Sony HID (`VID 0x054C`) USB & Bluetooth | DualSense та DualShock 4 (кілька геймпадів + статус зарядки) |

**Немає вашого бренду (Razer, Corsair, SteelSeries, Keychron тощо)?**
👉 **[Відкрити запит на підтримку пристрою](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** — Pull Request-и також вітаються!

---

## 🎯 Керування та індикатори

### Кольори стану
- **Зелений:** Достатній заряд (вище низького порогу)
- **Бурштиновий:** Низький заряд (за замовчуванням `20%`)
- **Червоний (пульсуючий):** Критичний заряд (за замовчуванням `10%`)
- **Блакитний + ⚡:** Заряджається зараз

### Взаємодія
- **Лівий клік** по капсулі або **`⚙`** (чи іконці в треї): Відкриває **BatteryDock Settings** (`BatteryDock.exe --settings`).
- **Правий клік** по капсулі чи трею: Нативне меню (**Settings**, **Refresh Devices**, **Reset to Taskbar Clock**, **Restart BatteryDock**, **Exit BatteryDock**).
- **Перетягування (> 5px):** Переміщуйте куди завгодно (навіть на інший монітор); правий клік → **Reset to Taskbar Clock** повертає до годинника.

---

## ⚙️ Конфігурація

Налаштування зберігаються у `%APPDATA%\jorjanion\BatteryDock\config\config.toml`:

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

## 🛠️ Збірка з вихідного коду

### Вимоги
- [Rust toolchain](https://rustup.rs/) (stable)
- Windows 10 або Windows 11 (MSVC)

### Команди

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

# Перевірка форматування, clippy та тестів
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked

# Збірка оптимізованого release-бінарника
cargo build --release --locked
```

Вихідний файл: `target\release\battery-dock.exe` (передайте `--settings` / `-s`, щоб одразу відкрити налаштування).

---

## 📄 Ліцензія

Поширюється за **ліцензією MIT**. Деталі в [LICENSE](../../LICENSE).

<div align="center">

**Якщо BatteryDock став у пригоді, поставте ⭐ репозиторію!**

</div>
