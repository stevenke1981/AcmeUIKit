use gpui::{
    App, IntoElement, ParentElement as _, RenderOnce, Styled as _, Window, div, px, relative,
};

use crate::ActiveTheme;

/// Determinate horizontal progress bar.
#[derive(IntoElement)]
pub struct Progress {
    value: f32,
}

impl Progress {
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0., 100.),
        }
    }
}

impl RenderOnce for Progress {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        div().w_full().h(px(8.)).rounded_full().bg(c.muted).child(
            div()
                .h_full()
                .w(relative(self.value / 100.))
                .rounded_full()
                .bg(c.primary),
        )
    }
}
