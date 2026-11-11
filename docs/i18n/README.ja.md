<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock ロゴ" width="160" style="border-radius: 24px;" />

# BatteryDock

**Windows 10/11 タスクバーに直接溶け込む、Rust製の超軽量バッテリーインジケーター — メーカー製常駐ソフト不要。**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[最新インストーラーをダウンロード](https://github.com/jorjanion/BatteryDock/releases/latest) · [バグを報告](https://github.com/jorjanion/BatteryDock/issues) · [対応デバイスをリクエスト](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)

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
<b>日本語</b> ·
<a href="README.ko.md">한국어</a> ·
<a href="README.zh-CN.md">简体中文</a> ·
<a href="README.zh-TW.md">繁體中文</a>
</sub>

</div>

---

## 📸 プレビュー

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="42%">
        <img src="../screenshots/dock-closeup.png" alt="ヘッドセットのバッテリー92%を表示するBatteryDockカプセルの拡大図" /><br /><br />
        <sub><b>タスクバーカプセル</b> — モニターごとのDPIスケーリング・残量カラー・⚡充電バッジ付きでタスクバーに直接表示。</sub>
      </td>
      <td align="center" valign="middle" width="58%">
        <img src="../screenshots/settings.png" alt="BatteryDock 設定ウィンドウ v0.1.11" width="340" /><br />
        <sub><b>設定ウィンドウ</b> — 通知しきい値、サウンドテスト、表示位置、接続中デバイス一覧。</sub>
      </td>
    </tr>
  </table>
</div>

---

## ⚡ なぜ BatteryDock なのか？

多くのワイヤレス周辺機器では、バッテリー残量を確認するためだけに多数のバックグラウンドサービスが動く重い純正ソフトが必要です。**BatteryDock** はすべてをタスクバー上の小さなピル型ウィジェットにまとめます：

- **肥大化ゼロ：** 約 **4 MB** の単一ネイティブ Rust バイナリ（Electron 不使用・テレメトリ無し、Windows 11 実測プライベートメモリ約 77 MB）。
- **ハードウェア直接読み取り：** **Logitech HID++ 2.0**、**Windows Bluetooth / BLE GATT**、**XInput**、**Sony DualSense / DualShock 4 HID** を直接参照。Logitech デバイスは G HUB を開かなくても動作します（G HUB IPC にも対応）。
- **賢いフルスクリーン制御：** 全画面ゲーム・全画面動画・自動非表示タスクバー時は自動で隠れ、`Win` キー押下・スタート/検索/ウィジェット・デスクトップクリックでは消えません。
- **リアルタイム充電表示：** 充電中はデバイスを **シアン** + **⚡稲妻アイコン** で分かりやすく表示。
- **バックグラウンド通知：** ゲーム中にピルが非表示になっていても、低バッテリー通知音とデスクトップ通知が確実に動作します。

---

## 📥 ダウンロードとインストール

**[Releases ページ](https://github.com/jorjanion/BatteryDock/releases/latest)** から最新版を入手してください：

1. **`BatteryDock-Setup-v0.1.11.exe`**（約 4.4 MB）をダウンロード（インストール不要の `BatteryDock-Windows-Portable.zip` もあります）。
2. インストーラーを実行し、必要に応じて **"Start BatteryDock when Windows boots"** にチェックを入れます。
3. タスクバーの時計横にカプセルが表示されます。

### 🛡️ Windows SmartScreen について
BatteryDock は有料の EV コード署名証明書を持たない無料オープンソースアプリのため、初回起動時に *"Windows によって PC が保護されました"* と表示される場合があります（**詳細情報 → 実行**）。
- 各リリースバイナリは [`.github/workflows/release.yml`](../../.github/workflows/release.yml) により GitHub Actions 上で公開ビルドされています。
- Rust ソースコードをすべて確認し、1分未満で[ご自身でビルド](#build-from-source)することも可能です。

---

## 🎧 対応ハードウェア

| カテゴリ | プロトコル / 取得元 | 例・備考 |
| :--- | :--- | :--- |
| **Logitech ワイヤレス** | 直接 HID++ 2.0 (`0x1004` / `0x1000`) & G HUB IPC | Lightspeed ヘッドセット (G733, G535, G935)、マウス (G PRO, Superlight, G502)、キーボード |
| **Bluetooth & BLE** | Windows WinRT PnP + BLE GATT (`0x180F` / `0x2A19`) | ヘッドホン (Sony WH/WF, Bose, AirPods)、BT マウス・キーボード |
| **Xbox コントローラー** | Windows `XInput` | Xbox Wireless Controllers (Series X\|S, One, 360) 有線充電検知含む |
| **PlayStation** | Sony HID (`VID 0x054C`) USB & Bluetooth | DualSense & DualShock 4（複数台接続＋充電状態対応） |

**お使いのメーカーがありませんか（Razer、Corsair、SteelSeries、Keychron など）？**
👉 **[デバイス対応リクエストを開く](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** — Pull Request も大歓迎です！

---

## 🎯 操作と表示

### ステータスカラー
- **緑：** 十分な残量（低バッテリーしきい値超）
- **アンバー：** 低バッテリー（既定 `20%`）
- **赤（点滅）：** 危険残量（既定 `10%`）
- **シアン + ⚡：** 充電中

### 操作方法
- カプセル・**`⚙`** またはトレイアイコンを **左クリック**：**BatteryDock Settings**（`BatteryDock.exe --settings`）を開きます。
- カプセル・トレイアイコンを **右クリック**：ネイティブメニュー（**Settings**, **Refresh Devices**, **Reset to Taskbar Clock**, **Restart BatteryDock**, **Exit BatteryDock**）を表示。
- **ドラッグ（> 5px）：** サブモニターを含め自由に移動。右クリック → **Reset to Taskbar Clock** で時計横に戻ります。

---

## ⚙️ 設定ファイル

設定は `%APPDATA%\jorjanion\BatteryDock\config\config.toml` に自動保存されます：

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

## 🛠️ ソースからビルド

### 必要環境
- [Rust ツールチェーン](https://rustup.rs/)（stable）
- Windows 10 または Windows 11（MSVC）

### コマンド

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
cargo build --release --locked
```

生成される実行ファイル：`target\release\battery-dock.exe`（`--settings` / `-s` で設定画面を直接開けます）。

---

## 📄 ライセンス

**MIT ライセンス**の下で公開されています。詳細は [LICENSE](../../LICENSE) をご覧ください。

<div align="center">

**BatteryDock が気に入ったら、ぜひリポジトリに ⭐ スターをお願いします！**

</div>
