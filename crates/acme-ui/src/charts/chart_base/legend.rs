//! Legend component with colored swatches and labels.

use gpui::{
    App, ElementId, Hsla, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// A single legend entry.
#[derive(Clone)]
pub struct LegendItem {
    /// Display name.
    pub name: SharedString,
    /// Color swatch.
    pub color: Hsla,
    /// Whether the series is visible.
    pub visible: bool,
}

impl LegendItem {
    /// Creates a new legend item.
    pub fn new(name: impl Into<SharedString>, color: Hsla) -> Self {
        Self {
            name: name.into(),
            color,
            visible: true,
        }
    }
}

/// Legend layout direction.
#[derive(Clone, Debug, PartialEq)]
pub enum LegendLayout {
    /// Items arranged horizontally.
    Horizontal,
    /// Items arranged vertically.
    Vertical,
}

/// A chart legend component.
///
/// Renders colored swatches with labels for each data series.
#[derive(IntoElement)]
pub struct Legend {
    id: ElementId,
    items: Vec<LegendItem>,
    layout: LegendLayout,
}

impl Legend {
    /// Creates a new legend with the given `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            layout: LegendLayout::Horizontal,
        }
    }

    /// Adds a single legend item.
    pub fn item(mut self, item: LegendItem) -> Self {
        self.items.push(item);
        self
    }

    /// Replaces all legend items.
    pub fn items(mut self, items: Vec<LegendItem>) -> Self {
        self.items = items;
        self
    }

    /// Sets the layout direction.
    pub fn layout(mut self, layout: LegendLayout) -> Self {
        self.layout = layout;
        self
    }
}

impl RenderOnce for Legend {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let is_horizontal = self.layout == LegendLayout::Horizontal;
        let items: Vec<_> = self
            .items
            .into_iter()
            .map(|item| {
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(
                        div()
                            .size(px(10.))
                            .rounded(px(2.))
                            .bg(item.color)
                            .flex_none(),
                    )
                    .child(
                        div()
                            .text_size(theme.font_sizes.caption)
                            .text_color(if item.visible {
                                theme.colors.foreground
                            } else {
                                theme.colors.muted_foreground
                            })
                            .child(item.name),
                    )
                    .into_any_element()
            })
            .collect();

        if is_horizontal {
            div()
                .id(self.id)
                .flex()
                .flex_wrap()
                .gap_3()
                .children(items)
                .into_any_element()
        } else {
            div()
                .id(self.id)
                .v_flex()
                .gap_2()
                .children(items)
                .into_any_element()
        }
    }
}
