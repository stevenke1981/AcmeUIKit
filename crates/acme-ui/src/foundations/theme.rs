use gpui::{App, FontWeight, Global, Hsla, Pixels, Window, hsla, px};

/// A typography token with size, line height, and weight.
#[derive(Debug, Clone, Copy)]
pub struct TextStyle {
    pub size: Pixels,
    pub line_height: Pixels,
    pub weight: FontWeight,
    pub letter_spacing: Pixels,
}

/// Named typography scale from the Acme Design Bible.
#[derive(Debug, Clone, Copy)]
pub struct Typography {
    pub display: TextStyle,
    pub title_large: TextStyle,
    pub title: TextStyle,
    pub body: TextStyle,
    pub body_compact: TextStyle,
    pub label: TextStyle,
    pub caption: TextStyle,
}

impl Default for Typography {
    fn default() -> Self {
        let regular = |size, line_height| TextStyle {
            size: px(size),
            line_height: px(line_height),
            weight: FontWeight(400.),
            letter_spacing: px(0.),
        };
        let semibold = |size, line_height| TextStyle {
            size: px(size),
            line_height: px(line_height),
            weight: FontWeight(600.),
            letter_spacing: px(0.),
        };
        Self {
            display: semibold(24., 32.),
            title_large: semibold(20., 28.),
            title: semibold(16., 24.),
            body: regular(13., 20.),
            body_compact: regular(12., 18.),
            label: TextStyle {
                size: px(12.),
                line_height: px(16.),
                weight: FontWeight(500.),
                letter_spacing: px(0.),
            },
            caption: regular(11., 16.),
        }
    }
}

/// Three supported desktop density presets.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Density {
    Compact,
    #[default]
    Comfortable,
    Spacious,
}

/// Radius tokens for controls and surfaces.
#[derive(Debug, Clone, Copy)]
pub struct RadiusScale {
    pub none: Pixels,
    pub xs: Pixels,
    pub sm: Pixels,
    pub md: Pixels,
    pub lg: Pixels,
    pub xl: Pixels,
    pub pill: Pixels,
}

impl Default for RadiusScale {
    fn default() -> Self {
        Self {
            none: px(0.),
            xs: px(3.),
            sm: px(5.),
            md: px(7.),
            lg: px(10.),
            xl: px(14.),
            pill: px(999.),
        }
    }
}

/// Standard control dimensions.
#[derive(Debug, Clone, Copy)]
pub struct ControlSizes {
    pub xs: Pixels,
    pub small: Pixels,
    pub medium: Pixels,
    pub large: Pixels,
}

impl Default for ControlSizes {
    fn default() -> Self {
        Self {
            xs: px(24.),
            small: px(28.),
            medium: px(32.),
            large: px(38.),
        }
    }
}

/// Motion duration tokens in milliseconds.
#[derive(Debug, Clone, Copy)]
pub struct Motion {
    pub instant: u64,
    pub fast: u64,
    pub normal: u64,
    pub slow: u64,
}

impl Default for Motion {
    fn default() -> Self {
        Self {
            instant: 0,
            fast: 80,
            normal: 140,
            slow: 220,
        }
    }
}

/// Elevation shadow tokens represented as intensity hints for GPUI surfaces.
#[derive(Debug, Clone, Copy, Default)]
pub struct Shadows {
    pub small: Pixels,
    pub medium: Pixels,
    pub large: Pixels,
}

/// Typography scale tokens following UI_DESIGN_PRINCIPLES.md §2.1:
/// - Heading: 18–20px, Bold
/// - Body: 13px, Regular
/// - Caption: 11–12px
#[derive(Debug, Clone, Copy)]
pub struct FontSizes {
    pub heading: Pixels,
    pub body: Pixels,
    pub caption: Pixels,
}

impl Default for FontSizes {
    fn default() -> Self {
        Self {
            heading: px(18.),
            body: px(13.),
            caption: px(11.),
        }
    }
}

