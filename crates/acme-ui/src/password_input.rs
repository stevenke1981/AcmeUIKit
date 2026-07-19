use std::rc::Rc;

use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StatefulInteractiveElement as _, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

type ToggleHandler = Rc<dyn Fn(&mut Window, &mut App)>;

/// A password input with show/hide toggle.
///
/// # Example
///
/// ```ignore
/// PasswordInput::new("pwd")
///     .placeholder("Enter password")
/// ```
#[derive(IntoElement)]
pub struct PasswordInput {
    id: ElementId,
    placeholder: SharedString,
    value: SharedString,
    visible: bool,
    on_toggle: Option<ToggleHandler>,
}

impl PasswordInput {
    /// Creates a new password input.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            placeholder: SharedString::new(""),
            value: SharedString::new(""),
            visible: false,
            on_toggle: None,
        }
    }

    /// Sets the placeholder text.
    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Sets the current value.
    pub fn value(mut self, val: impl Into<SharedString>) -> Self {
        self.value = val.into();
        self
    }

    /// Sets the initial visibility state.
    pub fn visible(mut self, v: bool) -> Self {
        self.visible = v;
        self
    }

    /// Registers a toggle visibility handler.
    pub fn on_toggle(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for PasswordInput {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        let display = if self.visible {
            self.value.clone()
        } else {
            SharedString::from("*".repeat(self.value.len()))
        };

        let handler = self.on_toggle.clone();

        div()
            .id(self.id)
            .h_flex()
            .items_center()
            .h(px(32.))
            .px(px(10.))
            .rounded(theme.radius)
            .bg(c.muted)
            .gap_2()
            .child(
                div()
                    .flex_1()
                    .text_size(theme.font_sizes.body)
                    .text_color(if display.is_empty() {
                        c.muted_foreground
                    } else {
                        c.foreground
                    })
                    .child(if display.is_empty() {
                        self.placeholder
                    } else {
                        display
                    }),
            )
            .child(
                div()
                    .id("pwd-toggle")
                    .cursor_pointer()
                    .text_color(c.muted_foreground)
                    .hover(|style| style.text_color(c.foreground))
                    .on_click(move |_event, window, cx| {
                        if let Some(ref h) = handler {
                            h(window, cx);
                        }
                    })
                    .child(if self.visible {
                        Icon::new(IconName::Eye)
                    } else {
                        Icon::new(IconName::EyeOff)
                    }),
            )
    }
}
