<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock Logo" width="148" style="border-radius: 24px;" />

# BatteryDock

**Semua baterai perangkat nirkabel Anda dalam satu kapsul native di taskbar Windows. Tanpa aplikasi bawaan yang berat.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Unduh v1.0.0](https://github.com/jorjanion/BatteryDock/releases/latest) · [Laporkan Bug](https://github.com/jorjanion/BatteryDock/issues/new) · [Minta Dukungan Perangkat](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md) · [Changelog](../../CHANGELOG.md)

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
<a href="README.hi.md">हिन्दी</a> ·
<b>Bahasa Indonesia</b> ·
<a href="README.vi.md">Tiếng Việt</a> ·
<a href="README.ja.md">日本語</a> ·
<a href="README.ko.md">한국어</a> ·
<a href="README.zh-CN.md">简体中文</a> ·
<a href="README.zh-TW.md">繁體中文</a>
</sub>

</div>

---

Jika Anda memakai mouse nirkabel, headset, dan kontroler Bluetooth dari merek berbeda, mengecek baterai di Windows sering kali berarti menjalankan beberapa aplikasi berat di latar belakang.

**BatteryDock** menggantikan semuanya dengan satu aplikasi native Rust berukuran ~4 MB di taskbar Windows 10/11. Tanpa Electron, tanpa telemetri, tanpa akun.

---

## Pratinjau

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-single.png" alt="BatteryDock single peripheral pill" /><br /><br />
        <sub><b>Satu perangkat</b>: Ikon vektor tajam, bilah baterai berwarna, dan persentase langsung di taskbar.</sub>
      </td>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-multi.png" alt="BatteryDock multiple peripherals pill" /><br /><br />
        <sub><b>Banyak perangkat sekaligus</b>: Urutkan perangkat, ubah nama, atau sembunyikan yang tidak diperlukan.</sub>
      </td>
    </tr>
  </table>
  <br />
  <table>
    <tr>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-peripherals.jpg" alt="BatteryDock Peripherals dashboard v1.0.0" /><br />
        <sub><b>Perangkat</b>: Estimasi daya tahan, grafik 24 jam, nama panggilan, dan peringatan per perangkat.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-alerts.jpg" alt="BatteryDock Alerts and Audio settings v1.0.0" /><br />
        <sub><b>Peringatan & Audio</b>: Notifikasi baterai lemah, kritis, dan penuh 100% dengan tes suara.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-customization.jpg" alt="BatteryDock Customization settings v1.0.0" /><br />
        <sub><b>Kustomisasi</b>: Tema Liquid Glass / Gelap / Terang / Sistem, HUD game, pintasan global, dan 21 bahasa.</sub>
      </td>
    </tr>
  </table>
</div>

---

## Fitur Utama

### Kapsul taskbar minimalis
- **4 tema tampilan:** **Liquid Glass** (transparan), **Gelap**, **Terang**, atau **Sistem** (mengikuti tema Windows).
- **Bersih dan praktis:** Klik kiri membuka Pengaturan, klik kanan membuka menu cepat, atau seret ke monitor mana saja.
- **Perilaku taskbar cerdas:** Tetap tampil di atas taskbar meski Start atau Search dibuka, dan otomatis tersembunyi saat video atau game layar penuh.
- **Tata letak fleksibel:** Ubah urutan perangkat sekali klik, beri nama khusus, sembunyikan perangkat, atau tampilkan/sembunyikan persentase.

### Overlay Game & Layar Penuh opsional (HUD)
- **Nonaktif secara default** agar film dan YouTube layar penuh tetap bersih.
- Saat diaktifkan, baterai tampil di sudut layar selama bermain game borderless fullscreen.
- **Posisi bebas:** 4 sudut dengan jarak tepi yang bisa diatur atau posisi bebas X/Y (0% - 100%) dengan pratinjau langsung.
- **Mode Click-through:** Klik mouse menembus HUD langsung ke game.
- **Pintasan global:** Atur tombol apa saja (default `Alt + B`) untuk menampilkan atau menyembunyikan HUD tanpa Alt+Tab.

### Telemetri cerdas & peringatan
- **Riwayat 24 jam:** Setiap kartu menampilkan grafik lokal pengisian dan pemakaian baterai.
- **Estimasi waktu & mode tidur:** Memperkirakan sisa waktu atau waktu isi penuh dan mendeteksi perangkat tidur.
- **Peringatan lemah, kritis, dan penuh 100%:** Batas global atau per perangkat, bisukan per perangkat, suara lembut, dan notifikasi Windows.

### 21 bahasa bawaan
- Otomatis mendeteksi bahasa Windows saat pertama dijalankan (default Inggris jika belum didukung).
- Memuat font sistem Windows sesuai kebutuhan agar setiap aksara tampil tajam.

---

## Perangkat Keras yang Didukung

BatteryDock berkomunikasi langsung lewat HID, WinRT, dan Bluetooth tanpa software vendor.

| Kategori | Protokol / Metode | Contoh yang Didukung |
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

## Unduh & Instalasi

Dapatkan versi terbaru di **[halaman Releases](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Unduh **`BatteryDock-Setup-v1.0.0.exe`** (installer) atau **`BatteryDock-Windows-Portable.zip`** (portabel).
2. Jalankan dan aktifkan opsi mulai otomatis bersama Windows jika diinginkan.
3. Indikator akan muncul di samping jam dengan pembaruan otomatis GitHub.

### Catatan Windows SmartScreen
Karena BatteryDock adalah aplikasi open-source gratis tanpa sertifikat berbayar, SmartScreen mungkin menampilkan peringatan pertama kali. Klik **More info** lalu **Run anyway**. Semua rilis dibangun secara publik di [`.github/workflows/release.yml`](../../.github/workflows/release.yml).

---

## Konfigurasi

Pengaturan disimpan di `%APPDATA%\jorjanion\BatteryDock\config\config.toml` (riwayat di `history.toml`) dan langsung diterapkan.

---

## Bangun dari Kode Sumber

Persyaratan: Windows 10/11 dan [Rust](https://rustup.rs/) (MSVC stabil).

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock
cargo test --locked
cargo build --release --locked
.\target\release\battery-dock.exe
```

---

## Kontribusi, Issue & Pull Request

BatteryDock bersifat open source dan terbuka untuk komunitas. Laporan bug, terjemahan, permintaan perangkat, dan PR sangat diterima.

- **Perangkat tidak terdeteksi?** Buka **[permintaan dukungan perangkat](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)**. Klik **Pengaturan > Kustomisasi > Salin Diagnostik Perangkat Keras** lalu tempel laporan anonimnya.
- **Bug atau ide baru?** Buka **[Issue](https://github.com/jorjanion/BatteryDock/issues)** beserta versi Windows Anda.
- **Ingin berkontribusi kode?** Pull Request sangat diterima. Jalankan `cargo test --locked` dan `cargo clippy --locked` sebelum mengirim.

---

## Lisensi

Lisensi MIT. Lihat [LICENSE](../../LICENSE).
