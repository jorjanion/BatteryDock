<div align="center">

<img src="../../assets/logo.png" alt="شعار BatteryDock" width="160" style="border-radius: 24px;" />

# BatteryDock

**مؤشر بطارية أنيق ومكتوب بلغة Rust مدمج مباشرة في شريط مهام Windows 10/11 — بدون برامج الشركات الثقيلة.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[تحميل أحدث مثبّت](https://github.com/jorjanion/BatteryDock/releases/latest) · [الإبلاغ عن خطأ](https://github.com/jorjanion/BatteryDock/issues) · [طلب دعم جهاز](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)

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
<a href="README.uk.md">Українська</a> ·
<a href="README.tr.md">Türkçe</a> ·
<b>العربية</b> ·
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

<div dir="rtl">

## 📸 معاينة

</div>

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="42%">
        <img src="../screenshots/dock-closeup.png" alt="لقطة قريبة لكبسولة BatteryDock تعرض بطارية السماعة بنسبة 92%" /><br /><br />
        <sub><b>كبسولة شريط المهام</b> — مدمجة مباشرة في شريط مهام Windows مع دعم DPI لكل شاشة وألوان الحالة وشارة الشحن ⚡.</sub>
      </td>
      <td align="center" valign="middle" width="58%">
        <img src="../screenshots/settings.png" alt="نافذة إعدادات BatteryDock v0.1.11" width="340" /><br />
        <sub><b>إعدادات مستقلة</b> — حدود التنبيه، معاينة الصوت، أوضاع التموضع وقائمة الأجهزة المتصلة.</sub>
      </td>
    </tr>
  </table>
</div>

---

<div dir="rtl">

## ⚡ لماذا BatteryDock؟

تخفي معظم الأجهزة اللاسلكية نسبة البطارية داخل برامج ثقيلة تشغّل عشرات الخدمات في الخلفية. يجمع **BatteryDock** كل أجهزتك في كبسولة أنيقة داخل شريط المهام:

- **بدون برامج ثقيلة:** ملف تنفيذي واحد بلغة Rust بحجم **~4 ميجابايت** (بدون Electron أو تتبع، ~77 ميجابايت ذاكرة خاصة على Windows 11).
- **قراءة مباشرة من العتاد:** يستعلم مباشرة عن **Logitech HID++ 2.0** و**Windows Bluetooth / BLE GATT** و**XInput** و**Sony DualSense / DualShock 4 HID** — وتعمل أجهزة Logitech حتى دون فتح G HUB.
- **قواعد ذكية لملء الشاشة:** يختفي تلقائياً أثناء الألعاب أو الفيديو بملء الشاشة أو إخفاء شريط المهام، ويبقى ثابتاً عند الضغط على مفتاح `Win` أو فتح القائمة أو النقر على سطح المكتب.
- **مؤشر شحن حي:** يلوّن الجهاز باللون **السماوي** مع أيقونة **البرق ⚡** أثناء الشحن.
- **تنبيهات موثوقة في الخلفية:** تعمل أصوات وإشعارات النظام حتى عندما تكون الكبسولة مخفية أثناء اللعب.

---

## 📥 التحميل والتثبيت

حمّل أحدث إصدار من **[صفحة الإصدارات](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. حمّل **`BatteryDock-Setup-v0.1.11.exe`** (~4.4 ميجابايت) — أو `BatteryDock-Windows-Portable.zip` للنسخة المحمولة.
2. شغّل المثبّت وفعّل اختيارياً **"Start BatteryDock when Windows boots"**.
3. ستظهر الكبسولة بجانب الساعة في شريط المهام.

### 🛡️ ملاحظة حول Windows SmartScreen
لأن BatteryDock تطبيق مفتوح المصدر ومجاني بدون شهادة توقيع EV مدفوعة، قد يعرض SmartScreen رسالة *"قام Windows بحماية جهاز الكمبيوتر"* عند التشغيل الأول (**معلومات إضافية ← التشغيل على أي حال**).
- يتم بناء كل إصدار علناً على خوادم GitHub Actions في [`.github/workflows/release.yml`](../../.github/workflows/release.yml).
- يمكنك فحص الكود المصدري بالكامل و[بناؤه بنفسك](#build-from-source) في أقل من دقيقة.

---

## 🎧 الأجهزة المدعومة

</div>

| الفئة | البروتوكول / المصدر | أمثلة وملاحظات |
| :--- | :--- | :--- |
| **أجهزة Logitech اللاسلكية** | HID++ 2.0 مباشر (`0x1004` / `0x1000`) و G HUB IPC | سماعات Lightspeed (G733, G535, G935)، ماوس (G PRO, Superlight, G502)، لوحات مفاتيح |
| **Bluetooth & BLE** | Windows WinRT PnP + BLE GATT (`0x180F` / `0x2A19`) | سماعات (Sony WH/WF, Bose, AirPods)، ماوس ولوحات مفاتيح BT |
| **أذرع تحكم Xbox** | Windows `XInput` | Xbox Wireless Controllers (Series X\|S, One, 360) مع كشف الشحن السلكي |
| **PlayStation** | Sony HID (`VID 0x054C`) USB و Bluetooth | DualSense و DualShock 4 (دعم تعدد الأذرع + حالة الشحن) |

<div dir="rtl">

**هل علامتك التجارية غير موجودة (Razer, Corsair, SteelSeries, Keychron...)؟**
👉 **[افتح طلب دعم جهاز](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** — نرحب أيضاً بطلبات السحب (PRs)!

---

## 🎯 التحكم والمؤشرات

### ألوان الحالة
- **أخضر:** بطارية جيدة (أعلى من الحد المنخفض)
- **كهرماني:** بطارية منخفضة (الافتراضي `20%`)
- **أحمر (نابض):** مستوى حرج (الافتراضي `10%`)
- **سماوي + ⚡:** الجهاز قيد الشحن حالياً

### التفاعل
- **نقر أيسر** على الكبسولة أو **`⚙`** (أو أيقونة شريط النظام): يفتح **BatteryDock Settings** (`BatteryDock.exe --settings`).
- **نقر أيمن** على الكبسولة أو أيقونة النظام: يفتح القائمة الأصلية (**Settings**, **Refresh Devices**, **Reset to Taskbar Clock**, **Restart BatteryDock**, **Exit BatteryDock**).
- **السحب والإفلات (> 5px):** انقل الكبسولة إلى أي مكان (بما في ذلك الشاشات الثانوية)؛ والنقر الأيمن ← **Reset to Taskbar Clock** يعيدها بجانب الساعة.

---

## ⚙️ الإعدادات

يتم حفظ الإعدادات تلقائياً في `%APPDATA%\jorjanion\BatteryDock\config\config.toml`:

</div>

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

<div dir="rtl">

## 🛠️ البناء من الكود المصدري

### المتطلبات
- [أدوات Rust](https://rustup.rs/) (stable)
- Windows 10 أو Windows 11 (MSVC)

### الأوامر

</div>

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
cargo build --release --locked
```

<div dir="rtl">

الملف الناتج: `target\release\battery-dock.exe` (مرر `--settings` أو `-s` لفتح الإعدادات مباشرة).

---

## 📄 الرخصة

مرخص بموجب **رخصة MIT**. راجع [LICENSE](../../LICENSE).

</div>

<div align="center">

**إذا أفادك BatteryDock، يسعدنا دعمك للمستودع بوضع نجمة ⭐!**

</div>
