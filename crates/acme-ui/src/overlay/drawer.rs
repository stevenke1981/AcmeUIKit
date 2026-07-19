use gpui::{
    App, IntoElement, ParentElement as _, Pixels, RenderOnce, SharedString, Styled as _, Window,
    div, px,
};

use crate::{ActiveTheme, StyledExt};

/// An overlay drawer that slides in from the side.
///
/// # Example
///
/// ```ignore
/// Drawer::new("prefs", "Preferences")
///     .open(true)
///     .child(div().child("Settings"))
/// ```
#[derive(IntoElement)]
pub struct Drawer {
    title: SharedString,
    open: bool,
    child: Option<gpui::AnyElement>,
    /// Width in pixels (default 280)
    width: Pixels,
}

impl Drawer {
    /// Creates a new drawer.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            open: false,
            child: None,
            width: px(280.),
        }
    }

    /// Sets the open state.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Sets the drawer width.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the child content.
    pub fn child(mut self, child: impl IntoElement + 'static) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl RenderOnce for Drawer {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        if !self.open {
            return div().into_any_element();
        }

        div()
            .absolute()
            .right(px(0.))
            .top(px(0.))
            .h_full()
            .w(self.width)
            .v_flex()
            .bg(c.surface)
            .border_l_1()
            .border_color(c.border)
            .child(
                div()
                    .h_flex()
                    .items_center()
                    .justify_between()
                    .px(px(16.))
                    .py(px(12.))
                    .child(
                        div()
                            .text_size(theme.font_sizes.body)
                            .text_color(c.foreground)
                            .child(self.title.clone()),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .px(px(16.))
                    .child(if let Some(child) = self.child {
                        child
                    } else {
                        div().into_any_element()
                    }),
            )
            .into_any_element()
    }
}
