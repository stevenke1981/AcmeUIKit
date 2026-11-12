use std::rc::Rc;

use gpui::{
    App, ElementId, FocusHandle, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div,
};

use crate::{ActiveTheme, RovingOrientation, RovingTabIndex, Size, StyledExt};

/// A segmented control (button group with single selection).
///
/// # Example
///
/// ```ignore
/// SegmentedControl::new("view")
///     .items(&["List", "Grid", "Split"])
///     .selected(1)
///     .on_select(|index, _window, _cx| { })
/// ```
#[allow(clippy::type_complexity)]
#[derive(IntoElement)]
pub struct SegmentedControl {
    id: ElementId,
    items: Vec<SharedString>,
    selected: usize,
    focus_handle: Option<FocusHandle>,
    size: Size,
    on_select: Option<Rc<dyn Fn(usize, &mut Window, &mut App)>>,
}

impl SegmentedControl {
    /// Creates a new segmented control.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            selected: 0,
            focus_handle: None,
            size: Size::Small,
            on_select: None,
        }
    }

    /// Sets the segment items.
    pub fn items(mut self, items: &[&str]) -> Self {
        self.items = items.iter().map(|s| SharedString::from(*s)).collect();
        self
    }

    /// Sets the selected index.
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    /// Enables horizontal arrows and Home/End when `on_select` is also set.
    ///
    /// The caller retains the handle across renders and updates `selected` in
    /// the callback. Navigation clamps at boundaries. This adds one tab stop
    /// and a theme focus border, but does not implement screen-reader semantics.
    pub fn focus_handle(mut self, focus_handle: FocusHandle) -> Self {
        self.focus_handle = Some(focus_handle);
        self
    }

    /// Sets the control size.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Registers a selection handler.
    pub fn on_select(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for SegmentedControl {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        let mut control = div()
            .id(self.id)
            .h_flex()
            .items_center()
            .rounded(theme.radius)
            .border_1()
            .border_color(c.border)
            .overflow_hidden();

        if let (Some(focus_handle), Some(handler)) =
            (self.focus_handle.as_ref(), self.on_select.clone())
        {
            let nav = RovingTabIndex::new(self.items.len())
                .orientation(RovingOrientation::Horizontal)
                .focus(self.selected);
            let click_focus = focus_handle.clone();
            control = control
                .track_focus(focus_handle)
                .tab_stop(true)
                .focus(|style| style.border_color(c.ring))
                .on_click(move |_, window, cx| click_focus.focus(window, cx))
                .on_key_down(move |event, window, cx| {
                    let mut nav = nav.clone();
                    let old = nav.focused_index;
                    if let Some(index) =
                        nav.handle_key(&event.keystroke.key, &event.keystroke.modifiers)
                    {
                        window.prevent_default();
                        cx.stop_propagation();
                        if index != old {
                            handler(index, window, cx);
                        }
                    }
                });
        }

        let keyboard_enabled = self.focus_handle.is_some();
        control.children(self.items.into_iter().enumerate().map({
            let on_select = self.on_select.clone();
            move |(i, label)| {
                let is_selected = i == self.selected;
                let bg = if is_selected {
                    c.primary
                } else {
                    gpui::transparent_black()
                };
                let text_color = if is_selected {
                    c.primary_foreground
                } else {
                    c.muted_foreground
                };

                let mut seg = div()
                    .id(ElementId::Name(format!("seg-control-seg-{}", i).into()))
                    .h_flex()
                    .items_center()
                    .justify_center()
                    .px(self.size.horizontal_padding())
                    .h(self.size.height())
                    .bg(bg)
                    .text_color(text_color)
                    .text_size(self.size.text_size())
                    .cursor_pointer()
                    .child(label);

                if let Some(handler) = on_select.clone() {
                    seg = seg.on_click(move |_event, window, cx| {
                        if !keyboard_enabled || i != self.selected {
                            handler(i, window, cx);
                        }
                    });
                }

                seg
            }
        }))
    }
}
