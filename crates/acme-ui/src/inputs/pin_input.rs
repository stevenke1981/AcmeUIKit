use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// A pin / OTP input with individual digit boxes.
///
/// # Example
///
/// ```ignore
/// PinInput::new("otp")
///     .digits(6)
///     .value("123")
/// ```
#[derive(IntoElement)]
pub struct PinInput {
    id: ElementId,
    digits: usize,
    value: SharedString,
}

impl PinInput {
    /// Creates a new pin input.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            digits: 4,
            value: SharedString::new(""),
        }
    }

    /// Sets the number of digit boxes.
    pub fn digits(mut self, n: usize) -> Self {
        self.digits = n.clamp(1, 10);
        self
    }

    /// Sets the current value.
    pub fn value(mut self, val: impl Into<SharedString>) -> Self {
        self.value = val.into();
        self
    }
}

impl RenderOnce for PinInput {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let chars: Vec<char> = self.value.chars().collect();

        let boxes: Vec<_> = (0..self.digits)
            .map(|i| {
                let ch = chars.get(i).copied().unwrap_or(' ');
                div()
                    .w(px(36.))
                    .h(px(40.))
                    .h_flex()
                    .items_center()
                    .justify_center()
                    .rounded(theme.radius)
                    .bg(c.muted)
                    .border_1()
                    .border_color(c.border)
                    .text_size(theme.font_sizes.body)
                    .text_color(c.foreground)
                    .child(ch.to_string())
            })
            .collect();

        div().id(self.id).h_flex().gap_2().children(boxes)
    }
}
