use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// A draggable window region indicator bar.
///
/// Displays a drag handle visual with a label, styled as a title-bar-like strip.
///
/// # Example
///
/// ```ignore
/// DragRegion::new("window-drag")
///     .label("Drag to move")
/// ```
#[derive(IntoElement)]
pub struct DragRegion {
    id: ElementId,
    label: SharedString,
}

impl DragRegion {
    /// Creates a new drag region with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: SharedString::default(),
        }
    }

    /// Sets the label text shown beside the drag handle.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
}

impl RenderOnce for DragRegion {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;

        div()
            .id(self.id)
            .h_flex()
            .items_center()
            .justify_center()
            .h(px(32.))
            .bg(c.surface)
            .border_1()
            .border_color(c.border)
            .rounded(cx.theme().radius)
            .gap_2()
            .text_color(c.muted_foreground)
            .cursor_pointer()
            .child(div().child("≡"))
            .child(self.label)
    }
}
