# todos.md

## V1 Foundation (已建立, 2026-07-19)
- [x] Workspace 與 pinned GPUI dependencies
- [x] Theme 與 Light/Dark tokens
- [x] Size / Tone primitives
- [x] Button, Card, Badge, Progress
- [x] Switch, FieldShell, Tabs, Separator, Skeleton
- [x] Interactive Gallery source
- [x] Windows / Unix scripts, CI workflow
- [x] Architecture / API / roadmap documents

## V1 驗證修復 (已完成)
- [x] `ParentElement` trait 匯入 (badge, field, progress)
- [x] Gallery 缺少的 trait 匯入 (`InteractiveElement`, `StatefulInteractiveElement`, `AppContext`)
- [x] `acme-ui` crate root re-export
- [x] `overflow_y_scroll()` .id() 修正
- [x] Gallery 視窗正常開啟

## V2 Components (已完成)
- [x] TextInput (Entity+Render, 可編輯, IME 支援)
- [x] Textarea (Entity+Render, 多行編輯, line-aware 游標)
- [x] Checkbox
- [x] Radio / RadioGroup
- [x] Select (RenderOnce, dropdown)
- [x] Combobox (Entity+Render, 可過濾 dropdown)
- [x] Menu / Context Menu
- [x] Dialog / Popover / Tooltip
- [x] Notification / Toast
- [x] Icon provider (`icon_provider.rs`)

## V3 Components (已完成)
- [x] **Pagination** (RenderOnce, 391 行, smart ellipsis, 10 單元測試)
- [x] **Sidebar** (RenderOnce, 85 行, 主題化側欄)
- [x] **Resizable** (Entity+Render, 198 行, 拖拉分隔條)
- [x] **LoadingState** (`loading_state.rs`, 115 行, 4 列舉變體 + dispatch)
- [x] **VirtualList** (`virtual_list.rs`, 74 行, GPUI `uniform_list` 包裝)

## Theme & Design Principles (已完成)
- [x] `FontSizes` token struct (heading/body/caption)
- [x] `Spacing` token struct (widget/group/section/panel)
- [x] 全元件字型大小遷移 (16 個 .rs, 27 處 `px(NN)` → `font_sizes.*`)
- [x] `UI_DESIGN_PRINCIPLES.md` 產出 (egui 設計規則, 3 層字型, 8/16/24/12 間距)

## Gallery (已完成)
- [x] Gallery 已擴充至 ~965 行, 所有 V1+V2+V3 元件都有互動示範

## 待辦
- [ ] Gallery 視覺截圖 (需手動操作)
- [ ] 合併 V2+V3 分支到 master
- [ ] Release build 測試
