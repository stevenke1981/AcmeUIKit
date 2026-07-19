use gpui::{
    App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// A color picker swatch display.
///
/// # Example
///
/// ```ignore
/// ColorPicker::new("accent", gpui::hsla(0.6, 0.7, 0.5, 1.0))
/// ```
#[derive(IntoElement)]
pub struct ColorPicker {
    label: SharedString,
    color: gpui::Hsla,
}

impl ColorPicker {
    /// Creates a new color picker display.
    pub fn new(label: impl Into<SharedString>, color: gpui::Hsla) -> Self {
        Self {
            label: label.into(),
            color,
        }
    }
}

impl RenderOnce for ColorPicker {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        div()
            .h_flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .w(px(24.))
                    .h(px(24.))
                    .rounded(theme.radius)
                    .bg(self.color)
                    .border_1()
                    .border_color(c.border),
            )
            .child(
                div()
                    .v_flex()
                    .child(
                        div()
                            .text_size(theme.font_sizes.caption)
                            .text_color(c.foreground)
                            .child(self.label.clone()),
                    )
                    .child(
                        div()
                            .text_size(px(10.))
                            .text_color(c.muted_foreground)
                            .child("HSL"),
                    ),
            )
    }
}
