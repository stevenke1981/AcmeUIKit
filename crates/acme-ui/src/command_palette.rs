use gpui::{
    App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

/// A command palette / search overlay.
///
/// # Example
///
/// ```ignore
/// CommandPalette::new()
///     .open(true)
/// ```
#[derive(IntoElement)]
pub struct CommandPalette {
    open: bool,
    #[allow(dead_code)]
    query: SharedString,
}

impl CommandPalette {
    /// Creates a new command palette.
    pub fn new() -> Self {
        Self {
            open: false,
            query: SharedString::default(),
        }
    }

    /// Sets the open state.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for CommandPalette {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        if !self.open {
            return div().into_any_element();
        }

        div()
            .absolute()
            .inset_0()
            .bg(gpui::black().opacity(0.5))
            .flex()
            .items_start()
            .justify_center()
            .pt(px(80.))
            .child(
                div()
                    .v_flex()
                    .w(px(480.))
                    .bg(c.surface)
                    .rounded(px(8.))
                    .border_1()
                    .border_color(c.border)
                    .child(
                        div()
                            .h_flex()
                            .items_center()
                            .gap_2()
                            .px(px(12.))
                            .py(px(10.))
                            .border_b_1()
                            .border_color(c.border)
                            .child(
                                Icon::new(IconName::Search)
                                    .with_size(px(14.))
                                    .into_any_element(),
                            )
                            .child(
                                div()
                                    .text_color(c.muted_foreground)
                                    .text_size(theme.font_sizes.body)
                                    .child("Type a command…"),
                            ),
                    )
                    .child(
                        div().px(px(12.)).py(px(16.)).child(
                            div()
                                .text_color(c.muted_foreground)
                                .text_size(theme.font_sizes.caption)
                                .child("No recent commands"),
                        ),
                    ),
            )
            .into_any_element()
    }
}
