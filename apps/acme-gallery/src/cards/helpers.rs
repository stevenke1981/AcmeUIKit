//! Shared helper functions for gallery cards.

use acme_ui::{ActiveTheme, Icon, IconName, StyledExt, ThemeColors};
use gpui::{App, IntoElement, ParentElement as _, Styled as _, div, px};

/// Renders a section title + description header.
pub fn section_title(title: &'static str, description: &'static str, cx: &App) -> impl IntoElement {
    let c = cx.theme().colors;
    div()
        .v_flex()
        .gap_1()
        .child(
            div()
                .text_size(px(18.))
                .text_color(c.foreground)
                .child(title),
        )
        .child(
            div()
                .text_size(px(12.))
                .text_color(c.muted_foreground)
                .child(description),
        )
}

/// Renders an icon demo item with label.
pub fn icon_demo(label: &'static str, icon: IconName, c: ThemeColors) -> impl IntoElement {
    div()
        .v_flex()
        .items_center()
        .gap_1()
        .child(
            div()
                .text_color(c.foreground)
                .child(Icon::new(icon).with_size(px(20.))),
        )
        .child(
            div()
                .text_size(px(10.))
                .text_color(c.muted_foreground)
                .child(label),
        )
}
