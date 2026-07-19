use std::rc::Rc;

use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    StatefulInteractiveElement as _, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

type ClickHandler = Rc<dyn Fn(&mut Window, &mut App)>;

/// A set of window control buttons (minimize, maximize, close).
///
/// Renders three small clickable buttons in a horizontal row.
/// Typically used inside a [`TitleBar`](crate::TitleBar) or standalone in a custom chrome.
///
/// # Example
///
/// ```ignore
/// WindowControls::new()
///     .on_minimize(|window, _cx| window.minimize())
///     .on_close(|window, cx| window.quit(cx))
/// ```
#[derive(IntoElement, Default)]
pub struct WindowControls {
    on_minimize: Option<ClickHandler>,
    on_maximize: Option<ClickHandler>,
    on_close: Option<ClickHandler>,
}

impl WindowControls {
    /// Creates a new `WindowControls` with no callbacks registered.
    pub fn new() -> Self {
        Self {
            on_minimize: None,
            on_maximize: None,
            on_close: None,
        }
    }

    /// Registers a handler for the minimize button.
    pub fn on_minimize(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_minimize = Some(Rc::new(handler));
        self
    }

    /// Registers a handler for the maximize button.
    pub fn on_maximize(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_maximize = Some(Rc::new(handler));
        self
    }

    /// Registers a handler for the close button.
    pub fn on_close(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for WindowControls {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let on_minimize = self.on_minimize;
        let on_maximize = self.on_maximize;
        let on_close = self.on_close;

        // Minimize button
        let mut min_btn = div()
            .id(ElementId::Name("wc-minimize".into()))
            .cursor_pointer()
            .rounded(px(4.))
            .px(px(6.))
            .py(px(2.))
            .hover(|style| style.bg(c.muted))
            .child(Icon::new(IconName::Minus).with_size(px(14.)));
        if let Some(handler) = on_minimize {
            min_btn = min_btn.on_click(move |_event, window, cx| handler(window, cx));
        }

        // Maximize button
        let mut max_btn = div()
            .id(ElementId::Name("wc-maximize".into()))
            .cursor_pointer()
            .rounded(px(4.))
            .px(px(6.))
            .py(px(2.))
            .hover(|style| style.bg(c.muted))
            .child(div().text_size(px(14.)).child("□"));
        if let Some(handler) = on_maximize {
            max_btn = max_btn.on_click(move |_event, window, cx| handler(window, cx));
        }

        // Close button
        let mut close_btn = div()
            .id(ElementId::Name("wc-close".into()))
            .cursor_pointer()
            .rounded(px(4.))
            .px(px(6.))
            .py(px(2.))
            .hover(|style| style.bg(c.danger))
            .child(Icon::new(IconName::Close).with_size(px(14.)));
        if let Some(handler) = on_close {
            close_btn = close_btn.on_click(move |_event, window, cx| handler(window, cx));
        }

        div()
            .h_flex()
            .gap_2()
            .child(min_btn)
            .child(max_btn)
            .child(close_btn)
    }
}
