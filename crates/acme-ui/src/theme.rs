use gpui::{App, Global, Hsla, Pixels, Window, hsla, px};

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
    pub radius_sm: Pixels,
    pub radius: Pixels,
    pub radius_lg: Pixels,
}

impl Global for Theme {}

impl Theme {
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            colors: ThemeColors {
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
            radius_sm: px(4.),
            radius: px(7.),
            radius_lg: px(12.),
        }
    }

    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            colors: ThemeColors {
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
            radius_sm: px(4.),
            radius: px(7.),
            radius_lg: px(12.),
        }
    }

    pub fn global(cx: &App) -> &Self {
        cx.global::<Self>()
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
}
