use gpui::{App, IntoElement, RenderOnce, Styled as _, Window, div, px};

use crate::ActiveTheme;

/// Horizontal separator line.
#[derive(IntoElement, Default)]
pub struct Separator;

impl Separator {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for Separator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div().w_full().h(px(1.)).bg(cx.theme().colors.border)
    }
}
