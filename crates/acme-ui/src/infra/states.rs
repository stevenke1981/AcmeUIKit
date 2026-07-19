use gpui::prelude::FluentBuilder;
use gpui::{App, Hsla, ParentElement as _, SharedString, Styled, px};

use crate::{ActiveTheme, StyledExt};

// ── Loading overlay ──

/// A reusable loading overlay that renders a spinner and optional message.
pub fn render_loading_overlay(cx: &App, message: Option<SharedString>) -> gpui::Div {
    let theme = cx.theme();
    let c = theme.colors;

    gpui::div()
        .absolute()
        .top(px(0.))
        .left(px(0.))
        .w_full()
        .h_full()
        .flex()
        .items_center()
        .justify_center()
        .bg(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.0,
            a: 0.3,
        })
        .child(
            gpui::div()
                .v_flex()
                .items_center()
                .gap_2()
                .child(crate::foundations::spinner::Spinner::new().size(crate::Size::Medium))
                .when_some(message, |this, msg| {
                    this.child(
                        gpui::div()
                            .text_size(theme.font_sizes.caption)
                            .text_color(c.foreground)
                            .child(msg),
                    )
                }),
        )
}

// ── Disabled overlay ──

/// Applies a semi-transparent disabled overlay to block interaction.
pub fn render_disabled_overlay() -> gpui::Div {
    gpui::div()
        .absolute()
        .top(px(0.))
        .left(px(0.))
        .w_full()
        .h_full()
        .bg(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.0,
            a: 0.05,
        })
}

// ── Validation styling ──

/// Returns a border color for validation state.
pub fn validation_border_color(
    cx: &App,
    invalid: bool,
    _required: bool,
    _focused: bool,
) -> Option<Hsla> {
    let c = cx.theme().colors;
    if invalid {
        return Some(c.danger);
    }
    None
}

// ── Opacity for disabled state ──

/// Returns the opacity value for a disabled element.
pub fn disabled_opacity() -> f32 {
    0.5
}

// ── Aria attribute helpers ──

/// Builds a "aria-label" SharedString from a label + optional description.
pub fn aria_label(label: &str, description: Option<&str>) -> SharedString {
    match description {
        Some(desc) => format!("{}. {}", label, desc).into(),
        None => label.into(),
    }
}

/// Renders a visually hidden label (screen-reader only text).
pub fn sr_only_label(label: impl Into<SharedString>) -> gpui::Div {
    let text: SharedString = label.into();
    gpui::div()
        .absolute()
        .size(px(1.))
        .overflow_hidden()
        .child(gpui::div().child(text))
}

// ── StyledExt extension for common state styling ──

/// Extension trait adding convenience methods for component states.
pub trait StateStyling: Styled + Sized {
    /// Applies styling for disabled state (lower opacity).
    fn state_disabled(mut self, disabled: bool) -> Self {
        if disabled {
            self = self.opacity(disabled_opacity());
        }
        self
    }

    /// Applies styling for invalid state (red border).
    fn state_invalid(mut self, invalid: bool, cx: &App) -> Self {
        if invalid {
            let border = cx.theme().colors.danger;
            self = self.border_color(border);
        }
        self
    }

    /// Applies styling for focused state (focus ring).
    fn state_focused(mut self, focused: bool, cx: &App) -> Self {
        if focused {
            let ring = cx.theme().colors.ring;
            self = self.border_color(ring);
        }
        self
    }
}

impl<T: Styled> StateStyling for T {}
