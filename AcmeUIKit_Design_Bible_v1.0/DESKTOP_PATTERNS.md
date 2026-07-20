# Desktop Patterns

## Settings

Sidebar + Content。每頁 3–6 個 Section；即時生效用 Switch；需儲存使用固定 Footer；Danger Zone 放底部。

## Dashboard

Header、Primary Action、最多四個 KPI、Main Chart、Recent Activity。避免每個區塊都使用 Elevated Card。

## File Manager

NavigationRail、Tree Sidebar、Toolbar、Breadcrumb、DataGrid/Thumbnail、StatusBar。支援多選、Context Menu、Drag/Drop、Rename、Sort、Filter、Empty/Error。

## Editor / IDE

TitleBar、AppMenuBar、Toolbar、Sidebar、Editor、Inspector/Terminal、StatusBar。Compact density、低動畫、快捷鍵優先、CommandPalette。

## SpeakType Cloud

```text
TitleBar
├── NavigationRail
│   ├── Home
│   ├── History
│   ├── Providers
│   └── Settings
├── Main
│   ├── Recording Status
│   ├── Provider Status
│   ├── Hotkey Hint
│   ├── Recent Transcript
│   └── Primary Record Action
└── StatusBar
```

待命、錄音、辨識、錯誤狀態要明確；Provider 用 Badge；熱鍵用 Kbd；API Key 放獨立設定頁或 Drawer。
