<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock Logo" width="148" style="border-radius: 24px;" />

# BatteryDock

**Tüm kablosuz çevre birimlerinin pil seviyesi Windows görev çubuğunda tek ve hafif bir göstergede. Üretici yazılımı yok.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[v1.0.0 İndir](https://github.com/jorjanion/BatteryDock/releases/latest) · [Hata Bildir](https://github.com/jorjanion/BatteryDock/issues/new) · [Cihaz Desteği İste](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md) · [Değişiklikler](../../CHANGELOG.md)

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

Farklı markalardan kablosuz fare, kulaklık ve Bluetooth cihazlar kullanıyorsanız Windows'ta pil görmek için arka planda birkaç ağır yazılım çalıştırmak zorunda kalabilirsiniz.

**BatteryDock** bunların hepsini Windows 10/11 görev çubuğuna yerleşen ~4 MB boyutunda tek bir yerel Rust uygulamasıyla değiştirir. Electron yok, telemetri yok, hesap yok.

---

## Önizleme

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-single.png" alt="BatteryDock single peripheral pill" /><br /><br />
        <sub><b>Tek cihaz</b>: Net vektör simgesi, renkli pil çubuğu ve görev çubuğunda doğrudan yüzde.</sub>
      </td>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-multi.png" alt="BatteryDock multiple peripherals pill" /><br /><br />
        <sub><b>Birden fazla cihaz</b>: Cihazları yeniden sıralayın, adlandırın veya istemediklerinizi gizleyin.</sub>
      </td>
    </tr>
  </table>
  <br />
  <table>
    <tr>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-peripherals.jpg" alt="BatteryDock Peripherals dashboard v1.0.0" /><br />
        <sub><b>Cihazlar</b>: Tahmini pil ömrü, 24 saatlik pil grafiği, özel adlar ve cihaz bazlı uyarılar.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-alerts.jpg" alt="BatteryDock Alerts and Audio settings v1.0.0" /><br />
        <sub><b>Uyarılar ve Ses</b>: Düşük, kritik ve %100 tam şarj bildirimleri ve ses önizlemesi.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-customization.jpg" alt="BatteryDock Customization settings v1.0.0" /><br />
        <sub><b>Özelleştirme</b>: Liquid Glass / Koyu / Açık / Sistem temaları, oyun HUD katmanı, kısayol ve 21 dil.</sub>
      </td>
    </tr>
  </table>
</div>

---

## Özellikler

### Minimalist görev çubuğu göstergesi
- **4 görsel tema:** **Liquid Glass** (şeffaf), **Koyu**, **Açık** veya **Sistem** (Windows temasını otomatik izler).
- **Sade kullanım:** Sol tık Ayarları açar, sağ tık hızlı menüyü gösterir, sürükleyerek istediğiniz ekrana koyabilirsiniz.
- **Akıllı görev çubuğu uyumu:** Başlat veya Arama açıkken bile görünür kalır, tam ekran video ve oyunlarda otomatik gizlenir.
- **Esnek düzen:** Tek tıkla cihaz sırasını değiştirin, takma ad verin, cihaz gizleyin veya yüzde yazısını açıp kapatın.

### İsteğe bağlı Oyun ve Tam Ekran Katmanı (HUD)
- **Varsayılan olarak kapalıdır**, böylece tam ekran filmler ve YouTube kesintisiz kalır.
- Açıldığında çerçevesiz tam ekran oyunlarda ekranın bir köşesinde pil durumunu gösterir.
- **İstediğiniz yere yerleştirin:** Ayarlanabilir kenar boşluklu 4 köşe veya canlı önizlemeli serbest X/Y (%0 - %100).
- **Click-through modu:** Fare tıklamaları katmandan geçerek doğrudan oyuna ulaşır.
- **Yapılandırılabilir genel kısayol:** Alt+Tab yapmadan katmanı açıp kapatmak için herhangi bir tuş atayın (varsayılan `Alt + B`).

### Akıllı telemetri ve uyarılar
- **24 saatlik pil geçmişi:** Her cihaz kartı şarj ve deşarj eğrisini yerel olarak çizer.
- **Süre tahmini ve uyku algılama:** Kalan kullanım veya dolum süresini tahmin eder ve uyku modunu algılar.
- **Düşük, kritik ve %100 şarj uyarıları:** Genel veya cihaza özel eşikler, cihaz bazlı sessize alma, hafif ses ve Windows bildirimleri.

### Dahili 21 dil
- İlk açılışta Windows dilinizi otomatik algılar ve destekleniyorsa uygular (aksi halde İngilizce).
- Tüm alfabelerin net görünmesi için Windows sistem yazı tiplerini gerektiğinde otomatik yükler.

---

## Desteklenen Donanım

BatteryDock üretici yazılımına gerek kalmadan HID, WinRT ve Bluetooth üzerinden doğrudan donanımla konuşur.

| Kategori | Protokol / Yöntem | Desteklenen Örnekler |
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

## İndirme ve Kurulum

En son sürümü **[Releases sayfasından](https://github.com/jorjanion/BatteryDock/releases/latest)** indirin:

1. **`BatteryDock-Setup-v1.0.0.exe`** (kurulum) veya **`BatteryDock-Windows-Portable.zip`** (taşınabilir) dosyasını indirin.
2. Çalıştırın ve isterseniz Ayarlar'dan Windows ile başlatmayı etkinleştirin.
3. Gösterge saatin yanında belirir ve dahili GitHub güncelleyici sizi güncel tutar.

### Windows SmartScreen Notu
Ücretsiz ve açık kaynaklı bir proje olduğundan SmartScreen ilk açılışta uyarı gösterebilir. **Ek bilgi** ve ardından **Yine de çalıştır** seçeneğine tıklayın. Tüm sürümler [`.github/workflows/release.yml`](../../.github/workflows/release.yml) üzerinden açıkça derlenir.

---

## Yapılandırma

Ayarlar `%APPDATA%\jorjanion\BatteryDock\config\config.toml` dosyasında (geçmiş `history.toml`) saklanır ve anında uygulanır.

---

## Kaynaktan Derleme

Gereksinimler: Windows 10/11 ve [Rust](https://rustup.rs/) (kararlı MSVC).

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock
cargo test --locked
cargo build --release --locked
.\target\release\battery-dock.exe
```

---

## Katkı, Issue ve Pull Request

BatteryDock açık kaynaklı ve topluluk odaklıdır. Hata bildirimleri, çeviriler, cihaz istekleri ve PR'lar her zaman açıktır.

- **Cihazınız görünmüyor mu?** Bir **[cihaz desteği isteği](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** açın. **Ayarlar > Özelleştirme > Donanım Tanılamasını Kopyala** ile anonim raporu tek tıkla kopyalayıp yapıştırabilirsiniz.
- **Hata veya fikir mi var?** Windows sürümünüzle birlikte bir **[Issue](https://github.com/jorjanion/BatteryDock/issues)** açın.
- **Kod katkısı mı yapmak istiyorsunuz?** Pull Request'ler memnuniyetle karşılanır. Göndermeden önce `cargo test --locked` ve `cargo clippy --locked` çalıştırın.

---

## Lisans

MIT Lisansı. Detaylar için [LICENSE](../../LICENSE).
