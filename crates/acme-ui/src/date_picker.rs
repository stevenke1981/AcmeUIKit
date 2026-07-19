use gpui::{
    App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// A date display / picker placeholder.
///
/// Shows a formatted date string. Interactive date selection
/// requires entity integration.
///
/// # Example
///
/// ```ignore
/// DatePicker::new("2024-12-25")
/// ```
#[derive(IntoElement)]
pub struct DatePicker {
    date: SharedString,
}

impl DatePicker {
    /// Creates a new date picker with the given date string.
    pub fn new(date: impl Into<SharedString>) -> Self {
        Self { date: date.into() }
    }
}

impl RenderOnce for DatePicker {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        div()
            .h_flex()
            .items_center()
            .gap_2()
            .h(px(32.))
            .px(px(10.))
            .rounded(theme.radius)
            .border_1()
            .border_color(c.border)
            .child(
                div()
                    .text_size(theme.font_sizes.body)
                    .text_color(c.foreground)
                    .child(self.date.clone()),
            )
    }
}
