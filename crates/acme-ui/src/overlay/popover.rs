use gpui::{
    AnyElement, App, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    Styled as _, Window, div,
};

use crate::{ActiveTheme, StyledExt};

/// A popover that wraps its children in a styled panel when open.
///
/// When `open` is false, the children are rendered inline without special styling.
/// When `open` is true, the children are wrapped in a styled popup container
/// with border, background, and rounded corners.
///
/// The caller is responsible for placing a trigger element (e.g. a button) alongside
/// the popover and controlling the `open` state.
///
/// # Example
///
/// ```ignore
/// Popover::new("my-popover")
///     .open(true)
///     .child(div().child("Popover content here"))
/// ```
#[derive(IntoElement)]
pub struct Popover {
    id: ElementId,
    open: bool,
    children: Vec<AnyElement>,
}

impl Popover {
    /// Creates a new popover with the given element ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            children: Vec::new(),
        }
    }

    /// Sets whether the popover is expanded.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

impl ParentElement for Popover {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Popover {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        if !self.open {
            return div().id(self.id);
        }

        div()
            .id(self.id)
            .v_flex()
            .p_2()
            .rounded(theme.radius)
            .border_1()
            .border_color(c.border)
            .bg(c.surface)
            .children(self.children)
    }
}
