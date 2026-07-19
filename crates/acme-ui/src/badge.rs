use gpui::{
    App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Tone};

/// Compact semantic status label.
#[derive(IntoElement)]
pub struct Badge {
    label: SharedString,
    tone: Tone,
}

impl Badge {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            tone: Tone::Neutral,
        }
    }

    pub fn with_tone(mut self, tone: Tone) -> Self {
        self.tone = tone;
        self
    }

    pub fn primary(self) -> Self {
        self.with_tone(Tone::Primary)
    }

    pub fn success(self) -> Self {
        self.with_tone(Tone::Success)
    }

    pub fn warning(self) -> Self {
        self.with_tone(Tone::Warning)
    }

    pub fn danger(self) -> Self {
        self.with_tone(Tone::Danger)
    }
}

impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let (background, foreground) = match self.tone {
            Tone::Neutral => (c.muted, c.muted_foreground),
            Tone::Primary => (c.primary, c.primary_foreground),
            Tone::Success => (c.success, c.primary_foreground),
            Tone::Warning => (c.warning, c.foreground),
            Tone::Danger => (c.danger, c.primary_foreground),
        };

        div()
            .h(px(22.))
            .px_2()
            .flex()
            .items_center()
            .rounded_full()
            .bg(background)
            .text_color(foreground)
            .text_size(px(11.))
            .child(self.label)
    }
}
