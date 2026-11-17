<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock Logo" width="148" style="border-radius: 24px;" />

# BatteryDock

**모든 무선 주변기기 배터리를 Windows 작업 표시줄의 깔끔한 네이티브 캡슐 하나로. 무거운 제조사 앱은 필요 없습니다.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[v1.0.0 다운로드](https://github.com/jorjanion/BatteryDock/releases/latest) · [버그 제보](https://github.com/jorjanion/BatteryDock/issues/new) · [장치 지원 요청](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md) · [변경 기록](../../CHANGELOG.md)

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

서로 다른 브랜드의 무선 마우스, 헤드셋, 블루투스 컨트롤러를 함께 쓰면 배터리 확인을 위해 무거운 백그라운드 프로그램을 여러 개 켜두어야 합니다.

**BatteryDock**은 이를 Windows 10/11 작업 표시줄에 조용히 자리 잡는 약 4 MB 크기의 단일 네이티브 Rust 앱으로 대체합니다. Electron 없음, 텔레메트리 없음, 계정 불필요.

---

## 미리보기

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-single.png" alt="BatteryDock single peripheral pill" /><br /><br />
        <sub><b>단일 장치</b>: 선명한 벡터 아이콘, 색상별 배터리 막대, 백분율을 작업 표시줄에 바로 표시.</sub>
      </td>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-multi.png" alt="BatteryDock multiple peripherals pill" /><br /><br />
        <sub><b>여러 장치 동시 표시</b>: 순서 변경, 별명 지정, 원하지 않는 장치 숨기기 지원.</sub>
      </td>
    </tr>
  </table>
  <br />
  <table>
    <tr>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-peripherals.jpg" alt="BatteryDock Peripherals dashboard v1.0.0" /><br />
        <sub><b>장치</b>: 예상 잔여 시간, 24시간 배터리 그래프, 별명 및 장치별 알림 설정.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-alerts.jpg" alt="BatteryDock Alerts and Audio settings v1.0.0" /><br />
        <sub><b>알림 및 오디오</b>: 배터리 부족, 위험, 100% 완충 알림과 사운드 테스트.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-customization.jpg" alt="BatteryDock Customization settings v1.0.0" /><br />
        <sub><b>사용자 지정</b>: Liquid Glass / 다크 / 라이트 / 시스템 테마, 게임 HUD, 전역 단축키, 21개 언어.</sub>
      </td>
    </tr>
  </table>
</div>

---

## 주요 기능

### 미니멀한 작업 표시줄 캡슐
- **4가지 테마:** **Liquid Glass** (초투명), **다크**, **라이트**, **시스템** (Windows 테마 자동 연동).
- **군더더기 없는 조작:** 왼쪽 클릭으로 설정 열기, 오른쪽 클릭으로 빠른 메뉴, 드래그하여 어느 모니터에나 배치 가능.
- **스마트 작업 표시줄 동작:** 시작 메뉴나 검색을 열어도 작업 표시줄 위에 유지되며 전체 화면 영상과 게임에서는 자동으로 숨겨집니다.
- **유연한 배치:** 클릭 한 번으로 장치 순서 변경, 별명 설정, 개별 숨기기, 백분율 표시 전환.

### 선택형 게임 및 전체 화면 오버레이 (HUD)
- **기본값은 꺼짐**이므로 전체 화면 영상이나 YouTube 시청을 방해하지 않습니다.
- 켜면 테두리 없는 전체 화면 게임 중 화면 모서리에 배터리가 자동으로 표시됩니다.
- **원하는 위치에 배치:** 여백 조절이 가능한 4개 모서리 또는 실시간 미리보기 지원 자유 위치 X/Y (0%~100%).
- **클릭 통과(Click-through):** 마우스 클릭이 HUD를 통과해 게임으로 바로 전달됩니다.
- **전역 단축키:** 원하는 키 조합(기본 `Alt + B`)으로 Alt+Tab 없이 게임 중 바로 켜고 끌 수 있습니다.

### 스마트 텔레메트리 및 알림
- **24시간 배터리 기록:** 각 장치 카드에 충전 및 방전 추이 그래프가 표시됩니다.
- **잔여 시간 및 절전 감지:** 남은 사용 시간 또는 완충 시간을 추정하고 절전 모드를 감지합니다.
- **부족, 위험 및 100% 완충 알림:** 전체 또는 장치별 임계값, 개별 음소거, 부드러운 알림음, Windows 데스크톱 알림 지원.

### 21개 언어 기본 내장
- 처음 실행 시 Windows 언어를 자동 감지해 적용합니다(미지원 언어는 영어 기본값).
- Windows 기본 시스템 글꼴을 필요 시 불러와 모든 문자를 선명하게 표시합니다.

---

## 지원 하드웨어

BatteryDock은 제조사 프로그램 없이 HID, WinRT, Bluetooth로 하드웨어를 직접 읽습니다.

| 분류 | 프로토콜 / 방식 | 지원 예시 |
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

## 다운로드 및 설치

**[Releases 페이지](https://github.com/jorjanion/BatteryDock/releases/latest)**에서 최신 버전을 받으세요:

1. **`BatteryDock-Setup-v1.0.0.exe`** (설치 파일) 또는 **`BatteryDock-Windows-Portable.zip`** (무설치 포터블)을 다운로드합니다.
2. 실행 후 필요하면 설정에서 Windows 시작 시 자동 실행을 켭니다.
3. 시계 옆에 표시되며 내장 GitHub 자동 업데이트를 지원합니다.

### Windows SmartScreen 안내
유료 코드 서명 인증서가 없는 무료 오픈소스 앱이므로 첫 실행 시 SmartScreen 경고가 뜰 수 있습니다. **추가 정보** 후 **실행**을 클릭하세요. 모든 빌드는 [`.github/workflows/release.yml`](../../.github/workflows/release.yml)에서 공개 빌드됩니다.

---

## 설정 파일

설정은 `%APPDATA%\jorjanion\BatteryDock\config\config.toml` (기록은 `history.toml`)에 저장되며 재시작 없이 즉시 반영됩니다.

---

## 소스에서 빌드

요구 사항: Windows 10/11 및 [Rust](https://rustup.rs/) (안정 MSVC 툴체인).

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock
cargo test --locked
cargo build --release --locked
.\target\release\battery-dock.exe
```

---

## 기여, Issue 및 Pull Request

BatteryDock은 커뮤니티와 함께 만드는 오픈소스 프로젝트입니다. 버그 제보, 번역, 장치 지원 요청, PR을 언제든 환영합니다.

- **장치가 보이지 않나요?** **[장치 지원 요청](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)**을 열어주세요. **설정 > 사용자 지정 > 하드웨어 진단 복사**를 눌러 익명 리포트를 바로 붙여넣을 수 있습니다.
- **버그나 아이디어가 있나요?** Windows 버전과 함께 **[Issue](https://github.com/jorjanion/BatteryDock/issues)**를 열어주세요.
- **코드로 기여하고 싶나요?** Pull Request를 환영합니다. 보내기 전에 `cargo test --locked`와 `cargo clippy --locked`를 실행해 주세요.

---

## 라이선스

MIT 라이선스. 자세한 내용은 [LICENSE](../../LICENSE)를 참고하세요.
