use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, Pixels, RenderOnce,
    Styled as _, Window, div, px,
};

use crate::ActiveTheme;

/// A drawing canvas area.
///
/// Renders a bordered div with a "Canvas" label centred inside.
///
/// # Example
///
/// ```ignore
/// Canvas::new("draw-area").size(px(400.), px(300.))
/// ```
#[derive(IntoElement)]
pub struct Canvas {
    id: ElementId,
    width: Pixels,
    height: Pixels,
}

impl Canvas {
    /// Creates a new canvas with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            width: px(400.),
            height: px(300.),
        }
    }

    /// Sets the canvas dimensions.
    pub fn size(mut self, width: impl Into<Pixels>, height: impl Into<Pixels>) -> Self {
        self.width = width.into();
        self.height = height.into();
        self
    }
}

impl RenderOnce for Canvas {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let font_sizes = cx.theme().font_sizes;

        div()
            .id(self.id)
            .w(self.width)
            .h(self.height)
            .flex()
            .items_center()
            .justify_center()
            .bg(c.background)
            .border_1()
            .border_color(c.border)
            .rounded(px(4.))
            .text_color(c.muted_foreground)
            .text_size(font_sizes.body)
            .child("Canvas")
    }
}
