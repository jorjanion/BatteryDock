<div align="center">

<img src="../../assets/logo.png" alt="Logo BatteryDock" width="160" style="border-radius: 24px;" />

# BatteryDock

**Trình hiển thị pin gọn nhẹ bằng Rust nhúng trực tiếp vào thanh tác vụ Windows 10/11 — không cần phần mềm nặng nề của hãng.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Tải bộ cài đặt mới nhất](https://github.com/jorjanion/BatteryDock/releases/latest) · [Báo lỗi](https://github.com/jorjanion/BatteryDock/issues) · [Yêu cầu hỗ trợ thiết bị](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)

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
<a href="README.id.md">Bahasa Indonesia</a> ·
<b>Tiếng Việt</b> ·
<a href="README.ja.md">日本語</a> ·
<a href="README.ko.md">한국어</a> ·
<a href="README.zh-CN.md">简体中文</a> ·
<a href="README.zh-TW.md">繁體中文</a>
</sub>

</div>

---

## 📸 Xem trước

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="42%">
        <img src="../screenshots/dock-closeup.png" alt="Cận cảnh thanh BatteryDock hiển thị pin tai nghe 92%" /><br /><br />
        <sub><b>Thanh pin trên Taskbar</b> — Gắn trực tiếp vào thanh tác vụ Windows với DPI theo từng màn hình, màu trạng thái và biểu tượng sạc ⚡.</sub>
      </td>
      <td align="center" valign="middle" width="58%">
        <img src="../screenshots/settings.png" alt="Cửa sổ Cài đặt BatteryDock v0.1.11" width="340" /><br />
        <sub><b>Cài đặt độc lập</b> — Ngưỡng cảnh báo, nghe thử âm thanh, chế độ vị trí và danh sách thiết bị đang kết nối.</sub>
      </td>
    </tr>
  </table>
</div>

---

## ⚡ Tại sao chọn BatteryDock?

Phần lớn thiết bị không dây giấu phần trăm pin trong các bộ phần mềm cồng kềnh chạy hàng chục dịch vụ nền. **BatteryDock** gom tất cả vào một thanh nhỏ gọn trên taskbar:

- **Không bloatware:** Chỉ một file thực thi Rust thuần **~4 MB** (không Electron, không thu thập dữ liệu, ~77 MB bộ nhớ riêng trên Windows 11).
- **Đọc phần cứng trực tiếp:** Truy vấn **Logitech HID++ 2.0**, **Windows Bluetooth / BLE GATT**, **XInput** và **Sony DualSense / DualShock 4 HID** — thiết bị Logitech vẫn hiển thị ngay cả khi chưa mở G HUB.
- **Quy tắc toàn màn hình thông minh:** Tự động ẩn khi chơi game, xem video toàn màn hình hoặc ẩn taskbar, và luôn ổn định khi bấm phím `Win`, mở Start/Widget hay nhấp desktop.
- **Báo sạc trực tiếp:** Tô màu **xanh lơ** cùng biểu tượng **tia sét ⚡** khi thiết bị đang cắm sạc.
- **Cảnh báo nền đáng tin cậy:** Âm báo và thông báo desktop vẫn hoạt động từ tiến trình nền kể cả khi thanh bị ẩn trong game.

---

## 📥 Tải về và Cài đặt

Tải phiên bản mới nhất từ **[Trang Releases](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Tải **`BatteryDock-Setup-v0.1.11.exe`** (~4,4 MB) — hoặc `BatteryDock-Windows-Portable.zip` cho bản chạy ngay không cần cài.
2. Chạy bộ cài và chọn **"Start BatteryDock when Windows boots"** nếu muốn khởi động cùng Windows.
3. Thanh pin sẽ xuất hiện ngay cạnh đồng hồ khay hệ thống.

### 🛡️ Lưu ý về Windows SmartScreen
Do BatteryDock là ứng dụng mã nguồn mở miễn phí chưa có chứng chỉ ký số EV trả phí, Windows SmartScreen có thể hiện *"Windows protected your PC"* ở lần chạy đầu (**More info → Run anyway**).
- Mọi bản phát hành đều được biên dịch công khai trên GitHub Actions tại [`.github/workflows/release.yml`](../../.github/workflows/release.yml).
- Bạn có thể kiểm tra toàn bộ mã nguồn Rust và [tự biên dịch](#build-from-source) chưa tới một phút.

---

## 🎧 Phần cứng hỗ trợ

| Danh mục | Giao thức / Nguồn | Ví dụ & Ghi chú |
| :--- | :--- | :--- |
| **Logitech Không dây** | HID++ 2.0 trực tiếp (`0x1004` / `0x1000`) & G HUB IPC | Tai nghe Lightspeed (G733, G535, G935), chuột (G PRO, Superlight, G502), bàn phím |
| **Bluetooth & BLE** | Windows WinRT PnP + BLE GATT (`0x180F` / `0x2A19`) | Tai nghe (Sony WH/WF, Bose, AirPods), chuột & bàn phím BT |
| **Tay cầm Xbox** | Windows `XInput` | Xbox Wireless Controllers (Series X\|S, One, 360) gồm cả phát hiện sạc qua dây |
| **PlayStation** | Sony HID (`VID 0x054C`) USB & Bluetooth | DualSense & DualShock 4 (hỗ trợ nhiều tay cầm + trạng thái sạc) |

**Chưa có thương hiệu của bạn (Razer, Corsair, SteelSeries, Keychron...)?**
👉 **[Mở yêu cầu hỗ trợ thiết bị](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** — rất hoan nghênh các đóng góp Pull Request!

---

## 🎯 Điều khiển & Chỉ báo

### Màu trạng thái
- **Xanh lá:** Pin khỏe (trên ngưỡng thấp)
- **Hổ phách:** Pin yếu (mặc định `20%`)
- **Đỏ (nhấp nháy):** Pin rất yếu (mặc định `10%`)
- **Xanh lơ + ⚡:** Thiết bị đang sạc

### Thao tác
- **Nhấp chuột trái** vào thanh hoặc **`⚙`** (hay biểu tượng khay hệ thống): Mở **BatteryDock Settings** (`BatteryDock.exe --settings`).
- **Nhấp chuột phải** vào thanh hoặc biểu tượng khay: Menu (**Settings**, **Refresh Devices**, **Reset to Taskbar Clock**, **Restart BatteryDock**, **Exit BatteryDock**).
- **Kéo thả (> 5px):** Di chuyển tự do (kể cả màn hình phụ); chuột phải → **Reset to Taskbar Clock** để gắn lại cạnh đồng hồ.

---

## ⚙️ Cấu hình

Cài đặt tự động lưu tại `%APPDATA%\jorjanion\BatteryDock\config\config.toml`:

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

## 🛠️ Biên dịch từ mã nguồn

### Yêu cầu
- [Rust toolchain](https://rustup.rs/) (stable)
- Windows 10 hoặc Windows 11 (MSVC)

### Câu lệnh

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
cargo build --release --locked
```

File đầu ra nằm tại `target\release\battery-dock.exe` (dùng `--settings` / `-s` để mở thẳng cài đặt).

---

## 📄 Giấy phép

Phân phối theo **Giấy phép MIT**. Xem chi tiết tại [LICENSE](../../LICENSE).

<div align="center">

**Nếu BatteryDock hữu ích với bạn, hãy tặng repo một ⭐ nhé!**

</div>
