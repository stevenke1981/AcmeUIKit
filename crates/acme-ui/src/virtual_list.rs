use std::ops::Range;

use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement as _, RenderOnce, Styled as _, Window,
    div, uniform_list,
};

use crate::{ActiveTheme, StyledExt};

/// A convenience wrapper around GPUI's [`uniform_list`] that applies Acme
/// theme styling.
///
/// Renders a scrollable, virtualized list of uniformly‑sized items. Only the
/// visible subset is rendered, making it suitable for large lists (>100 items).
///
/// The item height is determined automatically by measuring the first rendered
/// element.
///
/// # Example
///
/// ```ignore
/// VirtualList::new("my-list")
///     .total(1000)
///     .render(|range, _window, _cx| {
///         range.map(|i| div().h(px(32.)).child(format!("Item {i}")).into_any_element()).collect()
///     })
/// ```
#[derive(IntoElement)]
#[allow(clippy::type_complexity)]
pub struct VirtualList {
    id: ElementId,
    item_count: usize,
    render_items: Box<dyn Fn(Range<usize>, &mut Window, &mut App) -> Vec<AnyElement>>,
}

impl VirtualList {
    /// Create a new [`VirtualList`] with the given stable element `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            item_count: 0,
            render_items: Box::new(|_, _, _| Vec::new()),
        }
    }

    /// Set the total number of items in the list.
    pub fn total(mut self, total: usize) -> Self {
        self.item_count = total;
        self
    }

    /// Set the render function that produces elements for a visible range.
    ///
    /// The closure receives a `Range<usize>` of item indices to render,
    /// along with the `Window` and `App` contexts.
    pub fn render(
        mut self,
        f: impl Fn(Range<usize>, &mut Window, &mut App) -> Vec<AnyElement> + 'static,
    ) -> Self {
        self.render_items = Box::new(f);
        self
    }
}

impl RenderOnce for VirtualList {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;

        div().v_flex().flex_1().bg(c.background).child(uniform_list(
            self.id,
            self.item_count,
            self.render_items,
        ))
    }
}
