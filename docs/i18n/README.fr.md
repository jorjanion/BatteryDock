<div align="center">

<img src="../../assets/logo.png" alt="BatteryDock Logo" width="148" style="border-radius: 24px;" />

# BatteryDock

**Toutes les batteries de vos périphériques sans fil dans une pastille native sur la barre des tâches Windows. Sans logiciel lourd.**

[![GitHub stars](https://img.shields.io/github/stars/jorjanion/BatteryDock?style=flat-square&logo=github)](https://github.com/jorjanion/BatteryDock/stargazers)
[![Latest Release](https://img.shields.io/github/v/release/jorjanion/BatteryDock?style=flat-square&color=22c55e)](https://github.com/jorjanion/BatteryDock/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/jorjanion/BatteryDock/total?style=flat-square&color=38bdf8)](https://github.com/jorjanion/BatteryDock/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jorjanion/BatteryDock/release.yml?branch=main&style=flat-square)](https://github.com/jorjanion/BatteryDock/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](../../LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6?style=flat-square&logo=windows)](https://github.com/jorjanion/BatteryDock)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Télécharger v1.0.0](https://github.com/jorjanion/BatteryDock/releases/latest) · [Signaler un bug](https://github.com/jorjanion/BatteryDock/issues/new) · [Demander un appareil](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md) · [Changelog](../../CHANGELOG.md)

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

Si vous utilisez une souris sans fil d'une marque, un casque d'une autre et une manette Bluetooth en plus, vérifier vos batteries sur Windows oblige souvent à garder plusieurs suites lourdes en arrière-plan.

**BatteryDock** remplace tout cela par une seule application native en Rust d'environ 4 Mo intégrée à votre barre des tâches Windows 10/11. Sans Electron, sans télémétrie, sans compte.

---

## Aperçu

<div align="center">
  <table>
    <tr>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-single.png" alt="BatteryDock single peripheral pill" /><br /><br />
        <sub><b>Un périphérique</b>: Icône vectorielle nette, jauge colorée et pourcentage sur la barre des tâches.</sub>
      </td>
      <td align="center" valign="middle" width="50%">
        <img src="../screenshots/dock-multi.png" alt="BatteryDock multiple peripherals pill" /><br /><br />
        <sub><b>Plusieurs périphériques</b>: Réorganisez vos appareils, renommez-les ou masquez ceux que vous ne voulez pas voir.</sub>
      </td>
    </tr>
  </table>
  <br />
  <table>
    <tr>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-peripherals.jpg" alt="BatteryDock Peripherals dashboard v1.0.0" /><br />
        <sub><b>Périphériques</b>: Autonomie estimée, courbe d'historique 24h, surnoms et alertes par appareil.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-alerts.jpg" alt="BatteryDock Alerts and Audio settings v1.0.0" /><br />
        <sub><b>Alertes et Audio</b>: Alertes batterie faible, critique et charge complète à 100% avec aperçu sonore.</sub>
      </td>
      <td align="center" valign="top" width="33%">
        <img src="../screenshots/settings-customization.jpg" alt="BatteryDock Customization settings v1.0.0" /><br />
        <sub><b>Personnalisation</b>: Thèmes Liquid Glass / Sombre / Clair / Système, HUD jeu, raccourcis et 21 langues.</sub>
      </td>
    </tr>
  </table>
</div>

---

## Fonctionnalités

### Pastille minimaliste sur la barre des tâches
- **4 thèmes visuels:** Choisissez **Liquid Glass** (ultra-transparent), **Sombre**, **Clair** ou **Système** (suit automatiquement le thème de Windows).
- **Zéro encombrement:** Clic gauche sur la pastille pour ouvrir les paramètres, clic droit pour le menu rapide, ou glissez-la sur n'importe quel écran.
- **Gestion intelligente:** Reste visible au-dessus de la barre des tâches même avec Démarrer ou Recherche ouverts, et se masque automatiquement en plein écran.
- **Disposition personnalisée:** Réorganisez vos appareils en un clic, donnez-leur des surnoms, masquez des périphériques ou affichez/masquez le pourcentage.

### Overlay optionnel pour jeux et plein écran (HUD)
- **Désactivé par défaut** pour ne jamais gêner vos vidéos en plein écran.
- Une fois activé, BatteryDock s'affiche dans un coin de l'écran pendant vos jeux en plein écran sans bordure.
- **Placement libre:** 4 coins prédéfinis avec marge réglable, ou position libre X/Y (0% à 100%) avec aperçu en direct.
- **Mode Click-through:** Les clics de la souris traversent le HUD directement vers votre jeu.
- **Raccourci global configurable:** Définissez n'importe quelle touche (par défaut `Alt + B`) pour afficher ou masquer l'overlay sans Alt+Tab.

### Télémétrie intelligente et alertes
- **Historique sur 24 heures:** Chaque carte affiche une courbe légère de charge et décharge.
- **Autonomie et veille:** Estime le temps restant (ou temps avant 100%) et détecte la mise en veille.
- **Alertes faible, critique et 100%:** Seuils globaux ou par appareil, mise en sourdine individuelle, son discret et notifications Windows natives.

### 21 langues intégrées
- Détecte automatiquement la langue de Windows au premier lancement (avec repli sur l'anglais si non prise en charge).
- Charge les polices natives de Windows à la demande pour un rendu net dans tous les alphabets.

---

## Matériel pris en charge

BatteryDock interroge directement votre matériel via HID, WinRT et Bluetooth sans logiciel fabricant.

| Catégorie | Protocole / Méthode | Exemples pris en charge |
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

## Téléchargement et installation

Téléchargez la dernière version sur la **[page Releases](https://github.com/jorjanion/BatteryDock/releases/latest)**:

1. Téléchargez **`BatteryDock-Setup-v1.0.0.exe`** (installateur Windows) ou **`BatteryDock-Windows-Portable.zip`** pour la version portable.
2. Lancez l'application et activez éventuellement le démarrage avec Windows dans les paramètres.
3. BatteryDock apparaît près de l'horloge avec mises à jour automatiques GitHub (désactivables).

### Note sur Windows SmartScreen
Projet open source gratuit sans certificat payant: SmartScreen peut afficher un avertissement au premier lancement. Cliquez sur **Informations complémentaires** puis **Exécuter quand même**. Tous les binaires sont compilés publiquement dans [`.github/workflows/release.yml`](../../.github/workflows/release.yml).

---

## Configuration

Les paramètres sont enregistrés dans `%APPDATA%\jorjanion\BatteryDock\config\config.toml` (et l'historique dans `history.toml`). Les modifications s'appliquent instantanément.

---

## Compiler depuis les sources

Prérequis: Windows 10/11 et [Rust](https://rustup.rs/) (MSVC stable).

```powershell
git clone https://github.com/jorjanion/BatteryDock.git
cd BatteryDock
cargo test --locked
cargo build --release --locked
.\target\release\battery-dock.exe
```

---

## Contribuer, Issues et Pull Requests

BatteryDock est open source et ouvert à la communauté. Vos idées, traductions, signalements et PR sont les bienvenus.

- **Appareil non détecté ?** Ouvrez une **[demande de support matériel](https://github.com/jorjanion/BatteryDock/issues/new?template=device-support.md)**. Cliquez sur **Paramètres > Personnalisation > Copier le diagnostic matériel** et collez le rapport anonymisé dans l'issue.
- **Bug ou suggestion ?** Ouvrez une **[Issue](https://github.com/jorjanion/BatteryDock/issues)** avec votre version de Windows et les étapes pour reproduire.
- **Envie de contribuer au code ?** Les Pull Requests sont les bienvenues. Lancez `cargo test --locked` et `cargo clippy --locked` avant d'ouvrir votre PR.

---

## Licence

Licence MIT. Voir [LICENSE](../../LICENSE).
