<div align="center">

<img src="../../assets/logo.png" alt="Logo do BatteryDock" width="160" style="border-radius: 24px;" />

# BatteryDock

**Um indicador de bateria nativo em Rust integrado diretamente à barra de tarefas do Windows 10/11 — sem programas pesados de fabricantes.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Baixar último instalador](https://github.com/jorjanion/BatteryDock/releases/latest) · [Reportar um bug](https://github.com/jorjanion/BatteryDock/issues) · [Pedir suporte a dispositivo](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)

<br />

<sub>
<a href="../../README.md">English</a> ·
<a href="README.es.md">Español</a> ·
<a href="README.fr.md">Français</a> ·
<a href="README.de.md">Deutsch</a> ·
<b>Português</b> ·
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

## 📸 Prévia

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="42%">
        <img src="../screenshots/dock-closeup.png" alt="Close da cápsula do BatteryDock mostrando fone em 92%" /><br /><br />
        <sub><b>Cápsula na barra de tarefas</b> — Integrada diretamente à barra do Windows com escala DPI por monitor, cores por nível e ícone ⚡ de carregamento.</sub>
      </td>
      <td align="center" valign="middle" width="58%">
        <img src="../screenshots/settings.png" alt="Janela de configurações do BatteryDock v0.1.11" width="340" /><br />
        <sub><b>Configurações independentes</b> — Limites de alerta, teste de som, posicionamento e lista de dispositivos conectados.</sub>
      </td>
    </tr>
  </table>
</div>

---

## ⚡ Por que usar o BatteryDock?

A maioria dos periféricos sem fio prende a porcentagem da bateria dentro de softwares pesados rodando dezenas de serviços. O **BatteryDock** mostra tudo em uma cápsula minimalista na barra de tarefas:

- **Zero bloatware:** Um único binário nativo em Rust de **~4 MB** (sem Electron, sem telemetria, ~77 MB de memória privada medidos ao vivo no Windows 11).
- **Leitura direta de hardware:** Consulta **Logitech HID++ 2.0**, **Windows Bluetooth / BLE GATT**, **XInput** e **Sony DualSense / DualShock 4 HID** diretamente — dispositivos Logitech reportam mesmo sem o G HUB aberto (e também lê via G HUB IPC quando ativo).
- **Regras inteligentes de tela cheia:** Oculta automaticamente quando jogos em tela cheia, vídeos ou a barra oculta cobrem a taskbar — e continua firme ao pressionar a tecla `Win`, abrir Iniciar/Pesquisa/Widgets ou clicar na área de trabalho.
- **Indicador de carregamento ao vivo:** Destaca o dispositivo em **ciano** com um **raio ⚡** quando estiver carregando.
- **Alertas funcionam em segundo plano:** Sinais sonoros e notificações disparam mesmo quando a cápsula está oculta durante um jogo em tela cheia.

---

## 📥 Download e Instalação

Baixe a versão mais recente na **[página de Releases](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Baixe **`BatteryDock-Setup-v0.1.11.exe`** (~4,4 MB) — ou `BatteryDock-Windows-Portable.zip` para usar sem instalar.
2. Execute o instalador e marque **"Start BatteryDock when Windows boots"** se desejar iniciar junto com o Windows.
3. A cápsula aparecerá ao lado do relógio na barra de tarefas.

### 🛡️ Aviso sobre o Windows SmartScreen
Como o BatteryDock é um projeto open-source gratuito sem certificado pago de assinatura EV, o SmartScreen pode exibir *"O Windows protegeu o computador"* na primeira execução (**Mais informações → Executar assim mesmo**).
- Todos os executáveis e instaladores são compilados publicamente pelo GitHub Actions em [`.github/workflows/release.yml`](../../.github/workflows/release.yml).
- Você também pode revisar todo o código Rust e [compilar localmente](#build-from-source) em menos de um minuto.

---

## 🎧 Hardware Suportado

| Categoria | Protocolo / Fonte | Exemplos e notas |
| :--- | :--- | :--- |
| **Logitech Sem Fio** | HID++ 2.0 direto (`0x1004` / `0x1000`) & G HUB IPC | Headsets Lightspeed (G733, G535, G935), mouses (G PRO, Superlight, G502), teclados |
| **Bluetooth & BLE** | Windows WinRT PnP + BLE GATT (`0x180F` / `0x2A19`) | Fones (Sony WH/WF, Bose, AirPods), mouses e teclados BT |
| **Controles Xbox** | Windows `XInput` | Xbox Wireless Controllers (Series X\|S, One, 360) com detecção de carga via cabo |
| **PlayStation** | Sony HID (`VID 0x054C`) USB e Bluetooth | DualSense e DualShock 4 (múltiplos controles + status de carregamento) |

**Faltou sua marca (Razer, Corsair, SteelSeries, Keychron, etc.)?**
👉 **[Solicite suporte a um dispositivo](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** — Pull Requests também são muito bem-vindos!

---

## 🎯 Controles e Indicadores

### Cores de Status
- **Verde:** Bateria saudável (acima do limite baixo)
- **Âmbar:** Bateria baixa (configurável, padrão `20%`)
- **Vermelho (pulsando):** Bateria crítica (padrão `10%`)
- **Ciano + ⚡:** Carregando no momento

### Interação
- **Clique esquerdo** na cápsula ou em **`⚙`** (ou no ícone da bandeja): Abre **BatteryDock Settings** (`BatteryDock.exe --settings`).
- **Clique direito** na cápsula ou na bandeja: Abre menu nativo (**Settings**, **Refresh Devices**, **Reset to Taskbar Clock**, **Restart BatteryDock**, **Exit BatteryDock**).
- **Clicar e arrastar (> 5px):** Mova para qualquer lugar (inclusive outros monitores); clique direito → **Reset to Taskbar Clock** volta para o relógio.

---

## ⚙️ Configuração

Salva automaticamente em `%APPDATA%\jorjanion\BatteryDock\config\config.toml`:

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

## 🛠️ Compilar do Código-Fonte

### Pré-requisitos
- [Toolchain Rust](https://rustup.rs/) (stable)
- Windows 10 ou Windows 11 (target MSVC)

### Comandos

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

# Verificar formatação, lints e testes
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked

# Compilar binário release otimizado
cargo build --release --locked
```

Executável gerado em `target\release\battery-dock.exe` (passe `--settings` / `-s` para abrir configurações diretamente).

---

## 📄 Licença

Distribuído sob a **Licença MIT**. Veja [LICENSE](../../LICENSE).

<div align="center">

**Se o BatteryDock ajudou a manter seu setup limpo e seus periféricos carregados, deixe uma ⭐ no repositório!**

</div>
