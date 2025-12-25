<img align="left" width="80" height="80" src="assets/icon.png">

# Hachimi-Edge

[English](README.md) | [Español](README.es.md) | [Tiếng Việt](README.vi.md) | [简体中文](README.zh-CN.md) | 繁體中文

[![Discord server](https://dcbadge.limes.pink/api/server/https://discord.gg/BVEt5FcxEn)](https://discord.gg/BVEt5FcxEn)

UM:PD 的遊戲增強和翻譯模組。

<img height="400" src="assets/screenshot.jpg">

# ⚠️ 請不要連結到此儲存庫或 Hachimi-Edge 的網站
我們理解您想幫助人們安裝 Hachimi-Edge 並獲得更好的遊戲體驗。然而，這個專案本質上違反了遊戲的 TOS，如果遊戲開發者知道這件事，他們肯定希望它消失。

雖然在您自己管理的聊天服務和私訊中分享是可以的，但我們謙虛地要求您不要在面向公眾的網站上分享此專案的連結，或任何相關工具的連結。

或者分享它們並為數十名 Hachimi-Edge 使用者毀掉它。這取決於您。

### 如果您無論如何都要分享
做您必須做的事，但我們懇請您嘗試將遊戲標記為 "UM:PD" 或 "The Honse Game" 而不是遊戲的實際名稱，以避免搜尋引擎解析。

# 關於本分支

本分支專注於 **Windows 和 Linux (Proton) 優化**。基於 Mario0051 的 DXVK 改進，並額外專注於:

- ✅ **程式碼品質** - 大規模重構以實現正確的 Rust linting (snake_case, 警告修復)
- ✅ **Linux/Proton 支援** - 增強的 DXVK 相容性和 Proton 特定優化
- ✅ **僅 Windows/Proton** - 精簡的程式碼庫，移除 Android 支援以進行專注開發

**儲存庫傳承:**
```
kairusds/Hachimi-Edge → Mario0051/Hachimi-Edge → tenshou170/Hachimi-Edge (本分支)
     (主儲存庫)             (初始 DXVK 支援)         (增強 Proton + 程式碼品質)
```

# 功能
- **高品質翻譯:** Hachimi-Edge 配備了先進的翻譯功能，使翻譯感覺更自然(複數形式、序數等)，並防止給 UI 帶來問題。它還支援翻譯大多數遊戲內元件；無需手動修補資源！

    支援的元件:
    - UI 文字
    - master.mdb (技能名稱、技能描述等)
    - 比賽故事
    - 主線故事/主頁對話
    - 歌詞
    - 材質替換
    - 精靈圖集替換

    此外，Hachimi-Edge 不僅為單一語言提供翻譯功能；它被設計為可完全設定任何語言。

- **簡單設定:** 即插即用。所有設定都在遊戲內完成，無需外部應用程式！
- **翻譯自動更新:** 內建翻譯更新器讓您在更新時正常玩遊戲，完成後在遊戲內重新載入，無需重啟！
- **內建 GUI:** 配備設定編輯器，您可以在不退出遊戲的情況下修改設定！
- **圖形設定:** 您可以調整遊戲的圖形設定以充分利用裝置規格，包括:
  - FPS 解鎖
  - 解析度縮放
  - 抗鋸齒 (MSAA) 選項
  - VSync 控制
  - 全螢幕模式選項
- **Linux/Proton 支援:** 增強與 DXVK 和 Proton 的相容性，實現無縫 Linux 遊戲體驗。
- **跨平台:** 從一開始就設計為可攜式，支援 Windows 和 Linux (Proton)。

# 安裝
請查看 [Getting started](https://hachimi.noccu.art/docs/hachimi/getting-started.html) 頁面。

# 特別感謝
這些專案是 Hachimi-Edge 開發的基礎；沒有它們，Hachimi-Edge 不可能以現在的形式存在:

- [Trainers' Legend G](https://github.com/MinamiChiwa/Trainers-Legend-G)
- [umamusume-localify-android](https://github.com/Kimjio/umamusume-localify-android)
- [umamusume-localify](https://github.com/GEEKiDoS/umamusume-localify)
- [Carotenify](https://github.com/KevinVG207/Uma-Carotenify)
- [umamusu-translate](https://github.com/noccu/umamusu-translate)
- [frida-il2cpp-bridge](https://github.com/vfsfitvnm/frida-il2cpp-bridge)

**分支特定鳴謝:**
- **原始 Hachimi** - [LeadRDRK](https://github.com/LeadRDRK) 和 Hachimi 團隊
- **Hachimi-Edge (主要)** - [kairusds](https://github.com/kairusds)
- **DXVK/Linux 改進** - [Mario0051](https://github.com/Mario0051)

# 授權
[GNU GPLv3](LICENSE)
