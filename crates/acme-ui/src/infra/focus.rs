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
    /// Number of items in the collection.
    pub item_count: usize,
    /// Current focused index; zero is the empty-collection sentinel.
    pub focused_index: usize,
    /// Arrow directions accepted for navigation.
    pub orientation: RovingOrientation,
    /// Whether arrows wrap at collection boundaries.
    pub wrap: bool,
}

/// Orientation for roving tab index arrow key behavior.
#[derive(Clone, Debug, PartialEq)]
pub enum RovingOrientation {
    /// Left and right arrows.
    Horizontal,
    /// Up and down arrows.
    Vertical,
    /// All four arrow directions.
    Both,
}

impl RovingTabIndex {
    /// Creates vertical, clamped navigation with the first item focused.
    pub fn new(item_count: usize) -> Self {
        Self {
            item_count,
            focused_index: 0,
            orientation: RovingOrientation::Vertical,
            wrap: false,
        }
    }

    /// Sets which arrow keys navigate the collection.
    pub fn orientation(mut self, orientation: RovingOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Enables wrapping for arrows; Home and End always jump absolutely.
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// Focuses an index clamped to the collection, or zero for an empty collection.
    pub fn focus(mut self, index: usize) -> Self {
        self.focused_index = index.min(self.item_count.saturating_sub(1));
        self
    }

    /// Returns "0" for the focused item, or "-1" for other or empty items.
    pub fn tab_index_str(&self, index: usize) -> &'static str {
        if self.item_count != 0 && index < self.item_count && index == self.focused_index {
            "0"
        } else {
            "-1"
        }
    }

    /// Handles GPUI arrow names and legacy `Arrow*` names, case-insensitively.
    ///
    /// Home/End jump to the first/last item regardless of wrapping. Modifiers
    /// do not affect navigation. Returns the target even at a clamped boundary;
    /// unhandled keys and empty collections return `None` without changing state.
    pub fn handle_key(&mut self, key: &str, _modifiers: &gpui::Modifiers) -> Option<usize> {
        let target = roving_target(self, key)?;
        self.focused_index = target;
        Some(target)
    }
}

fn roving_target(nav: &RovingTabIndex, key: &str) -> Option<usize> {
    let last = nav.item_count.checked_sub(1)?;
    let current = nav.focused_index.min(last);
    let horizontal = nav.orientation != RovingOrientation::Vertical;
    let vertical = nav.orientation != RovingOrientation::Horizontal;
    let key = key.to_ascii_lowercase();
    match key.as_str() {
        "home" => Some(0),
        "end" => Some(last),
        "down" | "arrowdown" if vertical => Some(if current == last {
            if nav.wrap { 0 } else { last }
        } else {
            current + 1
        }),
        "right" | "arrowright" if horizontal => Some(if current == last {
            if nav.wrap { 0 } else { last }
        } else {
            current + 1
        }),
        "up" | "arrowup" if vertical => Some(if current == 0 {
            if nav.wrap { last } else { 0 }
        } else {
            current - 1
        }),
        "left" | "arrowleft" if horizontal => Some(if current == 0 {
            if nav.wrap { last } else { 0 }
        } else {
            current - 1
        }),
        _ => None,
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
