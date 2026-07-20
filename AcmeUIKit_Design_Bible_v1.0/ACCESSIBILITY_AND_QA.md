# Accessibility and Visual QA

## Keyboard

Tab、Shift+Tab、Enter、Space、Escape、Arrow、Home、End；長清單支援 PageUp/PageDown。

## Focus

Focus Visible 清楚；Modal Focus Trap；Overlay 關閉後還原焦點；Focus ring 不可被裁切。

## Contrast

一般文字 4.5:1；大文字與互動邊界 3:1。狀態不可只靠顏色。

## Screen Reader

元件提供 Role、Label、Description、State、Value、Expanded、Selected、Checked、Invalid。

## Reduced Motion

停用大幅位移、bounce、parallax；縮短 transition；Spinner 可保留但降低速度。

## Screenshot Matrix

尺寸：1280×800、1024×700、800×600。模式：Light、Dark、High Contrast、Compact、Comfortable、Spacious。

## Required Screens

Design Tokens、Component States、Settings、Dashboard、DataGrid、File Manager、Dialog、Menu、SpeakType Cloud、Empty/Error/Loading。

## Stress Tests

200% DPI、150% 字體、超長繁體中文、1000 rows、100 menu items、50 tabs、20 層 Tree、無資料、慢速、API 錯誤、縮放視窗。

## Release Gate

```text
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
Gallery smoke
Screenshot diff
Keyboard pass
```

每個視覺 PR 必須附 Before/After、Light/Dark、小視窗、Focus、Disabled/Error 截圖。
