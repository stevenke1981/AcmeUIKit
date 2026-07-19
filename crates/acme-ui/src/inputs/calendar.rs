use gpui::{
    App, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce, SharedString,
    Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// A simplified calendar month grid.
///
/// # Example
///
/// ```ignore
/// Calendar::new("2024-12")
/// ```
#[derive(IntoElement)]
pub struct Calendar {
    label: SharedString,
}

impl Calendar {
    /// Creates a new calendar with a month label.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

impl RenderOnce for Calendar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

        div()
            .v_flex()
            .gap_1()
            .child(
                div()
                    .text_size(theme.font_sizes.body)
                    .text_color(c.foreground)
                    .child(self.label.clone()),
            )
            .child(
                div()
                    .grid()
                    .grid_cols(7)
                    .gap(px(2.))
                    .children(days.iter().map(|d| {
                        div()
                            .text_size(px(10.))
                            .text_color(c.muted_foreground)
                            .h_flex()
                            .items_center()
                            .justify_center()
                            .h(px(24.))
                            .child(*d)
                    })),
            )
            .child(
                div()
                    .grid()
                    .grid_cols(7)
                    .gap(px(2.))
                    .children((1..=31).map(|day| {
                        div()
                            .text_size(theme.font_sizes.caption)
                            .text_color(c.foreground)
                            .h_flex()
                            .items_center()
                            .justify_center()
                            .h(px(24.))
                            .rounded(px(4.))
                            .hover(|style| style.bg(c.muted))
                            .child(format!("{}", day))
                    })),
            )
    }
}
