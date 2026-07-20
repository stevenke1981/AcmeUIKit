# AcmeUIKit Design Bible v1.0

**框架**：GPUI  
**主要平台**：Windows 10/11、Linux Desktop  
**次要平台**：macOS  
**適用產品**：AI 工具、設定中心、Dashboard、IDE、檔案管理器、媒體工具、SpeakType Cloud

---

## 1. 設計使命

AcmeUIKit 不只是元件集合，而是一套讓 Rust/GPUI 團隊能穩定建立專業桌面軟體的設計系統。

每個設計決策必須同時服務：

1. 清楚
2. 高效率
3. 一致
4. 可維護
5. 無障礙
6. 可主題化
7. 可由 AI Agent 正確實作

## 2. 品牌個性

AcmeUIKit 應呈現：精準、安靜、現代、可信任、工具導向、高密度但不壓迫。

禁止：

- 過度漸層
- 巨型圓角
- 大量發光效果
- 過重陰影
- 所有內容都包成 Card
- 只為展示技術而增加動畫

## 3. 核心原則

### 3.1 留白優先於框線
優先用間距、背景層級、字體層級分組；Border 只用於需要明確邊界的控制項和區塊。

### 3.2 每個區域只有一個主行動
每個 Panel、Page 或 Dialog 最多一個 Primary Button。

### 3.3 狀態不可只靠顏色
Selected、Invalid、Warning、Disabled 必須搭配圖示、文字、形狀或邊框差異。

### 3.4 桌面密度優先
預設 Comfortable；資料表、IDE、工具列使用 Compact；歡迎頁使用 Spacious。

### 3.5 互動必須可預測
所有互動元件必須支援 Default、Hover、Pressed、Focus Visible、Selected、Disabled、Loading、Invalid。

### 3.6 向後相容
Visual System v2 優先透過新增 Builder methods 擴充，不任意破壞既有公開 API。

## 4. 視覺方向

### Linear
低飽和背景、精準選取狀態、清楚資料層級。

### Zed
桌面工具密度、快捷鍵優先、Sidebar/Toolbar/Editor 清楚分工。

### Arc
柔和浮層、短動畫、精緻 Hover 與 Focus。

### Apple Settings
設定群組、標題、說明、控制項與危險區域分層清楚。

## 5. 標準桌面架構

```text
Application
├── TitleBar                 32–40 px
├── AppMenuBar / Toolbar     28–40 px
├── NavigationRail           48–64 px
├── Sidebar                  200–280 px
├── Main Content             padding 16–24 px
└── StatusBar                22–28 px
```

## 6. 資訊層級

- Page Title：18–20 px / Semibold
- Section Title：14–16 px / Semibold
- Body：12–14 px / Regular
- Caption：11–12 px / Regular

一般畫面最多 3–4 個明確層級，不用大量字級製造層次。

## 7. 容器策略

Card 只用於：獨立摘要、Dashboard 指標、可點擊物件、浮動內容、明確狀態群組。

設定頁面優先使用：

```text
Page
├── Section title
├── Description
├── Rows
└── Separator
```

Card Variant：Plain、Outlined、Elevated、Interactive、Muted。

## 8. 表單策略

標準結構：

```text
Label
Optional description
Control
Validation message
```

規則：

- Placeholder 不可取代 Label
- 錯誤訊息貼近欄位
- 即時套用用 Switch
- 延後套用使用 Save/Cancel Footer
- 危險設定獨立放在頁面底部

## 9. 導航策略

- 主要模組：Sidebar / NavigationRail
- 同頁切換：Tabs
- 階層位置：Breadcrumb
- 快速操作：CommandPalette
- 暫時上下文：Popover / Menu

使用者必須隨時知道目前位置、上一層、可前往區域及未儲存狀態。

## 10. Feedback 策略

- 即時：元件狀態、Inline validation
- 暫時：Toast
- 持續：Alert、StatusBar
- 阻塞：Dialog
- 背景：Progress、Spinner、Loading overlay

不要使用 Dialog 顯示所有訊息。

## 11. Motion

動畫只用於狀態轉換、浮層、展開收合、頁面切換與進度回饋。

禁止 bounce、長 fade、大幅縮放及會降低輸入效率的動畫。所有動畫支援 Reduced Motion。

## 12. Dark Mode

Dark Mode 不使用純黑。Elevated surface 應比背景亮 2–6%，邊框降低對比，Primary 不過度發光，長時間閱讀區域保持舒適。

## 13. 國際化

所有元件必須支援繁體中文、英文、CJK 字體、文字擴張 30–50%、長字串與未來 RTL。

## 14. 元件完成定義

元件必須同時具備：

- 穩定 API
- Light/Dark
- 三種 Density
- 完整互動狀態
- Keyboard
- Accessibility
- 中文長字串
- Gallery demo
- Tests
- Documentation
- Visual Regression

缺少任一項，只能標示 Preview 或 Experimental。
