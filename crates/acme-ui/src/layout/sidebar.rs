use gpui::{
    AnyElement, App, ElementId, InteractiveElement as _, IntoElement, ParentElement, Pixels,
    RenderOnce, SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// Vertical navigation or secondary-content panel with optional header.
///
/// Renders as a full-height column with a surface background, border separator,
/// optional title header, and a padded content area for children.
///
/// ### Example
/// ```ignore
/// Sidebar::new("nav")
///     .title("Navigation")
///     .width(px(240.))
///     .child(label)
///     .child(another_item);
/// ```
#[derive(IntoElement)]
pub struct Sidebar {
    id: ElementId,
    title: Option<SharedString>,
    width: Pixels,
    children: Vec<AnyElement>,
}

impl Sidebar {
    /// Creates a new `Sidebar` with the given [`ElementId`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            title: None,
            width: px(220.),
            children: Vec::new(),
        }
    }

    /// Sets an optional header title.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the sidebar width in pixels (default: `px(220.)`).
    pub fn width(mut self, width: Pixels) -> Self {
        self.width = width;
        self
    }
}

impl ParentElement for Sidebar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;

        let mut panel = div()
            .id(self.id)
            .v_flex()
            .h_full()
            .w(self.width)
            .bg(c.surface)
            .border_1()
            .border_color(c.border);

        if let Some(title) = self.title {
            panel = panel.child(
                div()
                    .px(cx.theme().spacing.group)
                    .py(cx.theme().spacing.widget)
                    .text_color(c.foreground)
                    .text_size(cx.theme().typography.title.size)
                    .child(title),
            );
        }

        panel.child(
            div()
                .flex_1()
                .px(cx.theme().spacing.group)
                .py(cx.theme().spacing.widget)
                .children(self.children),
        )
    }
}
