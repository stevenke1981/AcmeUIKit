# Architecture

## 分層

### `acme-ui`
只負責設計系統與可重用元件。不得依賴 Gallery 或業務資料。

### `acme-gallery`
負責元件展示、互動驗證與視覺回歸基線。任何新元件都要先在 Gallery 有 story，再提供給應用專案使用。

## 狀態模型

- 無狀態元件：`RenderOnce`，資料由 builder 傳入。
- 應用狀態：Gallery Entity 持有，callback 使用 `cx.listener` 更新並 `cx.notify()`。
- 全域外觀：Theme 實作 GPUI `Global`，元件透過 `ActiveTheme` 取得。
- 複雜元件：未來 Input/Table/Dock 應拆成 State Entity + View Element。

## 相依方向

```text
GPUI / gpui_platform
       ↑
   acme-ui
       ↑
 acme-gallery
```

禁止 `acme-ui -> acme-gallery` 反向依賴。
