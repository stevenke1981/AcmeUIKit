# Acme UI Kit — Rust GUI 全新開發包

一套以 **GPUI**（Zed 原生 UI 框架）重新設計的 Rust 桌面 GUI 元件庫。程式碼為全新撰寫，不直接複製原專案實作。

## 目前進度 — V1+V2+V3+V4

| 階段 | 元件 | 狀態 |
|------|------|------|
| **V1 基礎** | Button、Card、Badge、Progress、Switch、FieldShell、Tabs、Separator、Skeleton、Theme（Light/Dark）、primitives | ✅ |
| **V2 控制項** | TextInput、Textarea、Checkbox、Radio/RadioGroup、Select、Combobox、Menu、Dialog、Popover、Tooltip、Notification、IconProvider | ✅ |
| **V3 資料與佈局** | Pagination（+10 測試）、Sidebar、Resizable、LoadingState、VirtualList、**Tree**、**Form+Validation**、**Table（可排序）** | ✅ |
| **V4 豐富內容** | **SettingsPage**、**Tiles**、**Markdown**、**BarChart**、**Dock** | ✅ |

**總計**：28 個元件，約 8000+ 行程式碼，零警告編譯。

## 快速開始

```powershell
# 必要條件：
#   - Visual Studio 2022 Build Tools（桌面 C++ 工作負載）
#   - Windows 10/11 SDK
#   - Rust nightly（rust-toolchain.toml 會自動設定）

# 執行 Gallery 展示程式：
cargo run -p acme-gallery
```

Gallery 功能：切換 Light/Dark 主題、所有元件的互動示範。

## 專案結構

```
acme-ui-kit/
├── apps/acme-gallery/       # 互動式元件展示
├── crates/acme-ui/src/      # 32 個原始檔
│   ├── lib.rs               # 模組宣告 + re-export
│   ├── theme.rs             # Theme、FontSizes、Spacing、ThemeColors
│   ├── styled.rs            # StyledExt 輔助函式（h_flex、v_flex）
│   ├── primitives.rs        # Size / Tone 列舉
│   ├── icons.rs             # IconProvider、IconName
│   └── *.rs                 # 每元件一個檔案
├── docs/                    # 架構、設計系統、API、路線圖
├── scripts/                 # Windows / Unix 輔助腳本
├── AGENTS.md                # Agent 工作流程規則（含 git push）
├── UI_DESIGN_PRINCIPLES.md  # 字型與間距設計限制
├── spec.md / plan.md / todos.md / test.md
└── Cargo.toml               # GPUI 鎖定在某個 Zed revision
```

## 設計原則

- **Theme 優先**：所有顏色從 `cx.theme().colors.*` 取得，無寫死色碼。
- **Token 系統**：`FontSizes`（heading/body/caption）、`Spacing`（widget/group/section/panel）。
- **RenderOnce 優先**：無狀態視圖用 `RenderOnce`；需要焦點、非同步或 IME 才升級為 `Entity + Render`。
- **Gallery 驅動**：每新增一個元件必須同步加入互動示範。
- **Clean-room**：重新實作 API 形狀與 UX，不複製原始碼。

## 使用元件

```rust
use acme_ui::{Button, ActiveTheme, StyledExt};

// 全元件使用 Builder 模式：
Button::new("id")
    .label("按我")
    .primary()
    .small()
    .on_click(|_event, _window, cx| { cx.notify(); });
```

## 建置與驗證

```powershell
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## GPUI 升級

根目錄 `Cargo.toml` 將 GPUI 鎖定在某個 Zed commit。所有 GPUI-family 相依套件必須同步更新。詳細流程見 `docs/UPGRADE_GPUI.md`。

## Agent 指示

本專案專為 AI agent 驅動開發設計。工作流程規則、元件慣例與 git push 程序請見 `AGENTS.md`。

## English

See `README.md` for English documentation.
