# Acme UI Kit — 改善建議報告 (Improvement Report)

> 由 CBM (codebase-memory) MCP 索引分析產出。
> 索引統計：162 檔案、1746 符號、3135 邊（CALLS 909、IMPORTS 319、IMPLEMENTS 162、CONTAINS 1745）。
> 分析基準：`crates/acme-ui/src/`（132 個 .rs 檔）、`apps/acme-gallery/src/main.rs`（~2245 行）。

---

## 1. 專案現狀摘要

| 指標 | 數值 |
|------|------|
| 原始檔 (src) | 132 個 `.rs` |
| 程式碼行數 (src) | ~16,500 行 |
| `lib.rs` `pub mod` 宣告 | 9（目錄入口） |
| `lib.rs` `pub use` re-export | 133（回溯相容） |
| 含測試的檔案數 | 2（`pagination.rs`, `theme.rs`） |
| 測試屬性總數 | 16（14 個實際測試函式） |
| `todo!` / `unimplemented!` / `panic!` | 0 |
| `.unwrap()` 於 src | 0 |
| 最大單檔 | `data_grid.rs` 883 行、`chart_base.rs` 621 行 |
| Gallery `main.rs` | ~2245 行（單檔承載所有示範） |

整體評價：**編譯乾淨、零警告、無不安全 stub**，元件 API 一致性高（統一 Builder 模式、統一 `RenderOnce`）。
主要風險集中在**扁平結構難維護**、**測試覆蓋率極低**、**大型檔案單體化**三點。

---

## 2. 改善建議

### 2.1 模組分類（高優先 / 架構）

**現況**：132 個元件全部平鋪在 `crates/acme-ui/src/` 下，無分類目錄。

**建議**：引入二層目錄分類，降低認知負載並對齊 V1–V10 / P2 階段劃分：

```
crates/acme-ui/src/
├── foundations/    # button, card, badge, progress, theme, primitives
├── inputs/         # text_input, textarea, select, combobox, date_picker, ...
├── data/           # table, tree, data_grid, pagination
├── layout/         # dock, tiles, sidebar, split_view, resizable
├── overlay/        # dialog, popover, menu, drawer, tooltip, overlay_manager
├── charts/         # chart_base, pie_chart, line_chart, ...
├── media/          # canvas, image, hex_viewer, log_viewer, ...
└── infra/          # states, accessibility, focus
```

**效益**：新成員上手更快；`lib.rs` 的 128 行 `pub mod` 壓縮為 9 個目錄入口；CI 增量檢查更精準。

**注意**：此為大規模重構，需保留 `pub use` 舊路徑別名（`pub use inputs::text_input::TextInput;`）以維持回溯相容，符合 AGENTS.md「既有元件 API 不應無版本說明就破壞」規範。

✅ **已完成狀態**（2026-07-20）：132 個扁平檔案已重新分類至 `foundations/`、`inputs/`、`data/`、`layout/`、`overlay/`、`charts/`、`media/`、`infra/`、`desktop/` 九個目錄。`lib.rs` 完全重寫，保留 133 項 `pub use` 回溯相容。外部路徑（如 `acme_ui::Button`）完全不受影響。

---

### 2.2 提升測試覆蓋率（高優先 / 品質）

**現況**：132 個檔案中僅 2 個有測試，11 個測試函式全在 `pagination` 與 `theme`。其餘 130 個元件（含 `data_grid`、`combobox`、`form` 等狀態複雜元件）零單元測試。

**建議分階段補測**：
1. **純函式優先**：`chart_base::Scale` 的 `map()`、各 `ColorPicker`/`hsl()` 轉換、`validation_border_color` 等無 UI 依賴邏輯，應先補 100% 覆蓋。
2. **Builder 鏈測試**：仿 `pagination::pagination_builder_chaining` 模式，為 `Button`/`Card`/表單 `Field` 補鏈式呼叫與預設值斷言。
3. **實體元件快照測試**：`DataGrid`、`Combobox`、`Form` 等 `Entity + Render` 元件，可用 GPUI 的 `AppContext` 測試 harness 驗證狀態轉換（開關、選取、排序）。

**目標**：每個 `crates/acme-ui/src/*.rs` 至少 1 個 `#[test]`；CI 加入 `cargo tarpaulin` 或 `cargo llvm-cov` 門檻（初期 30%，逐步拉高）。

---

### 2.3 拆分巨型檔案（中優先 / 可維護性）

**現況**：`data_grid.rs` 883 行、`chart_base.rs` 621 行、`combobox.rs` 414 行、`overlay_manager.rs` 342 行。

**建議**：
- `chart_base.rs` 已涵蓋 `Scale`/`Axis`/`Legend`/`ChartSeries`/`ChartTooltip`/`Crosshair` 共 6 個型別。可拆為 `chart_base/{scale.rs, axis.rs, legend.rs, series.rs, tooltip.rs}` 子模組，主檔僅留 re-export。
- `data_grid.rs` 的編輯、排序、篩選、CSV 匯出可抽為 `data_grid/{sort.rs, filter.rs, csv.rs}` 自由函式，降低單檔認知負載。
- `combobox.rs` 與 `select.rs`（213 行）邏輯高度重疊，可抽共用 `dropdown_list` 基礎型別。

