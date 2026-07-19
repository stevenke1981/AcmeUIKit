use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled as _, Window,
    div, px,
};

use crate::{ActiveTheme, StyledExt};

/// A simple tooltip that renders the trigger element with a styled caption below it.
///
/// The tooltip content is always visible as a small label beneath the trigger element.
/// This is a V2 minimal approach — no hover-based visibility, just an inline caption.
///
/// # Example
///
/// ```ignore
/// Tooltip::new("Click to save the document")
///     .child(Button::new("save-btn").label("Save"))
/// ```
#[derive(IntoElement)]
pub struct Tooltip {
    content: SharedString,
    children: Vec<AnyElement>,
}

impl Tooltip {
    /// Creates a new tooltip with the given content text.
    pub fn new(content: impl Into<SharedString>) -> Self {
        Self {
            content: content.into(),
            children: Vec::new(),
        }
    }
}

impl ParentElement for Tooltip {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Tooltip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;

        div().v_flex().gap_1().children(self.children).child(
            div()
                .text_size(px(11.))
                .text_color(c.muted_foreground)
                .child(self.content),
        )
    }
}
