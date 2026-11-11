<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock 标志" width="160" style="border-radius: 24px;" />

# BatteryDock

**使用 Rust 编写的原生 Windows 10/11 任务栏外设电量指示胶囊 — 彻底告别臃肿的厂商驱动套件。**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[下载最新安装包](https://github.com/jorjanion/BatteryDock/releases/latest) · [反馈 Bug](https://github.com/jorjanion/BatteryDock/issues) · [请求支持新设备](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)

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
<b>简体中文</b> ·
<a href="README.zh-TW.md">繁體中文</a>
</sub>

</div>

---

## 📸 界面预览

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="42%">
        <img src="../screenshots/dock-closeup.png" alt="显示耳机电量 92% 的 BatteryDock 胶囊特写" /><br /><br />
        <sub><b>任务栏胶囊</b> — 直接嵌入 Windows 任务栏，支持每显示器 DPI 缩放、电量颜色分级与实时 ⚡ 充电图标。</sub>
      </td>
      <td align="center" valign="middle" width="58%">
        <img src="../screenshots/settings.png" alt="BatteryDock 设置窗口 v0.1.11" width="340" /><br />
        <sub><b>独立设置窗口</b> — 警告阈值滑块、提示音试听、位置模式与已连接设备列表。</sub>
      </td>
    </tr>
  </table>
</div>

---

## ⚡ 为什么选择 BatteryDock？

大多数无线外设把电量信息锁在动辄几十个后台服务的臃肿厂商软件里。**BatteryDock** 将所有外设汇总在任务栏的一个精致小胶囊中：

- **零臃肿软件：** 单个 **约 4 MB** 原生 Rust 可执行文件（无 Electron、无遥测，Windows 11 实测私有内存约 77 MB）。
- **硬件直连读取：** 直接查询 **Logitech HID++ 2.0**、**Windows Bluetooth / BLE GATT**、**XInput** 与 **Sony DualSense / DualShock 4 HID** — 罗技设备无需打开 G HUB 即可读取（运行时也兼容 G HUB IPC）。
- **智能全屏规则：** 全屏游戏、全屏视频或任务栏自动隐藏时自动隐藏；按下 `Win` 键、打开开始菜单/搜索/小组件或点击桌面时依然稳固显示。
- **实时充电指示：** 充电中的设备以**青色**高亮并带有 **⚡ 闪电标志**。
- **后台告警始终有效：** 即使在全屏游戏中胶囊隐藏，低电量音效和系统通知依然由后台轮询引擎正常触发。

---

## 📥 下载与安装

前往 **[Releases 页面](https://github.com/jorjanion/BatteryDock/releases/latest)** 获取最新版本：

1. 下载 **`BatteryDock-Setup-v0.1.11.exe`**（约 4.4 MB）— 如需免安装绿色版可选择 `BatteryDock-Windows-Portable.zip`。
2. 运行安装程序，按需勾选 **"Start BatteryDock when Windows boots"**。
3. 胶囊会出现在任务栏系统托盘时钟左侧。

### 🛡️ 关于 Windows SmartScreen 提示
由于 BatteryDock 是一款免费开源软件，尚未购买昂贵的 EV 代码签名证书，首次运行可能弹出 *"Windows 已保护你的电脑"*（点击**更多信息 → 仍要运行**）。
- 所有 Release 二进制和安装包均在 GitHub Actions 公开环境中通过 [`.github/workflows/release.yml`](../../.github/workflows/release.yml) 编译生成。
- 你也可以随时审计全部 Rust 源码并在一分钟内[自行编译](#build-from-source)。

---

## 🎧 支持硬件

| 类别 | 协议 / 来源 | 示例与说明 |
| :--- | :--- | :--- |
| **罗技无线设备** | 直连 HID++ 2.0 (`0x1004` / `0x1000`) & G HUB IPC | Lightspeed 耳机 (G733, G535, G935)、鼠标 (G PRO, Superlight, G502)、键盘 |
| **蓝牙 & BLE** | Windows WinRT PnP + BLE GATT (`0x180F` / `0x2A19`) | 耳机 (Sony WH/WF, Bose, AirPods)、蓝牙鼠标与键盘 |
| **Xbox 手柄** | Windows `XInput` | Xbox Wireless Controllers (Series X\|S, One, 360) 含插线充电检测 |
| **PlayStation** | Sony HID (`VID 0x054C`) USB & Bluetooth | DualSense & DualShock 4（支持多手柄 + 充电状态） |

**还没有你的品牌（Razer、Corsair、SteelSeries、Keychron 等）？**
👉 **[提交设备支持请求](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** — 也欢迎直接提交 PR！

---

## 🎯 操作与状态指示

### 状态颜色
- **绿色：** 电量充足（高于低电量阈值）
- **琥珀色：** 低电量（默认 `20%`）
- **红色（闪烁）：** 严重低电量（默认 `10%`）
- **青色 + ⚡：** 正在充电

### 交互方式
- **左键点击**胶囊或 **`⚙`** 齿轮（或托盘图标）：打开 **BatteryDock Settings**（`BatteryDock.exe --settings`）。
- **右键点击**胶囊或托盘图标：弹出原生菜单（**Settings**, **Refresh Devices**, **Reset to Taskbar Clock**, **Restart BatteryDock**, **Exit BatteryDock**）。
- **点击拖动 (> 5px)：** 自由移动到屏幕任意位置（含副屏）；右键 → **Reset to Taskbar Clock** 一键吸附回时钟旁。

---

## ⚙️ 配置文件

配置会自动保存在 `%APPDATA%\jorjanion\BatteryDock\config\config.toml`：

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

## 🛠️ 从源码构建

### 环境要求
- [Rust 工具链](https://rustup.rs/)（stable）
- Windows 10 或 Windows 11（MSVC）

### 命令

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
cargo build --release --locked
```

生成的可执行文件位于 `target\release\battery-dock.exe`（附加 `--settings` 或 `-s` 可直接打开设置窗口）。

---

## 📄 开源协议

基于 **MIT 协议**开源。详情请参阅 [LICENSE](../../LICENSE)。

<div align="center">

**如果 BatteryDock 帮到了你，欢迎给仓库点一颗 ⭐ Star！**

</div>
