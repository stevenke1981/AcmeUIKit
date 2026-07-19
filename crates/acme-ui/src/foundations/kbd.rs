use gpui::{
    App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// A keyboard shortcut indicator.
///
/// # Example
///
/// ```ignore
/// Kbd::new("⌘K")
/// ```
#[derive(IntoElement)]
pub struct Kbd {
    keys: SharedString,
}

impl Kbd {
    /// Creates a new keyboard indicator with the given key label.
    pub fn new(keys: impl Into<SharedString>) -> Self {
        Self { keys: keys.into() }
    }
}

impl RenderOnce for Kbd {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        div()
            .h_flex()
            .items_center()
            .px(px(6.))
            .py(px(2.))
            .rounded(px(4.))
            .bg(c.muted)
            .text_size(theme.font_sizes.caption)
            .text_color(c.muted_foreground)
            .child(self.keys.clone())
    }
}
