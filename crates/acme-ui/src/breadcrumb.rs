use gpui::{
    App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div,
    prelude::FluentBuilder as _,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

/// A breadcrumb navigation trail.
///
/// # Example
///
/// ```ignore
/// Breadcrumb::new()
///     .item("Home")
///     .item("Products")
///     .item("Details")
/// ```
#[derive(IntoElement)]
pub struct Breadcrumb {
    items: Vec<SharedString>,
}

impl Breadcrumb {
    /// Creates a new breadcrumb.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Adds a breadcrumb item.
    pub fn item(mut self, label: impl Into<SharedString>) -> Self {
        self.items.push(label.into());
        self
    }
}

impl Default for Breadcrumb {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Breadcrumb {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let total = self.items.len();

        div()
            .h_flex()
            .items_center()
            .gap_1()
            .children(self.items.into_iter().enumerate().map(|(i, label)| {
                let is_last = i == total - 1;

                div()
                    .h_flex()
                    .items_center()
                    .gap_1()
                    .child(
                        div()
                            .text_size(theme.font_sizes.body)
                            .text_color(if is_last {
                                c.foreground
                            } else {
                                c.muted_foreground
                            })
                            .child(label),
                    )
                    .when(!is_last, |this| {
                        this.child(Icon::new(IconName::ChevronRight).with_size(px(10.)))
                    })
            }))
    }
}

use gpui::px;
