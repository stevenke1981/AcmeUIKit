use gpui::{
    AnyElement, App, IntoElement, ParentElement as _, RenderOnce, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// A container for toolbar actions (icons, buttons, separators).
///
/// # Example
///
/// ```ignore
/// Toolbar::new()
///     .child(button_a)
///     .separator()
///     .child(button_b)
/// ```
#[derive(IntoElement)]
pub struct Toolbar {
    children: Vec<ToolbarChild>,
}

enum ToolbarChild {
    Element(AnyElement),
    Separator,
}

impl Toolbar {
    /// Creates a new toolbar.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    /// Adds a child element.
    pub fn child(mut self, child: impl IntoElement + 'static) -> Self {
        self.children
            .push(ToolbarChild::Element(child.into_any_element()));
        self
    }

    /// Adds a visual divider between items.
    pub fn separator(mut self) -> Self {
        self.children.push(ToolbarChild::Separator);
        self
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Toolbar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        div()
            .h_flex()
            .items_center()
            .gap_0()
            .px(px(4.))
            .py(px(4.))
            .rounded(px(6.))
            .bg(c.surface)
            .border_1()
            .border_color(c.border)
            .children(self.children.into_iter().map(|child| {
                match child {
                    ToolbarChild::Element(el) => el,
                    ToolbarChild::Separator => div()
                        .w(px(1.))
                        .h(px(16.))
                        .mx(px(4.))
                        .bg(c.border)
                        .into_any_element(),
                }
            }))
    }
}
