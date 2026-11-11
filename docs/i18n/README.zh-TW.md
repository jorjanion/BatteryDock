<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock 標誌" width="160" style="border-radius: 24px;" />

# BatteryDock

**以 Rust 打造的原生 Windows 10/11 工作列週邊電量指示膠囊 — 無需臃腫的原廠軟體。**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[下載最新安裝檔](https://github.com/jorjanion/BatteryDock/releases/latest) · [回報錯誤](https://github.com/jorjanion/BatteryDock/issues) · [請求支援裝置](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)

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
<a href="README.vi.md">Tiếng Việt</a> ·
<a href="README.ja.md">日本語</a> ·
<a href="README.ko.md">한국어</a> ·
<a href="README.zh-CN.md">简体中文</a> ·
<b>繁體中文</b>
</sub>

</div>

---

## 📸 畫面預覽

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="42%">
        <img src="../screenshots/dock-closeup.png" alt="顯示耳機電量 92% 的 BatteryDock 膠囊特寫" /><br /><br />
        <sub><b>工作列膠囊</b> — 直接嵌入 Windows 工作列，支援各螢幕獨立 DPI 縮放、電量分級色彩與即時 ⚡ 充電圖示。</sub>
      </td>
      <td align="center" valign="middle" width="58%">
        <img src="../screenshots/settings.png" alt="BatteryDock 設定視窗 v0.1.11" width="340" /><br />
        <sub><b>獨立設定視窗</b> — 電量警示門檻、提示音試聽、顯示位置模式與已連線裝置列表。</sub>
      </td>
    </tr>
  </table>
</div>

---

## ⚡ 為什麼選擇 BatteryDock？

多數無線週邊將電量百分比鎖在背景執行大量服務的臃腫原廠軟體裡。**BatteryDock** 將一切收進工作列的一顆俐落膠囊中：

- **零臃腫軟體：** 單一 **約 4 MB** 原生 Rust 執行檔（無 Electron、無遙測，Windows 11 實測私人記憶體約 77 MB）。
- **硬體直接讀取：** 直接查詢 **Logitech HID++ 2.0**、**Windows Bluetooth / BLE GATT**、**XInput** 與 **Sony DualSense / DualShock 4 HID** — 羅技裝置免開 G HUB 也能回報（執行時亦支援 G HUB IPC）。
- **智慧全螢幕規則：** 全螢幕遊戲、影片或自動隱藏工作列時會自動隱藏；按下 `Win` 鍵、開啟開始/搜尋/小工具或點擊桌面時保持穩定顯示。
- **即時充電顯示：** 充電中的裝置會以**青色**顯示並附上 **⚡ 閃電圖示**。
- **可靠的背景通知：** 即使在遊戲中膠囊處於隱藏狀態，低電量音效與系統通知仍會正常觸發。

---

## 📥 下載與安裝

前往 **[Releases 頁面](https://github.com/jorjanion/BatteryDock/releases/latest)** 下載最新版本：

1. 下載 **`BatteryDock-Setup-v0.1.11.exe`**（約 4.4 MB）— 或選擇免安裝的 `BatteryDock-Windows-Portable.zip`。
2. 執行安裝程式並依需求勾選 **"Start BatteryDock when Windows boots"**。
3. 膠囊會顯示於工作列系統匣時鐘左側。

### 🛡️ Windows SmartScreen 說明
因 BatteryDock 為免費開源專案且尚未購買付費 EV 程式碼簽章憑證，首次執行可能跳出 *"Windows 已保護您的電腦"*（點擊**其他資訊 → 仍要執行**）。
- 所有 Release 檔案皆透過 [`.github/workflows/release.yml`](../../.github/workflows/release.yml) 於 GitHub Actions 公開編譯產出。
- 您也可以檢視完整 Rust 原始碼並在 1 分鐘內[自行編譯](#build-from-source)。

---

## 🎧 支援硬體

| 類別 | 協定 / 來源 | 範例與說明 |
| :--- | :--- | :--- |
| **Logitech 無線** | 直接 HID++ 2.0 (`0x1004` / `0x1000`) & G HUB IPC | Lightspeed 耳機 (G733, G535, G935)、滑鼠 (G PRO, Superlight, G502)、鍵盤 |
| **藍牙 & BLE** | Windows WinRT PnP + BLE GATT (`0x180F` / `0x2A19`) | 耳機 (Sony WH/WF, Bose, AirPods)、藍牙滑鼠與鍵盤 |
| **Xbox 控制器** | Windows `XInput` | Xbox Wireless Controllers (Series X\|S, One, 360) 含插線充電偵測 |
| **PlayStation** | Sony HID (`VID 0x054C`) USB & Bluetooth | DualSense & DualShock 4（支援多手把 + 充電狀態） |

**找不到您的品牌（Razer、Corsair、SteelSeries、Keychron 等）？**
👉 **[提交裝置支援請求](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** — 也歡迎提交 PR！

---

## 🎯 操作與狀態指示

### 狀態色彩
- **綠色：** 電量充足（高於低電量門檻）
- **琥珀色：** 電量偏低（預設 `20%`）
- **紅色（閃爍）：** 嚴重低電量（預設 `10%`）
- **青色 + ⚡：** 充電中

### 互動方式
- **左鍵點擊**膠囊或 **`⚙`** 圖示（或系統匣圖示）：開啟 **BatteryDock Settings**（`BatteryDock.exe --settings`）。
- **右鍵點擊**膠囊或系統匣圖示：開啟原生選單（**Settings**, **Refresh Devices**, **Reset to Taskbar Clock**, **Restart BatteryDock**, **Exit BatteryDock**）。
- **點擊拖曳 (> 5px)：** 可移至螢幕任意位置（含副螢幕）；右鍵 → **Reset to Taskbar Clock** 即可恢復至時鐘旁。

---

## ⚙️ 設定檔

設定自動儲存於 `%APPDATA%\jorjanion\BatteryDock\config\config.toml`：

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

## 🛠️ 從原始碼編譯

### 系統需求
- [Rust 工具鏈](https://rustup.rs/)（stable）
- Windows 10 或 Windows 11（MSVC）

### 指令

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
cargo build --release --locked
```

編譯完成後的執行檔位於 `target\release\battery-dock.exe`（傳入 `--settings` / `-s` 可直接開啟設定視窗）。

---

## 📄 授權條款

採用 **MIT 授權條款**。詳情請參閱 [LICENSE](../../LICENSE)。

<div align="center">

**若 BatteryDock 對您有所幫助，歡迎幫這個專案按一顆 ⭐ Star！**

</div>
