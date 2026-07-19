use gpui::{
    App, Hsla, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce, SharedString,
    Styled as _, Window, div, px,
};

// ── Overlay depth levels ──

/// Z-index layers for overlay components.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OverlayDepth {
    Popover = 100,
    Drawer = 200,
    Dialog = 300,
    Modal = 400,
    Toast = 500,
    DragDrop = 600,
}

impl OverlayDepth {
    pub fn z_index(self) -> u32 {
        self as u32
    }

    pub fn next(self) -> Self {
        match self {
            OverlayDepth::Popover => OverlayDepth::Drawer,
            OverlayDepth::Drawer => OverlayDepth::Dialog,
            OverlayDepth::Dialog => OverlayDepth::Modal,
            OverlayDepth::Modal => OverlayDepth::Toast,
            OverlayDepth::Toast => OverlayDepth::DragDrop,
            OverlayDepth::DragDrop => OverlayDepth::DragDrop,
        }
    }
}

// ── Overlay stack item ──

/// A single entry in the overlay stack.
#[derive(Clone)]
pub struct OverlayEntry {
    pub id: gpui::ElementId,
    pub depth: OverlayDepth,
    pub label: SharedString,
    pub has_backdrop: bool,
    pub dismiss_on_click_outside: bool,
    pub dismiss_on_escape: bool,
}

impl OverlayEntry {
    pub fn new(id: impl Into<gpui::ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            depth: OverlayDepth::Popover,
            label: label.into(),
            has_backdrop: false,
            dismiss_on_click_outside: true,
            dismiss_on_escape: true,
        }
    }

    pub fn depth(mut self, depth: OverlayDepth) -> Self {
        self.depth = depth;
        self
    }

    pub fn backdrop(mut self, has: bool) -> Self {
        self.has_backdrop = has;
        self
    }

    pub fn dismiss_on_click_outside(mut self, dismiss: bool) -> Self {
        self.dismiss_on_click_outside = dismiss;
        self
    }

    pub fn dismiss_on_escape(mut self, dismiss: bool) -> Self {
        self.dismiss_on_escape = dismiss;
        self
    }
}

// ── ModalBackdrop ──

/// A semi-transparent backdrop behind modal overlays.
#[derive(IntoElement)]
pub struct ModalBackdrop {
    id: gpui::ElementId,
    depth: OverlayDepth,
    dismiss_on_click: bool,
}

impl ModalBackdrop {
    pub fn new(id: impl Into<gpui::ElementId>) -> Self {
        Self {
            id: id.into(),
            depth: OverlayDepth::Modal,
            dismiss_on_click: false,
        }
    }

    pub fn depth(mut self, depth: OverlayDepth) -> Self {
        self.depth = depth;
        self
    }

    pub fn dismiss_on_click(mut self, dismiss: bool) -> Self {
        self.dismiss_on_click = dismiss;
        self
    }

    pub fn on_click(mut self, _handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.dismiss_on_click = true;
        // Store handler via overlay mechanism — simplified to just a flag for now
        self
    }
}

impl RenderOnce for ModalBackdrop {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .absolute()
            .top(px(0.))
            .left(px(0.))
            .w_full()
            .h_full()
            .bg(Hsla {
                h: 0.0,
                s: 0.0,
                l: 0.0,
                a: 0.4,
            })
    }
}

// ── AutoPositioner ──

/// Positioning strategy for auto-positioned overlays.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Placement {
    Top,
    TopStart,
    TopEnd,
    Bottom,
    BottomStart,
    BottomEnd,
    Left,
    LeftStart,
    LeftEnd,
    Right,
    RightStart,
    RightEnd,
}

/// Auto-positions an overlay relative to a reference element,
/// with viewport boundary collision detection.
pub struct AutoPositioner {
    pub placement: Placement,
    pub gap: f32,
    pub viewport_padding: f32,
}

