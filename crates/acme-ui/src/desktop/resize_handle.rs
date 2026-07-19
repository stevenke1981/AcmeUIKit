use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    Styled as _, Window, div, px,
};

use crate::ActiveTheme;

/// A window resize grip indicator rendered in the bottom-right corner style.
///
/// Displays a small diagonal grip visual using a "↘" character.
///
/// # Example
///
/// ```ignore
/// ResizeHandle::new("resize-grip")
/// ```
#[derive(IntoElement)]
pub struct ResizeHandle {
    id: ElementId,
}

impl ResizeHandle {
    /// Creates a new resize handle with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }
}

impl RenderOnce for ResizeHandle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;

        div()
            .id(self.id)
            .size(px(12.))
            .flex()
            .items_center()
            .justify_center()
            .text_color(c.muted_foreground)
            .cursor_pointer()
            .child("↘")
    }
}
