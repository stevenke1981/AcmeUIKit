# Design System

## Semantic colors

- `background`：應用底色。
- `surface`：Card、Panel、Popover 底色。
- `foreground`：主要文字。
- `muted` / `muted_foreground`：次要區域與文字。
- `border`：一般邊界。
- `primary` / `primary_hover` / `primary_foreground`：主要操作。
- `danger`：破壞性操作。
- `success`、`warning`：狀態提示。
- `ring`：selected/focus 強調。

## Spacing

以 GPUI 內建 spacing scale 為主：`gap_1`、`gap_2`、`gap_3`、`gap_4`、`p_4`、`px_3`、`py_2`。

## Radius

- Small：4px
- Medium：7px
- Large：12px

## Component sizing

- XS：24px 高
- SM：30px 高
- MD：36px 高
- LG：42px 高

## API style

```rust
Button::new("save")
    .label("Save")
    .primary()
    .medium()
    .on_click(...)
```