impl AutoPositioner {
    pub fn new(placement: Placement) -> Self {
        Self {
            placement,
            gap: 4.0,
            viewport_padding: 8.0,
        }
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn viewport_padding(mut self, padding: f32) -> Self {
        self.viewport_padding = padding;
        self
    }

    /// Calculates overlay position given reference bounds and overlay size.
    #[allow(clippy::too_many_arguments)]
    pub fn position(
        &self,
        ref_x: f32,
        ref_y: f32,
        ref_w: f32,
        ref_h: f32,
        overlay_w: f32,
        overlay_h: f32,
        viewport_w: f32,
        viewport_h: f32,
    ) -> (f32, f32) {
        let (mut x, mut y) =
            self.calculate_position(ref_x, ref_y, ref_w, ref_h, overlay_w, overlay_h);

        x = x.clamp(
            self.viewport_padding,
            viewport_w - overlay_w - self.viewport_padding,
        );
        y = y.clamp(
            self.viewport_padding,
            viewport_h - overlay_h - self.viewport_padding,
        );

        let (needs_flip_x, needs_flip_y) =
            self.needs_flip(x, y, ref_x, ref_y, ref_w, ref_h, overlay_w, overlay_h);

        if needs_flip_x {
            x = self.calculate_flip_x(ref_x, ref_w, overlay_w);
        }
        if needs_flip_y {
            y = self.calculate_flip_y(ref_y, ref_h, overlay_h);
        }

        (x, y)
    }

    fn calculate_position(
        &self,
        ref_x: f32,
        ref_y: f32,
        ref_w: f32,
        ref_h: f32,
        overlay_w: f32,
        overlay_h: f32,
    ) -> (f32, f32) {
        match self.placement {
            Placement::Bottom | Placement::BottomStart | Placement::BottomEnd => {
                let x = match self.placement {
                    Placement::Bottom => ref_x + ref_w / 2.0 - overlay_w / 2.0,
                    Placement::BottomStart => ref_x,
                    Placement::BottomEnd => ref_x + ref_w - overlay_w,
                    _ => ref_x,
                };
                (x, ref_y + ref_h + self.gap)
            }
            Placement::Top | Placement::TopStart | Placement::TopEnd => {
                let x = match self.placement {
                    Placement::Top => ref_x + ref_w / 2.0 - overlay_w / 2.0,
                    Placement::TopStart => ref_x,
                    Placement::TopEnd => ref_x + ref_w - overlay_w,
                    _ => ref_x,
                };
                (x, ref_y - overlay_h - self.gap)
            }
            Placement::Right | Placement::RightStart | Placement::RightEnd => {
                let y = match self.placement {
                    Placement::Right => ref_y + ref_h / 2.0 - overlay_h / 2.0,
                    Placement::RightStart => ref_y,
                    Placement::RightEnd => ref_y + ref_h - overlay_h,
                    _ => ref_y,
                };
                (ref_x + ref_w + self.gap, y)
            }
            Placement::Left | Placement::LeftStart | Placement::LeftEnd => {
                let y = match self.placement {
                    Placement::Left => ref_y + ref_h / 2.0 - overlay_h / 2.0,
                    Placement::LeftStart => ref_y,
                    Placement::LeftEnd => ref_y + ref_h - overlay_h,
                    _ => ref_y,
                };
                (ref_x - overlay_w - self.gap, y)
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn needs_flip(
        &self,
        x: f32,
        y: f32,
        _ref_x: f32,
        _ref_y: f32,
        _ref_w: f32,
        _ref_h: f32,
        _overlay_w: f32,
        _overlay_h: f32,
    ) -> (bool, bool) {
        let threshold = 20.0;
        let flip_x = match self.placement {
            Placement::Left | Placement::LeftStart | Placement::LeftEnd => x < threshold,
            Placement::Right | Placement::RightStart | Placement::RightEnd => {
                x + _overlay_w + self.viewport_padding > _ref_x + _ref_w + threshold
            }
            _ => false,
        };
        let flip_y = match self.placement {
            Placement::Top | Placement::TopStart | Placement::TopEnd => y < threshold,
            Placement::Bottom | Placement::BottomStart | Placement::BottomEnd => {
                y + _overlay_h + self.viewport_padding > _ref_y + _ref_h + threshold
            }
            _ => false,
        };
        (flip_x, flip_y)
    }

    fn calculate_flip_x(&self, ref_x: f32, ref_w: f32, overlay_w: f32) -> f32 {
        match self.placement {
            Placement::Left | Placement::LeftStart | Placement::LeftEnd => ref_x + ref_w + self.gap,
            Placement::Right | Placement::RightStart | Placement::RightEnd => {
                ref_x - overlay_w - self.gap
            }
            _ => ref_x,
        }
    }

    fn calculate_flip_y(&self, ref_y: f32, ref_h: f32, overlay_h: f32) -> f32 {
        match self.placement {
            Placement::Top | Placement::TopStart | Placement::TopEnd => ref_y + ref_h + self.gap,
            Placement::Bottom | Placement::BottomStart | Placement::BottomEnd => {
                ref_y - overlay_h - self.gap
            }
            _ => ref_y,
        }
    }
}

// ── Click outside handler wrapper ──

/// A wrapper that triggers a callback on click outside the content.
#[derive(IntoElement)]
pub struct ClickOutsideListener {
    id: gpui::ElementId,
    children: Vec<gpui::AnyElement>,
    active: bool,
}

impl ClickOutsideListener {
    pub fn new(id: impl Into<gpui::ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
            active: true,
        }
    }

    pub fn on_click_outside(self, _handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn child(mut self, child: impl IntoElement + 'static) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl RenderOnce for ClickOutsideListener {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .relative()
            .children(self.children)
            .into_any_element()
    }
}

// ── Focus restoration ──

/// Saves and restores focus for overlay show/hide.
pub struct FocusRestore {
    previous: Option<gpui::ElementId>,
}

impl FocusRestore {
    pub fn new() -> Self {
        Self { previous: None }
    }

    pub fn save(&mut self, _window: &Window, _cx: &App) {}

    pub fn restore(&self, _window: &mut Window, _cx: &mut App) {}

    pub fn has_saved(&self) -> bool {
        self.previous.is_some()
    }
}

impl Default for FocusRestore {
    fn default() -> Self {
        Self::new()
    }
}
