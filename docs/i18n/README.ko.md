<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock 로고" width="160" style="border-radius: 24px;" />

# BatteryDock

**무거운 제조사 프로그램 없이 Windows 10/11 작업 표시줄에 바로 통합되는 깔끔한 Rust 네이티브 배터리 표시기.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[최신 설치 파일 다운로드](https://github.com/jorjanion/BatteryDock/releases/latest) · [버그 신고](https://github.com/jorjanion/BatteryDock/issues) · [기기 지원 요청](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)

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
<b>한국어</b> ·
<a href="README.zh-CN.md">简体中文</a> ·
<a href="README.zh-TW.md">繁體中文</a>
</sub>

</div>

---

## 📸 미리보기

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="42%">
        <img src="../screenshots/dock-closeup.png" alt="헤드셋 배터리 92%를 표시하는 BatteryDock 캡슐 확대 이미지" /><br /><br />
        <sub><b>작업 표시줄 캡슐</b> — 모니터별 DPI 스케일링, 배터리 상태 색상 및 실시간 ⚡ 충전 배지와 함께 Windows 작업 표시줄에 직접 통합.</sub>
      </td>
      <td align="center" valign="middle" width="58%">
        <img src="../screenshots/settings.png" alt="BatteryDock 설정 창 v0.1.11" width="340" /><br />
        <sub><b>독립 실행형 설정</b> — 경고 임계값, 알림음 미리 듣기, 위치 모드 및 연결된 장치 목록.</sub>
      </td>
    </tr>
  </table>
</div>

---

## ⚡ 왜 BatteryDock인가요?

대부분의 무선 주변기기는 수십 개의 백그라운드 서비스를 실행하는 무거운 제조사 프로그램 안에 배터리 잔량을 가둬둡니다. **BatteryDock**은 모든 장치를 작업 표시줄의 깔끔한 캡슐 하나에 보여줍니다:

- **블로트웨어 제로:** **~4 MB** 단일 네이티브 Rust 바이너리 (Electron 없음, 원격 측정 없음, Windows 11 실측 전용 메모리 약 77 MB).
- **직접 하드웨어 조회:** **Logitech HID++ 2.0**, **Windows Bluetooth / BLE GATT**, **XInput**, **Sony DualSense / DualShock 4 HID**를 직접 읽습니다 — Logitech 장치는 G HUB를 켜지 않아도 표시됩니다 (실행 중이면 G HUB IPC도 지원).
- **스마트 전체 화면 규칙:** 전체 화면 게임, 영상 또는 자동 숨김 작업 표시줄에서는 자동으로 숨겨지고, `Win` 키, 시작 메뉴, 검색, 위젯, 바탕화면 클릭 시에도 사라지지 않습니다.
- **실시간 충전 표시:** 충전 중인 장치는 **시안(Cyan)** 색상과 **⚡ 번개 아이콘**으로 즉시 구분됩니다.
- **게임 중에도 작동하는 알림:** 전체 화면으로 캡슐이 숨겨져 있어도 백그라운드 엔진에서 알림음과 Windows 알림이 정상 발송됩니다.

---

## 📥 다운로드 및 설치

**[Releases 페이지](https://github.com/jorjanion/BatteryDock/releases/latest)**에서 최신 버전을 다운로드하세요:

1. **`BatteryDock-Setup-v0.1.11.exe`** (~4.4 MB) 또는 무설치 포터블용 `BatteryDock-Windows-Portable.zip`을 다운로드합니다.
2. 설치 프로그램을 실행하고 필요 시 **"Start BatteryDock when Windows boots"**를 선택합니다.
3. 작업 표시줄 시계 옆에 캡슐이 나타납니다.

### 🛡️ Windows SmartScreen 안내
BatteryDock은 유료 EV 코드 서명 인증서가 없는 무료 오픈 소스 앱이므로 첫 실행 시 *"Windows의 PC 보호"* 경고가 뜰 수 있습니다(**추가 정보 → 실행** 클릭).
- 모든 릴리스는 [`.github/workflows/release.yml`](../../.github/workflows/release.yml)을 통해 GitHub Actions에서 공개 빌드됩니다.
- 전체 Rust 코드를 직접 감사하고 1분 이내에 [직접 빌드](#build-from-source)할 수 있습니다.

---

## 🎧 지원 하드웨어

| 분류 | 프로토콜 / 소스 | 예시 및 참고 |
| :--- | :--- | :--- |
| **Logitech 무선** | 직접 HID++ 2.0 (`0x1004` / `0x1000`) & G HUB IPC | Lightspeed 헤드셋 (G733, G535, G935), 마우스 (G PRO, Superlight, G502), 키보드 |
| **Bluetooth & BLE** | Windows WinRT PnP + BLE GATT (`0x180F` / `0x2A19`) | 헤드폰 (Sony WH/WF, Bose, AirPods), BT 마우스 & 키보드 |
| **Xbox 컨트롤러** | Windows `XInput` | Xbox Wireless Controllers (Series X\|S, One, 360) 유선 충전 감지 포함 |
| **PlayStation** | Sony HID (`VID 0x054C`) USB & Bluetooth | DualSense & DualShock 4 (다중 컨트롤러 + 충전 상태 지원) |

**사용 중인 브랜드가 없나요 (Razer, Corsair, SteelSeries, Keychron 등)?**
👉 **[기기 지원 요청 열기](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** — Pull Request 기여도 환영합니다!

---

## 🎯 조작 및 상태 색상

### 상태 색상
- **초록:** 양호한 배터리 (낮은 임계값 초과)
- **주황(Amber):** 배터리 부족 (기본 `20%`)
- **빨강(깜빡임):** 위험 수준 (기본 `10%`)
- **시안 + ⚡:** 현재 충전 중

### 조작 방법
- 캡슐 또는 **`⚙`** (또는 트레이 아이콘) **왼쪽 클릭**: **BatteryDock Settings** 열기 (`BatteryDock.exe --settings`).
- 캡슐 또는 트레이 아이콘 **오른쪽 클릭**: 기본 메뉴 열기 (**Settings**, **Refresh Devices**, **Reset to Taskbar Clock**, **Restart BatteryDock**, **Exit BatteryDock**).
- **클릭 후 드래그 (> 5px):** 보조 모니터를 포함해 원하는 위치로 이동; 우클릭 → **Reset to Taskbar Clock**으로 시계 옆 복귀.

---

## ⚙️ 설정

설정은 `%APPDATA%\jorjanion\BatteryDock\config\config.toml`에 자동 저장됩니다:

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

## 🛠️ 소스에서 빌드

### 사전 요구 사항
- [Rust 툴체인](https://rustup.rs/) (stable)
- Windows 10 또는 Windows 11 (MSVC)

### 명령어

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
cargo build --release --locked
```

생성된 실행 파일: `target\release\battery-dock.exe` (`--settings` 또는 `-s` 옵션으로 설정 창 바로 실행).

---

## 📄 라이선스

**MIT 라이선스**에 따라 배포됩니다. 자세한 내용은 [LICENSE](../../LICENSE)를 참조하세요.

<div align="center">

**BatteryDock이 유용했다면 저장소에 ⭐ 스타를 눌러 주세요!**

</div>
