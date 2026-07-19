use gpui::{
    App, IntoElement, ParentElement as _, Pixels, RenderOnce, SharedString, Styled as _, Window,
    div, px,
};

use crate::ActiveTheme;

/// Named icon rendered as a text character (no SVG dependency in V2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconName {
    /// Checkmark.
    Check,
    /// Close / X.
    Close,
    /// Hamburger menu.
    Menu,
    /// Chevron pointing down.
    ChevronDown,
    /// Chevron pointing right.
    ChevronRight,
    /// Information.
    Info,
    /// Warning triangle.
    Warning,
    /// Error cross.
    Error,
    /// Success checkmark.
    Success,
}

impl IconName {
    fn character(self) -> &'static str {
        match self {
            Self::Check => "✓",
            Self::Close => "✕",
            Self::Menu => "☰",
            Self::ChevronDown => "▼",
            Self::ChevronRight => "▶",
            Self::Info => "ℹ",
            Self::Warning => "⚠",
            Self::Error => "✗",
            Self::Success => "✔",
        }
    }
}

/// Text-character-based icon with configurable size.
///
/// Renders a single Unicode character as a styled element.
/// Icon colour defaults to the current foreground from the active theme.
#[derive(IntoElement)]
pub struct Icon {
    name: IconName,
    size: Pixels,
}

impl Icon {
    pub fn new(name: IconName) -> Self {
        Self {
            name,
            size: px(14.),
        }
    }

    pub fn with_size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into();
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .text_color(cx.theme().colors.foreground)
            .text_size(self.size)
            .child(SharedString::from(self.name.character()))
    }
}
