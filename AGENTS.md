# AGENTS.md — Acme UI Kit

## 任務目標

建立與維護一套乾淨、可測試、可擴充的 Rust GPUI 元件庫。不得直接複製參考專案原始碼；可參考公開 API 形狀、功能分類與 UX 行為後重新設計。

## 強制流程

1. 先讀 `spec.md`、`plan.md`、`todos.md`、`test.md`。
2. 變更前列出影響範圍：元件 API、Theme token、Gallery、測試、文件。
3. 元件新增或修改必須同步更新 Gallery。
4. 完成後執行：
   - `cargo fmt --all -- --check`
   - `cargo check --workspace --all-targets`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test --workspace`
5. 不得使用 `todo!()`、`unimplemented!()` 或空殼函式冒充完成。
6. 不得自行更新 GPUI revision；需獨立升級任務與相容性紀錄。
7. 禁止刪除檔案、`git push --force`、重寫歷史，除非使用者明確同意。

## Git Push 流程

Agent 執行 `git push` 前必須遵守以下順序：

1. **摘要變更**：條列即將推送的所有檔案與變更性質（新檔案／修改／刪除）。
2. **請求許可**：使用 `question` 工具詢問使用者是否允許推送，選項包含「推送」與「稍後再推」。
3. **推送後回報**：顯示推送結果（舊 hash → 新 hash, 目標分支）。

禁止事項：
- 未經許可不得推送。
- 不得將不相關的變更與推送混在一起。
- 不得使用 `git push --force` 或 `git push origin +master`。
- 不得在推送前改寫已推送的歷史（如 `git rebase`、`git commit --amend`）。
- .env 或機密檔案一律排除，不進入版本控制也不推送。

## 元件規則

- 無狀態視圖使用 `RenderOnce`。
- 需要焦點、文字編輯、非同步任務或持久狀態時使用 Entity + `Render`。
- 顏色只從 Theme 取得，禁止直接在元件內散落色碼。
- 所有可點擊元件需有穩定 `ElementId`。
- 互動元件需支援 disabled、focus/selected 可視狀態與鍵盤規劃。
- Builder 命名要一致：`new`、`label`、`value`、`checked`、`disabled`、`with_size`、`with_tone`。
- 公開 API 必須有 rustdoc。

## 驗收邊界

- Build Gate：Workspace 可檢查。
- Visual Gate：Gallery 可開啟且 Light/Dark 無明顯對比錯誤。
- Interaction Gate：所有示範按鈕可操作且狀態立即更新。
- Regression Gate：既有元件 API 不應無版本說明就破壞。
- License Gate：新資產與程式碼來源需可追溯。
