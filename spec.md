# spec.md — Acme UI Kit V1

## 1. 產品定位

提供一套全新 Rust + GPUI 桌面 GUI 開發基底，讓 Codex/OpenCode 可直接從元件庫、Gallery、主題系統與測試規格開始擴充，逐步達到大型桌面 UI Kit 的能力。

## 2. V1 必要功能

### 2.1 Workspace
- `crates/acme-ui`：元件與 Theme。
- `apps/acme-gallery`：視覺與互動展示。
- 單一 GPUI revision，避免 API 漂移。

### 2.2 主題
- Light / Dark。
- 語意 token：background、surface、foreground、muted、border、primary、danger、success、warning、ring。
- 字型大小與 radius 集中管理。

### 2.3 元件
- Button：Default、Primary、Secondary、Danger、Ghost；XS/SM/MD/LG；disabled；selected。
- Card：title、description、children。
- Badge：Neutral、Primary、Success、Warning、Danger。
- Progress：0–100。
- Switch：checked、disabled、on_click。
- FieldShell：label、value/placeholder、helper、error 狀態；V1 為展示層，不宣稱文字編輯能力。
- Tabs：靜態選取呈現。
- Separator、Skeleton。

### 2.4 Gallery
- 可切換 Light/Dark。
- Button counter。
- Switch 狀態。
- 進度加減。
- Tabs 選取示範。
- 顯示所有 V1 元件。

## 3. 非功能要求

- Rust 2024 edition。
- 禁止 unsafe。
- 公開 API 有文件。
- 元件不直接持有應用業務狀態。
- 主題切換後整個視窗刷新。
- Windows 為第一優先，Linux/macOS 保留相容性。

## 4. V1 不包含

- 完整文字輸入與 IME。
- 複雜 Overlay、Dialog、Popover 定位。
- 虛擬化 Table/List。
- Dock/Tiles。
- Markdown、HTML、Chart、Code Editor、LSP。
- WASM Gallery。

以上功能列入 V2–V4 Roadmap，避免第一版過度承諾。
