<div align="center">

<img src="../../assets/logo.png" alt="Logo de BatteryDock" width="160" style="border-radius: 24px;" />

# BatteryDock

**Un indicador de batería nativo en Rust integrado directamente en tu barra de tareas de Windows 10/11 — sin software pesado de fabricantes.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Descargar último instalador](https://github.com/jorjanion/BatteryDock/releases/latest) · [Reportar un error](https://github.com/jorjanion/BatteryDock/issues) · [Solicitar soporte de dispositivo](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)

<br />

<sub>
<a href="../../README.md">English</a> ·
<b>Español</b> ·
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
<a href="README.zh-TW.md">繁體中文</a>
</sub>

</div>

---

## 📸 Vista previa

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="42%">
        <img src="../screenshots/dock-closeup.png" alt="Detalle de la cápsula BatteryDock mostrando batería al 92%" /><br /><br />
        <sub><b>Cápsula en la barra de tareas</b> — Integrada directamente en la barra de Windows con escalado DPI por monitor, colores por nivel e indicador de carga ⚡ en vivo.</sub>
      </td>
      <td align="center" valign="middle" width="58%">
        <img src="../screenshots/settings.png" alt="Ventana de configuración de BatteryDock v0.1.11" width="340" /><br />
        <sub><b>Ajustes independientes</b> — Umbrales de alerta, prueba de sonido, posición en barra y lista de dispositivos conectados.</sub>
      </td>
    </tr>
  </table>
</div>

---

## ⚡ ¿Por qué BatteryDock?

La mayoría de periféricos inalámbricos esconden el porcentaje de batería dentro de suites pesadas con decenas de servicios en segundo plano. **BatteryDock** reúne tus dispositivos en una cápsula minimalista dentro de la barra de tareas:

- **Sin bloatware de fabricantes:** Un único binario nativo en Rust de **~4 MB** (sin Electron, sin telemetría, ~77 MB de memoria privada observados en vivo en Windows 11).
- **Lectura directa de hardware:** Consulta **Logitech HID++ 2.0**, **Windows Bluetooth / BLE GATT**, **XInput** y **Sony DualSense / DualShock 4 HID** directamente — los dispositivos Logitech pueden reportar incluso sin abrir G HUB (y también se integra con G HUB IPC si está abierto).
- **Reglas inteligentes para barra y pantalla completa:** Se oculta automáticamente en juegos, vídeos a pantalla completa o barras auto-ocultadas, y se mantiene visible al pulsar la tecla `Win`, abrir Inicio/Búsqueda/Widgets o hacer clic en el escritorio.
- **Indicador de carga en vivo:** Resalta en **cian** con un **rayo ⚡** cuando el periférico está cargando.
- **Alertas que siempre funcionan:** Los avisos sonoros y notificaciones de escritorio se disparan desde el motor en segundo plano aunque la píldora esté oculta durante una partida.

---

## 📥 Descarga e instalación

Descarga la última versión desde la **[página de Releases](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Descarga **`BatteryDock-Setup-v0.1.11.exe`** (~4.4 MB) — o `BatteryDock-Windows-Portable.zip` si prefieres la versión portable.
2. Ejecuta el instalador (disponible en español e inglés) y marca **"Start BatteryDock when Windows boots"** si quieres inicio automático.
3. La píldora aparecerá integrada junto al reloj de la barra de tareas.

### 🛡️ Aviso sobre Windows SmartScreen
Al ser una app gratuita y open-source sin certificado de firma EV de pago, Windows SmartScreen puede mostrar *"Windows protegió su PC"* la primera vez (pulsa **Más información → Ejecutar de todas formas**).
- Cada ejecutable e instalador se compila públicamente en los servidores limpios de GitHub Actions desde [`.github/workflows/release.yml`](../../.github/workflows/release.yml).
- Si prefieres verificarlo al 100%, puedes revisar todo el código fuente en Rust y [compilarlo tú mismo](#build-from-source) en menos de un minuto.

---

## 🎧 Hardware compatible

| Categoría | Protocolo / Fuente | Ejemplos y notas |
| :--- | :--- | :--- |
| **Logitech inalámbrico** | HID++ 2.0 directo (`0x1004` / `0x1000`) y G HUB IPC | Auriculares Lightspeed (G733, G535, G935), ratones (G PRO, Superlight, G502), teclados |
| **Bluetooth y BLE** | Windows WinRT PnP + BLE GATT (`0x180F` / `0x2A19`) | Auriculares (Sony WH/WF, Bose, AirPods), ratones y teclados BT |
| **Mandos Xbox** | Windows `XInput` | Mandos Xbox Wireless (Series X\|S, One, 360) con detección de carga por cable |
| **PlayStation** | Sony HID (`VID 0x054C`) USB y Bluetooth | DualSense y DualShock 4 (soporte multimando y estado de carga) |

**¿Falta tu marca (Razer, Corsair, SteelSeries, Keychron, etc.)?**
👉 **[Abre una solicitud de soporte de dispositivo](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** — ¡también se aceptan PRs con perfiles HID/Bluetooth!

---

## 🎯 Controles e indicadores visuales

### Colores de estado
- **Verde:** Batería saludable (por encima del umbral bajo)
- **Ámbar:** Batería baja (configurable, por defecto `20%`)
- **Rojo (pulsante):** Batería crítica (configurable, por defecto `10%`)
- **Cian + ⚡:** Dispositivo cargando actualmente

### Interacción
- **Clic izquierdo** en la píldora o en **`⚙`** (o clic en el icono de bandeja): Abre **BatteryDock Settings** (`BatteryDock.exe --settings`).
- **Clic derecho** sobre la píldora o el icono de bandeja: Abre menú nativo (**Settings**, **Refresh Devices**, **Reset to Taskbar Clock**, **Restart BatteryDock**, **Exit BatteryDock**).
- **Clic y arrastrar (> 5px):** Mueve la píldora donde quieras (incluso a otro monitor); clic derecho → **Reset to Taskbar Clock** para devolverla al reloj.

---

## ⚙️ Configuración

Los ajustes se guardan automáticamente en `%APPDATA%\jorjanion\BatteryDock\config\config.toml`:

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

## 🛠️ Compilar desde el código fuente

### Requisitos
- [Toolchain de Rust](https://rustup.rs/) (stable)
- Windows 10 o Windows 11 (target MSVC)

### Comandos

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

# Verificar formato, lints y tests
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked

# Compilar binario optimizado de release
cargo build --release --locked
```

Ejecutable generado en `target\release\battery-dock.exe` (usa `--settings` / `-s` para abrir directamente los ajustes).

---

## 📄 Licencia

Distribuido bajo la **Licencia MIT**. Consulta [LICENSE](../../LICENSE) para más detalles.

<div align="center">

**Si BatteryDock te ayuda a tener tu escritorio limpio y tus periféricos cargados, ¡considera dejar una ⭐ en el repo!**

</div>
