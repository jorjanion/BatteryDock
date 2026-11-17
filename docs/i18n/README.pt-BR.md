<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock Logo" width="148" style="border-radius: 24px;" />

# BatteryDock

**Todas as baterias dos seus periféricos sem fio em uma cápsula nativa na barra de tarefas do Windows. Sem bloatware.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Baixar v1.0.0](https://github.com/jorjanion/BatteryDock/releases/latest) · [Reportar bug](https://github.com/jorjanion/BatteryDock/issues/new) · [Pedir suporte a dispositivo](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md) · [Changelog](../../CHANGELOG.md)

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

Se você usa mouse sem fio de uma marca, headset de outra e um controle ou fone Bluetooth, ver a bateria no Windows geralmente significa manter vários programas pesados abertos.

**BatteryDock** substitui tudo por um único aplicativo nativo em Rust de ~4 MB direto na barra de tarefas do Windows 10/11. Sem Electron, sem telemetria e sem contas.

---

## Prévia

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-single.png" alt="BatteryDock single peripheral pill" /><br /><br />
        <sub><b>Um periférico</b>: Ícone vetorial nítido, barra colorida e porcentagem direto na barra de tarefas.</sub>
      </td>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-multi.png" alt="BatteryDock multiple peripherals pill" /><br /><br />
        <sub><b>Vários periféricos</b>: Reordene dispositivos, renomeie ou oculte os que não quiser ver.</sub>
      </td>
    </tr>
  </table>
  <br />
  <table>
    <tr>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-peripherals.jpg" alt="BatteryDock Peripherals dashboard v1.0.0" /><br />
        <sub><b>Periféricos</b>: Autonomia estimada, gráfico de 24h, apelidos e alertas individuais.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-alerts.jpg" alt="BatteryDock Alerts and Audio settings v1.0.0" /><br />
        <sub><b>Alertas e Áudio</b>: Avisos de bateria fraca, crítica e carga completa (100%) com teste de som.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-customization.jpg" alt="BatteryDock Customization settings v1.0.0" /><br />
        <sub><b>Personalização</b>: Temas Liquid Glass / Escuro / Claro / Sistema, HUD para jogos, atalho global e 21 idiomas.</sub>
      </td>
    </tr>
  </table>
</div>

---

## Recursos

### Cápsula minimalista na barra de tarefas
- **4 temas visuais:** Escolha entre **Liquid Glass** (ultratransparente), **Escuro**, **Claro** ou **Sistema** (acompanha o tema do Windows).
- **Sem poluição visual:** Clique esquerdo abre as configurações, clique direito abre o menu rápido, ou arraste para qualquer monitor.
- **Comportamento inteligente:** Permanece acima da barra de tarefas mesmo ao abrir Iniciar ou Busca, e se oculta automaticamente em vídeos e jogos em tela cheia.
- **Layout flexível:** Reordene dispositivos com um clique, defina apelidos, oculte periféricos ou alterne o texto de porcentagem.

### Overlay opcional para jogos e tela cheia (HUD)
- **Desativado por padrão** para não atrapalhar filmes ou YouTube em tela cheia.
- Quando ativado, aparece em um canto da tela durante jogos em tela cheia sem bordas.
- **Posicione em qualquer lugar:** 4 cantos com margem ajustável ou posição livre X/Y (0% a 100%) com prévia ao vivo.
- **Modo Click-through:** Cliques do mouse passam direto para o jogo.
- **Atalho global configurável:** Defina qualquer combinação (padrão `Alt + B`) para mostrar ou ocultar o HUD sem dar Alt+Tab.

### Telemetria inteligente e alertas
- **Tendência de 24 horas:** Cada cartão exibe um gráfico leve com o histórico de carga e descarga.
- **Autonomia e modo espera:** Estima o tempo restante ou tempo até 100% e detecta periféricos inativos.
- **Alertas de bateria fraca, crítica e 100%:** Limites globais ou por dispositivo, silenciamento individual, som suave e notificações nativas do Windows.

### 21 idiomas incluídos
- Detecta automaticamente o idioma do Windows na primeira execução (com inglês como padrão caso não suportado).
- Carrega fontes nativas do Windows sob demanda para renderização nítida em todos os alfabetos.

---

## Hardware suportado

BatteryDock conversa diretamente com o hardware via HID, WinRT e Bluetooth sem precisar de programas de fabricantes.

| Categoria | Protocolo / Método | Exemplos suportados |
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

## Download e instalação

Baixe a versão mais recente na **[página de Releases](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Baixe **`BatteryDock-Setup-v1.0.0.exe`** (instalador) ou **`BatteryDock-Windows-Portable.zip`** (versão portátil).
2. Execute e, se desejar, ative **Iniciar BatteryDock com o Windows** nas configurações.
3. O indicador aparecerá ao lado do relógio, com atualizações automáticas pelo GitHub.

### Nota sobre o Windows SmartScreen
Por ser um projeto open source gratuito sem certificado pago, o SmartScreen pode exibir um aviso na primeira execução. Clique em **Mais informações** e depois em **Executar assim mesmo**. Todos os binários são compilados publicamente em [`.github/workflows/release.yml`](../../.github/workflows/release.yml).

---

## Configuração

As configurações ficam em `%APPDATA%\jorjanion\BatteryDock\config\config.toml` (e histórico em `history.toml`). Tudo é aplicado em tempo real.

---

## Compilar a partir do código-fonte

Requisitos: Windows 10/11 e [Rust](https://rustup.rs/) (toolchain MSVC estável).

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock
cargo test --locked
cargo build --release --locked
.\target\release\battery-dock.exe
```

---

## Contribuições, Issues e Pull Requests

BatteryDock é open source e aberto à comunidade. Relatos de bugs, traduções, pedidos de dispositivos e PRs são muito bem-vindos.

- **Seu dispositivo não apareceu?** Abra uma **[solicitação de dispositivo](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)**. Vá em **Configurações > Personalização > Copiar diagnóstico de hardware** e cole o relatório anônimo na issue.
- **Encontrou um bug ou tem uma ideia?** Abra uma **[Issue](https://github.com/jorjanion/BatteryDock/issues)** com sua versão do Windows e passos para reproduzir.
- **Quer contribuir com código?** Pull Requests são bem-vindas. Execute `cargo test --locked` e `cargo clippy --locked` antes de enviar.

---

## Licença

Licença MIT. Veja [LICENSE](../../LICENSE).
