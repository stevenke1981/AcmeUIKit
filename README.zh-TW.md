# Acme UI Kit — Rust GUI 全新開發包

一套以 **GPUI**（Zed 原生 UI 框架）重新設計的 Rust 桌面 GUI 元件庫。程式碼為全新撰寫，不直接複製原專案實作。

## 目前進度 — V1–V10 + P2 圖表 + P2 基礎設施

| 階段 | 元件 | 狀態 |
|------|------|------|
| **V1 基礎** | Button、Card、Badge、Progress、Switch、FieldShell、Tabs、Separator、Skeleton、Theme（Light/Dark）、primitives | ✅ |
| **V2 控制項** | TextInput、Textarea、Checkbox、Radio/RadioGroup、Select、Combobox、Menu、Dialog、Popover、Tooltip、Notification、IconProvider | ✅ |
| **V3 資料與佈局** | Pagination（+10 測試）、Sidebar、Resizable、LoadingState、VirtualList、Tree、Form+Validation、Table（可排序） | ✅ |
| **V4 豐富內容** | SettingsPage、Tiles、Markdown、BarChart、Dock | ✅ |
| **V5 工具元件** | Alert、Tag、Slider、Avatar、Breadcrumb、Stepper、Toolbar、List、Kbd | ✅ |
| **V6 更多元件** | Label、ScrollArea、Stack、Grid、IconButton、ToggleButton、SegmentedControl、Spinner、Collapsible、Accordion、Drawer、CommandPalette、EmptyState、ErrorState、StatusBar、NumberInput、SearchInput、DatePicker、Calendar、ColorPicker、PropertyGrid | ✅ |
| **V7 P1 輸入與選擇** | PasswordInput、MaskedInput、PinInput、TimePicker、DateRangePicker、FilePicker、MultiSelect、RangeSlider、Rating、FormMessage、Autocomplete | ✅ |
| **V8 桌面殼層** | TitleBar、WindowControls、AppMenuBar、NavigationRail、NavigationView、SplitView、InspectorPanel、ContextToolbar、ShortcutManager、SystemTray、FocusRing、FocusScope、DragRegion、DropZone、ResizeHandle、WindowOverlay、AboutDialog | ✅ |
| **V9 DataGrid** | DataGrid（Entity 架構，支援排序/篩選/編輯/鍵盤導航/CSV 匯出） | ✅ |
| **V10 內容與媒體** | RichText、HtmlView、LineNumbers、DiffViewer、MarkdownPreview、DocumentOutline、FindReplace、LogViewer、HexViewer、ImageView、AvatarGroup、Carousel、Lightbox、Canvas、ZoomView、PanView、ThumbnailStrip、Cropper、AnnotationLayer | ✅ |
| **P2 圖表** | PieChart、DonutChart、Gauge、Sparkline、LineChart、AreaChart、ScatterChart、Histogram、Heatmap、CandlestickChart、StreamingChart + 共用 chart_base（Scale、Axis、Legend、ChartColors） | ✅ |
| **P2 基礎設施** | Component States（載入/停用遮罩、驗證邊框）、Accessibility（40+ ARIA 角色、17 個屬性）、Focus（FocusTrap、RovingTabIndex、鍵盤處理器）、Overlay Manager（ModalBackdrop、AutoPositioner、ClickOutsideListener、FocusRestore） | ✅ |

**總計**：132 個原始檔，約 16500+ 行程式碼，零警告編譯。

## 快速開始

```powershell
# 必要條件：
#   - Visual Studio 2022 Build Tools（桌面 C++ 工作負載）
#   - Windows 10/11 SDK
#   - Rust nightly（rust-toolchain.toml 會自動設定）

# 執行 Gallery 展示程式：
cargo run -p acme-gallery
```

Gallery 功能：約 100 個元件展示、切換 Light/Dark 主題、所有元件的互動示範。

## 專案結構

```
acme-ui-kit/
├── apps/acme-gallery/       # 互動式元件展示
├── crates/acme-ui/src/      # 132 個原始檔
│   ├── lib.rs               # 模組宣告 + re-export
│   ├── theme.rs             # Theme、FontSizes、Spacing、ThemeColors
│   ├── styled.rs            # StyledExt 輔助函式（h_flex、v_flex）
│   ├── primitives.rs        # Size / Tone 列舉
│   ├── chart_base.rs        # 共用圖表基礎設施（Scale、Axis、Legend）
│   ├── states.rs            # 載入/停用遮罩、驗證、StateStyling trait
│   ├── accessibility.rs     # ARIA 角色/屬性、減少動畫、高對比度
│   ├── focus.rs             # FocusTrap、RovingTabIndex、鍵盤處理器
│   ├── overlay_manager.rs   # ModalBackdrop、AutoPositioner、ClickOutsideListener
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
