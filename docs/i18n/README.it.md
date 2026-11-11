<div align="center">

<img src="../../assets/logo.png" alt="Logo BatteryDock" width="160" style="border-radius: 24px;" />

# BatteryDock

**Un indicatore di batteria nativo in Rust integrato nella barra delle applicazioni di Windows 10/11 — zero bloatware dei produttori.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Scarica l'ultimo installer](https://github.com/jorjanion/BatteryDock/releases/latest) · [Segnala un bug](https://github.com/jorjanion/BatteryDock/issues) · [Richiedi supporto dispositivo](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)

<br />

<sub>
<a href="../../README.md">English</a> ·
<a href="README.es.md">Español</a> ·
<a href="README.fr.md">Français</a> ·
<a href="README.de.md">Deutsch</a> ·
<a href="README.pt-BR.md">Português</a> ·
<b>Italiano</b> ·
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

## 📸 Anteprima

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="42%">
        <img src="../screenshots/dock-closeup.png" alt="Dettaglio della capsula BatteryDock con cuffie al 92%" /><br /><br />
        <sub><b>Capsula sulla barra delle applicazioni</b> — Integrata direttamente sulla taskbar di Windows con scaling DPI per monitor, colori di stato e badge di ricarica ⚡.</sub>
      </td>
      <td align="center" valign="middle" width="58%">
        <img src="../screenshots/settings.png" alt="Finestra impostazioni di BatteryDock v0.1.11" width="340" /><br />
        <sub><b>Impostazioni dedicate</b> — Soglie di avviso, anteprima suono, posizionamento e dispositivi connessi.</sub>
      </td>
    </tr>
  </table>
</div>

---

## ⚡ Perché BatteryDock?

Quasi tutte le periferiche wireless nascondono la carica della batteria dentro suite pesanti con decine di servizi in background. **BatteryDock** mostra tutto in una capsula pulita sulla barra delle applicazioni:

- **Zero bloatware:** Un singolo eseguibile nativo Rust da **~4 MB** (niente Electron, nessuna telemetria, ~77 MB di memoria privata misurati su Windows 11).
- **Lettura hardware diretta:** Interroga direttamente **Logitech HID++ 2.0**, **Windows Bluetooth / BLE GATT**, **XInput** e **Sony DualSense / DualShock 4 HID** — i dispositivi Logitech funzionano anche senza aprire G HUB (supportando anche G HUB IPC se attivo).
- **Gestione intelligente schermo intero:** Si nasconde automaticamente con giochi o video a schermo intero e barra a scomparsa, restando visibile alla pressione del tasto `Win`, Start/Ricerca/Widget o clic sul desktop.
- **Indicatore di ricarica live:** Evidenzia in **ciano** con un **fulmine ⚡** quando il dispositivo è in carica.
- **Avvisi attivi in background:** Suoni e notifiche desktop funzionano anche mentre la capsula è nascosta durante il gioco a schermo intero.

---

## 📥 Download e Installazione

Scarica l'ultima versione dalla **[pagina Releases](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Scarica **`BatteryDock-Setup-v0.1.11.exe`** (~4,4 MB) — oppure `BatteryDock-Windows-Portable.zip` per la versione portatile.
2. Avvia l'installer e seleziona opzionalmente **"Start BatteryDock when Windows boots"**.
3. La capsula apparirà accanto all'orologio nella barra delle applicazioni.

### 🛡️ Nota su Windows SmartScreen
Essendo un'app open-source gratuita senza certificato EV a pagamento, SmartScreen potrebbe mostrare *"Windows ha protetto il PC"* al primo avvio (**Ulteriori informazioni → Esegui comunque**).
- Ogni release viene compilata pubblicamente sui runner Windows di GitHub Actions in [`.github/workflows/release.yml`](../../.github/workflows/release.yml).
- Puoi ispezionare tutto il codice e [compilarlo in locale](#build-from-source) in meno di un minuto.

---

## 🎧 Hardware Supportato

| Categoria | Protocollo / Sorgente | Esempi e note |
| :--- | :--- | :--- |
| **Logitech Wireless** | HID++ 2.0 diretto (`0x1004` / `0x1000`) & G HUB IPC | Cuffie Lightspeed (G733, G535, G935), mouse (G PRO, Superlight, G502), tastiere |
| **Bluetooth & BLE** | Windows WinRT PnP + BLE GATT (`0x180F` / `0x2A19`) | Cuffie (Sony WH/WF, Bose, AirPods), mouse e tastiere BT |
| **Controller Xbox** | Windows `XInput` | Controller Xbox Wireless (Series X\|S, One, 360) incluso stato carica via cavo |
| **PlayStation** | Sony HID (`VID 0x054C`) USB & Bluetooth | DualSense e DualShock 4 (supporto multi-controller + stato ricarica) |

**Manca la tua marca (Razer, Corsair, SteelSeries, Keychron, ecc.)?**
👉 **[Richiedi supporto per un dispositivo](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** — anche le Pull Request sono benvenute!

---

## 🎯 Controlli e Indicatori

### Colori di stato
- **Verde:** Batteria carica (sopra la soglia bassa)
- **Ambra:** Batteria scarica (configurabile, predefinito `20%`)
- **Rosso (pulsante):** Livello critico (`10%`)
- **Ciano + ⚡:** Dispositivo in carica

### Interazione
- **Clic sinistro** su capsula o **`⚙`** (o icona tray): Apre **BatteryDock Settings** (`BatteryDock.exe --settings`).
- **Clic destro** su capsula o tray: Apre menu nativo (**Settings**, **Refresh Devices**, **Reset to Taskbar Clock**, **Restart BatteryDock**, **Exit BatteryDock**).
- **Clic e trascinamento (> 5px):** Sposta la capsula ovunque (anche su altri monitor); clic destro → **Reset to Taskbar Clock** per riagganciarla.

---

## ⚙️ Configurazione

Le impostazioni vengono salvate in `%APPDATA%\jorjanion\BatteryDock\config\config.toml`:

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

## 🛠️ Compilazione dal Codice Sorgente

### Prerequisiti
- [Toolchain Rust](https://rustup.rs/) (stable)
- Windows 10 o Windows 11 (target MSVC)

### Comandi

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

# Verifica formattazione, Clippy e test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked

# Compila binario release ottimizzato
cargo build --release --locked
```

Eseguibile generato in `target\release\battery-dock.exe` (usa `--settings` / `-s` per aprire le impostazioni).

---

## 📄 Licenza

Rilasciato sotto **Licenza MIT**. Vedi [LICENSE](../../LICENSE).

<div align="center">

**Se BatteryDock ti aiuta a tenere in ordine la postazione, lascia una ⭐ al repository!**

</div>
