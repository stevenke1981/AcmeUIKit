use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div,
};

use crate::{ActiveTheme, StyledExt};

/// A container that groups focusable elements with a labelled border.
///
/// # Example
///
/// ```ignore
/// FocusScope::new("address-group")
///     .label("Address")
/// ```
#[derive(IntoElement)]
pub struct FocusScope {
    id: ElementId,
    label: SharedString,
}

impl FocusScope {
    /// Creates a new focus scope with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: SharedString::default(),
        }
    }

    /// Sets the header label for this scope.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
}

impl RenderOnce for FocusScope {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        div()
            .id(self.id)
            .v_flex()
            .gap_2()
            .p_2()
            .rounded(theme.radius)
            .border_1()
            .border_color(c.border)
            .child(
                div()
                    .text_size(theme.font_sizes.caption)
                    .text_color(c.muted_foreground)
                    .child(self.label),
            )
    }
}