**效益**：減少合併衝突；利於 2.1 分類後的單元測試放置。

✅ **已完成狀態**（2026-07-20）：`chart_base.rs`（685 行）拆分為 `scale.rs`（含 3 個新增單元測試）`palette.rs`、`series.rs`、`legend.rs`、`tooltip.rs`、`crosshair.rs`、`axis.rs` 共 7 子模組；`data_grid.rs`（987 行）拆分出 `types.rs`（DataGridColumn / DataGridRow）；`combobox.rs`（457 行）拆分出 `options.rs`（ComboboxOption）。全部主檔保留為 `mod.rs` re-export hub。驗證：`cargo test` 14/14 通過（新增 3 個 Scale 測試）。

---

### 2.4 Gallery 拆分（中優先 / 開發體驗）

**現況**：`apps/acme-gallery/src/main.rs` 單檔 2245 行，所有卡片（V1–V10 + P2）內聯在同一 `render()`。

**建議**：為每個階段建立 `apps/acme-gallery/src/cards/vN_*.rs` 模組，各自匯出 `fn vN_card(cx: &mut Context<Gallery>) -> impl IntoElement`。`main.rs` 僅負責佈局組裝。

**效益**：示範與元件實作可並行修改；新人貢獻不需碰觸 2000+ 行檔；每卡片可獨立 `#[cfg]` 條件編譯以加速 debug build。

---

### 2.5 基礎設施模組實用性強化（低優先 / 功能完整度）

**現況**（來自 CBM 符號與原始碼）：`accessibility.rs` 的 `prefers_reduced_motion` / `prefers_high_contrast` 目前回傳常數 `false`；`focus.rs` 的 `arrow_key_nav_handler()` / `escape_close_handler()` 回傳 `FnMut` 但未在 Gallery 實際綁定到 `on_key_down`；`overlay_manager.rs` 的 `AutoPositioner::position()` 接受 9 個裸參數（已用 `#[allow(clippy::too_many_arguments)]` 抑制警告）。

**建議**：
- `prefers_reduced_motion` 改讀 `Window` 的 `prefers_reduced_motion` 設定（GPUI 已提供），移除硬編碼 `false`，讓 ARIA 基礎設施真正可用。
- `arrow_key_nav_handler` / `escape_close_handler` 在 Gallery 的 Focus card 實際串接 `on_key_down`，避免「有 API 無示範」的半成品感。
- `AutoPositioner::position()` 改收 `ReferenceRect` + `OverlaySize` + `ViewportSize` 三個 struct，消除 9 參數告警並提升可讀性。

---

### 2.6 文件與發現性（低優先 / 協作）

**現況**：`README.md` 與 `README.zh-TW.md` 已涵蓋 V1–V10 + P2 階段，但 `docs/` 下缺一份**元件 API 索引**（每元件一行：檔案 / Builder 方法 / 主題色依賴）。

**建議**：由 CBM `query_graph` 自動生成 `docs/COMPONENT_INDEX.md`，列出 132 個 `pub struct` 與其 `pub fn new` 簽章，作為 LLM / 開發者查表。

---

## 3. 優先順序總覽

| 優先級 | 項目 | 預估工作量 | 風險 | 狀態 |
|--------|------|-----------|------|------|
| P0 | 2.2 測試覆蓋率（先純函式） | 中 | 低 | ⏳ 未開始 |
| P1 | 2.1 模組分類（含 re-export 別名） | 高 | 中（需回溯相容） | ✅ 已完成 |
| P1 | 2.3 拆分巨型檔案 | 中 | 低 | ✅ 已完成 |
| P2 | 2.4 Gallery 拆分 | 中 | 低 | ⏳ 未開始 |
| P2 | 2.5 基礎設施實用性 | 低 | 低 | ⏳ 未開始 |
| P3 | 2.6 自動生成元件索引 | 低 | 低 | ⏳ 未開始 |

---

## 4. CBM 證據出處

- 索引：`cbm_index_repository` → `cbm+gpui-kit`（161 檔、1407 符號、3131 邊）
- 架構：`cbm_get_architecture` → 10 個 top_communities、edge_types 分布
- 符號：`cbm_search_graph(label=Class)` → 544 個 Class 節點，確認元件數量與結構
- 原始碼交叉驗證：`data_grid.rs` 883 行、`chart_base.rs` 621 行（PowerShell 行數統計）
- 測試基底：`cargo test --workspace` 輸出僅 `pagination` + `theme` 11 passed

---

*報告產生方式：CBM MCP 索引 + 圖譜檢索 + 本地原始碼統計交叉驗證。所有數據均來自實際索引與檔案測量，非推測。*
