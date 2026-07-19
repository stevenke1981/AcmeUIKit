# test.md

## 自動檢查

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Gallery Smoke Test

1. 啟動 `cargo run -p acme-gallery`。
2. 視窗可正常開啟，沒有 panic。
3. 點選 Theme 按鈕，Light/Dark 背景、文字、Card 與 border 同步改變。
4. 點選 Counter 按鈕，數字遞增。
5. 點選 Reset，數字歸零。
6. 點選 Switch，thumb 位置與文字狀態改變。
7. 點選進度 +/-，值限制在 0–100。
8. 點選 Tabs 按鈕，選取狀態改變。
9. Disabled button 不觸發 callback。
10. 1280×800 與 1024×700 下內容不重疊。

## Theme 邊界

- 主文字對 background 有足夠可讀性。
- muted text 仍可辨識。
- Primary/Danger button 的文字清楚。
- Focus/selected ring 不被 border 吃掉。

## 平台

- Windows 10/11 + MSVC：必測。
- Ubuntu Wayland/X11：CI 或人工抽測。
- macOS：至少能完成 `cargo check`。

## 失敗分類

- Dependency：GPUI/Zed revision 無法下載或 API 改變。
- Compile：trait import、builder API 或 callback signature 不相容。
- Render：視窗可開但 layout/顏色錯誤。
- Interaction：callback 未更新 entity 或未 `cx.notify()`。
- Platform：只在特定 OS 編譯失敗。
