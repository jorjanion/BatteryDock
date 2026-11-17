<div align="center" dir="rtl">

<img src="../../assets/logo.png" alt="BatteryDock Logo" width="148" style="border-radius: 24px;" />

# BatteryDock

**مستويات بطارية جميع أجهزتك اللاسلكية في شريط واحد خفيف على شريط مهام ويندوز. بدون برامج الشركات الثقيلة.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[تنزيل v1.0.0](https://github.com/jorjanion/BatteryDock/releases/latest) · [الإبلاغ عن خطأ](https://github.com/jorjanion/BatteryDock/issues/new) · [طلب دعم جهاز](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md) · [سجل التغييرات](../../CHANGELOG.md)

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

إذا كنت تستخدم فأرة لاسلكية من شركة وسماعة من شركة أخرى ويد تحكم بلوتوث، فمعرفة نسبة البطارية على ويندوز تتطلب عادةً تشغيل عدة برامج ثقيلة في الخلفية.

**BatteryDock** يستبدل كل ذلك بتطبيق واحد مكتوب بلغة Rust بحجم ~4 ميجابايت يعمل بهدوء على شريط مهام ويندوز 10/11. بدون Electron، بدون تتبع، وبدون حسابات.

---

## معاينة

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-single.png" alt="BatteryDock single peripheral pill" /><br /><br />
        <sub><b>جهاز واحد</b>: أيقونة واضحة ومؤشر بطارية ملون ونسبة مئوية مباشرة على شريط المهام.</sub>
      </td>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-multi.png" alt="BatteryDock multiple peripherals pill" /><br /><br />
        <sub><b>عدة أجهزة معاً</b>: أعد ترتيب الأجهزة أو غيّر أسماءها أو أخفِ ما لا تريد رؤيته.</sub>
      </td>
    </tr>
  </table>
  <br />
  <table>
    <tr>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-peripherals.jpg" alt="BatteryDock Peripherals dashboard v1.0.0" /><br />
        <sub><b>الأجهزة</b>: تقدير الوقت المتبقي، رسم بياني لآخر 24 ساعة، أسماء مخصصة وتنبيهات لكل جهاز.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-alerts.jpg" alt="BatteryDock Alerts and Audio settings v1.0.0" /><br />
        <sub><b>التنبيهات والصوت</b>: إشعارات البطارية المنخفضة والحرجة واكتمال الشحن 100% مع تجربة الصوت.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-customization.jpg" alt="BatteryDock Customization settings v1.0.0" /><br />
        <sub><b>التخصيص</b>: سمات Liquid Glass / داكن / فاتح / النظام، تراكب الألعاب HUD، اختصار عالمي و21 لغة.</sub>
      </td>
    </tr>
  </table>
</div>

---

## المميزات

### شريط بسيط على شريط المهام
- **4 سمات بصرية:** اختر بين **Liquid Glass** (شفاف)، **داكن**، **فاتح**، أو **النظام** (يتبع سمة ويندوز تلقائياً).
- **بدون ازدحام:** النقر الأيسر يفتح الإعدادات، النقر الأيمن يفتح القائمة السريعة، ويمكنك سحبه لأي شاشة.
- **سلوك ذكي:** يبقى فوق شريط المهام حتى عند فتح قائمة ابدأ أو البحث، ويختفي تلقائياً أثناء الفيديو والألعاب بملء الشاشة.
- **تخطيط مخصص:** أعد ترتيب الأجهزة بنقرة واحدة، أضف أسماء مخصصة، أخفِ أجهزة معينة أو أظهر/أخفِ النسبة المئوية.

### تراكب اختياري للألعاب وملء الشاشة (HUD)
- **معطّل افتراضياً** حتى تبقى الأفلام ويوتيوب بملء الشاشة دون أي عائق.
- عند التفعيل، يظهر تلقائياً في زاوية الشاشة أثناء الألعاب بوضع ملء الشاشة بدون حدود.
- **ضعه في أي مكان:** 4 زوايا مع هامش قابل للتعديل أو موضع حر X/Y (من 0% إلى 100%) مع معاينة مباشرة.
- **تجاهل النقرات (Click-through):** تمر نقرات الفأرة مباشرة إلى اللعبة.
- **اختصار لوحة مفاتيح عالمي:** عيّن أي اختصار (الافتراضي `Alt + B`) لإظهار أو إخفاء التراكب أثناء اللعب دون Alt+Tab.

### مراقبة ذكية وتنبيهات
- **سجل البطارية لآخر 24 ساعة:** تعرض كل بطاقة جهاز رسماً بيانياً خفيفاً للشحن والتفريغ.
- **الوقت المتبقي ووضع السكون:** يقدر الوقت المتبقي للاستخدام أو اكتمال الشحن ويكتشف الأجهزة الخاملة.
- **تنبيهات الانخفاض والخطر والشحن 100%:** حدود عامة أو لكل جهاز، كتم أجهزة معينة، نغمة لطيفة وإشعارات ويندوز الأصلية.

### 21 لغة مدمجة
- يكتشف لغة نظام ويندوز تلقائياً عند التشغيل الأول (أو يستخدم الإنجليزية إذا لم تكن مدعومة).
- يحمّل خطوط ويندوز الأصلية عند الحاجة لعرض واضح لكل اللغات.

---

## الأجهزة المدعومة

يتصل BatteryDock مباشرة بأجهزتك عبر HID وWinRT والبلوتوث دون الحاجة لبرامج الشركات.

| الفئة | البروتوكول / الطريقة | أمثلة مدعومة |
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

## التنزيل والتثبيت

احصل على أحدث إصدار من **[صفحة الإصدارات](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. نزّل **`BatteryDock-Setup-v1.0.0.exe`** (المثبّت) أو **`BatteryDock-Windows-Portable.zip`** للنسخة المحمولة.
2. شغّله ويمكنك تفعيل التشغيل التلقائي مع ويندوز من الإعدادات.
3. سيظهر الشريط بجانب الساعة مع تحديثات تلقائية من GitHub.

### ملاحظة حول Windows SmartScreen
لأن BatteryDock مشروع مجاني مفتوح المصدر بدون شهادة توقيع مدفوعة، قد يعرض SmartScreen تحذيراً عند التشغيل الأول. اضغط **معلومات إضافية** ثم **التشغيل على أي حال**. يتم بناء كل إصدار علناً في [`.github/workflows/release.yml`](../../.github/workflows/release.yml).

---

## الإعدادات

تُحفظ الإعدادات في `%APPDATA%\jorjanion\BatteryDock\config\config.toml` (والسجل في `history.toml`) وتُطبق فوراً بدون إعادة تشغيل.

---

## البناء من الكود المصدري

المتطلبات: Windows 10/11 و[Rust](https://rustup.rs/) (نسخة MSVC المستقرة).

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock
cargo test --locked
cargo build --release --locked
.\target\release\battery-dock.exe
```

---

## المساهمة والمشكلات وطلبات الأجهزة

BatteryDock مفتوح المصدر ومرحب بمساهمات المجتمع. نرحب بتقارير الأخطاء والترجمات وطلبات الأجهزة وPull Requests.

- **جهازك لا يظهر؟** افتح **[طلب دعم جهاز](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)**. افتح **الإعدادات > التخصيص > نسخ تشخيص الأجهزة** والصق التقرير المجهول مباشرة في الطلب.
- **وجدت خطأ أو لديك فكرة؟** افتح **[Issue](https://github.com/jorjanion/BatteryDock/issues)** مع إصدار ويندوز وخطوات التكرار.
- **تريد المساهمة بالكود؟** نرحب بـ Pull Requests. شغّل `cargo test --locked` و`cargo clippy --locked` قبل الإرسال.

---

## الترخيص

ترخيص MIT. انظر [LICENSE](../../LICENSE).
