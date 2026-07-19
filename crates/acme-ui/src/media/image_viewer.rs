use crate::{ActiveTheme, Icon, IconName, StyledExt};
use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

/// How the image fits within the viewer bounds.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ImageFit {
    /// Image is contained within the viewport.
    #[default]
    Contain,
    /// Image covers the entire viewport.
    Cover,
    /// Image is stretched to fill the viewport.
    Fill,
    /// Image is displayed at its natural size.
    None,
}

/// Full image viewer with toolbar controls.
///
/// # Example
///
/// ```ignore
/// ImageViewer::new("viewer")
///     .src("/path/to/photo.png")
///     .zoom(1.5)
///     .fit(ImageFit::Contain)
/// ```
#[derive(IntoElement)]
pub struct ImageViewer {
    id: ElementId,
    src: SharedString,
    zoom: f64,
    fit: ImageFit,
}

impl ImageViewer {
    /// Creates a new image viewer with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            src: SharedString::default(),
            zoom: 1.0,
            fit: ImageFit::Contain,
        }
    }

    /// Sets the image source URL/path.
    pub fn src(mut self, src: impl Into<SharedString>) -> Self {
        self.src = src.into();
        self
    }

    /// Sets the zoom level (1.0 = 100%).
    pub fn zoom(mut self, zoom: f64) -> Self {
        self.zoom = zoom;
        self
    }

    /// Sets the image fit mode.
    pub fn fit(mut self, fit: ImageFit) -> Self {
        self.fit = fit;
        self
    }
}

impl RenderOnce for ImageViewer {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let font_sizes = cx.theme().font_sizes;

        let zoom_percent = SharedString::from(format!("{}%", (self.zoom * 100.0) as u32));

        let fit_label = SharedString::from(match self.fit {
            ImageFit::Contain => "Contain",
            ImageFit::Cover => "Cover",
            ImageFit::Fill => "Fill",
            ImageFit::None => "None",
        });

        div()
            .id(self.id)
            .v_flex()
            .w_full()
            .h(px(300.))
            .border_1()
            .border_color(c.border)
            .rounded(px(6.))
            .overflow_hidden()
            .bg(c.surface)
            // Toolbar
            .child(
                div()
                    .h_flex()
                    .h(px(32.))
                    .px(px(8.))
                    .bg(c.muted)
                    .gap_2()
                    .child(
                        div()
                            .id(ElementId::Name(SharedString::from("viewer-zoom-out")))
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
                            .id(ElementId::Name(SharedString::from("viewer-zoom-in")))
                            .cursor_pointer()
                            .text_color(c.foreground)
                            .text_size(font_sizes.body)
                            .child("+"),
                    )
                    .child(
                        div()
                            .id(ElementId::Name(SharedString::from("viewer-fit-btn")))
                            .cursor_pointer()
                            .text_color(c.foreground)
                            .text_size(font_sizes.caption)
                            .child(fit_label),
                    ),
            )
            // Content area
            .child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(c.muted)
                    .child(
                        div()
                            .v_flex()
                            .items_center()
                            .gap_1()
                            .text_color(c.muted_foreground)
                            .text_size(font_sizes.caption)
                            .child(Icon::new(IconName::Folder).with_size(px(32.)))
                            .child(self.src),
                    ),
            )
    }
}
