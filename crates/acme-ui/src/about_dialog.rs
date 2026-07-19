use std::rc::Rc;

use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Button, Icon, IconName, StyledExt};

type CloseHandler = Rc<dyn Fn(&mut Window, &mut App)>;

/// An About dialog showing application name, version, and description.
///
/// # Example
///
/// ```ignore
/// AboutDialog::new("about")
///     .app_name("Acme Editor")
///     .version("1.0.0")
///     .description("A clean GPUI component kit.")
///     .on_close(|_window, _cx| { /* close handler */ })
/// ```
#[derive(IntoElement)]
pub struct AboutDialog {
    id: ElementId,
    app_name: SharedString,
    version: SharedString,
    description: SharedString,
    on_close: Option<CloseHandler>,
}

impl AboutDialog {
    /// Creates a new about dialog with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            app_name: SharedString::default(),
            version: SharedString::default(),
            description: SharedString::default(),
            on_close: None,
        }
    }

    /// Sets the application name.
    pub fn app_name(mut self, name: impl Into<SharedString>) -> Self {
        self.app_name = name.into();
        self
    }

    /// Sets the version string.
    pub fn version(mut self, version: impl Into<SharedString>) -> Self {
        self.version = version.into();
        self
    }

    /// Sets the description text.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = description.into();
        self
    }

    /// Registers a close handler invoked when the close button is clicked.
    pub fn on_close(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for AboutDialog {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        div()
            .id(self.id)
            .v_flex()
            .items_center()
            .gap_4()
            .p_6()
            .bg(c.surface)
            .rounded(theme.radius_lg)
            .border_1()
            .border_color(c.border)
            .max_w(px(320.))
            .child(Icon::new(IconName::Info).with_size(px(32.)))
            .child(
                div()
                    .text_color(c.foreground)
                    .text_size(theme.font_sizes.heading)
                    .child(self.app_name),
            )
            .child(
                div()
                    .text_color(c.muted_foreground)
                    .text_size(theme.font_sizes.caption)
                    .child(self.version),
            )
            .child(
                div()
                    .text_color(c.muted_foreground)
                    .text_size(theme.font_sizes.body)
                    .child(self.description),
            )
            .child(match self.on_close {
                Some(handler) => {
                    let close_btn = Button::new("about-close").label("Close").primary();
                    close_btn
                        .on_click(move |_event, window, cx| handler(window, cx))
                        .into_any_element()
                }
                None => div().into_any_element(),
            })
    }
}
