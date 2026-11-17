<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock Logo" width="148" style="border-radius: 24px;" />

# BatteryDock

**Xem pin mọi thiết bị không dây trong một thanh nhỏ gọn ngay trên thanh tác vụ Windows. Không phần mềm hãng nặng nề.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Tải v1.0.0](https://github.com/jorjanion/BatteryDock/releases/latest) · [Báo lỗi](https://github.com/jorjanion/BatteryDock/issues/new) · [Yêu cầu hỗ trợ thiết bị](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md) · [Lịch sử thay đổi](../../CHANGELOG.md)

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

Nếu bạn dùng chuột không dây của một hãng, tai nghe hãng khác và tay cầm Bluetooth, việc xem pin trên Windows thường buộc phải mở nhiều phần mềm nền nặng nề.

**BatteryDock** thay thế tất cả bằng một ứng dụng Rust thuần ~4 MB gọn nhẹ ngay trên thanh tác vụ Windows 10/11. Không Electron, không thu thập dữ liệu, không cần tài khoản.

---

## Xem trước

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-single.png" alt="BatteryDock single peripheral pill" /><br /><br />
        <sub><b>Một thiết bị</b>: Biểu tượng vector sắc nét, thanh pin đổi màu và phần trăm ngay trên thanh tác vụ.</sub>
      </td>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-multi.png" alt="BatteryDock multiple peripherals pill" /><br /><br />
        <sub><b>Nhiều thiết bị cùng lúc</b>: Sắp xếp thứ tự, đổi tên hoặc ẩn thiết bị không cần xem.</sub>
      </td>
    </tr>
  </table>
  <br />
  <table>
    <tr>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-peripherals.jpg" alt="BatteryDock Peripherals dashboard v1.0.0" /><br />
        <sub><b>Thiết bị</b>: Ước tính thời lượng pin, biểu đồ 24 giờ, tên gợi nhớ và cảnh báo riêng.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-alerts.jpg" alt="BatteryDock Alerts and Audio settings v1.0.0" /><br />
        <sub><b>Cảnh báo & Âm thanh</b>: Thông báo pin yếu, nguy kịch và sạc đầy 100% kèm nghe thử âm báo.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-customization.jpg" alt="BatteryDock Customization settings v1.0.0" /><br />
        <sub><b>Tùy chỉnh</b>: Giao diện Liquid Glass / Tối / Sáng / Hệ thống, lớp phủ HUD chơi game, phím tắt và 21 ngôn ngữ.</sub>
      </td>
    </tr>
  </table>
</div>

---

## Tính năng

### Thanh pin tối giản trên thanh tác vụ
- **4 giao diện:** **Liquid Glass** (trong suốt), **Tối**, **Sáng**, hoặc **Hệ thống** (tự động khớp giao diện Windows).
- **Gọn gàng:** Nhấp trái mở Cài đặt, nhấp phải mở menu nhanh, hoặc kéo thả sang bất kỳ màn hình nào.
- **Tự động thông minh:** Luôn nổi trên thanh tác vụ kể cả khi mở Start hay Tìm kiếm, và tự ẩn khi xem video hoặc chơi game toàn màn hình.
- **Bố cục linh hoạt:** Đổi thứ tự thiết bị chỉ với một cú nhấp, đặt tên riêng, ẩn thiết bị hoặc bật/tắt số phần trăm.

### Lớp phủ Chơi game & Toàn màn hình tùy chọn (HUD)
- **Tắt theo mặc định** để không che khuất phim hay YouTube toàn màn hình.
- Khi bật, thanh pin tự động hiện ở góc màn hình khi chơi game toàn màn hình không viền.
- **Đặt ở bất cứ đâu:** 4 góc với lề tùy chỉnh hoặc vị trí tự do X/Y (0% đến 100%) có xem trước trực tiếp.
- **Xuyên chuột (Click-through):** Nhấp chuột đi thẳng vào game mà không bị vướng.
- **Phím tắt toàn cục:** Gán bất kỳ tổ hợp phím nào (mặc định `Alt + B`) để bật/tắt HUD ngay trong game.

### Thống kê thông minh & Cảnh báo
- **Lịch sử pin 24 giờ:** Mỗi thẻ thiết bị hiển thị biểu đồ cục bộ quá trình sạc và xả.
- **Ước tính thời gian & chế độ ngủ:** Tính thời gian còn lại hoặc thời gian sạc đầy và nhận diện thiết bị đang ngủ.
- **Cảnh báo pin yếu, nguy kịch và đầy 100%:** Ngưỡng chung hoặc riêng từng thiết bị, tắt tiếng riêng, âm báo nhẹ và thông báo Windows.

### Tích hợp 21 ngôn ngữ
- Tự động nhận diện ngôn ngữ Windows ở lần chạy đầu tiên (mặc định tiếng Anh nếu chưa hỗ trợ).
- Tự động nạp phông chữ gốc của Windows khi cần để hiển thị sắc nét mọi bộ chữ.

---

## Phần cứng hỗ trợ

BatteryDock đọc trực tiếp qua HID, WinRT và Bluetooth mà không cần phần mềm hãng chạy nền.

| Danh mục | Giao thức / Phương thức | Ví dụ hỗ trợ |
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

## Tải xuống & Cài đặt

Tải bản mới nhất tại **[trang Releases](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Tải **`BatteryDock-Setup-v1.0.0.exe`** (bộ cài) hoặc **`BatteryDock-Windows-Portable.zip`** (bản chạy ngay).
2. Mở ứng dụng và tùy chọn bật khởi động cùng Windows trong Cài đặt.
3. Thanh pin sẽ hiện cạnh đồng hồ cùng tính năng tự cập nhật từ GitHub.

### Lưu ý Windows SmartScreen
Vì BatteryDock là dự án mã nguồn mở miễn phí không có chứng chỉ ký trả phí, SmartScreen có thể cảnh báo lần đầu. Chọn **More info** rồi **Run anyway**. Mọi bản phát hành được biên dịch công khai tại [`.github/workflows/release.yml`](../../.github/workflows/release.yml).

---

## Cấu hình

Cài đặt lưu tại `%APPDATA%\jorjanion\BatteryDock\config\config.toml` (lịch sử ở `history.toml`) và áp dụng ngay lập tức.

---

## Tự biên dịch từ mã nguồn

Yêu cầu: Windows 10/11 và [Rust](https://rustup.rs/) (bản MSVC ổn định).

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock
cargo test --locked
cargo build --release --locked
.\target\release\battery-dock.exe
```

---

## Đóng góp, Issues & Pull Requests

BatteryDock là mã nguồn mở do cộng đồng cùng phát triển. Mọi báo lỗi, bản dịch, yêu cầu thiết bị và PR luôn được hoan nghênh.

- **Thiết bị chưa hiển thị?** Mở **[yêu cầu hỗ trợ thiết bị](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)**. Vào **Cài đặt > Tùy chỉnh > Sao chép chẩn đoán phần cứng** rồi dán báo cáo ẩn danh vào issue.
- **Phát hiện lỗi hoặc có ý tưởng?** Mở **[Issue](https://github.com/jorjanion/BatteryDock/issues)** kèm phiên bản Windows của bạn.
- **Muốn đóng góp mã nguồn?** Pull Request luôn được chào đón. Hãy chạy `cargo test --locked` và `cargo clippy --locked` trước khi gửi.

---

## Giấy phép

Giấy phép MIT. Xem [LICENSE](../../LICENSE).
