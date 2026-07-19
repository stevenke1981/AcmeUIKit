use gpui::{
    App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt, Tone};

/// An alert banner showing contextual feedback.
///
/// # Example
///
/// ```ignore
/// Alert::new("File saved successfully.")
///     .tone(Tone::Success)
/// ```
#[derive(IntoElement)]
pub struct Alert {
    message: SharedString,
    tone: Tone,
    dismissible: bool,
}

impl Alert {
    /// Creates a new alert with the given message.
    pub fn new(message: impl Into<SharedString>) -> Self {
        Self {
            message: message.into(),
            tone: Tone::Primary,
            dismissible: false,
        }
    }

    /// Sets the tone variant.
    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = tone;
        self
    }

    /// Enables the dismiss button.
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    fn tone_icon(&self) -> Option<IconName> {
        match self.tone {
            Tone::Neutral => None,
            Tone::Primary => Some(IconName::Info),
            Tone::Success => Some(IconName::Success),
            Tone::Warning => Some(IconName::Warning),
            Tone::Danger => Some(IconName::Error),
        }
    }

    fn tone_color(&self, c: &crate::ThemeColors) -> gpui::Hsla {
        match self.tone {
            Tone::Neutral => c.muted_foreground,
            Tone::Primary => c.primary,
            Tone::Success => c.success,
            Tone::Warning => c.warning,
            Tone::Danger => c.danger,
        }
    }
}

impl RenderOnce for Alert {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let accent = self.tone_color(&c);

        div()
            .h_flex()
            .items_start()
            .gap_2()
            .p(px(12.))
            .rounded(px(8.))
            .bg(accent.opacity(0.1))
            .border_1()
            .border_color(accent.opacity(0.3))
            .child(if let Some(icon) = self.tone_icon() {
                div()
                    .text_color(accent)
                    .child(Icon::new(icon).with_size(px(16.)))
                    .into_any_element()
            } else {
                div().into_any_element()
            })
            .child(
                div()
                    .flex_1()
                    .text_size(theme.font_sizes.body)
                    .text_color(c.foreground)
                    .child(self.message.clone()),
            )
            .when(self.dismissible, |this| {
                this.child(
                    div()
                        .text_color(c.muted_foreground)
                        .child(Icon::new(IconName::Close).with_size(px(12.))),
                )
            })
    }
}
