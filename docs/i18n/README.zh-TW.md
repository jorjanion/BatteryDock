<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock Logo" width="148" style="border-radius: 24px;" />

# BatteryDock

**將所有無線周邊電量集中到 Windows 工作列的一個輕量原生膠囊中，告別臃腫的廠商軟體。**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[下載 v1.0.0](https://github.com/jorjanion/BatteryDock/releases/latest) · [回報問題](https://github.com/jorjanion/BatteryDock/issues/new) · [請求裝置支援](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md) · [更新日誌](../../CHANGELOG.md)

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

如果你同時使用不同品牌的無線滑鼠、耳機與藍牙手把，在 Windows 上查看電量通常得在背景開著好幾個沉重的廠商套件。

**BatteryDock** 用一個僅約 4 MB 的原生 Rust 應用程式取代它們，安靜地停靠在 Windows 10/11 工作列上。無 Electron、無遙測、免帳號。

---

## 介面預覽

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-single.png" alt="BatteryDock single peripheral pill" /><br /><br />
        <sub><b>單一裝置</b>: 清晰向量圖示、彩色電量條與百分比直接顯示在工作列上。</sub>
      </td>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-multi.png" alt="BatteryDock multiple peripherals pill" /><br /><br />
        <sub><b>多裝置並排</b>: 自由排序、自訂別名或隱藏不想看到的裝置。</sub>
      </td>
    </tr>
  </table>
  <br />
  <table>
    <tr>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-peripherals.jpg" alt="BatteryDock Peripherals dashboard v1.0.0" /><br />
        <sub><b>周邊裝置</b>: 即時續航預估、24 小時電量趨勢圖、別名與個別警示。</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-alerts.jpg" alt="BatteryDock Alerts and Audio settings v1.0.0" /><br />
        <sub><b>警示與音效</b>: 低電量、極低電量與 100% 充滿提醒，支援提示音試聽。</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-customization.jpg" alt="BatteryDock Customization settings v1.0.0" /><br />
        <sub><b>個人化</b>: Liquid Glass / 深色 / 淺色 / 跟隨系統主題、遊戲 HUD 懸浮窗、快捷鍵與 21 種語言。</sub>
      </td>
    </tr>
  </table>
</div>

---

## 核心功能

### 極簡工作列電量條
- **4 種視覺主題:** 可選 **Liquid Glass** (超透明玻璃)、**深色**、**淺色** 或 **跟隨系統** (自動符合 Windows 主題)。
- **簡潔操作:** 左鍵點擊電量條開啟設定、右鍵開啟快捷選單，也可拖曳至任意螢幕。
- **智慧工作列相容:** 開啟開始選單、搜尋或小工具時依然保持在最上層，全螢幕看影片或玩遊戲時自動隱藏。
- **自訂配置:** 一鍵調整裝置順序、設定別名、隱藏個別裝置，或切換百分比顯示。

### 可選遊戲與全螢幕懸浮窗 (HUD)
- **預設關閉**，讓全螢幕影片與 YouTube 完全不受干擾。
- 啟用後，在無邊框全螢幕遊戲中會自動顯示於螢幕角落，避免遊戲中途沒電。
- **任意位置:** 4 個角落預設 (可調邊距) 或自由位置 X/Y (0% 至 100%) 並支援即時預覽。
- **滑鼠穿透 (Click-through):** 點擊直接穿透至遊戲畫面，不干擾操作。
- **全域快捷鍵:** 自訂任意按鍵組合 (預設 `Alt + B`)，免 Alt+Tab 即可在遊戲內快速開關。

### 智慧遙測與提醒
- **24 小時電量趨勢:** 每張裝置卡片皆繪製輕量本地充放電曲線。
- **續航與休眠偵測:** 估算剩餘時間或充滿所需時間，並偵測裝置休眠狀態。
- **低電量、極低電量與 100% 充滿提醒:** 支援全域或個別門檻、單獨靜音、柔和提示音與 Windows 桌面通知。

### 內建 21 種語言
- 首次啟動自動偵測 Windows 系統語言並套用 (尚未支援的語言預設英文)。
- 依需求載入 Windows 原生字型，讓各國文字皆清晰顯示。

---

## 支援硬體

BatteryDock 透過 HID、WinRT 與藍牙直接讀取硬體電量，不需常駐廠商軟體。

| 類別 | 協定 / 方式 | 支援範例 |
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

## 下載與安裝

前往 **[Releases 頁面](https://github.com/jorjanion/BatteryDock/releases/latest)** 下載最新版:

1. 下載 **`BatteryDock-Setup-v1.0.0.exe`** (安裝版) 或 **`BatteryDock-Windows-Portable.zip`** (免安裝可攜版)。
2. 執行後可在設定中選擇開機自動啟動。
3. 電量條會出現在時鐘旁，並內建 GitHub 自動更新檢查。

### Windows SmartScreen 說明
因 BatteryDock 為免費開源專案且未購買付費憑證，首次執行可能出現 SmartScreen 提示。請點擊 **其他資訊** 再選 **仍要執行**。所有發行檔皆於 [`.github/workflows/release.yml`](../../.github/workflows/release.yml) 公開建置。

---

## 設定說明

設定檔儲存於 `%APPDATA%\jorjanion\BatteryDock\config\config.toml` (趨勢紀錄於 `history.toml`)，變更會立即生效無需重啟。

---

## 從原始碼建置

需求: Windows 10/11 與 [Rust](https://rustup.rs/) (穩定版 MSVC)。

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock
cargo test --locked
cargo build --release --locked
.\target\release\battery-dock.exe
```

---

## 參與貢獻、Issue 與 Pull Request

BatteryDock 是社群驅動的開源專案，歡迎提交錯誤回報、翻譯、新裝置支援請求與 PR。

- **沒偵測到你的裝置？** 開啟 **[裝置支援請求](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)**。在 **設定 > 個人化 > 複製硬體診斷資訊** 一鍵複製匿名報告貼上即可。
- **發現 Bug 或有建議？** 歡迎提交 **[Issue](https://github.com/jorjanion/BatteryDock/issues)** 並附上 Windows 版本與重現步驟。
- **想貢獻程式碼？** 歡迎提交 Pull Request。送出前請執行 `cargo test --locked` 與 `cargo clippy --locked`。

---

## 授權條款

MIT 授權條款，詳見 [LICENSE](../../LICENSE)。
