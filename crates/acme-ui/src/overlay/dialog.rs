use std::rc::Rc;

use gpui::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div, hsla, px,
};

use crate::{ActiveTheme, StyledExt};

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// Modal dialog overlay with backdrop and close button.
///
/// The caller manages open/close state. When `open` is false, the dialog renders
/// an empty div. When open, it renders a full-screen semi-transparent backdrop
/// with a centered modal surface containing the title, close button, and children.
///
/// The dialog uses absolute positioning and should be placed at the root of the
/// view tree (or inside a positioned container) for proper full-screen overlay.
///
/// # Example
///
/// ```ignore
/// Dialog::new()
///     .title("Confirm")
///     .open(true)
///     .on_close(|_, _, cx| { /* handle close */ })
///     .child(div().child("Are you sure?"))
/// ```
#[derive(IntoElement)]
pub struct Dialog {
    title: Option<SharedString>,
    open: bool,
    children: Vec<AnyElement>,
    on_close: Option<ClickHandler>,
}

impl Dialog {
    /// Creates a new closed dialog with no title.
    pub fn new() -> Self {
        Self {
            title: None,
            open: false,
            children: Vec::new(),
            on_close: None,
        }
    }

    /// Sets the dialog title text.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets whether the dialog is visible.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Sets a callback invoked when the close button is clicked.
    pub fn on_close(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_close = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for Dialog {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Default for Dialog {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Dialog {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        if !self.open {
            return div();
        }

        let on_close = self.on_close;

        // Backdrop: full-screen semi-transparent overlay
        let backdrop = div()
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0., 0., 0., 0.5));

        // Modal surface
        let mut surface = div()
            .relative()
            .w(px(480.))
            .max_h(px(600.))
            .v_flex()
            .rounded(theme.radius_lg)
            .border_1()
            .border_color(c.border)
            .bg(c.surface)
            .p_4()
            .gap_4()
            .overflow_hidden();

        // Header with title and close button
        if self.title.is_some() || on_close.is_some() {
            let mut header = div().flex().items_center().justify_between().gap_2();

            if let Some(ref title) = self.title {
                header = header.child(
                    div()
                        .flex_1()
                        .text_color(c.foreground)
                        .text_size(theme.font_sizes.heading)
                        .child(title.clone()),
                );
            }

            if let Some(handler) = &on_close {
                let handler = handler.clone();
                header = header.child(
                    div()
                        .id(ElementId::Name("dialog-close".into()))
                        .flex()
                        .items_center()
                        .justify_center()
                        .size(px(28.))
                        .rounded(theme.radius_sm)
                        .cursor_pointer()
                        .hover(move |style| style.bg(c.muted))
                        .child(
                            div()
                                .text_color(c.muted_foreground)
                                .text_size(theme.font_sizes.heading)
                                .child("×"),
                        )
                        .on_click(move |event, window, cx| handler(event, window, cx)),
                );
            }

            surface = surface.child(header);
        }

        // Body content
        surface = surface.children(self.children);

        backdrop.child(surface)
    }
}
