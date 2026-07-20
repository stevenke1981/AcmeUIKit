# Agent Implementation Guide

## 角色

- Orchestrator：控制範圍、API、驗收
- Implementer：實作 token/元件、Gallery、tests
- Reviewer：檢查一致性、鍵盤、無障礙、hardcode
- Validator：執行 fmt/check/clippy/test、Gallery 與 screenshot

## 強制規則

1. 不破壞公開 API。
2. 新功能優先新增 Builder method。
3. 禁止元件內 hardcode RGB。
4. 每個元件同步更新 Gallery、tests、文件。
5. 所有互動元件有 Focus Visible。
6. 所有視覺變更支援 Light/Dark。
7. 狀態不可只靠顏色。
8. 一個 PR 不同時重寫超過 10 個核心元件。
9. 每個 PR 可獨立回滾。
10. 未通過驗證不得標示完成。

## PR 切分

1. Theme tokens
2. Typography + density
3. Button + IconButton
4. Card + Section
5. TextInput + Select
6. Checkbox + Radio + Switch
7. Tabs + Navigation
8. Dialog + Popover + Menu
9. DataGrid visual pass
10. Real Screens
11. Visual regression
12. Documentation

## 可直接使用的 Agent Prompt

```text
請依照 docs/design/ACME_DESIGN_BIBLE_v1.0.md 執行本次 AcmeUIKit UI 改造。

限制：
- 不破壞既有公開 API。
- 不擴大本次元件範圍。
- 顏色、尺寸、字體、圓角、動畫全部使用 design tokens。
- 同步更新 Gallery、tests 與文件。
- 支援 Light/Dark、Focus、Disabled、Loading、Invalid。
- 完成後執行 fmt/check/clippy/test。
- final.md 列出變更、驗證、風險與未完成項目。
```
