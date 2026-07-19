use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

/// A simple image placeholder display.
///
/// Shows an icon and alt text inside a styled placeholder.
/// Actual GPUI Image element should be used for real images.
///
/// # Example
///
/// ```ignore
/// ImageView::new("photo")
///     .src("/path/to/image.png")
///     .alt("A beautiful landscape")
///     .width(px(200.))
///     .height(px(150.))
/// ```
#[derive(IntoElement)]
pub struct ImageView {
    id: ElementId,
    src: SharedString,
    alt: SharedString,
    width: Option<gpui::Pixels>,
    height: Option<gpui::Pixels>,
}

impl ImageView {
    /// Creates a new image view with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            src: SharedString::default(),
            alt: SharedString::default(),
            width: None,
            height: None,
        }
    }

    /// Sets the image source URL/path.
    pub fn src(mut self, src: impl Into<SharedString>) -> Self {
        self.src = src.into();
        self
    }

    /// Sets the alt text.
    pub fn alt(mut self, alt: impl Into<SharedString>) -> Self {
        self.alt = alt.into();
        self
    }

    /// Sets the display width.
    pub fn width(mut self, width: impl Into<gpui::Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the display height.
    pub fn height(mut self, height: impl Into<gpui::Pixels>) -> Self {
        self.height = Some(height.into());
        self
    }
}

impl RenderOnce for ImageView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let font_sizes = cx.theme().font_sizes;

        let mut container = div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .v_flex()
            .gap_1()
            .bg(c.muted)
            .rounded(px(6.))
            .overflow_hidden()
            .text_color(c.muted_foreground)
            .text_size(font_sizes.caption);

        if let Some(w) = self.width {
            container = container.w(w);
        }
        if let Some(h) = self.height {
            container = container.h(h);
        }

        container
            .child(Icon::new(IconName::Folder).with_size(px(24.)))
            .child(self.alt)
    }
}
