<div align="center">

<img src="../../assets/logo.png" alt="Logo BatteryDock" width="160" style="border-radius: 24px;" />

# BatteryDock

**Indikator baterai Rust yang ringan dan native langsung di taskbar Windows 10/11 — tanpa bloatware pabrikan.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Unduh Installer Terbaru](https://github.com/jorjanion/BatteryDock/releases/latest) · [Laporkan Bug](https://github.com/jorjanion/BatteryDock/issues) · [Minta Dukungan Perangkat](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)

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

## 📸 Pratinjau

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="42%">
        <img src="../screenshots/dock-closeup.png" alt="Tampilan dekat kapsul BatteryDock menampilkan baterai headset 92%" /><br /><br />
        <sub><b>Kapsul Taskbar</b> — Terintegrasi langsung di taskbar Windows dengan skalabilitas DPI per monitor dan lencana pengisian ⚡.</sub>
      </td>
      <td align="center" valign="middle" width="58%">
        <img src="../screenshots/settings.png" alt="Jendela Pengaturan BatteryDock v0.1.11" width="340" /><br />
        <sub><b>Pengaturan Mandiri</b> — Ambang peringatan, pratinjau suara, mode posisi, dan daftar perangkat terhubung.</sub>
      </td>
    </tr>
  </table>
</div>

---

## ⚡ Mengapa BatteryDock?

Sebagian besar periferal nirkabel mengunci informasi baterai di dalam suite pabrikan berat yang menjalankan puluhan layanan latar belakang. **BatteryDock** menyatukan semuanya dalam satu kapsul bersih di taskbar:

- **Tanpa Bloatware:** Satu file biner native Rust berukuran **~4 MB** (tanpa Electron, tanpa telemetri, ~77 MB memori privat di Windows 11).
- **Pembacaan Hardware Langsung:** Mengakses **Logitech HID++ 2.0**, **Windows Bluetooth / BLE GATT**, **XInput**, dan **Sony DualSense / DualShock 4 HID** secara langsung — perangkat Logitech tetap terbaca meski G HUB tidak dibuka (mendukung IPC G HUB juga).
- **Aturan Layar Penuh Cerdas:** Otomatis bersembunyi saat game/video layar penuh atau taskbar tersembunyi, namun tetap stabil saat menekan tombol `Win`, membuka Start/Widget, atau klik desktop.
- **Indikator Pengisian Daya Langsung:** Menandai perangkat yang sedang diisi dengan warna **sian** dan **ikon petir ⚡**.
- **Peringatan Latar Belakang:** Suara lembut dan notifikasi Windows tetap aktif meski kapsul sedang tersembunyi di layar penuh.

---

## 📥 Unduh & Instalasi

Unduh rilis terbaru dari **[Halaman Releases](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Unduh **`BatteryDock-Setup-v0.1.11.exe`** (~4,4 MB) — atau `BatteryDock-Windows-Portable.zip` untuk versi portabel.
2. Jalankan installer dan centang **"Start BatteryDock when Windows boots"** jika diperlukan.
3. Kapsul akan muncul di taskbar sebelah jam sistem.

### 🛡️ Catatan Windows SmartScreen
Karena BatteryDock adalah aplikasi open-source gratis tanpa sertifikat penandatanganan EV berbayar, Windows SmartScreen mungkin menampilkan *"Windows protected your PC"* saat pertama dijalankan (**More info → Run anyway**).
- Setiap rilis dikompilasi secara publik di runner Windows GitHub Actions lewat [`.github/workflows/release.yml`](../../.github/workflows/release.yml).
- Anda dapat meninjau semua kode Rust dan [mengompilasinya sendiri](#build-from-source) dalam waktu kurang dari satu menit.

---

## 🎧 Perangkat yang Didukung

| Kategori | Protokol / Sumber | Contoh & Catatan |
| :--- | :--- | :--- |
| **Logitech Nirkabel** | HID++ 2.0 langsung (`0x1004` / `0x1000`) & G HUB IPC | Headset Lightspeed (G733, G535, G935), mouse (G PRO, Superlight, G502), keyboard |
| **Bluetooth & BLE** | Windows WinRT PnP + BLE GATT (`0x180F` / `0x2A19`) | Headphone (Sony WH/WF, Bose, AirPods), mouse & keyboard BT |
| **Kontroler Xbox** | Windows `XInput` | Xbox Wireless Controllers (Series X\|S, One, 360) termasuk deteksi cas kabel |
| **PlayStation** | Sony HID (`VID 0x054C`) USB & Bluetooth | DualSense & DualShock 4 (banyak kontroler + status pengisian) |

**Merek Anda belum ada (Razer, Corsair, SteelSeries, Keychron, dll.)?**
👉 **[Buka Permintaan Dukungan Perangkat](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** — Pull Request juga sangat diterima!

---

## 🎯 Kontrol & Indikator Visual

### Warna Status
- **Hijau:** Baterai aman (di atas ambang rendah)
- **Kuning/Amber:** Baterai rendah (default `20%`)
- **Merah (berdenyut):** Baterai kritis (default `10%`)
- **Sian + ⚡:** Sedang mengisi daya

### Interaksi
- **Klik kiri** pada kapsul atau **`⚙`** (atau ikon tray): Membuka **BatteryDock Settings** (`BatteryDock.exe --settings`).
- **Klik kanan** pada kapsul atau ikon tray: Menu native (**Settings**, **Refresh Devices**, **Reset to Taskbar Clock**, **Restart BatteryDock**, **Exit BatteryDock**).
- **Klik & seret (> 5px):** Pindahkan ke mana saja (termasuk monitor lain); klik kanan → **Reset to Taskbar Clock** mengembalikannya ke samping jam.

---

## ⚙️ Konfigurasi

Pengaturan tersimpan di `%APPDATA%\jorjanion\BatteryDock\config\config.toml`:

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

## 🛠️ Build dari Kode Sumber

### Prasyarat
- [Rust toolchain](https://rustup.rs/) (stable)
- Windows 10 atau Windows 11 (MSVC)

### Perintah

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
cargo build --release --locked
```

File executable berada di `target\release\battery-dock.exe` (gunakan `--settings` / `-s` untuk membuka pengaturan langsung).

---

## 📄 Lisensi

Didistribusikan di bawah **Lisensi MIT**. Lihat [LICENSE](../../LICENSE).

<div align="center">

**Jika BatteryDock membantu Anda, silakan beri ⭐ pada repositori ini!**

</div>
