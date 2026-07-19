use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div,
};

use crate::ActiveTheme;

/// A visual focus indicator ring that wraps content.
///
/// Displays a labeled region with a border that highlights when focused.
///
/// # Example
///
/// ```ignore
/// FocusRing::new("name-field")
///     .label("Name")
///     .focused(true)
/// ```
#[derive(IntoElement)]
pub struct FocusRing {
    id: ElementId,
    label: SharedString,
    focused: bool,
}

impl FocusRing {
    /// Creates a new focus ring with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: SharedString::default(),
            focused: false,
        }
    }

    /// Sets the label text displayed inside the ring.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    /// Sets whether the ring is in a focused state.
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }
}

impl RenderOnce for FocusRing {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let border_color = if self.focused { c.ring } else { c.border };

        div()
            .id(self.id)
            .px_2()
            .py_1()
            .rounded(cx.theme().radius)
            .border_1()
            .border_color(border_color)
            .bg(c.background)
            .child(self.label)
    }
}