/// Spacing tokens following UI_DESIGN_PRINCIPLES.md §1.2.
#[derive(Debug, Clone, Copy)]
pub struct Spacing {
    /// Between sibling widgets.
    pub widget: Pixels,
    /// Between related groups.
    pub group: Pixels,
    /// Between major sections.
    pub section: Pixels,
    /// Inner padding of panels / cards.
    pub panel: Pixels,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            widget: px(8.),
            group: px(16.),
            section: px(24.),
            panel: px(12.),
        }
    }
}

/// Creates an opaque GPUI HSLA color using familiar HSL units.
pub fn hsl(h: f32, s: f32, l: f32) -> Hsla {
    hsla(h / 360., s / 100., l / 100., 1.)
}

/// Supported application theme modes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

/// Semantic color tokens consumed by components.
#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    pub app_background: Hsla,
    pub panel_background: Hsla,
    pub elevated_surface: Hsla,
    pub overlay_surface: Hsla,
    pub foreground_secondary: Hsla,
    pub foreground_tertiary: Hsla,
    pub border_subtle: Hsla,
    pub border_default: Hsla,
    pub border_strong: Hsla,
    pub primary_pressed: Hsla,
    pub primary_soft: Hsla,
    pub success_soft: Hsla,
    pub warning_soft: Hsla,
    pub danger_soft: Hsla,
    pub selection: Hsla,
    pub focus_ring: Hsla,
    pub background: Hsla,
    pub surface: Hsla,
    pub foreground: Hsla,
    pub muted: Hsla,
    pub muted_foreground: Hsla,
    pub border: Hsla,
    pub primary: Hsla,
    pub primary_hover: Hsla,
    pub primary_foreground: Hsla,
    pub secondary: Hsla,
    pub secondary_hover: Hsla,
    pub danger: Hsla,
    pub danger_hover: Hsla,
    pub success: Hsla,
    pub warning: Hsla,
    pub ring: Hsla,
}

/// Global Acme design system configuration.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub mode: ThemeMode,
    pub colors: ThemeColors,
    pub font_sizes: FontSizes,
    pub spacing: Spacing,
    pub radius_sm: Pixels,
    pub radius: Pixels,
    pub radius_lg: Pixels,
    pub typography: Typography,
    pub radius_scale: RadiusScale,
    pub controls: ControlSizes,
    pub density: Density,
    pub motion: Motion,
    pub shadows: Shadows,
}

impl Global for Theme {}

