<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock लोगो" width="160" style="border-radius: 24px;" />

# BatteryDock

**Windows 10/11 टास्कबार में सीधे एम्बेड किया गया Rust में बना हल्का बैटरी इंडिकेटर — बिना भारी कंपनी सॉफ्टवेयर के।**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[नवीनतम इंस्टॉलर डाउनलोड करें](https://github.com/jorjanion/BatteryDock/releases/latest) · [बग रिपोर्ट करें](https://github.com/jorjanion/BatteryDock/issues) · [डिवाइस सपोर्ट का अनुरोध करें](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)

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
<a href="README.ar.md">العربية</a> ·
<b>हिन्दी</b> ·
<a href="README.id.md">Bahasa Indonesia</a> ·
<a href="README.vi.md">Tiếng Việt</a> ·
<a href="README.ja.md">日本語</a> ·
<a href="README.ko.md">한국어</a> ·
<a href="README.zh-CN.md">简体中文</a> ·
<a href="README.zh-TW.md">繁體中文</a>
</sub>

</div>

---

## 📸 झलक

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="42%">
        <img src="../screenshots/dock-closeup.png" alt="BatteryDock कैप्सूल का क्लोज़-अप जिसमें हेडसेट की बैटरी 92% दिख रही है" /><br /><br />
        <sub><b>टास्कबार कैप्सूल</b> — Per-Monitor DPI स्केलिंग, बैटरी रंग और लाइव ⚡ चार्जिंग बैज के साथ सीधे Windows टास्कबार में।</sub>
      </td>
      <td align="center" valign="middle" width="58%">
        <img src="../screenshots/settings.png" alt="BatteryDock सेटिंग्स विंडो v0.1.11" width="340" /><br />
        <sub><b>स्टैंडअलोन सेटिंग्स</b> — अलर्ट सीमाएं, साउंड प्रीव्यू, पोजिशन मोड और कनेक्टेड डिवाइस सूची।</sub>
      </td>
    </tr>
  </table>
</div>

---

## ⚡ BatteryDock क्यों?

ज़्यादातर वायरलेस डिवाइस बैटरी प्रतिशत को भारी-भरकम निर्माता सॉफ़्टवेयर में छिपा देते हैं जो बैकग्राउंड में दर्जनों सर्विस चलाते हैं। **BatteryDock** सब कुछ टास्कबार की एक साफ़ कैप्सूल में दिखाता है:

- **कोई ब्लोटवेयर नहीं:** केवल **~4 MB** की एक नेटिव Rust बाइनरी (बिना Electron, बिना टेलीमेट्री, Windows 11 पर लाइव ~77 MB प्राइवेट मेमोरी)।
- **सीधा हार्डवेयर रीडिंग:** **Logitech HID++ 2.0**, **Windows Bluetooth / BLE GATT**, **XInput** और **Sony DualSense / DualShock 4 HID** से सीधे पढ़ता है — Logitech डिवाइस बिना G HUB खोले भी काम करते हैं।
- **स्मार्ट फ़ुलस्क्रीन नियम:** फ़ुलस्क्रीन गेम, वीडियो या ऑटो-हाइड टास्कबार होने पर अपने-आप छिप जाता है, और `Win` कुंजी दबाने, Start/Widgets खोलने या डेस्कटॉप क्लिक करने पर दिखता रहता है।
- **लाइव चार्जिंग इंडिकेटर:** चार्ज होते समय डिवाइस को **सियान** रंग और **⚡ बिजली आइकन** से दिखाता है।
- **बैकग्राउंड अलर्ट:** गेम के दौरान कैप्सूल छिपी होने पर भी लो-बैटरी साउंड और डेस्कटॉप नोटिफिकेशन चलते रहते हैं।

---

## 📥 डाउनलोड और इंस्टॉलेशन

**[Releases पेज](https://github.com/jorjanion/BatteryDock/releases/latest)** से नवीनतम वर्ज़न डाउनलोड करें:

1. **`BatteryDock-Setup-v0.1.11.exe`** (~4.4 MB) डाउनलोड करें — या पोर्टेबल उपयोग के लिए `BatteryDock-Windows-Portable.zip` चुनें।
2. इंस्टॉलर चलाएँ और चाहें तो **"Start BatteryDock when Windows boots"** चुनें।
3. टास्कबार में घड़ी के पास कैप्सूल दिखाई देगी।

### 🛡️ Windows SmartScreen सूचना
चूँकि BatteryDock बिना महंगे EV कोड-साइनिंग सर्टिफिकेट के एक मुफ़्त ओपन-सोर्स ऐप है, पहली बार चलाने पर SmartScreen *"Windows protected your PC"* दिखा सकता है (**More info → Run anyway** चुनें)।
- हर रिलीज़ [`.github/workflows/release.yml`](../../.github/workflows/release.yml) में GitHub Actions द्वारा सार्वजनिक रूप से बिल्ड होती है।
- आप पूरा Rust कोड देख सकते हैं और एक मिनट से कम में [स्वयं बिल्ड](#build-from-source) कर सकते हैं।

---

## 🎧 समर्थित हार्डवेयर

| श्रेणी | प्रोटोकॉल / स्रोत | उदाहरण व नोट्स |
| :--- | :--- | :--- |
| **Logitech वायरलेस** | डायरेक्ट HID++ 2.0 (`0x1004` / `0x1000`) & G HUB IPC | Lightspeed हेडसेट (G733, G535, G935), माउस (G PRO, Superlight, G502), कीबोर्ड |
| **Bluetooth & BLE** | Windows WinRT PnP + BLE GATT (`0x180F` / `0x2A19`) | हेडफ़ोन (Sony WH/WF, Bose, AirPods), BT माउस और कीबोर्ड |
| **Xbox कंट्रोलर** | Windows `XInput` | Xbox Wireless Controllers (Series X\|S, One, 360) वायर्ड चार्जिंग सपोर्ट सहित |
| **PlayStation** | Sony HID (`VID 0x054C`) USB और Bluetooth | DualSense व DualShock 4 (मल्टी-कंट्रोलर + चार्जिंग स्थिति) |

**आपका ब्रांड मौजूद नहीं है (Razer, Corsair, SteelSeries, Keychron आदि)?**
👉 **[डिवाइस सपोर्ट अनुरोध खोलें](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** — Pull Requests का भी स्वागत है!

---

## 🎯 नियंत्रण और संकेतक

### स्थिति रंग
- **हरा:** स्वस्थ बैटरी (कम सीमा से ऊपर)
- **एम्बर:** कम बैटरी (डिफ़ॉल्ट `20%`)
- **लाल (पल्सिंग):** गंभीर स्तर (डिफ़ॉल्ट `10%`)
- **सियान + ⚡:** डिवाइस अभी चार्ज हो रहा है

### उपयोग
- कैप्सूल या **`⚙`** (या ट्रे आइकन) पर **बायाँ क्लिक:** **BatteryDock Settings** खोलता है (`BatteryDock.exe --settings`)।
- कैप्सूल या ट्रे आइकन पर **दायाँ क्लिक:** नेटिव मेनू (**Settings**, **Refresh Devices**, **Reset to Taskbar Clock**, **Restart BatteryDock**, **Exit BatteryDock**)।
- **क्लिक करके खींचें (> 5px):** कहीं भी ले जाएँ (दूसरे मॉनिटर पर भी); दायाँ क्लिक → **Reset to Taskbar Clock** से वापस घड़ी के पास लाएँ।

---

## ⚙️ कॉन्फ़िगरेशन

सेटिंग्स `%APPDATA%\jorjanion\BatteryDock\config\config.toml` में सहेजी जाती हैं:

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

## 🛠️ सोर्स कोड से बिल्ड करें

### आवश्यकताएँ
- [Rust toolchain](https://rustup.rs/) (stable)
- Windows 10 या Windows 11 (MSVC)

### कमांड

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
cargo build --release --locked
```

निर्मित फ़ाइल: `target\release\battery-dock.exe` (`--settings` / `-s` से सीधे सेटिंग्स खुलती हैं)।

---

## 📄 लाइसेंस

**MIT लाइसेंस** के अंतर्गत वितरित। विवरण के लिए [LICENSE](../../LICENSE) देखें।

<div align="center">

**यदि BatteryDock आपके काम आया हो, तो रिपॉज़िटरी को ⭐ ज़रूर दें!**

</div>
