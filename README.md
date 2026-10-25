<div align="center">

<img src="icons/128x128.png" alt="NeonWhat logo" width="112" height="112">

# NeonWhat

**WhatsApp Web, but native. Tiny, fast, and built for Linux.**

A lightweight desktop wrapper for WhatsApp Web, built with **Rust** and **Tauri 2**.

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Tauri 2](https://img.shields.io/badge/Tauri-2-24C8DB?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app/)
[![Platform](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)](#-building-from-source)
[![License: MIT](https://img.shields.io/badge/License-MIT-22c55e?style=for-the-badge)](LICENSE)

[Features](#-features) · [Install](#-building-from-source) · [Themes](#-custom-themes) · [Known Issues](#%EF%B8%8F-known-limitations-linux--webkitgtk) · [License](#-license)

</div>

---

## 📖 About

NeonWhat wraps `web.whatsapp.com` in your system's native webview. The result is a **~5 MB** app that behaves like a real desktop client: native notifications, a tray icon with an unread badge, close-to-tray, and a global shortcut. No bundled Chromium required.

## ✨ Features

| | Feature | Description |
|---|---|---|
| 🔔 | **Native notifications** | Intercepts WhatsApp's browser notifications and routes them to your OS notification center. |
| 🔴 | **Tray icon & unread badge** | The tray icon changes when you have unread messages, and the tooltip shows the exact count. |
| 📥 | **Close to tray** | Clicking the window's **X** hides the app to the tray instead of quitting. |
| ⌨️ | **Global shortcut** | Press <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>W</kbd> from anywhere to show or hide the window. |
| 🔐 | **Session persistence** | Stay logged in across restarts, using Tauri's default app data directory. |
| 🎨 | **Custom CSS themes** | Load your own CSS from your local config folder to restyle WhatsApp Web. |
| 🕵️ | **User-agent spoofing** | Presents a modern Chrome user agent so WhatsApp Web doesn't block the WebKitGTK engine. |
| 1️⃣ | **Single instance** | Launching the app again focuses the existing window instead of opening a second one. |

## 🛠️ Tech Stack

| Layer | Technology |
|---|---|
| Core | [Rust](https://www.rust-lang.org/) |
| Framework | [Tauri 2](https://tauri.app/) |
| Webview (Linux) | WebKitGTK |
| Plugins | `tauri-plugin-notification`, `tauri-plugin-global-shortcut`, `tauri-plugin-single-instance` |

## 🚀 Building from Source

### Prerequisites

You need the [Rust toolchain](https://rustup.rs/) and the Tauri system dependencies.

**Ubuntu / Debian**

```bash
sudo apt update
sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  gstreamer1.0-plugins-good \
  gstreamer1.0-plugins-bad \
  gstreamer1.0-libav
```

> The GStreamer plugins are needed for audio and video playback inside the webview.

### Build

**1. Clone the repository**

```bash
git clone https://github.com/Neontoshi/neonwha.git
cd neonwha
```

**2. Run in development mode**

```bash
cargo run
```

**3. Build release bundles** (`.deb`, `.rpm`, `.AppImage`)

```bash
cargo tauri build
```

The installers are written to `target/release/bundle/`.

> If `cargo tauri` isn't found, install the CLI first with `cargo install tauri-cli --version "^2"`.

## 🎨 Custom Themes

NeonWhat can inject your own CSS into WhatsApp Web.

**1. Create the themes directory**

```bash
mkdir -p ~/.config/com.neonwhat.app/assets/themes/
```

**2. Add your stylesheet**

Create or copy a CSS file named `example.css` into that folder.

**3. Restart NeonWhat**

Your styles are applied on the next launch.

> **Note:** The folder name comes from the `identifier` in `tauri.conf.json`. If you change the identifier, update the path above to match.

## ⚠️ Known Limitations (Linux / WebKitGTK)

On Linux, Tauri uses the system's native WebKitGTK engine instead of bundling Chromium like Electron does. That keeps the app small, but it comes with a few quirks:

| Issue | Details |
|---|---|
| 🔇 **Status audio** | WhatsApp Status videos may play without sound. This is a known WebKitGTK bug with Media Source Extensions (MSE) and multi-track streams. Voice notes work fine. |
| 📞 **Voice / video calls** | WebRTC support in WebKitGTK is unreliable. Calls are best-effort and may not connect. |
| ⚠️ **Deprecation warning** | You may see `libayatana-appindicator is deprecated` in the terminal. It's a harmless warning from the Linux tray library. |

## 🤝 Contributing

Issues and pull requests are welcome. If you're fixing a bug, please include your distro and WebKitGTK version in the report.

## 📄 License

Released under the **MIT License**. See [`LICENSE`](LICENSE) for details.

## 🙏 Acknowledgements

- [Tauri](https://tauri.app/) for the lightweight framework that makes this possible.
- The open-source community for the reverse-engineered knowledge of WhatsApp Web's title-based unread counters and notification APIs.

---

<div align="center">

Made with 🦀 Rust · Not affiliated with WhatsApp or Meta

</div>
