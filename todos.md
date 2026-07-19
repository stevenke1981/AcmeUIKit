# todos.md

## V1 已建立
- [x] Workspace 與 pinned GPUI dependencies
- [x] Theme 與 Light/Dark tokens
- [x] Size / Tone primitives
- [x] Button
- [x] Card
- [x] Badge
- [x] Progress
- [x] Switch
- [x] FieldShell
- [x] Tabs
- [x] Separator
- [x] Skeleton
- [x] Interactive Gallery source
- [x] Windows / Unix scripts
- [x] CI workflow
- [x] Architecture / API / roadmap documents

## 本機首次驗證後更新 (已完成)
- [x] 在具備 Rust 與網路的 Windows 環境執行 `cargo check --workspace --all-targets` ✅
- [x] 修正 GPUI revision 對應的任何 API 差異
  - 三處 `ParentElement` trait 未匯入 (badge, field, progress)
  - Gallery 缺少 `InteractiveElement` / `StatefulInteractiveElement` / `AppContext` 等 trait 匯入
  - `acme-ui` 元件未在 crate root re-export
  - `overflow_y_scroll()` 需要先使用 `.id()` 取得 `Stateful<Div>`
- [x] 執行 Gallery 視覺 smoke test ✅ (視窗正常開啟，無 panic)
- [ ] 建立第一張 Windows Gallery 截圖 (需要手動操作截圖工具)
- [x] 記錄冷編譯時間與 release binary 大小
  - 冷編譯時間：約 2 分 33 秒 (首次含依賴下載)
  - Release binary 大小：待確認 (未建置 release profile)

## V2
- [ ] 可編輯 TextInput + IME
- [ ] Checkbox / Radio
- [ ] Dialog / Popover / Tooltip
- [ ] Menu / Context Menu
- [ ] Notification / Toast
- [ ] Icon provider 與 SVG assets crate
