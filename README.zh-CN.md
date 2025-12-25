<img align="left" width="80" height="80" src="assets/icon.png">

# Hachimi-Edge

[English](README.md) | [Español](README.es.md) | [Tiếng Việt](README.vi.md) | 简体中文 | [繁體中文](README.zh-TW.md)

[![Discord server](https://dcbadge.limes.pink/api/server/https://discord.gg/BVEt5FcxEn)](https://discord.gg/BVEt5FcxEn)

UM:PD 的游戏增强和翻译模组。

<img height="400" src="assets/screenshot.jpg">

# ⚠️ 请不要链接到此仓库或 Hachimi-Edge 的网站
我们理解您想帮助人们安装 Hachimi-Edge 并获得更好的游戏体验。然而，这个项目本质上违反了游戏的 TOS，如果游戏开发者知道这件事，他们肯定希望它消失。

虽然在您自己管理的聊天服务和私信中分享是可以的，但我们谦虚地要求您不要在面向公众的网站上分享此项目的链接，或任何相关工具的链接。

或者分享它们并为数十名 Hachimi-Edge 用户毁掉它。这取决于您。

### 如果您无论如何都要分享
做您必须做的事，但我们恳请您尝试将游戏标记为 "UM:PD" 或 "The Honse Game" 而不是游戏的实际名称，以避免搜索引擎解析。

# 关于本分支

本分支专注于 **Windows 和 Linux (Proton) 优化**。基于 Mario0051 的 DXVK 改进，并额外专注于:

- ✅ **代码质量** - 大规模重构以实现正确的 Rust linting (snake_case, 警告修复)
- ✅ **Linux/Proton 支持** - 增强的 DXVK 兼容性和 Proton 特定优化
- ✅ **仅 Windows/Proton** - 精简的代码库，移除 Android 支持以进行专注开发

**仓库传承:**
```
kairusds/Hachimi-Edge → Mario0051/Hachimi-Edge → tenshou170/Hachimi-Edge (本分支)
     (主仓库)               (初始 DXVK 支持)         (增强 Proton + 代码质量)
```

# 功能
- **高质量翻译:** Hachimi-Edge 配备了先进的翻译功能，使翻译感觉更自然(复数形式、序数等)，并防止给 UI 带来问题。它还支持翻译大多数游戏内组件；无需手动修补资源！

    支持的组件:
    - UI 文本
    - master.mdb (技能名称、技能描述等)
    - 比赛故事
    - 主线故事/主页对话
    - 歌词
    - 纹理替换
    - 精灵图集替换

    此外，Hachimi-Edge 不仅为单一语言提供翻译功能；它被设计为可完全配置任何语言。

- **简单设置:** 即插即用。所有设置都在游戏内完成，无需外部应用程序！
- **翻译自动更新:** 内置翻译更新器让您在更新时正常玩游戏，完成后在游戏内重新加载，无需重启！
- **内置 GUI:** 配备配置编辑器，您可以在不退出游戏的情况下修改设置！
- **图形设置:** 您可以调整游戏的图形设置以充分利用设备规格，包括:
  - FPS 解锁
  - 分辨率缩放
  - 抗锯齿 (MSAA) 选项
  - VSync 控制
  - 全屏模式选项
- **Linux/Proton 支持:** 增强与 DXVK 和 Proton 的兼容性，实现无缝 Linux 游戏体验。
- **跨平台:** 从一开始就设计为可移植，支持 Windows 和 Linux (Proton)。

# 安装
请查看 [Getting started](https://hachimi.noccu.art/docs/hachimi/getting-started.html) 页面。

# 特别感谢
这些项目是 Hachimi-Edge 开发的基础；没有它们，Hachimi-Edge 不可能以现在的形式存在:

- [Trainers' Legend G](https://github.com/MinamiChiwa/Trainers-Legend-G)
- [umamusume-localify-android](https://github.com/Kimjio/umamusume-localify-android)
- [umamusume-localify](https://github.com/GEEKiDoS/umamusume-localify)
- [Carotenify](https://github.com/KevinVG207/Uma-Carotenify)
- [umamusu-translate](https://github.com/noccu/umamusu-translate)
- [frida-il2cpp-bridge](https://github.com/vfsfitvnm/frida-il2cpp-bridge)

**分支特定鸣谢:**
- **原始 Hachimi** - [LeadRDRK](https://github.com/LeadRDRK) 和 Hachimi 团队
- **Hachimi-Edge (主要)** - [kairusds](https://github.com/kairusds)
- **DXVK/Linux 改进** - [Mario0051](https://github.com/Mario0051)

# 许可
[GNU GPLv3](LICENSE)
