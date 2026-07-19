use gpui::{
    App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div,
};

use crate::{ActiveTheme, Size};

/// A styled text label with size variants.
///
/// # Example
///
/// ```ignore
/// Label::new("Username")
///     .size(Size::Small)
/// ```
#[derive(IntoElement)]
pub struct Label {
    text: SharedString,
    size: Size,
    color: Option<gpui::Hsla>,
}

impl Label {
    /// Creates a new label.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            size: Size::Medium,
            color: None,
        }
    }

    /// Sets the label size.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Sets a custom text color.
    pub fn color(mut self, color: impl Into<Option<gpui::Hsla>>) -> Self {
        self.color = color.into();
        self
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        div()
            .text_size(self.size.text_size())
            .text_color(self.color.unwrap_or(c.foreground))
            .child(self.text)
    }
}
