use gpui::{App, IntoElement, ParentElement as _, RenderOnce, Styled as _, Window, div};

use crate::{ActiveTheme, Size};

/// A loading spinner indicator.
///
/// # Example
///
/// ```ignore
/// Spinner::new()
///     .size(Size::Medium)
/// ```
#[derive(IntoElement)]
pub struct Spinner {
    size: Size,
}

impl Spinner {
    /// Creates a new spinner.
    pub fn new() -> Self {
        Self { size: Size::Medium }
    }

    /// Sets the spinner size.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Spinner {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let dim = self.size.height();

        div()
            .w(dim)
            .h(dim)
            .rounded_full()
            .border_3()
            .border_color(c.muted)
            .border_color(c.primary)
            .child(div())
    }
}
