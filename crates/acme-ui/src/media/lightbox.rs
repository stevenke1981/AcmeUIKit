use std::rc::Rc;

use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StatefulInteractiveElement as _, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

/// Full-screen image lightbox overlay.
///
/// Renders as a semi-transparent dark backdrop with a centered
/// image placeholder and caption. A close button is shown at the top-right.
///
/// Since this is a `RenderOnce` component, it is always visible when rendered.
///
/// # Example
///
/// ```ignore
/// Lightbox::new("gallery-lightbox")
///     .src("/path/to/image.png")
///     .caption("A beautiful sunset")
///     .on_close(|window, cx| { /* close action */ })
/// ```
#[allow(clippy::type_complexity)]
#[derive(IntoElement)]
pub struct Lightbox {
    id: ElementId,
    src: SharedString,
    caption: SharedString,
    #[allow(dead_code)]
    open: bool,
    on_close: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
}

impl Lightbox {
    /// Creates a new lightbox with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            src: SharedString::default(),
            caption: SharedString::default(),
            open: true,
            on_close: None,
        }
    }

    /// Sets the image source URL/path.
    pub fn src(mut self, src: impl Into<SharedString>) -> Self {
        self.src = src.into();
        self
    }

    /// Sets the caption text.
    pub fn caption(mut self, caption: impl Into<SharedString>) -> Self {
        self.caption = caption.into();
        self
    }

    /// Sets a close handler.
    pub fn on_close(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Lightbox {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let font_sizes = cx.theme().font_sizes;

        // Close button
        let close_btn = div()
            .id(ElementId::Name(SharedString::from("lightbox-close")))
            .cursor_pointer()
            .text_color(gpui::white())
            .text_size(px(24.))
            .child(Icon::new(IconName::Close).with_size(px(24.)));

        let close_handler = self.on_close;
        let close_btn = if let Some(handler) = close_handler {
            close_btn.on_click(move |_event, window, cx| handler(window, cx))
        } else {
            close_btn
        };

        // Overlay
        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .h_full()
            .bg(gpui::hsla(0., 0., 0., 0.7))
            .child(
                div()
                    .v_flex()
                    .items_center()
                    .gap_2()
                    .child(
                        // Content box
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(400.))
                            .h(px(300.))
                            .bg(c.muted)
                            .rounded(px(8.))
                            .text_color(c.muted_foreground)
                            .text_size(font_sizes.body)
                            .child(Icon::new(IconName::Folder).with_size(px(48.))),
                    )
                    .child(
                        div()
                            .text_color(gpui::white())
                            .text_size(font_sizes.body)
                            .child(self.caption),
                    )
                    // Close button positioned at top-right via a wrapper
                    .child(close_btn),
            )
    }
}
