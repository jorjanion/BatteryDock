<div align="center">

<img src="../../assets/logo.png" alt="Logo BatteryDock" width="160" style="border-radius: 24px;" />

# BatteryDock

**Un indicateur de batterie natif en Rust intégré directement dans la barre des tâches Windows 10/11 — sans logiciel constructeur lourd.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Télécharger l'installateur](https://github.com/jorjanion/BatteryDock/releases/latest) · [Signaler un bug](https://github.com/jorjanion/BatteryDock/issues) · [Demander un appareil](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)

<br />

<sub>
<a href="../../README.md">English</a> ·
<a href="README.es.md">Español</a> ·
<b>Français</b> ·
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

## 📸 Aperçu

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="42%">
        <img src="../screenshots/dock-closeup.png" alt="Gros plan de la capsule BatteryDock affichant 92 %" /><br /><br />
        <sub><b>Capsule barre des tâches</b> — Intégrée directement à la barre Windows avec mise à l'échelle DPI par moniteur, couleurs par seuil et badge ⚡ en charge.</sub>
      </td>
      <td align="center" valign="middle" width="58%">
        <img src="../screenshots/settings.png" alt="Fenêtre des paramètres BatteryDock v0.1.11" width="340" /><br />
        <sub><b>Paramètres dédiés</b> — Curseurs d'alerte, test audio, positionnement et liste des périphériques connectés.</sub>
      </td>
    </tr>
  </table>
</div>

---

## ⚡ Pourquoi choisir BatteryDock ?

La plupart des périphériques sans fil enferment l'état de la batterie dans des suites logicielles lourdes exécutant des dizaines de services. **BatteryDock** affiche tout dans une pilule discrète sur votre barre des tâches :

- **Zéro bloatware :** Un unique binaire natif Rust de **~4 Mo** (sans Electron, sans télémétrie, ~77 Mo de mémoire privée mesurés sous Windows 11).
- **Lecture matérielle directe :** Interroge **Logitech HID++ 2.0**, **Windows Bluetooth / BLE GATT**, **XInput** et **Sony DualSense / DualShock 4 HID** directement — vos appareils Logitech fonctionnent même sans ouvrir G HUB (et l'IPC G HUB est utilisé s'il tourne).
- **Gestion intelligente du plein écran :** Se masque automatiquement devant les jeux plein écran, vidéos plein écran ou barres masquées automatiquement, tout en restant stable lors de l'appui sur la touche `Win`, l'ouverture de Démarrer/Widgets ou un clic sur le bureau.
- **Indicateur de charge en temps réel :** Affiche l'appareil en **cyan** avec un **éclair ⚡** lorsqu'il est branché.
- **Alertes actives même en jeu :** Les alertes sonores et notifications Windows partent depuis le moteur d'arrière-plan même lorsque la pilule est masquée en plein écran.

---

## 📥 Téléchargement et installation

Récupérez la dernière version sur la **[page Releases](https://github.com/jorjanion/BatteryDock/releases/latest)** :

1. Téléchargez **`BatteryDock-Setup-v0.1.11.exe`** (~4,4 Mo) — ou `BatteryDock-Windows-Portable.zip` pour une utilisation portable.
2. Lancez l'installateur et cochez éventuellement **"Start BatteryDock when Windows boots"**.
3. La capsule s'affiche immédiatement à côté de l'horloge.

### 🛡️ Note sur Windows SmartScreen
BatteryDock étant un projet open-source gratuit sans certificat EV payant, Windows SmartScreen peut afficher *"Windows a protégé votre ordinateur"* au premier lancement (**Informations complémentaires → Exécuter quand même**).
- Chaque binaire est compilé publiquement sur les runners GitHub Actions via [`.github/workflows/release.yml`](../../.github/workflows/release.yml).
- Vous pouvez auditer le code et le [compiler vous-même](#build-from-source) en moins d'une minute.

---

## 🎧 Matériel pris en charge

| Catégorie | Protocole / Source | Exemples et notes |
| :--- | :--- | :--- |
| **Logitech sans fil** | HID++ 2.0 direct (`0x1004` / `0x1000`) & IPC G HUB | Casques Lightspeed (G733, G535, G935), souris (G PRO, Superlight, G502), claviers |
| **Bluetooth & BLE** | Windows WinRT PnP + BLE GATT (`0x180F` / `0x2A19`) | Casques (Sony WH/WF, Bose, AirPods), souris et claviers BT |
| **Manettes Xbox** | Windows `XInput` | Manettes Xbox Wireless (Series X\|S, One, 360) avec détection filaire en charge |
| **PlayStation** | Sony HID (`VID 0x054C`) USB & Bluetooth | DualSense & DualShock 4 (multi-manettes + état de charge) |

**Votre marque manque (Razer, Corsair, SteelSeries, Keychron...) ?**
👉 **[Ouvrir une demande d'appareil](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)** — les contributions via PR sont également bienvenues !

---

## 🎯 Contrôles et indicateurs

### Couleurs d'état
- **Vert :** Batterie saine (au-dessus du seuil bas)
- **Ambre :** Batterie faible (configurable, `20%` par défaut)
- **Rouge (pulsant) :** Batterie critique (`10%` par défaut)
- **Cyan + ⚡ :** Périphérique en charge

### Interactions
- **Clic gauche** sur la capsule ou **`⚙`** (ou clic sur l'icône système) : Ouvre **BatteryDock Settings** (`BatteryDock.exe --settings`).
- **Clic droit** sur la capsule ou l'icône de zone de notification : Menu natif (**Settings**, **Refresh Devices**, **Reset to Taskbar Clock**, **Restart BatteryDock**, **Exit BatteryDock**).
- **Cliquer-glisser (> 5px) :** Déplacez librement la capsule (y compris sur d'autres écrans) ; clic droit → **Reset to Taskbar Clock** pour la recentrer.

---

## ⚙️ Configuration

Les paramètres sont enregistrés dans `%APPDATA%\jorjanion\BatteryDock\config\config.toml` :

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

## 🛠️ Compiler depuis les sources

### Prérequis
- [Chaîne d'outils Rust](https://rustup.rs/) (stable)
- Windows 10 ou Windows 11 (cible MSVC)

### Commandes

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock

# Vérifier le formatage, clippy et les tests
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked

# Compiler le binaire release optimisé
cargo build --release --locked
```

Exécutable généré : `target\release\battery-dock.exe` (passez `--settings` / `-s` pour ouvrir les paramètres).

---

## 📄 Licence

Distribué sous **Licence MIT**. Voir [LICENSE](../../LICENSE).

<div align="center">

**Si BatteryDock vous est utile au quotidien, pensez à laisser une ⭐ sur le dépôt !**

</div>
