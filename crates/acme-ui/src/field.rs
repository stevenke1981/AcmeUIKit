use gpui::{
    App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// Presentational text-field shell for V1.
///
/// This component intentionally does not implement cursor, selection, keyboard, or IME behavior.
#[derive(IntoElement)]
pub struct FieldShell {
    label: SharedString,
    value: Option<SharedString>,
    placeholder: Option<SharedString>,
    helper: Option<SharedString>,
    error: bool,
}

impl FieldShell {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            value: None,
            placeholder: None,
            helper: None,
            error: false,
        }
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn helper(mut self, helper: impl Into<SharedString>) -> Self {
        self.helper = Some(helper.into());
        self
    }

    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }
}

impl RenderOnce for FieldShell {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let has_value = self.value.is_some();
        let text = self
            .value
            .or(self.placeholder)
            .unwrap_or_else(|| SharedString::from(""));

        let mut field = div()
            .w_full()
            .v_flex()
            .gap_1()
            .child(
                div()
                    .text_color(c.foreground)
                    .text_size(px(12.))
                    .child(self.label),
            )
            .child(
                div()
                    .h(px(36.))
                    .px_3()
                    .flex()
                    .items_center()
                    .rounded(theme.radius)
                    .border_1()
                    .border_color(if self.error { c.danger } else { c.border })
                    .bg(c.background)
                    .text_color(if has_value {
                        c.foreground
                    } else {
                        c.muted_foreground
                    })
                    .text_size(px(13.))
                    .child(text),
            );

        if let Some(helper) = self.helper {
            field = field.child(
                div()
                    .text_color(if self.error {
                        c.danger
                    } else {
                        c.muted_foreground
                    })
                    .text_size(px(11.))
                    .child(helper),
            );
        }

        field
    }
}
