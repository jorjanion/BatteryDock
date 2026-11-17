<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock Logo" width="148" style="border-radius: 24px;" />

# BatteryDock

**あらゆるワイヤレス周辺機器のバッテリー残量を Windows タスクバーに一つに集約。重いメーカー製ソフトは不要です。**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[v1.0.0 をダウンロード](https://github.com/jorjanion/BatteryDock/releases/latest) · [バグを報告](https://github.com/jorjanion/BatteryDock/issues/new) · [対応デバイスをリクエスト](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md) · [変更履歴](../../CHANGELOG.md)

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

ワイヤレスマウス、ヘッドセット、Bluetooth コントローラーを別々のメーカーで揃えていると、バッテリー確認のためだけに複数の常駐ソフトを動かす羽目になりがちです。

**BatteryDock** はそれらを約 4 MB の軽量なネイティブ Rust アプリ一つにまとめ、Windows 10/11 のタスクバーに静かに収めます。Electron 不使用、テレメトリなし、アカウント不要です。

---

## プレビュー

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-single.png" alt="BatteryDock single peripheral pill" /><br /><br />
        <sub><b>単一デバイス</b>: 鮮明なベクターアイコン、色分けされたバッテリーバー、残量(%)をタスクバーに表示。</sub>
      </td>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-multi.png" alt="BatteryDock multiple peripherals pill" /><br /><br />
        <sub><b>複数デバイス同時表示</b>: 並び替え、ニックネーム設定、不要なデバイスの非表示に対応。</sub>
      </td>
    </tr>
  </table>
  <br />
  <table>
    <tr>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-peripherals.jpg" alt="BatteryDock Peripherals dashboard v1.0.0" /><br />
        <sub><b>デバイス</b>: 推定残り時間、24時間バッテリー推移グラフ、個別アラート設定。</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-alerts.jpg" alt="BatteryDock Alerts and Audio settings v1.0.0" /><br />
        <sub><b>通知と音声</b>: 残量低下・危機・100%満充電の通知とチャイム音テスト。</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-customization.jpg" alt="BatteryDock Customization settings v1.0.0" /><br />
        <sub><b>カスタマイズ</b>: Liquid Glass / ダーク / ライト / システムテーマ、ゲーム HUD、ホットキー、21言語。</sub>
      </td>
    </tr>
  </table>
</div>

---

## 主な機能

### ミニマルなタスクバーピル
- **4種類のテーマ:** **Liquid Glass** (超透明)、**ダーク**、**ライト**、**システム** (Windows テーマに自動追従)。
- **シンプルな操作:** 左クリックで設定を開き、右クリックでクイックメニュー、ドラッグで好きなモニターへ移動できます。
- **スマートな表示制御:** スタートメニューや検索を開いてもタスクバー前面を維持し、フルスクリーン動画やゲーム時は自動で隠れます。
- **自由なレイアウト:** ワンクリックでの並び替え、ニックネーム変更、個別非表示、パーセント表示のオン/オフが可能。

### オプションのゲーム・フルスクリーンオーバーレイ (HUD)
- **初期設定ではオフ**のため、フルスクリーン動画や YouTube の邪魔をしません。
- オンにすると、ボーダーレスフルスクリーンゲーム中に画面の角へバッテリーを自動表示します。
- **自由な配置:** 余白調整できる4つの角、またはライブプレビュー付き自由配置 X/Y (0%〜100%) を選択可能。
- **クリックスルー:** マウスクリックが HUD を貫通してそのままゲームに入力されます。
- **グローバルホットキー:** 任意のショートカット (初期値 `Alt + B`) で Alt+Tab せずにゲーム中に表示を切り替えられます。

### スマートテレメトリと通知
- **24時間バッテリー推移:** 各デバイスカードに充放電の推移グラフを表示します。
- **残り時間とスリープ検知:** 残り稼働時間や満充電までの時間を推定し、スリープ状態も検知します。
- **低残量・危機・100%満充電アラート:** 全体またはデバイスごとのしきい値、個別ミュート、チャイム音、Windows 通知に対応。

### 21言語を標準搭載
- 初回起動時に Windows の表示言語を自動検出し、対応言語へ切り替えます (未対応の場合は英語)。
- Windows 標準フォントを必要に応じて読み込み、日本語・中国語・韓国語・アラビア語・ヒンディー語などを鮮明に表示します。

---

## 対応ハードウェア

BatteryDock は HID / WinRT / Bluetooth 経由でハードウェアから直接読み取るため、メーカー製ソフトを常駐させる必要がありません。

| カテゴリ | プロトコル / 方式 | 対応例 |
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

## ダウンロードとインストール

**[Releases ページ](https://github.com/jorjanion/BatteryDock/releases/latest)** から最新版を取得してください。

1. **`BatteryDock-Setup-v1.0.0.exe`** (インストーラー) または **`BatteryDock-Windows-Portable.zip`** (ポータブル版) をダウンロードします。
2. 起動後、必要に応じて設定で Windows 起動時の自動起動を有効にします。
3. タスクバーの時計の隣に表示されます。GitHub 自動アップデート機能付きです。

### Windows SmartScreen について
無料のオープンソースアプリのため初回起動時に SmartScreen の警告が出ることがあります。**詳細情報** から **実行** を選んでください。すべてのバイナリは [`.github/workflows/release.yml`](../../.github/workflows/release.yml) で公開ビルドされています。

---

## 設定ファイル

設定は `%APPDATA%\jorjanion\BatteryDock\config\config.toml` (履歴は `history.toml`) に保存され、再起動なしで即反映されます。

---

## ソースからビルド

必要環境: Windows 10/11 および [Rust](https://rustup.rs/) (stable MSVC)。

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock
cargo test --locked
cargo build --release --locked
.\target\release\battery-dock.exe
```

---

## コントリビュート・Issue・PR

BatteryDock はコミュニティ主導のオープンソースプロジェクトです。バグ報告、翻訳、対応デバイス要望、Pull Request を歓迎します。

- **デバイスが表示されない場合:** **[対応デバイスリクエスト](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** を作成してください。**設定 > カスタマイズ > ハードウェア診断情報をコピー** で匿名レポートをそのまま貼り付けられます。
- **バグやアイデアがある場合:** Windows のバージョンと再現手順を添えて **[Issue](https://github.com/jorjanion/BatteryDock/issues)** を作成してください。
- **コードで貢献したい場合:** Pull Request 大歓迎です。送信前に `cargo test --locked` と `cargo clippy --locked` を実行してください。

---

## ライセンス

MIT ライセンス。詳細は [LICENSE](../../LICENSE) をご覧ください。
