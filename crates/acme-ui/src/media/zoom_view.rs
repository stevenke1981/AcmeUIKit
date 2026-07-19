use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// Zoomable content container.
///
/// Displays a bordered container with zoom controls at the top
/// and a content area showing the label.
///
/// # Example
///
/// ```ignore
/// ZoomView::new("map-zoom")
///     .zoom(1.5)
///     .label("Map Content")
/// ```
#[derive(IntoElement)]
pub struct ZoomView {
    id: ElementId,
    zoom: f64,
    label: SharedString,
}

impl ZoomView {
    /// Creates a new zoom view with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            zoom: 1.0,
            label: SharedString::default(),
        }
    }

    /// Sets the zoom level (1.0 = 100%).
    pub fn zoom(mut self, zoom: f64) -> Self {
        self.zoom = zoom;
        self
    }

    /// Sets the label text shown in the content area.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
}

impl RenderOnce for ZoomView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let font_sizes = cx.theme().font_sizes;

        let zoom_percent = SharedString::from(format!("{}%", (self.zoom * 100.0) as u32));

        div()
            .id(self.id)
            .v_flex()
            .w_full()
            .h(px(200.))
            .border_1()
            .border_color(c.border)
            .rounded(px(6.))
            .overflow_hidden()
            // Zoom toolbar
            .child(
                div()
                    .h_flex()
                    .h(px(28.))
                    .px(px(8.))
                    .bg(c.muted)
                    .gap_2()
                    .child(
                        div()
                            .id(ElementId::Name(SharedString::from("zoom-out")))
                            .cursor_pointer()
                            .text_color(c.foreground)
                            .text_size(font_sizes.body)
                            .child("-"),
                    )
                    .child(
                        div()
                            .text_size(font_sizes.caption)
                            .text_color(c.muted_foreground)
                            .child(zoom_percent),
                    )
                    .child(
                        div()
                            .id(ElementId::Name(SharedString::from("zoom-in")))
                            .cursor_pointer()
                            .text_color(c.foreground)
                            .text_size(font_sizes.body)
                            .child("+"),
                    ),
            )
            // Content area
            .child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(c.background)
                    .text_color(c.muted_foreground)
                    .text_size(font_sizes.body)
                    .child(self.label),
            )
    }
}
