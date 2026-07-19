# Component API

Import `acme_ui::StyledExt` whenever using `.h_flex()` or `.v_flex()` helpers.

## Button

```rust
Button::new("id")
    .label("Create")
    .primary()
    .small()
    .disabled(false)
    .on_click(cx.listener(|this, _, _, cx| {
        this.create_item();
        cx.notify();
    }))
```

## Card

```rust
Card::new()
    .title("Account")
    .description("Profile and security settings")
    .child(...)
```

## Badge

```rust
Badge::new("Ready").success()
```

## Progress

```rust
Progress::new(72.0)
```

## Switch

```rust
Switch::new("notifications", enabled)
    .on_click(cx.listener(|this, _, _, cx| {
        this.enabled = !this.enabled;
        cx.notify();
    }))
```

## FieldShell

V1 只提供文字欄位外觀容器，不處理游標、選取、IME 與鍵盤事件。正式輸入元件應在 V2 以 Entity + FocusHandle 實作。
