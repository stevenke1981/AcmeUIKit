use std::rc::Rc;

use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StatefulInteractiveElement as _, Styled as _, Window, div, hsla, px,
};

use crate::{ActiveTheme, StyledExt};

type CloseHandler = Rc<dyn Fn(&mut Window, &mut App)>;

/// A semi-transparent modal overlay with title and optional close button.
///
/// The overlay always renders when mounted; the parent controls visibility
/// by mounting/unmounting this component.
///
/// # Example
///
/// ```ignore
/// WindowOverlay::new("settings-overlay")
///     .title("Settings")
///     .open(true)
///     .on_close(|_window, _cx| { /* close handler */ })
/// ```
#[derive(IntoElement)]
pub struct WindowOverlay {
    id: ElementId,
    title: SharedString,
    on_close: Option<CloseHandler>,
}

impl WindowOverlay {
    /// Creates a new window overlay with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            title: SharedString::default(),
            on_close: None,
        }
    }

    /// Sets the overlay title text.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }

    /// Registers a close handler invoked when the close button is clicked.
    pub fn on_close(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for WindowOverlay {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        // Backdrop: full-screen semi-transparent overlay
        let backdrop = div()
            .id(self.id)
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0., 0., 0., 0.5));

        // Modal surface
        let mut surface = div()
            .relative()
            .w(px(400.))
            .v_flex()
            .rounded(theme.radius_lg)
            .border_1()
            .border_color(c.border)
            .bg(c.surface)
            .p_4()
            .gap_4();

        // Header with title and close button
        let mut header = div().h_flex().justify_between().gap_2();

        header = header.child(
            div()
                .text_color(c.foreground)
                .text_size(theme.font_sizes.heading)
                .child(self.title),
        );

        if let Some(handler) = self.on_close {
            header = header.child(
                div()
                    .id(ElementId::Name("overlay-close".into()))
                    .flex()
                    .items_center()
                    .justify_center()
                    .size(px(28.))
                    .rounded(theme.radius_sm)
                    .cursor_pointer()
                    .hover(|style| style.bg(c.muted))
                    .child(
                        div()
                            .text_color(c.muted_foreground)
                            .text_size(theme.font_sizes.heading)
                            .child("×"),
                    )
                    .on_click(move |_event, window, cx| handler(window, cx)),
            );
        }

        surface = surface.child(header);

        backdrop.child(surface)
    }
}
