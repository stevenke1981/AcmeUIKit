use std::rc::Rc;

use gpui::prelude::FluentBuilder;
use gpui::{
    App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Button, Icon, IconName, StyledExt};

type RetryHandler = Rc<dyn Fn(&mut Window, &mut App)>;

/// An error state placeholder with icon, title, message, and retry button.
///
/// # Example
///
/// ```ignore
/// ErrorState::new("Connection lost")
///     .message("Check your network and try again")
///     .retry_label("Retry")
/// ```
#[derive(IntoElement)]
pub struct ErrorState {
    title: SharedString,
    message: Option<SharedString>,
    retry_label: Option<SharedString>,
    retry_click: Option<RetryHandler>,
}

impl ErrorState {
    /// Creates a new error state with the given title.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            message: None,
            retry_label: None,
            retry_click: None,
        }
    }

    /// Sets the error message/description.
    pub fn message(mut self, msg: impl Into<SharedString>) -> Self {
        self.message = Some(msg.into());
        self
    }

    /// Shows a retry button with the given label.
    pub fn retry(mut self, label: impl Into<SharedString>) -> Self {
        self.retry_label = Some(label.into());
        self
    }

    /// Registers a retry click handler.
    pub fn on_retry(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.retry_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for ErrorState {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        div()
            .v_flex()
            .items_center()
            .justify_center()
            .gap_3()
            .py(px(48.))
            .px(px(16.))
            .child(
                div()
                    .text_color(c.danger)
                    .child(Icon::new(IconName::Error).with_size(px(36.))),
            )
            .child(
                div()
                    .text_size(theme.font_sizes.body)
                    .text_color(c.foreground)
                    .child(self.title.clone()),
            )
            .when_some(self.message, |this, msg| {
                this.child(
                    div()
                        .text_size(theme.font_sizes.caption)
                        .text_color(c.muted_foreground)
                        .text_center()
                        .child(msg),
                )
            })
            .when_some(self.retry_label, |this, label| {
                let handler = self.retry_click;
                this.child(Button::new("error-retry").primary().label(label).on_click(
                    move |_event, window, cx| {
                        if let Some(ref h) = handler {
                            h(window, cx);
                        }
                    },
                ))
            })
    }
}
