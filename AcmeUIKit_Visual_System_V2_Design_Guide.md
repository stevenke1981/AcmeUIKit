# AcmeUIKit Visual System V2 改造建議

## 目標

將 AcmeUIKit 從功能完整的元件庫提升為成熟的 GPUI Desktop Design System。

## 設計方向

-   Linear
-   Zed
-   Arc Browser
-   Apple Settings

## 第一階段：Theme System

-   擴充 ThemeColors
-   Typography
-   Spacing Scale
-   Radius
-   Shadows
-   Motion
-   Density

### ThemeColors

-   app_background
-   panel_background
-   elevated_surface
-   overlay_surface
-   foreground_secondary
-   foreground_tertiary
-   border_subtle
-   border_default
-   border_strong
-   primary_pressed
-   primary_soft
-   success_soft
-   warning_soft
-   danger_soft
-   selection
-   focus_ring

### Typography

-   Display
-   Title Large
-   Title
-   Body
-   Body Compact
-   Label
-   Caption

每個 Style： - Size - Weight - Line Height - Letter Spacing

## 第二階段：核心元件

優先翻修： 1. Button 2. IconButton 3. Card 4. TextInput 5. Select 6.
Checkbox 7. Switch 8. Tabs 9. Dialog 10. Menu

所有元件支援： - Default - Hover - Pressed - Focus - Selected -
Disabled - Loading - Invalid

## 第三階段：Gallery

新增： - Design Tokens - Component States - Density - Real Screens -
Stress Tests

Real Screens： - Settings - Dashboard - File Manager - SpeakType Cloud

## 第四階段：Visual QA

建立 Visual Regression： - 1280×800 - 1024×700 - 800×600

測試： - Light - Dark - High DPI - 長中文字 - 大量資料 - RTL 預留

## 第五階段：文件

建立 ACME_DESIGN_SYSTEM.md

包含： - Brand - Colors - Typography - Layout - Motion - Components -
Accessibility - QA Checklist

## Agent 執行順序

1.  Theme V2
2.  Typography
3.  Density
4.  Motion
5.  Shadows
6.  核心元件翻修
7.  Gallery
8.  Demo Screens
9.  Visual Regression
10. Release

## 最終目標

讓 AcmeUIKit 成為可商業專案使用的 GPUI Desktop Design
System，而不只是元件庫。
