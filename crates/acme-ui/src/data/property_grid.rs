use gpui::{
    App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// A property grid with key-value rows.
///
/// # Example
///
/// ```ignore
/// PropertyGrid::new()
///     .property("Name", "Acme App")
///     .property("Version", "2.1.0")
///     .property("Author", "Acme Corp")
/// ```
#[derive(IntoElement)]
pub struct PropertyGrid {
    properties: Vec<(SharedString, SharedString)>,
}

impl PropertyGrid {
    /// Creates a new property grid.
    pub fn new() -> Self {
        Self {
            properties: Vec::new(),
        }
    }

    /// Adds a property row.
    pub fn property(
        mut self,
        key: impl Into<SharedString>,
        value: impl Into<SharedString>,
    ) -> Self {
        self.properties.push((key.into(), value.into()));
        self
    }
}

impl Default for PropertyGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for PropertyGrid {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        div()
            .v_flex()
            .w_full()
            .children(self.properties.into_iter().map(|(key, value)| {
                div()
                    .h_flex()
                    .items_center()
                    .w_full()
                    .px(px(8.))
                    .py(px(6.))
                    .child(
                        div()
                            .w(px(120.))
                            .text_size(theme.font_sizes.caption)
                            .text_color(c.muted_foreground)
                            .child(key),
                    )
                    .child(
                        div()
                            .flex_1()
                            .text_size(theme.font_sizes.body)
                            .text_color(c.foreground)
                            .child(value),
                    )
            }))
    }
}
