use gpui::prelude::FluentBuilder as _;
use gpui::{
    App, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce, SharedString,
    Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

/// A collapsible section with a clickable header.
///
/// # Example
///
/// ```ignore
/// Collapsible::new("details")
///     .title("More Info")
///     .open(true)
///     .child(div().child("Hidden content"))
/// ```
#[derive(IntoElement)]
pub struct Collapsible {
    title: SharedString,
    open: bool,
    child: Option<gpui::AnyElement>,
}

impl Collapsible {
    /// Creates a new collapsible section.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            open: false,
            child: None,
        }
    }

    /// Sets the open state.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Sets the child content.
    pub fn child(mut self, child: impl IntoElement + 'static) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl RenderOnce for Collapsible {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        div()
            .v_flex()
            .w_full()
            .child(
                div()
                    .h_flex()
                    .items_center()
                    .gap_2()
                    .px(px(8.))
                    .py(px(6.))
                    .rounded(theme.radius)
                    .hover(|style| style.bg(c.muted))
                    .child(
                        div()
                            .text_size(theme.font_sizes.body)
                            .text_color(c.foreground)
                            .child(self.title.clone()),
                    )
                    .child(
                        div().ml_auto().text_color(c.muted_foreground).child(
                            Icon::new(if self.open {
                                IconName::ChevronDown
                            } else {
                                IconName::ChevronRight
                            })
                            .with_size(px(12.)),
                        ),
                    ),
            )
            .when(self.open, |this| {
                this.child(
                    div()
                        .px(px(8.))
                        .pb(px(6.))
                        .child(if let Some(child) = self.child {
                            child
                        } else {
                            div().into_any_element()
                        }),
                )
            })
    }
}
