use crate::{ActiveTheme, StyledExt};
use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

/// Find and replace bar UI.
///
/// # Example
///
/// ```ignore
/// FindReplace::new("find-bar")
///     .find_text("hello")
///     .replace_text("world")
///     .matches(1, 5);
/// ```
#[derive(IntoElement)]
pub struct FindReplace {
    id: ElementId,
    find_text: SharedString,
    replace_text: SharedString,
    match_count: usize,
    current_match: usize,
}

impl FindReplace {
    /// Create a new [`FindReplace`] bar.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            find_text: SharedString::default(),
            replace_text: SharedString::default(),
            match_count: 0,
            current_match: 0,
        }
    }

    /// Set the find text value.
    pub fn find_text(mut self, text: impl Into<SharedString>) -> Self {
        self.find_text = text.into();
        self
    }

    /// Set the replace text value.
    pub fn replace_text(mut self, text: impl Into<SharedString>) -> Self {
        self.replace_text = text.into();
        self
    }

    /// Set the match counter display.
    pub fn matches(mut self, current: usize, total: usize) -> Self {
        self.current_match = current;
        self.match_count = total;
        self
    }
}

impl RenderOnce for FindReplace {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let theme = cx.theme();

        div()
            .id(self.id)
            .h_flex()
            .bg(c.surface)
            .border_b_1()
            .border_color(c.border)
            .px(px(8.))
            .py(px(4.))
            .gap_2()
            // Find input area
            .child(
                div()
                    .h(px(26.))
                    .px(px(8.))
                    .flex()
                    .items_center()
                    .bg(c.muted)
                    .rounded(theme.radius_sm)
                    .text_color(c.muted_foreground)
                    .text_size(px(12.))
                    .child(if self.find_text.is_empty() {
                        SharedString::from("Find")
                    } else {
                        self.find_text.clone()
                    }),
            )
            // Replace input area
            .child(
                div()
                    .h(px(26.))
                    .px(px(8.))
                    .flex()
                    .items_center()
                    .bg(c.muted)
                    .rounded(theme.radius_sm)
                    .text_color(c.muted_foreground)
                    .text_size(px(12.))
                    .child(if self.replace_text.is_empty() {
                        SharedString::from("Replace")
                    } else {
                        self.replace_text.clone()
                    }),
            )
            // Match counter
            .child(
                div()
                    .h(px(26.))
                    .flex()
                    .items_center()
                    .text_color(c.muted_foreground)
                    .text_size(px(11.))
                    .child(format!("{}/{}", self.current_match, self.match_count)),
            )
            // Previous button
            .child(
                div()
                    .h(px(22.))
                    .w(px(22.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(theme.radius_sm)
                    .text_color(c.muted_foreground)
                    .text_size(px(12.))
                    .hover(|style| style.bg(c.muted))
                    .child("◀"),
            )
            // Next button
            .child(
                div()
                    .h(px(22.))
                    .w(px(22.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(theme.radius_sm)
                    .text_color(c.muted_foreground)
                    .text_size(px(12.))
                    .hover(|style| style.bg(c.muted))
                    .child("▶"),
            )
    }
}
