<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock Logosu" width="160" style="border-radius: 24px;" />

# BatteryDock

**Windows 10/11 görev çubuğuna doğrudan gömülü, Rust ile yazılmış sade bir pil göstergesi — üretici yazılımlarının ağırlığı olmadan.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Son Yükleyiciyi İndir](https://github.com/jorjanion/BatteryDock/releases/latest) · [Hata Bildir](https://github.com/jorjanion/BatteryDock/issues) · [Cihaz Desteği İste](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)

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
<b>Türkçe</b> ·
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

## 📸 Önizleme

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="42%">
        <img src="../screenshots/dock-closeup.png" alt="Kulaklık pilini %92 gösteren BatteryDock kapsülü yakından" /><br /><br />
        <sub><b>Görev Çubuğu Kapsülü</b> — Windows görev çubuğuna doğrudan gömülü, monitör başına DPI ölçekleme ve canlı ⚡ şarj simgesi ile.</sub>
      </td>
      <td align="center" valign="middle" width="58%">
        <img src="../screenshots/settings.png" alt="BatteryDock Ayarlar penceresi v0.1.11" width="340" /><br />
        <sub><b>Bağımsız Ayarlar</b> — Uyarı eşikleri, ses önizlemesi, konum modları ve bağlı cihaz listesi.</sub>
      </td>
    </tr>
  </table>
</div>

---

## ⚡ Neden BatteryDock?

Kablosuz çevre birimlerinin çoğu pil yüzdesini arka planda onlarca servis çalıştıran hantal üretici yazılımlarının içine hapseder. **BatteryDock** tüm cihazlarınızı tek bir şık kapsülde toplar:

- **Sıfır Bloatware:** Tek bir **~4 MB** yerel Rust ikili dosyası (Electron yok, telemetri yok, Windows 11'de canlı ~77 MB özel bellek).
- **Doğrudan Donanım Okuma:** **Logitech HID++ 2.0**, **Windows Bluetooth / BLE GATT**, **XInput** ve **Sony DualSense / DualShock 4 HID**'yi doğrudan sorgular — Logitech cihazlar G HUB açılmadan bile rapor verebilir (açıksa G HUB IPC de desteklenir).
- **Akıllı Tam Ekran Kuralları:** Tam ekran oyunlar, videolar veya otomatik gizlenen görev çubuklarında kendini gizler; `Win` tuşuna basıldığında, Başlat/Widget açıldığında veya masaüstüne tıklandığında görünür kalır.
- **Canlı Şarj Göstergesi:** Şarj olan cihazları **camgöbeği** renkte ve **⚡ şimşek simgesiyle** vurgular.
- **Her Zaman Çalışan Arka Plan Uyarıları:** Kapsül oyunda gizliyken bile düşük ve kritik pil bildirimleri ile sesli uyarılar arka plandan tetiklenir.

---

## 📥 İndirme ve Kurulum

**[Releases Sayfasından](https://github.com/jorjanion/BatteryDock/releases/latest)** son sürümü indirin:

1. **`BatteryDock-Setup-v0.1.11.exe`** (~4,4 MB) dosyasını indirin — veya kurulumsuz kullanım için `BatteryDock-Windows-Portable.zip` seçin.
2. Yükleyiciyi çalıştırın ve isterseniz **"Start BatteryDock when Windows boots"** kutusunu işaretleyin.
3. Kapsül görev çubuğunda saatin yanında belirecektir.

### 🛡️ Windows SmartScreen Notu
BatteryDock ücretli EV imza sertifikasına sahip olmayan ücretsiz bir açık kaynak uygulaması olduğu için ilk çalıştırmada *"Windows kişisel bilgisayarınızı korudu"* uyarısı çıkabilir (**Ek bilgi → Yine de çalıştır**).
- Her sürüm [`.github/workflows/release.yml`](../../.github/workflows/release.yml) üzerinden temiz GitHub Actions sunucularında herkese açık derlenir.
- Tüm Rust kodunu inceleyip bir dakikadan kısa sürede [kendiniz derleyebilirsiniz](#build-from-source).

---

## 🎧 Desteklenen Donanımlar

| Kategori | Protokol / Kaynak | Örnekler ve Notlar |
| :--- | :--- | :--- |
| **Logitech Kablosuz** | Doğrudan HID++ 2.0 (`0x1004` / `0x1000`) & G HUB IPC | Lightspeed kulaklıklar (G733, G535, G935), fareler (G PRO, Superlight, G502), klavyeler |
| **Bluetooth & BLE** | Windows WinRT PnP + BLE GATT (`0x180F` / `0x2A19`) | Kulaklıklar (Sony WH/WF, Bose, AirPods), BT fare ve klavyeler |
| **Xbox Kontrolcüleri** | Windows `XInput` | Xbox Wireless Controllers (Series X\|S, One, 360) kablolu şarj algılama dahil |
| **PlayStation** | Sony HID (`VID 0x054C`) USB & Bluetooth | DualSense & DualShock 4 (çoklu kontrolcü + şarj durumu) |

**Markanız eksik mi (Razer, Corsair, SteelSeries, Keychron vb.)?**
👉 **[Cihaz Desteği Talebi Açın](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** — Pull Request katkıları da memnuniyetle karşılanır!

---

## 🎯 Kontroller ve Göstergeler

### Durum Renkleri
- **Yeşil:** Sağlıklı pil (düşük eşiğin üzerinde)
- **Kehribar:** Düşük pil (varsayılan `20%`)
- **Kırmızı (yanıp sönen):** Kritik seviye (varsayılan `10%`)
- **Camgöbeği + ⚡:** Cihaz şu anda şarj oluyor

### Etkileşim
- Kapsüle veya **`⚙`** simgesine **sol tık** (veya sistem tepsisi simgesi): **BatteryDock Settings** penceresini açar (`BatteryDock.exe --settings`).
- Kapsül veya sistem tepsisine **sağ tık**: Yerel menüyü açar (**Settings**, **Refresh Devices**, **Reset to Taskbar Clock**, **Restart BatteryDock**, **Exit BatteryDock**).
- **Tıkla ve sürükle (> 5px):** İstediğiniz yere taşıyın (ikinci monitörler dahil); sağ tık → **Reset to Taskbar Clock** ile saate geri döndürün.

---

## ⚙️ Yapılandırma

Ayarlar `%APPDATA%\jorjanion\BatteryDock\config\config.toml` konumuna kaydedilir:

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

## 🛠️ Kaynak Koddan Derleme

### Gereksinimler
- [Rust araç zinciri](https://rustup.rs/) (stable)
- Windows 10 veya Windows 11 (MSVC)

### Komutlar

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

# Biçimlendirme, clippy ve testleri doğrulayın
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked

# Optimize edilmiş release sürümünü derleyin
cargo build --release --locked
```

Çıktı dosyası: `target\release\battery-dock.exe` (doğrudan ayarları açmak için `--settings` / `-s` kullanın).

---

## 📄 Lisans

**MIT Lisansı** altında dağıtılır. Ayrıntılar için [LICENSE](../../LICENSE).

<div align="center">

**BatteryDock işinize yaradıysa depoya bir ⭐ bırakmayı düşünebilirsiniz!**

</div>
