# final.md — V1 交付說明

## 已交付

- 一套全新 GPUI Rust Workspace。
- 可重用 `acme-ui` 元件 crate。
- 可互動 `acme-gallery` 展示程式。
- Light/Dark 主題與十個 V1 元件。
- 完整規格、架構、API、Roadmap、測試與 Agent 規範。
- Windows/Unix 開發腳本與 GitHub Actions。

## 驗證狀態 (2026-07-19)

在 Windows 11 + Rust 1.94.1 nightly 環境完成全流程驗證：

### 通過閘門

| 閘門 | 結果 |
|---|---|
| `cargo check --workspace --all-targets` | ✅ |
| `cargo clippy --workspace --all-targets -- -D warnings` | ✅ |
| `cargo fmt --all -- --check` | ✅ |
| `cargo test --workspace` | ✅ (1 test) |
| Gallery 視窗正常啟動 | ✅ |

### 已知差異

1. **工具鏈需求**: GPUI pinned revision (`1a246efd`) 使用 `std::hint::cold_path()` — 需要 Rust nightly。已在 `rust-toolchain.toml` 更新為 `nightly`。
2. **GPUI API 差異**: 原始專案基於 Zed 專案內部 API snapshot 設計，有數處 trait import 與方法呼叫需修正（詳見 `todos.md`「本機首次驗證後更新」）。
3. **Gallery 截圖**: 尚未建立正式截圖，需手動操作。

### 首次開發環境設定

```powershell
# 1. 確認 Rust nightly 已安裝
rustup toolchain install nightly --profile minimal --component rustfmt --component clippy

# 2. 執行完整檢查
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

# 3. 啟動 Gallery
cargo run -p acme-gallery
```
