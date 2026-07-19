use crate::ActiveTheme;
use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

/// Simple HTML subset viewer that renders text with basic formatting.
///
/// Displays HTML content as styled plain text in a bordered container.
///
/// # Example
///
/// ```ignore
/// HtmlView::new("preview")
///     .html("<h1>Title</h1><p>Hello</p>");
/// ```
#[derive(IntoElement)]
pub struct HtmlView {
    id: ElementId,
    html: SharedString,
}

impl HtmlView {
    /// Create a new [`HtmlView`] with the given stable [`ElementId`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            html: SharedString::default(),
        }
    }

    /// Set the HTML source text.
    pub fn html(mut self, text: impl Into<SharedString>) -> Self {
        self.html = text.into();
        self
    }
}

impl RenderOnce for HtmlView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        div()
            .id(self.id)
            .bg(c.surface)
            .border_1()
            .border_color(c.border)
            .rounded(px(4.))
            .px(px(8.))
            .py(px(8.))
            .text_color(c.foreground)
            .child(self.html)
    }
}
