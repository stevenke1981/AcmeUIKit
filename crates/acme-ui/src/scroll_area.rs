use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    StatefulInteractiveElement as _, Styled as _, Window, div,
};

use crate::StyledExt;

/// A scrollable content area.
///
/// # Example
///
/// ```ignore
/// ScrollArea::new("content")
///     .child(div().h(px(1000.)).child("Tall content"))
/// ```
#[derive(IntoElement)]
pub struct ScrollArea {
    id: ElementId,
    child: Option<gpui::AnyElement>,
}

impl ScrollArea {
    /// Creates a new scroll area with the given [`ElementId`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            child: None,
        }
    }

    /// Sets the child content.
    pub fn child(mut self, child: impl IntoElement + 'static) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl RenderOnce for ScrollArea {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .v_flex()
            .w_full()
            .overflow_y_scroll()
            .child(if let Some(child) = self.child {
                child
            } else {
                div().into_any_element()
            })
    }
}
