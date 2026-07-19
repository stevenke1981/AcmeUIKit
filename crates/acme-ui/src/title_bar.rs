use std::rc::Rc;

use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StatefulInteractiveElement as _, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

type ClickHandler = Rc<dyn Fn(&mut Window, &mut App)>;

/// A window title bar with an optional icon, title, subtitle, and control buttons.
///
/// Renders as a horizontal bar with a surface background and a bottom border.
/// The left side displays an icon, a bold title, and an optional muted subtitle.
/// The right side contains minimize, maximize, and close buttons.
///
/// # Example
///
/// ```ignore
/// TitleBar::new("main-title")
///     .title("My App")
///     .icon(IconName::Settings)
///     .subtitle("v1.0")
///     .on_close(|_window, _cx| {})
/// ```
#[derive(IntoElement)]
pub struct TitleBar {
    id: ElementId,
    title: SharedString,
    icon: Option<IconName>,
    subtitle: Option<SharedString>,
    on_minimize: Option<ClickHandler>,
    on_maximize: Option<ClickHandler>,
    on_close: Option<ClickHandler>,
}

impl TitleBar {
    /// Creates a new `TitleBar` with the given [`ElementId`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            title: SharedString::default(),
            icon: None,
            subtitle: None,
            on_minimize: None,
            on_maximize: None,
            on_close: None,
        }
    }

    /// Sets the title text.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets the icon displayed before the title.
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets the subtitle text displayed after the title.
    pub fn subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
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

impl RenderOnce for TitleBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let on_minimize = self.on_minimize;
        let on_maximize = self.on_maximize;
        let on_close = self.on_close;

        // Left section: icon + title + subtitle
        let left = div()
            .h_flex()
            .gap_2()
            .flex_1()
            .child(match self.icon {
                Some(name) => Icon::new(name).into_any_element(),
                None => div().into_any_element(),
            })
            .child(
                div()
                    .h_flex()
                    .gap_1()
                    .child(
                        div()
                            .text_size(theme.font_sizes.body)
                            .text_color(c.foreground)
                            .child(self.title),
                    )
                    .children(self.subtitle.map(|sub| {
                        div()
                            .text_size(theme.font_sizes.caption)
                            .text_color(c.muted_foreground)
                            .child(sub)
                    })),
            );

        // Minimize button
        let mut min_btn = div()
            .id(ElementId::Name("titlebar-minimize".into()))
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
            .id(ElementId::Name("titlebar-maximize".into()))
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
            .id(ElementId::Name("titlebar-close".into()))
            .cursor_pointer()
            .rounded(px(4.))
            .px(px(6.))
            .py(px(2.))
            .hover(|style| style.bg(c.danger))
            .child(Icon::new(IconName::Close).with_size(px(14.)));
        if let Some(handler) = on_close {
            close_btn = close_btn.on_click(move |_event, window, cx| handler(window, cx));
        }

        // Right section: control buttons
        let right = div()
            .h_flex()
            .gap_2()
            .child(min_btn)
            .child(max_btn)
            .child(close_btn);

        div()
            .id(self.id)
            .h_flex()
            .h(px(40.))
            .px(px(8.))
            .gap_2()
            .bg(c.surface)
            .border_b_1()
            .border_color(c.border)
            .child(left)
            .child(right)
    }
}
