# Component Standards

## 共通狀態

所有互動元件必須實作 Default、Hover、Pressed、Focus Visible、Selected、Disabled、Loading、Invalid。

## Button

Anatomy：Container、Leading Icon、Label、Trailing Icon、Loading Indicator。

- Label 使用動詞片語
- 每個區域最多一個 Primary
- Loading 保持原寬度
- Disabled 不綁 callback
- Focus ring 不被 clip
- Icon/Label gap 4–6px

```rust
Button::new("save")
    .label("儲存設定")
    .leading_icon(IconName::Save)
    .primary()
    .loading(is_saving)
    .disabled(!is_valid)
```

## IconButton

必須提供 Tooltip、Accessible Label、至少 28×28 點擊面積。

## Card

Variant：Plain、Outlined、Elevated、Interactive、Muted。設定頁避免大量 Card 嵌套。

## TextInput

必須支援 IME、Selection、Clipboard、Readonly、Disabled、Invalid、Clear、Focus。

結構：Label、Description、Input、Validation Message。

## Select / Combobox

選項 ≤7 使用 Select；>7 使用 Combobox。支援方向鍵、Enter、Escape、視窗邊界碰撞與 Selected 狀態。

## Checkbox / Radio / Switch

- Checkbox：多選
- Radio：互斥單選
- Switch：立即生效

需按 Save 才生效的設定不使用 Switch。

## Tabs

支援左右方向鍵、Focus、Active state。超過 6 個主要頁籤時改用 Sidebar。

## Dialog

必須包含 Backdrop、Focus Trap、Escape、Focus Restore、Header/Body/Footer。不得無限巢狀 Dialog。

## Menu

Item 高 28–32px；支援 icon、check、shortcut、submenu、keyboard navigation。危險操作放底部。

## DataGrid

支援 sticky header、hover、selection、keyboard、empty/loading/error、resize、virtualization、compact density。

## Toast

同時最多 3 個；Success 3–5 秒、Warning 5–8 秒、Error 可持續。不可用於不可逆關鍵訊息。
