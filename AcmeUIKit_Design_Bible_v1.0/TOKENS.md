# Design Tokens

## Theme 結構

```rust
pub struct Theme {
    pub mode: ThemeMode,
    pub colors: ThemeColors,
    pub typography: Typography,
    pub spacing: SpacingScale,
    pub radius: RadiusScale,
    pub shadows: Shadows,
    pub controls: ControlSizes,
    pub density: Density,
    pub motion: Motion,
}
```

## Color

### Surface
`app_background`、`panel_background`、`surface`、`elevated_surface`、`overlay_surface`。

### Text
`foreground`、`foreground_secondary`、`foreground_tertiary`、`foreground_disabled`、`foreground_inverse`。

### Border
`border_subtle`、`border_default`、`border_strong`、`focus_ring`。

### Semantic
`primary`、`primary_hover`、`primary_pressed`、`primary_soft`、`success`、`success_soft`、`warning`、`warning_soft`、`danger`、`danger_soft`、`selection`。

## 建議 Light Palette

```text
App Background      hsl(220,20%,98%)
Panel Background    hsl(220,18%,97%)
Surface             hsl(0,0%,100%)
Elevated Surface    hsl(0,0%,100%)
Text Primary        hsl(222,35%,12%)
Text Secondary      hsl(218,14%,38%)
Text Tertiary       hsl(218,10%,52%)
Border Subtle       hsl(215,20%,93%)
Border Default      hsl(215,18%,87%)
Primary             hsl(221,75%,52%)
Primary Hover       hsl(221,75%,47%)
Primary Pressed     hsl(221,78%,42%)
Primary Soft        hsl(221,82%,95%)
```

## 建議 Dark Palette

```text
App Background      hsl(222,24%,8%)
Panel Background    hsl(222,22%,10%)
Surface             hsl(222,20%,12%)
Elevated Surface    hsl(222,18%,15%)
Overlay Surface     hsl(222,18%,17%)
Text Primary        hsl(210,24%,95%)
Text Secondary      hsl(215,14%,72%)
Text Tertiary       hsl(215,11%,58%)
Border Subtle       hsl(220,15%,17%)
Border Default      hsl(220,14%,23%)
Primary             hsl(217,82%,62%)
Primary Hover       hsl(217,85%,68%)
Primary Pressed     hsl(217,78%,56%)
Primary Soft        hsl(217,38%,18%)
```

## Typography

```rust
pub struct TextStyle {
    pub size: Pixels,
    pub line_height: Pixels,
    pub weight: FontWeight,
    pub letter_spacing: Pixels,
}
```

| Token | Size | Line Height | Weight |
|---|---:|---:|---:|
| Display | 24 | 32 | 600 |
| Title Large | 20 | 28 | 600 |
| Title | 16 | 24 | 600 |
| Body | 13 | 20 | 400 |
| Body Compact | 12 | 18 | 400 |
| Label | 12 | 16 | 500 |
| Caption | 11 | 16 | 400 |

字體順序：Segoe UI Variable、Microsoft JhengHei UI、Noto Sans CJK TC、sans-serif。

## Spacing

`0, 2, 4, 6, 8, 12, 16, 20, 24, 32, 40, 48, 64`

語意：4 圖示文字、8 widget、12 control padding、16 group、24 section、32 page group。

## Radius

| none | xs | sm | md | lg | xl | pill |
|---:|---:|---:|---:|---:|---:|---:|
| 0 | 3 | 5 | 7 | 10 | 14 | 999 |

桌面元件預設使用 sm/md；Dialog 使用 lg。

## Control Sizes

| Size | Height | Padding X | Font | Icon |
|---|---:|---:|---:|---:|
| XS | 24 | 8 | 11 | 14 |
| S | 28 | 10 | 12 | 16 |
| M | 32 | 12 | 13 | 16 |
| L | 38 | 16 | 14 | 20 |

## Density

- Compact：Table、Toolbar、IDE，row 24–28
- Comfortable：一般畫面，row 32–36
- Spacious：Welcome/Onboarding，row 38–44

## Elevation

- none：一般 Panel
- small：Dropdown、Interactive Card
- medium：Popover、CommandPalette
- large：Dialog、Modal

## Motion

| Token | Duration |
|---|---:|
| instant | 0ms |
| fast | 80ms |
| normal | 140ms |
| slow | 220ms |

Easing：`cubic-bezier(0.16,1,0.3,1)` 與 `cubic-bezier(0.2,0,0,1)`。
