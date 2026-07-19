use std::rc::Rc;

use gpui::{
    App, Context, ElementId, InteractiveElement as _, IntoElement, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, ParentElement as _, Render, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

type ResizeHandler = Rc<dyn Fn(f32, &mut Window, &mut App)>;

/// Split direction for the [`Resizable`] pane.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    /// Left and right panels separated by a vertical divider.
    Horizontal,
    /// Top and bottom panels separated by a horizontal divider.
    Vertical,
}

/// A resizable split pane with a draggable divider.
///
/// Stateful (Entity + Render) — the entity tracks drag state and split ratio.
///
/// The component renders a flex container with a draggable divider. Content
/// for each panel should be provided as children around the Resizable element
/// or by nesting inside the panels' wrapper divs.
///
/// # Example
///
/// ```ignore
/// cx.new(|cx| {
///     Resizable::new("splitter", cx)
///         .initial_split(0.6)
///         .direction(Direction::Horizontal)
/// })
/// ```
pub struct Resizable {
    id: ElementId,
    split_ratio: f32,
    direction: Direction,
    dragging: bool,
    drag_start_ratio: f32,
    drag_start_pos: f32,
    on_resize: Option<ResizeHandler>,
}

impl Resizable {
    /// Create a new [`Resizable`] entity.
    pub fn new(id: impl Into<ElementId>, _cx: &mut Context<Self>) -> Self {
        Self {
            id: id.into(),
            split_ratio: 0.5,
            direction: Direction::Horizontal,
            dragging: false,
            drag_start_ratio: 0.5,
            drag_start_pos: 0.,
            on_resize: None,
        }
    }

    /// Set the initial split ratio (0.0–1.0). Default is 0.5.
    pub fn initial_split(mut self, ratio: f32) -> Self {
        self.split_ratio = ratio.clamp(0.15, 0.85);
        self
    }

    /// Set the split direction. Default is [`Direction::Horizontal`].
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Register a callback that fires when the split ratio changes.
    pub fn on_resize(mut self, handler: impl Fn(f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_resize = Some(Rc::new(handler));
        self
    }
}

// ── Render ───────────────────────────────────────────────────────────────────

impl Render for Resizable {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;
        let is_horizontal = self.direction == Direction::Horizontal;

        // ── divider ─────────────────────────────────────────────────────────
        let divider_size = px(4.);

        let mut divider = div()
            .id(ElementId::Name(format!("{:?}-divider", self.id).into()))
            .cursor_pointer()
            .bg(if self.dragging { c.primary } else { c.border })
            .hover(|style| style.bg(c.primary));

        divider = if is_horizontal {
            divider
                .w(divider_size)
                .h_full()
                .cursor_col_resize()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(
                        |this: &mut Resizable, event: &MouseDownEvent, _window, _cx| {
                            this.dragging = true;
                            this.drag_start_ratio = this.split_ratio;
                            this.drag_start_pos = f32::from(event.position.x);
                        },
                    ),
                )
        } else {
            divider
                .h(divider_size)
                .w_full()
                .cursor_row_resize()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(
                        |this: &mut Resizable, event: &MouseDownEvent, _window, _cx| {
                            this.dragging = true;
                            this.drag_start_ratio = this.split_ratio;
                            this.drag_start_pos = f32::from(event.position.y);
                        },
                    ),
                )
        };

        // ── empty panel placeholders ────────────────────────────────────────
        let panel1 = div()
            .flex()
            .min_w(if is_horizontal { px(50.) } else { px(0.) })
            .min_h(if is_horizontal { px(0.) } else { px(50.) })
            .overflow_hidden();

        let panel2 = div()
            .flex()
            .min_w(if is_horizontal { px(50.) } else { px(0.) })
            .min_h(if is_horizontal { px(0.) } else { px(50.) })
            .overflow_hidden();

        let mut container = if is_horizontal {
            div().h_flex().h_full().w_full()
        } else {
            div().v_flex().h_full().w_full()
        };

        container = container.child(panel1).child(divider).child(panel2);

        if self.dragging {
            // Transparent full-size overlay to capture mouse move/up
            let overlay = div()
                .id(ElementId::Name(
                    format!("{:?}-drag-overlay", self.id).into(),
                ))
                .absolute()
                .inset_0()
                .on_mouse_move(cx.listener(
                    |this: &mut Resizable, event: &MouseMoveEvent, _window, _cx| {
                        if this.dragging {
                            let pos = if this.direction == Direction::Horizontal {
                                f32::from(event.position.x)
                            } else {
                                f32::from(event.position.y)
                            };
                            let delta = pos - this.drag_start_pos;
                            let ratio_delta = delta / 400.0;
                            let new_ratio = (this.drag_start_ratio + ratio_delta).clamp(0.15, 0.85);
                            this.split_ratio = new_ratio;
                        }
                    },
                ))
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|this: &mut Resizable, _event: &MouseUpEvent, window, cx| {
                        this.dragging = false;
                        if let Some(ref handler) = this.on_resize {
                            handler(this.split_ratio, window, cx);
                        }
                        cx.notify();
                    }),
                );

            container = container.child(overlay);
        };

        container.into_any_element()
    }
}
