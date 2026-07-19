use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt, Tone};

/// A form-level message for validation feedback.
///
/// # Example
///
/// ```ignore
/// FormMessage::new("msg")
///     .tone(Tone::Danger)
///     .message("This field is required")
/// ```
#[derive(IntoElement)]
pub struct FormMessage {
    id: ElementId,
    message: SharedString,
    tone: Tone,
}

impl FormMessage {
    /// Creates a new form message.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            message: SharedString::new(""),
            tone: Tone::Neutral,
        }
    }

    /// Sets the message text.
    pub fn message(mut self, msg: impl Into<SharedString>) -> Self {
        self.message = msg.into();
        self
    }

    /// Sets the tone (Neutral, Primary, Success, Warning, Danger).
    pub fn tone(mut self, t: Tone) -> Self {
        self.tone = t;
        self
    }
}

impl RenderOnce for FormMessage {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        let (icon, color) = match self.tone {
            Tone::Primary => (IconName::Info, c.primary),
            Tone::Success => (IconName::Success, c.success),
            Tone::Warning => (IconName::Warning, c.warning),
            Tone::Danger => (IconName::Error, c.danger),
            _ => (IconName::Info, c.muted_foreground),
        };

        div()
            .id(self.id)
            .h_flex()
            .items_center()
            .gap_2()
            .px(px(8.))
            .py(px(4.))
            .child(
                div()
                    .text_color(color)
                    .child(Icon::new(icon).with_size(px(12.))),
            )
            .child(
                div()
                    .text_size(theme.font_sizes.caption)
                    .text_color(color)
                    .child(self.message),
            )
    }
}
