use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// A floating horizontal toolbar with action labels.
///
/// # Example
///
/// ```ignore
/// ContextToolbar::new("format-bar")
///     .items(&["Bold", "Italic", "Link"])
/// ```
#[derive(IntoElement)]
pub struct ContextToolbar {
    id: ElementId,
    items: Vec<SharedString>,
}

impl ContextToolbar {
    /// Creates a new context toolbar.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
        }
    }

    /// Sets the toolbar button labels.
    pub fn items(mut self, items: &[&str]) -> Self {
        self.items = items.iter().map(|s| SharedString::from(*s)).collect();
        self
    }
}

impl RenderOnce for ContextToolbar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        let mut toolbar = div()
            .id(self.id)
            .h_flex()
            .px_2()
            .py_1()
            .gap_1()
            .bg(c.surface)
            .rounded(theme.radius)
            .border_1()
            .border_color(c.border);

        for label in self.items {
            toolbar = toolbar.child(
                div()
                    .px_2()
                    .py(px(3.))
                    .rounded(theme.radius_sm)
                    .text_size(theme.font_sizes.body)
                    .text_color(c.foreground)
                    .hover(|style| style.bg(c.muted))
                    .child(label.clone()),
            );
        }

        toolbar
    }
}
