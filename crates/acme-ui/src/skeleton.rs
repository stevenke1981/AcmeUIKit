use gpui::{App, IntoElement, RenderOnce, Styled as _, Window, div, px};

use crate::ActiveTheme;

/// Static skeleton placeholder.
#[derive(IntoElement)]
pub struct Skeleton {
    width: gpui::Pixels,
    height: gpui::Pixels,
}

impl Skeleton {
    pub fn new(width: gpui::Pixels, height: gpui::Pixels) -> Self {
        Self { width, height }
    }

    pub fn line() -> Self {
        Self::new(px(220.), px(12.))
    }
}

impl RenderOnce for Skeleton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .w(self.width)
            .h(self.height)
            .rounded(theme.radius_sm)
            .bg(theme.colors.muted)
    }
}
