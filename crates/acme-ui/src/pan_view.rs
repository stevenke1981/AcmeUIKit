use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// Pannable viewport.
///
/// Displays a clipped container with a label centred inside
/// and an offset indicator shown at the bottom-right corner.
///
/// # Example
///
/// ```ignore
/// PanView::new("map-pan")
///     .offset(50.0, -20.0)
///     .label("Map Area")
/// ```
#[derive(IntoElement)]
pub struct PanView {
    id: ElementId,
    label: SharedString,
    offset_x: f32,
    offset_y: f32,
}

impl PanView {
    /// Creates a new pan view with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: SharedString::default(),
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }

    /// Sets the pan offset.
    pub fn offset(mut self, x: f32, y: f32) -> Self {
        self.offset_x = x;
        self.offset_y = y;
        self
    }

    /// Sets the label text shown in the content area.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
}

impl RenderOnce for PanView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let font_sizes = cx.theme().font_sizes;

        let offset_text =
            SharedString::from(format!("x:{:.0} y:{:.0}", self.offset_x, self.offset_y));

        div()
            .id(self.id)
            .w_full()
            .h(px(200.))
            .overflow_hidden()
            .border_1()
            .border_color(c.border)
            .rounded(px(6.))
            .bg(c.surface)
            .child(
                div()
                    .w_full()
                    .h_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(c.muted_foreground)
                    .text_size(font_sizes.body)
                    .child(self.label),
            )
            .child(
                div()
                    .h_flex()
                    .justify_end()
                    .px(px(8.))
                    .py(px(4.))
                    .text_color(c.muted_foreground)
                    .text_size(font_sizes.caption)
                    .child(offset_text),
            )
    }
}
