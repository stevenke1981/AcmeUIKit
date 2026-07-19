use std::rc::Rc;

use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StatefulInteractiveElement as _, Styled as _, Window, div,
};

use crate::{ActiveTheme, Size, StyledExt};

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

        div()
            .id(self.id)
            .h_flex()
            .items_center()
            .rounded(theme.radius)
            .border_1()
            .border_color(c.border)
            .overflow_hidden()
            .children(self.items.into_iter().enumerate().map({
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
                            handler(i, window, cx);
                        });
                    }

                    seg
                }
            }))
    }
}
