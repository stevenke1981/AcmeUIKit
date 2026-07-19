# Acme UI Kit — Rust GUI 全新開發包

這是一套以 **GPUI** 直接開發、重新設計的 Rust 桌面 GUI 起始專案。架構參考 `longbridge/gpui-component` 的公開設計方向，但程式碼為全新撰寫，不直接複製原專案實作。

## V1 已整合

- Cargo Workspace：`acme-ui` 元件庫與 `acme-gallery` 展示程式
- Light / Dark 主題與語意化 Design Tokens
- 統一尺寸：XS / SM / MD / LG
- 元件：Button、Card、Badge、Progress、Switch、FieldShell、Tabs、Separator、Skeleton
- 可互動 Gallery：切換主題、計數器、Switch、Tabs、Progress
- Windows PowerShell 與 Linux/macOS Shell 腳本
- GitHub Actions、Clippy、rustfmt、測試與驗收文件
- `spec.md`、`plan.md`、`todos.md`、`test.md`、`final.md`、`AGENTS.md`

## 專案結構

```text
acme-ui-kit/
├─ apps/acme-gallery/              # 元件展示與互動驗證
├─ crates/acme-ui/          # 可重用 GPUI 元件庫
├─ docs/                      # 架構、設計系統、API 與路線圖
├─ scripts/                   # Windows / Unix 開發腳本
├─ .github/workflows/ci.yml   # CI
├─ spec.md
├─ plan.md
├─ todos.md
├─ test.md
└─ final.md
```

## Windows 10/11 開發需求

1. 安裝 Visual Studio 2022 Build Tools。
2. 勾選「使用 C++ 的桌面開發」。
3. 安裝 Windows 10/11 SDK。
4. 安裝 Rustup 與 stable toolchain。
5. 建議使用 PowerShell 7。

執行：

```powershell
Set-ExecutionPolicy -Scope Process Bypass
./scripts/bootstrap-windows.ps1
./scripts/run-gallery.ps1
```

或直接：

```powershell
cargo run -p acme-gallery
```

首次編譯需下載 Zed/GPUI Git 相依套件，時間取決於網路與電腦效能。

## 核心設計原則

- **Clean-room**：重做功能與 API 思路，不直接貼入參考專案原始碼。
- **Token first**：元件不寫死主題色，全部由語意化 Theme Token 取得。
- **RenderOnce first**：無狀態元件優先使用 `RenderOnce`；複雜互動才升級成 Entity。
- **Gallery driven**：每新增一個元件，必須同步新增展示、邊界狀態與驗收項目。
- **逐層擴充**：V1 先建立可靠基底，再加入輸入系統、Overlay、虛擬化、Dock、Markdown、Editor。

## 更新 GPUI

根目錄 `Cargo.toml` 目前固定 GPUI 與 `gpui_platform` 到同一個 Zed commit。更新時不得只改其中一個：

1. 建立 `chore/gpui-upgrade-YYYYMMDD` 分支。
2. 同步更新所有 GPUI-family revision。
3. 執行 `cargo check --workspace --all-targets`。
4. 執行 Gallery smoke test。
5. 確認 Windows、Linux、macOS 至少一個 CI 工作成功。
6. 將 API 破壞變更寫入 `docs/UPGRADE_GPUI.md`。

## 下一階段

V2 建議優先完成：真正可編輯的 TextInput、Checkbox/Radio、Popover/Dialog、Notification、Menu、Table/List virtualization、Dock layout 與圖示資產系統。完整內容見 `docs/ROADMAP.md`。
