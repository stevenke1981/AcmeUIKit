use gpui::prelude::FluentBuilder;
use gpui::{
    App, InteractiveElement as _, IntoElement, KeyDownEvent, ParentElement as _, RenderOnce,
    Styled as _, Window, div,
};

// ── FocusTrap ──

/// A container that traps keyboard focus within its children.
///
/// When active, Tab cycles through focusable elements inside the trap.
#[derive(IntoElement)]
pub struct FocusTrap {
    id: gpui::ElementId,
    active: bool,
    children: Vec<gpui::AnyElement>,
}

impl FocusTrap {
    pub fn new(id: impl Into<gpui::ElementId>) -> Self {
        Self {
            id: id.into(),
            active: true,
            children: Vec::new(),
        }
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

impl RenderOnce for FocusTrap {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id.clone())
            .relative()
            .when(self.active, |this| {
                this.on_key_down(|event: &KeyDownEvent, window, _cx| {
                    if event.keystroke.key == "tab" {
                        window.prevent_default();
                    }
                })
            })
            .children(self.children)
    }
}

// ── RovingTabIndex ──

/// A utility for managing roving tabindex within a composite widget.
#[derive(Clone, Debug)]
pub struct RovingTabIndex {
    pub item_count: usize,
    pub focused_index: usize,
    pub orientation: RovingOrientation,
    pub wrap: bool,
}

/// Orientation for roving tab index arrow key behavior.
#[derive(Clone, Debug, PartialEq)]
pub enum RovingOrientation {
    Horizontal,
    Vertical,
    Both,
}

impl RovingTabIndex {
    pub fn new(item_count: usize) -> Self {
        Self {
            item_count,
            focused_index: 0,
            orientation: RovingOrientation::Vertical,
            wrap: false,
        }
    }

    pub fn orientation(mut self, orientation: RovingOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn focus(mut self, index: usize) -> Self {
        self.focused_index = index.min(self.item_count.saturating_sub(1));
        self
    }

    /// Returns "0" if index is focused, "-1" otherwise.
    pub fn tab_index_str(&self, index: usize) -> &'static str {
        if index == self.focused_index {
            "0"
        } else {
            "-1"
        }
    }

    /// Handles a key event for arrow-based navigation.
    /// Returns the new focused index, or `None` if key was not handled.
    pub fn handle_key(&mut self, key: &str, _modifiers: &gpui::Modifiers) -> Option<usize> {
        let is_horiz = self.orientation == RovingOrientation::Horizontal
            || self.orientation == RovingOrientation::Both;
        let is_vert = self.orientation == RovingOrientation::Vertical
            || self.orientation == RovingOrientation::Both;

        let delta = match key {
            _ if is_vert && key == "ArrowDown" => 1,
            _ if is_vert && key == "ArrowUp" => -1,
            _ if is_horiz && key == "ArrowRight" => 1,
            _ if is_horiz && key == "ArrowLeft" => -1,
            _ if key == "Home" => i32::MIN / 2,
            _ if key == "End" => i32::MAX / 2,
            _ => return None,
        };

        let count = self.item_count.max(1) as i32;
        let current = self.focused_index as i32;
        let next = current + delta;

        let new_index = if self.wrap {
            ((next % count) + count) % count
        } else {
            next.clamp(0, count - 1)
        };

        self.focused_index = new_index as usize;
        Some(self.focused_index)
    }
}

// ── Arrow key navigation handler ──

/// Creates an `on_key_down` handler for arrow key navigation in a list.
pub fn arrow_key_nav_handler(
    item_count: usize,
    orientation: RovingOrientation,
    wrap: bool,
    mut on_focus: impl FnMut(usize, &mut Window, &mut App) + 'static,
) -> impl FnMut(&KeyDownEvent, &mut Window, &mut App) + 'static {
    let mut nav = RovingTabIndex::new(item_count)
        .orientation(orientation)
        .wrap(wrap);

    move |event: &KeyDownEvent, window, cx| {
        let old = nav.focused_index;
        if nav
            .handle_key(&event.keystroke.key, &event.keystroke.modifiers)
            .is_some()
            && nav.focused_index != old
        {
            on_focus(nav.focused_index, window, cx);
        }
    }
}

// ── Escape close handler ──

/// Creates an `on_key_down` handler that calls `on_close` when Escape is pressed.
pub fn escape_close_handler(
    mut on_close: impl FnMut(&mut Window, &mut App) + 'static,
) -> impl FnMut(&KeyDownEvent, &mut Window, &mut App) + 'static {
    move |event: &KeyDownEvent, window, cx| {
        if event.keystroke.key == "escape" {
            on_close(window, cx);
        }
    }
}

// ── Default / Cancel button contexts ──

/// Identifies the default (Enter) and cancel (Escape) buttons in a container.
#[derive(Clone)]
pub struct DefaultCancelButtons {
    pub default_id: Option<gpui::ElementId>,
    pub cancel_id: Option<gpui::ElementId>,
}

impl DefaultCancelButtons {
    pub fn new() -> Self {
        Self {
            default_id: None,
            cancel_id: None,
        }
    }

    pub fn default(mut self, id: impl Into<gpui::ElementId>) -> Self {
        self.default_id = Some(id.into());
        self
    }

    pub fn cancel(mut self, id: impl Into<gpui::ElementId>) -> Self {
        self.cancel_id = Some(id.into());
        self
    }
}

impl Default for DefaultCancelButtons {
    fn default() -> Self {
        Self::new()
    }
}
