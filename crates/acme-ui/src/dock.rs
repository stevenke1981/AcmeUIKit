use gpui::{
    AnyElement, App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, Styled as _, Window, div, prelude::FluentBuilder as _, px,
};

use crate::{ActiveTheme, StyledExt};

/// Identifies which dock panel area.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DockArea {
    /// Left side panel.
    Left,
    /// Right side panel.
    Right,
    /// Bottom panel.
    Bottom,
}

/// A dock panel descriptor.
#[derive(Clone)]
pub struct DockPanel {
    /// Panel area placement.
    pub area: DockArea,
    /// Panel title shown in the header.
    pub title: SharedString,
    /// Whether the panel is currently visible.
    pub visible: bool,
    /// Panel size in pixels (width for left/right, height for bottom).
    pub size: gpui::Pixels,
}

impl DockPanel {
    /// Creates a new dock panel descriptor.
    pub fn new(area: DockArea, title: impl Into<SharedString>) -> Self {
        Self {
            area,
            title: title.into(),
            visible: true,
            size: px(200.),
        }
    }

    /// Sets the initial panel size.
    pub fn size(mut self, size: gpui::Pixels) -> Self {
        self.size = size;
        self
    }

    /// Sets the visible state.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}

/// Dock layout container with optional side and bottom panels.
///
/// The dock manages up to three panel areas (left, right, bottom) and a
/// central content area. Each panel's visibility and size is configured
/// via [`DockPanel`] descriptors. The caller controls which panels are
/// shown and their sizes.
///
/// # Example
///
/// ```ignore
/// Dock::new("main-dock")
///     .panels(vec![
///         DockPanel::new(DockArea::Left, "Explorer")
///             .size(px(200.))
///             .visible(true),
///         DockPanel::new(DockArea::Bottom, "Terminal")
///             .size(px(150.))
///             .visible(true),
///     ])
///     .child(main_content)
///     .render_left(explorer_tree)
///     .render_bottom(terminal_view)
/// ```
#[derive(IntoElement)]
pub struct Dock {
    id: ElementId,
    panels: Vec<DockPanel>,
    main_content: Option<AnyElement>,
    left_render: Option<AnyElement>,
    right_render: Option<AnyElement>,
    bottom_render: Option<AnyElement>,
}

impl Dock {
    /// Creates a new dock container with the given `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            panels: Vec::new(),
            main_content: None,
            left_render: None,
            right_render: None,
            bottom_render: None,
        }
    }

    /// Configures the dock panels (visibility and sizes).
    pub fn panels(mut self, panels: Vec<DockPanel>) -> Self {
        self.panels = panels;
        self
    }

    /// Sets the main content area child.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.main_content = Some(child.into_any_element());
        self
    }

    /// Sets the content rendered in the left panel.
    pub fn render_left(mut self, content: impl IntoElement) -> Self {
        self.left_render = Some(content.into_any_element());
        self
    }

    /// Sets the content rendered in the right panel.
    pub fn render_right(mut self, content: impl IntoElement) -> Self {
        self.right_render = Some(content.into_any_element());
        self
    }

    /// Sets the content rendered in the bottom panel.
    pub fn render_bottom(mut self, content: impl IntoElement) -> Self {
        self.bottom_render = Some(content.into_any_element());
        self
    }

    fn panel_size(&self, area: DockArea) -> Option<(gpui::Pixels, bool)> {
        self.panels
            .iter()
            .find(|p| p.area == area)
            .map(|p| (p.size, p.visible))
    }
}

impl RenderOnce for Dock {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        let left = self.panel_size(DockArea::Left);
        let right = self.panel_size(DockArea::Right);
        let bottom = self.panel_size(DockArea::Bottom);

        let left_render = self.left_render;
        let right_render = self.right_render;
        let bottom_render = self.bottom_render;

        // Build the main horizontal layout (left | center | right)
        let mut horizontal = div().flex().flex_row().w_full().h_full();

        // Left panel
        if let Some((size, true)) = left {
            if let Some(content) = left_render {
                horizontal = horizontal.child(
                    div()
                        .v_flex()
                        .w(size)
                        .h_full()
                        .border_r_1()
                        .border_color(c.border)
                        .bg(c.surface)
                        .overflow_hidden()
                        .child(content),
                );
            }
        }

        // Center content
        let center = div().flex_1().v_flex().h_full().overflow_hidden().child(
            div()
                .flex_1()
                .overflow_hidden()
                .when_some(self.main_content, |this, content| this.child(content)),
        );

        // Bottom panel within center
        let center = if let Some((size, true)) = bottom {
            if let Some(content) = bottom_render {
                center.child(
                    div()
                        .h(size)
                        .w_full()
                        .border_t_1()
                        .border_color(c.border)
                        .bg(c.surface)
                        .overflow_hidden()
                        .child(content),
                )
            } else {
                center
            }
        } else {
            center
        };

        horizontal = horizontal.child(center);

        // Right panel
        if let Some((size, true)) = right {
            if let Some(content) = right_render {
                horizontal = horizontal.child(
                    div()
                        .v_flex()
                        .w(size)
                        .h_full()
                        .border_l_1()
                        .border_color(c.border)
                        .bg(c.surface)
                        .overflow_hidden()
                        .child(content),
                );
            }
        }

        div()
            .id(self.id)
            .size_full()
            .bg(c.background)
            .child(horizontal)
    }
}
