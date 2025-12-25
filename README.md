<img align="left" width="80" height="80" src="assets/icon.png">

# Hachimi-Edge

English | [Español](README.es.md) | [Tiếng Việt](README.vi.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md)

[![Discord server](https://dcbadge.limes.pink/api/server/https://discord.gg/BVEt5FcxEn)](https://discord.gg/BVEt5FcxEn)

Game enhancement and translation mod for UM:PD.

<img height="400" src="assets/screenshot.jpg">

# ⚠️ Please don't link to this repo or Hachimi-Edge's website
We understand that you want to help people install Hachimi-Edge and have a better experience playing the game. However, this project is inherently against the game's TOS and The Game Developer most definitely wants it gone if they were ever to learn about it.

While sharing in your self-managed chat services and through private messaging is fine, we humbly ask that you refrain from sharing links to this project on public facing sites, or to any of the tools involved.

Or share them and ruin it for the dozens of Hachimi-Edge users. It's up to you.

### If you're going to share it anyways
Do what you must, but we would respectfully request that you try to label the game as "UM:PD" or "The Honse Game" instead of the actual name of the game, to avoid search engine parsing.

# About This Fork

This fork focuses on **Windows and Linux (Proton) optimization**. Built upon Mario0051's DXVK improvements with additional focus on:

- ✅ **Code Quality** - Mass refactoring for proper Rust linting (snake_case, warning fixes)
- ✅ **Linux/Proton Support** - Enhanced DXVK compatibility and Proton-specific optimizations
- ✅ **Windows/Proton Only** - Streamlined codebase, Android support removed for focused development

**Repository Lineage:**
```
kairusds/Hachimi-Edge → Mario0051/Hachimi-Edge → tenshou170/Hachimi-Edge (This Fork)
     (Main repo)              (Initial DXVK support)    (Enhanced Proton + Code Quality)
```

# Features
- **High quality translations:** Hachimi-Edge comes with advanced translation features that help translations feel more natural (plural forms, ordinal numbers, etc.) and prevent introducing jank to the UI. It also supports translating most in-game components; no manual assets patching needed!

    Supported components:
    - UI text
    - master.mdb (skill name, skill desc, etc.)
    - Race story
    - Main story/Home dialog
    - Lyrics
    - Texture replacement
    - Sprite atlas replacement

    Additionally, Hachimi-Edge does not provide translation features for only a single language; it has been designed to be fully configurable for any language.

- **Easy setup:** Just plug and play. All setup is done within the game itself, no external application needed.
- **Translation auto update:** Built-in translation updater lets you play the game as normal while it updates, and reloads it in-game when it's done, no restart needed!
- **Built-in GUI:** Comes with a config editor so you can modify settings without even exiting the game!
- **Graphics settings:** You can adjust the game's graphics settings to make full use of your device's specs, including:
  - FPS unlocking
  - Resolution scaling
  - Anti-aliasing (MSAA) options
  - VSync control
  - Fullscreen mode options
- **Linux/Proton support:** Enhanced compatibility with DXVK and Proton for seamless Linux gaming experience.
- **Cross-platform:** Designed from the ground up to be portable, with Windows and Linux (Proton) support.

# Installation
Please see the [Getting started](https://hachimi.noccu.art/docs/hachimi/getting-started.html) page.

# Special thanks
These projects have been the basis for Hachimi-Edge's development; without them, Hachimi-Edge would never have existed in its current form:

- [Trainers' Legend G](https://github.com/MinamiChiwa/Trainers-Legend-G)
- [umamusume-localify-android](https://github.com/Kimjio/umamusume-localify-android)
- [umamusume-localify](https://github.com/GEEKiDoS/umamusume-localify)
- [Carotenify](https://github.com/KevinVG207/Uma-Carotenify)
- [umamusu-translate](https://github.com/noccu/umamusu-translate)
- [frida-il2cpp-bridge](https://github.com/vfsfitvnm/frida-il2cpp-bridge)

**Fork-specific credits:**
- **Original Hachimi** - [LeadRDRK](https://github.com/LeadRDRK) and the Hachimi Team
- **Hachimi-Edge (Main)** - [kairusds](https://github.com/kairusds)
- **DXVK/Linux Improvements** - [Mario0051](https://github.com/Mario0051)

# License
[GNU GPLv3](LICENSE)
