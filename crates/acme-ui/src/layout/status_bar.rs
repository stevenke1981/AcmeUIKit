use gpui::{
    App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// A bottom status bar with left and right sections.
///
/// # Example
///
/// ```ignore
/// StatusBar::new()
///     .left("Ready")
///     .right("Line 42, Col 8")
/// ```
#[derive(IntoElement)]
pub struct StatusBar {
    left_text: Option<SharedString>,
    right_text: Option<SharedString>,
}

impl StatusBar {
    /// Creates a new status bar.
    pub fn new() -> Self {
        Self {
            left_text: None,
            right_text: None,
        }
    }

    /// Sets the left-aligned text.
    pub fn left(mut self, text: impl Into<SharedString>) -> Self {
        self.left_text = Some(text.into());
        self
    }

    /// Sets the right-aligned text.
    pub fn right(mut self, text: impl Into<SharedString>) -> Self {
        self.right_text = Some(text.into());
        self
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for StatusBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        div()
            .h_flex()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(24.))
            .px(px(8.))
            .bg(c.muted)
            .child(
                div().h_flex().items_center().gap_2().child(
                    div()
                        .text_size(theme.font_sizes.caption)
                        .text_color(c.muted_foreground)
                        .child(self.left_text.unwrap_or_default()),
                ),
            )
            .child(
                div().h_flex().items_center().gap_2().child(
                    div()
                        .text_size(theme.font_sizes.caption)
                        .text_color(c.muted_foreground)
                        .child(self.right_text.unwrap_or_default()),
                ),
            )
    }
}