impl Theme {
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            colors: ThemeColors {
                app_background: hsl(220., 20., 98.),
                panel_background: hsl(220., 18., 97.),
                elevated_surface: hsl(0., 0., 100.),
                overlay_surface: hsl(0., 0., 100.),
                foreground_secondary: hsl(218., 14., 38.),
                foreground_tertiary: hsl(218., 10., 52.),
                border_subtle: hsl(215., 20., 93.),
                border_default: hsl(215., 18., 87.),
                border_strong: hsl(215., 18., 75.),
                primary_pressed: hsl(221., 78., 42.),
                primary_soft: hsl(221., 82., 95.),
                success_soft: hsl(142., 60., 93.),
                warning_soft: hsl(38., 80., 92.),
                danger_soft: hsl(0., 70., 94.),
                selection: hsl(221., 82., 95.),
                focus_ring: hsl(221., 75., 52.),
                background: hsl(210., 20., 98.),
                surface: hsl(0., 0., 100.),
                foreground: hsl(222., 47., 11.),
                muted: hsl(210., 30., 96.),
                muted_foreground: hsl(215., 16., 47.),
                border: hsl(214., 32., 90.),
                primary: hsl(221., 83., 53.),
                primary_hover: hsl(221., 83., 46.),
                primary_foreground: hsl(0., 0., 100.),
                secondary: hsl(210., 35., 94.),
                secondary_hover: hsl(210., 28., 88.),
                danger: hsl(0., 72., 51.),
                danger_hover: hsl(0., 72., 43.),
                success: hsl(142., 71., 38.),
                warning: hsl(38., 92., 45.),
                ring: hsl(221., 83., 63.),
            },
            font_sizes: FontSizes::default(),
            spacing: Spacing::default(),
            radius_sm: px(4.),
            radius: px(7.),
            radius_lg: px(12.),
            typography: Typography::default(),
            radius_scale: RadiusScale::default(),
            controls: ControlSizes::default(),
            density: Density::Comfortable,
            motion: Motion::default(),
            shadows: Shadows::default(),
        }
    }

    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            colors: ThemeColors {
                app_background: hsl(222., 24., 8.),
                panel_background: hsl(222., 22., 10.),
                elevated_surface: hsl(222., 18., 15.),
                overlay_surface: hsl(222., 18., 17.),
                foreground_secondary: hsl(215., 14., 72.),
                foreground_tertiary: hsl(215., 11., 58.),
                border_subtle: hsl(220., 15., 17.),
                border_default: hsl(220., 14., 23.),
                border_strong: hsl(220., 14., 32.),
                primary_pressed: hsl(217., 78., 56.),
                primary_soft: hsl(217., 38., 18.),
                success_soft: hsl(142., 35., 18.),
                warning_soft: hsl(38., 35., 20.),
                danger_soft: hsl(0., 35., 20.),
                selection: hsl(217., 38., 18.),
                focus_ring: hsl(217., 82., 62.),
                background: hsl(222., 47., 7.),
                surface: hsl(222., 40., 10.),
                foreground: hsl(210., 40., 96.),
                muted: hsl(217., 28., 15.),
                muted_foreground: hsl(215., 20., 65.),
                border: hsl(217., 24., 22.),
                primary: hsl(217., 91., 60.),
                primary_hover: hsl(217., 91., 67.),
                primary_foreground: hsl(222., 47., 8.),
                secondary: hsl(217., 24., 18.),
                secondary_hover: hsl(217., 24., 25.),
                danger: hsl(0., 63., 48.),
                danger_hover: hsl(0., 72., 58.),
                success: hsl(142., 70., 45.),
                warning: hsl(38., 92., 55.),
                ring: hsl(217., 91., 70.),
            },
            font_sizes: FontSizes::default(),
            spacing: Spacing::default(),
            radius_sm: px(4.),
            radius: px(7.),
            radius_lg: px(12.),
            typography: Typography::default(),
            radius_scale: RadiusScale::default(),
            controls: ControlSizes::default(),
            density: Density::Comfortable,
            motion: Motion::default(),
            shadows: Shadows::default(),
        }
    }

    pub fn global(cx: &App) -> &Self {
        cx.global::<Self>()
    }

    /// Sets the active density and refreshes the current window.
    pub fn set_density(density: Density, window: &mut Window, cx: &mut App) {
        if cx.has_global::<Self>() {
            cx.global_mut::<Self>().density = density;
        }
        window.refresh();
    }

    pub fn set_mode(mode: ThemeMode, window: &mut Window, cx: &mut App) {
        let next = match mode {
            ThemeMode::Light => Self::light(),
            ThemeMode::Dark => Self::dark(),
        };

        if cx.has_global::<Self>() {
            *cx.global_mut::<Self>() = next;
        } else {
            cx.set_global(next);
        }
        window.refresh();
    }
}

/// Convenience access to the active global theme.
pub trait ActiveTheme {
    fn theme(&self) -> &Theme;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Theme {
        Theme::global(self)
    }
}

pub(crate) fn init(cx: &mut App) {
    if !cx.has_global::<Theme>() {
        cx.set_global(Theme::light());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modes_have_distinct_backgrounds() {
        assert_ne!(
            Theme::light().colors.background,
            Theme::dark().colors.background
        );
    }

    #[test]
    fn design_bible_tokens_have_expected_control_scale() {
        let theme = Theme::light();
        assert_eq!(theme.controls.xs, px(24.));
        assert_eq!(theme.controls.small, px(28.));
        assert_eq!(theme.controls.medium, px(32.));
        assert_eq!(theme.controls.large, px(38.));
        assert_eq!(theme.typography.body.size, px(13.));
        assert_eq!(theme.radius_scale.md, px(7.));
    }
}
