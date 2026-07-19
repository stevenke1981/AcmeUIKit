use gpui::prelude::FluentBuilder as _;
use gpui::{
    App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

/// An empty state placeholder with icon, title, and description.
///
/// # Example
///
/// ```ignore
/// EmptyState::new("No results")
///     .icon(IconName::Search)
///     .description("Try adjusting your filters")
/// ```
#[derive(IntoElement)]
pub struct EmptyState {
    title: SharedString,
    description: Option<SharedString>,
    icon: Option<IconName>,
}

impl EmptyState {
    /// Creates a new empty state.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: None,
            icon: None,
        }
    }

    /// Sets the icon.
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets the description text.
    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

impl RenderOnce for EmptyState {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        div()
            .v_flex()
            .items_center()
            .justify_center()
            .gap_2()
            .py(px(40.))
            .child(if let Some(icon) = self.icon {
                div()
                    .text_color(c.muted_foreground)
                    .child(Icon::new(icon).with_size(px(32.)))
                    .into_any_element()
            } else {
                div().into_any_element()
            })
            .child(
                div()
                    .text_size(theme.font_sizes.body)
                    .text_color(c.muted_foreground)
                    .child(self.title.clone()),
            )
            .when_some(self.description, |this, desc| {
                this.child(
                    div()
                        .text_size(theme.font_sizes.caption)
                        .text_color(c.muted)
                        .child(desc),
                )
            })
    }
}
