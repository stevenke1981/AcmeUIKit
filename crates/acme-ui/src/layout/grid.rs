use gpui::{App, IntoElement, ParentElement as _, RenderOnce, Styled as _, Window, div};

/// A CSS grid layout container.
///
/// # Example
///
/// ```ignore
/// Grid::new()
///     .cols(3)
///     .gap(px(8.))
///     .child(div().child("A"))
///     .child(div().child("B"))
/// ```
#[derive(IntoElement)]
pub struct Grid {
    cols: usize,
    gap: Option<Pixels>,
    children: Vec<gpui::AnyElement>,
}

impl Grid {
    /// Creates a new grid.
    pub fn new() -> Self {
        Self {
            cols: 2,
            gap: None,
            children: Vec::new(),
        }
    }

    /// Sets the number of columns.
    pub fn cols(mut self, cols: usize) -> Self {
        self.cols = cols;
        self
    }

    /// Sets the gap between cells.
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = Some(gap.into());
        self
    }

    /// Adds a child element.
    pub fn child(mut self, child: impl IntoElement + 'static) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Grid {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut base = div().grid().grid_cols(self.cols as u16);
        if let Some(gap) = self.gap {
            base = base.gap(gap);
        }
        base.children(self.children)
    }
}

use gpui::Pixels;
