//! Crosshair overlay for chart interaction feedback.

use gpui::{
    App, ElementId, Hsla, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    Styled as _, Window, div, px,
};

use crate::ActiveTheme;

/// A crosshair overlay for chart interaction feedback.
#[derive(IntoElement)]
pub struct Crosshair {
    id: ElementId,
    x: Option<f32>,
    y: Option<f32>,
    color: Hsla,
    size: gpui::Pixels,
}

impl Crosshair {
    /// Creates a new crosshair with the given `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            x: None,
            y: None,
            color: Hsla {
                h: 0.0,
                s: 0.0,
                l: 0.5,
                a: 0.5,
            },
            size: px(8.),
        }
    }

    /// Sets the crosshair position in pixels.
    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }

    /// Sets the crosshair color.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = color;
        self
    }
}

impl RenderOnce for Crosshair {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let size_f32: f32 = self.size.into();

        if let (Some(x), Some(y)) = (self.x, self.y) {
            div()
                .id(self.id)
                .absolute()
                .left(px(x - 0.5))
                .top(px(0.))
                .w(px(1.))
                .h_full()
                .bg(self.color)
                .child(
                    div()
                        .absolute()
                        .left(px(-size_f32 / 2. + 0.5))
                        .top(px(y - size_f32 / 2.))
                        .size(self.size)
                        .rounded_full()
                        .border_2()
                        .border_color(self.color)
                        .bg(cx.theme().colors.background.alpha(0.5)),
                )
                .into_any_element()
        } else {
            div().id(self.id).into_any_element()
        }
    }
}
