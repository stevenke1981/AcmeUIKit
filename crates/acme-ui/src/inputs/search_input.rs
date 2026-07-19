use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

/// A search input with search icon and placeholder.
///
/// # Example
///
/// ```ignore
/// SearchInput::new("search")
///     .placeholder("Search files…")
/// ```
#[derive(IntoElement)]
pub struct SearchInput {
    id: ElementId,
    placeholder: SharedString,
}

impl SearchInput {
    /// Creates a new search input.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            placeholder: SharedString::from("Search…"),
        }
    }

    /// Sets the placeholder text.
    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = text.into();
        self
    }
}

impl RenderOnce for SearchInput {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        div()
            .id(self.id)
            .h_flex()
            .items_center()
            .gap_2()
            .h(px(32.))
            .px(px(10.))
            .rounded(theme.radius)
            .bg(c.muted)
            .child(
                div()
                    .text_color(c.muted_foreground)
                    .child(Icon::new(IconName::Search).with_size(px(14.))),
            )
            .child(
                div()
                    .text_size(theme.font_sizes.body)
                    .text_color(c.muted_foreground)
                    .child(self.placeholder.clone()),
            )
    }
}

use gpui::SharedString;
