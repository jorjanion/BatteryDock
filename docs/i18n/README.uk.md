<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock Logo" width="148" style="border-radius: 24px;" />

# BatteryDock

**Заряд усіх бездротових пристроїв в одній легкій нативній панелі на панелі завдань Windows. Без зайвого софту виробників.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Завантажити v1.0.0](https://github.com/jorjanion/BatteryDock/releases/latest) · [Повідомити про помилку](https://github.com/jorjanion/BatteryDock/issues/new) · [Запросити пристрій](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md) · [Зміни](../../CHANGELOG.md)

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

Якщо ви користуєтеся бездротовою мишею одного бренду, гарнітурою іншого та Bluetooth-навушниками або геймпадом, перевірка заряду у Windows зазвичай вимагає тримати у фоні кілька важких програм.

**BatteryDock** замінює все це одним нативним додатком на Rust (~4 МБ) прямо на панелі завдань Windows 10/11. Без Electron, телеметрії та акаунтів.

---

## Скріншоти

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-single.png" alt="BatteryDock single peripheral pill" /><br /><br />
        <sub><b>Один пристрій</b>: Чітка векторна іконка, кольоровий індикатор і відсоток на панелі завдань.</sub>
      </td>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-multi.png" alt="BatteryDock multiple peripherals pill" /><br /><br />
        <sub><b>Кілька пристроїв</b>: Змінюйте порядок, перейменовуйте або приховуйте зайві пристрої.</sub>
      </td>
    </tr>
  </table>
  <br />
  <table>
    <tr>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-peripherals.jpg" alt="BatteryDock Peripherals dashboard v1.0.0" /><br />
        <sub><b>Пристрої</b>: Оцінка автономності, графік заряду за 24 години, власні назви та окремі сповіщення.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-alerts.jpg" alt="BatteryDock Alerts and Audio settings v1.0.0" /><br />
        <sub><b>Сповіщення та звук</b>: Попередження про низький, критичний і 100% заряд із перевіркою звуку.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-customization.jpg" alt="BatteryDock Customization settings v1.0.0" /><br />
        <sub><b>Персоналізація</b>: Теми Liquid Glass / Темна / Світла / Системна, ігровий HUD, гаряча клавіша та 21 мова.</sub>
      </td>
    </tr>
  </table>
</div>

---

## Можливості

### Мінімалістична панель на панелі завдань
- **4 теми оформлення:** **Liquid Glass** (прозора), **Темна**, **Світла** або **Системна** (повторює тему Windows).
- **Нічого зайвого:** Лівий клік відкриває налаштування, правий викликає швидке меню, а перетягуванням панель можна розмістити на будь-якому моніторі.
- **Розумна поведінка:** Залишається поверх панелі завдань навіть при відкритому «Пуску» чи «Пошуку» і автоматично ховається в повноекранному відео та іграх.
- **Гнучкий макет:** Змінюйте порядок в один клік, давайте власні назви, приховуйте пристрої або вимикайте показ відсотків.

### Ігровий та повноекранний оверлей (HUD)
- **Вимкнений за замовчуванням**, щоб не заважати під час перегляду фільмів і YouTube.
- Після увімкнення автоматично з'являється у куті екрана в іграх у повноекранному режимі без рамки.
- **Будь-яке місце екрана:** 4 кути з регульованим відступом або вільна позиція X/Y (0% - 100%) з живим переглядом.
- **Режим Click-through:** Кліки миші проходять крізь оверлей прямо в гру.
- **Глобальна гаряча клавіша:** Призначте будь-яку комбінацію (за замовчуванням `Alt + B`), щоб перемикати HUD без Alt+Tab.

### Розумна телеметрія та сповіщення
- **Історія за 24 години:** На картці кожного пристрою відображається легкий локальний графік.
- **Автономність і режим сну:** Оцінює залишок роботи або зарядки та розпізнає сплячий режим.
- **Сповіщення про низький, критичний і 100% заряд:** Загальні чи індивідуальні пороги, вимкнення для окремих пристроїв, м'який звук і сповіщення Windows.

### 21 вбудована мова
- Автоматично визначає мову Windows при першому запуску (англійська за замовчуванням для непідтримуваних).
- Завантажує системні шрифти Windows за потреби для чіткого відображення всіх алфавітів.

---

## Підтримуване обладнання

BatteryDock читає заряд напряму через HID, WinRT та Bluetooth без софту виробників.

| Категорія | Протокол / Метод | Підтримувані приклади |
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

## Завантаження та встановлення

Завантажте останню версію зі **[сторінки Releases](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Завантажте **`BatteryDock-Setup-v1.0.0.exe`** (інсталятор) або **`BatteryDock-Windows-Portable.zip`** (портативна версія).
2. Запустіть і за бажанням увімкніть автозапуск із Windows у налаштуваннях.
3. Панель з'явиться біля годинника разом із вбудованими оновленнями з GitHub.

### Примітка про Windows SmartScreen
Оскільки BatteryDock є безкоштовним open-source проєктом без платного сертифіката, SmartScreen може показати попередження. Натисніть **Додатково** і **Виконати все одно**. Усі релізи збираються публічно в [`.github/workflows/release.yml`](../../.github/workflows/release.yml).

---

## Конфігурація

Налаштування зберігаються у `%APPDATA%\jorjanion\BatteryDock\config\config.toml` (історія у `history.toml`) і застосовуються миттєво.

---

## Збірка з вихідного коду

Вимоги: Windows 10/11 і [Rust](https://rustup.rs/) (стабільний MSVC).

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock
cargo test --locked
cargo build --release --locked
.\target\release\battery-dock.exe
```

---

## Участь, Issues та Pull Requests

BatteryDock розвивається відкрито зі спільнотою. Повідомлення про баги, переклади, запити пристроїв і PR завжди вітаються.

- **Пристрій не видно?** Створіть **[запит підтримки пристрою](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)**. Відкрийте **Налаштування > Персоналізація > Копіювати діагностику обладнання** і вставте анонімний звіт в issue.
- **Помилка чи ідея?** Створіть **[Issue](https://github.com/jorjanion/BatteryDock/issues)** із версією Windows і описом.
- **Хочете додати код?** Pull Requests вітаються. Запустіть `cargo test --locked` і `cargo clippy --locked` перед надсиланням.

---

## Ліцензія

Ліцензія MIT. Детальніше в [LICENSE](../../LICENSE).
