use gpui::{
    App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

/// A tag/chip label.
///
/// # Example
///
/// ```ignore
/// Tag::new("Rust")
///     .color(gpui::hsla(0.6, 0.7, 0.5, 1.0))
///     .removable(true)
/// ```
#[derive(IntoElement)]
pub struct Tag {
    label: SharedString,
    color: Option<gpui::Hsla>,
    removable: bool,
}

impl Tag {
    /// Creates a new tag with the given label.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            color: None,
            removable: false,
        }
    }

    /// Sets the tag accent color.
    pub fn color(mut self, color: impl Into<Option<gpui::Hsla>>) -> Self {
        self.color = color.into();
        self
    }

    /// Shows a remove button.
    pub fn removable(mut self, removable: bool) -> Self {
        self.removable = removable;
        self
    }

    fn accent(&self, c: &crate::ThemeColors) -> gpui::Hsla {
        self.color.unwrap_or(c.primary)
    }
}

impl RenderOnce for Tag {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let accent = self.accent(&c);

        div()
            .h_flex()
            .items_center()
            .gap_1()
            .px(px(8.))
            .py(px(2.))
            .rounded_full()
            .bg(accent.opacity(0.12))
            .child(
                div()
                    .text_size(theme.font_sizes.caption)
                    .text_color(accent)
                    .child(self.label.clone()),
            )
            .when(self.removable, |this| {
                this.child(
                    div()
                        .text_color(accent.opacity(0.6))
                        .child(Icon::new(IconName::Close).with_size(px(10.))),
                )
            })
    }
}
