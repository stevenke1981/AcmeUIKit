use gpui::{
    AnyElement, App, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    Styled as _, Window, div,
};

use crate::ActiveTheme;

/// Orientation for a tile split.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileDirection {
    /// Children are laid out horizontally (side by side).
    Horizontal,
    /// Children are laid out vertically (stacked).
    Vertical,
}

/// A tile within a [`Tiles`] layout.
///
/// Each tile has optional fixed or flex sizing, and contains a single child
/// element.
#[derive(IntoElement)]
pub struct Tile {
    /// Size ratio or fixed pixel width/height.
    /// `None` = flex / fill remaining space.
    pub size: Option<gpui::Pixels>,
    child: Option<AnyElement>,
}

impl Tile {
    /// Creates a new tile with flex sizing (fills remaining space).
    pub fn new() -> Self {
        Self {
            size: None,
            child: None,
        }
    }

    /// Sets a fixed pixel size for this tile.
    pub fn fixed(mut self, size: gpui::Pixels) -> Self {
        self.size = Some(size);
        self
    }

    /// Attaches child content.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Tile {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        if let Some(child) = self.child {
            child
        } else {
            div().into_any_element()
        }
    }
}

/// Tiling layout container.
///
/// Splits space between child [`Tile`]s in either horizontal or vertical
/// direction. Tiles with fixed size get their allocated space first; remaining
/// space is distributed equally among flex tiles.
///
/// # Example
///
/// ```ignore
/// Tiles::new(gpui::ElementId::Name("layout".into()))
///     .direction(TileDirection::Horizontal)
///     .tile(Tile::new().fixed(px(200.)).child(sidebar))
///     .tile(Tile::new().child(main_content))
/// ```
#[derive(IntoElement)]
pub struct Tiles {
    id: gpui::ElementId,
    direction: TileDirection,
    tiles: Vec<Tile>,
}

impl Tiles {
    /// Creates a new tiles container with the given `id`.
    pub fn new(id: impl Into<gpui::ElementId>) -> Self {
        Self {
            id: id.into(),
            direction: TileDirection::Horizontal,
            tiles: Vec::new(),
        }
    }

    /// Sets the layout direction.
    pub fn direction(mut self, dir: TileDirection) -> Self {
        self.direction = dir;
        self
    }

    /// Adds a tile child.
    pub fn tile(mut self, tile: Tile) -> Self {
        self.tiles.push(tile);
        self
    }

    /// Adds multiple tile children.
    pub fn tiles(mut self, tiles: Vec<Tile>) -> Self {
        self.tiles.extend(tiles);
        self
    }
}

impl RenderOnce for Tiles {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        let container = div()
            .id(self.id)
            .flex()
            .w_full()
            .h_full()
            .overflow_hidden()
            .bg(c.background);

        let container = match self.direction {
            TileDirection::Horizontal => container.flex_row(),
            TileDirection::Vertical => container.flex_col(),
        };

        container.children(self.tiles.into_iter().map(move |tile| {
            let mut el = div().overflow_hidden();

            if let Some(fixed) = tile.size {
                el = match self.direction {
                    TileDirection::Horizontal => el.w(fixed),
                    TileDirection::Vertical => el.h(fixed),
                };
            }

            if let Some(child) = tile.child {
                el = el.child(child);
            }

            el.border_1().border_color(c.border).into_any_element()
        }))
    }
}
