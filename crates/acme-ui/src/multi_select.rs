use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

/// A multi-select dropdown showing selected items.
///
/// # Example
///
/// ```ignore
/// MultiSelect::new("langs")
///     .items(&["Rust", "Go", "Python"])
///     .selected(&["Rust", "Go"])
/// ```
#[derive(IntoElement)]
pub struct MultiSelect {
    id: ElementId,
    items: Vec<SharedString>,
    selected: Vec<SharedString>,
    placeholder: SharedString,
}

impl MultiSelect {
    /// Creates a new multi-select.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            selected: Vec::new(),
            placeholder: SharedString::from("Select…"),
        }
    }

    /// Sets all available items.
    pub fn items(mut self, items: &[&str]) -> Self {
        self.items = items.iter().map(|s| SharedString::from(*s)).collect();
        self
    }

    /// Sets the currently selected items.
    pub fn selected_items(mut self, sel: &[&str]) -> Self {
        self.selected = sel.iter().map(|s| SharedString::from(*s)).collect();
        self
    }
}

impl RenderOnce for MultiSelect {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        let summary = if self.selected.is_empty() {
            self.placeholder
        } else {
            SharedString::from(format!("{} selected", self.selected.len()))
        };

        let items: Vec<_> = self
            .items
            .into_iter()
            .map(|item| {
                let checked = if self.selected.contains(&item) {
                    "✓ "
                } else {
                    "  "
                };
                div()
                    .h_flex()
                    .items_center()
                    .gap_2()
                    .h(px(28.))
                    .px(px(8.))
                    .text_size(theme.font_sizes.body)
                    .text_color(c.foreground)
                    .child(SharedString::from(format!("{}{}", checked, item)))
            })
            .collect();

        div()
            .id(self.id)
            .v_flex()
            .gap_1()
            .child(
                div()
                    .h_flex()
                    .items_center()
                    .gap_2()
                    .h(px(32.))
                    .px(px(10.))
                    .rounded(theme.radius)
                    .bg(c.muted)
                    .child(
                        div()
                            .flex_1()
                            .text_size(theme.font_sizes.body)
                            .text_color(if self.selected.is_empty() {
                                c.muted_foreground
                            } else {
                                c.foreground
                            })
                            .child(summary),
                    )
                    .child(
                        div()
                            .text_color(c.muted_foreground)
                            .child(Icon::new(IconName::ChevronDown).with_size(px(12.))),
                    ),
            )
            .child(
                div()
                    .v_flex()
                    .rounded(theme.radius)
                    .bg(c.background)
                    .border_1()
                    .border_color(c.border)
                    .overflow_hidden()
                    .children(items),
            )
    }
}
