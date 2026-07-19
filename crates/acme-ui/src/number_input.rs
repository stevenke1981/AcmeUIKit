use std::rc::Rc;

use gpui::prelude::FluentBuilder as _;
use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    StatefulInteractiveElement as _, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

/// A numeric input with stepper buttons.
///
/// # Example
///
/// ```ignore
/// NumberInput::new("age")
///     .value(42)
///     .on_change(|val, _window, _cx| { })
/// ```
#[allow(clippy::type_complexity)]
#[derive(IntoElement)]
pub struct NumberInput {
    id: ElementId,
    value: i32,
    min: i32,
    max: i32,
    on_change: Option<Rc<dyn Fn(i32, &mut Window, &mut App)>>,
}

impl NumberInput {
    /// Creates a new number input.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: 0,
            min: i32::MIN,
            max: i32::MAX,
            on_change: None,
        }
    }

    /// Sets the value.
    pub fn value(mut self, value: i32) -> Self {
        self.value = value.clamp(self.min, self.max);
        self
    }

    /// Sets the minimum value.
    pub fn min(mut self, min: i32) -> Self {
        self.min = min;
        self.value = self.value.clamp(self.min, self.max);
        self
    }

    /// Sets the maximum value.
    pub fn max(mut self, max: i32) -> Self {
        self.max = max;
        self.value = self.value.clamp(self.min, self.max);
        self
    }

    /// Registers a change handler.
    pub fn on_change(mut self, handler: impl Fn(i32, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for NumberInput {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let handler = self.on_change.clone();
        let can_dec = self.value > self.min;
        let can_inc = self.value < self.max;

        div()
            .id(self.id)
            .h_flex()
            .items_center()
            .h(px(32.))
            .rounded(theme.radius)
            .border_1()
            .border_color(c.border)
            .overflow_hidden()
            .child(
                div()
                    .id(ElementId::Name("num-dec".into()))
                    .h_flex()
                    .items_center()
                    .justify_center()
                    .w(px(28.))
                    .h_full()
                    .bg(if can_dec { c.surface } else { c.muted })
                    .cursor_pointer()
                    .when(can_dec && handler.is_some(), |this| {
                        let h = handler.clone();
                        let val = self.value;
                        this.on_click(move |_event, window, cx| {
                            if let Some(ref h) = h {
                                h(val - 1, window, cx);
                            }
                        })
                    })
                    .child(
                        div()
                            .text_color(if can_dec {
                                c.foreground
                            } else {
                                c.muted_foreground
                            })
                            .child(Icon::new(IconName::Minus).with_size(px(12.))),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .h_flex()
                    .items_center()
                    .justify_center()
                    .text_size(theme.font_sizes.body)
                    .text_color(c.foreground)
                    .child(format!("{}", self.value)),
            )
            .child(
                div()
                    .id(ElementId::Name("num-inc".into()))
                    .h_flex()
                    .items_center()
                    .justify_center()
                    .w(px(28.))
                    .h_full()
                    .bg(if can_inc { c.surface } else { c.muted })
                    .cursor_pointer()
                    .when(can_inc && handler.is_some(), |this| {
                        let val = self.value;
                        this.on_click(move |_event, window, cx| {
                            if let Some(ref h) = handler {
                                h(val + 1, window, cx);
                            }
                        })
                    })
                    .child(
                        div()
                            .text_color(if can_inc {
                                c.foreground
                            } else {
                                c.muted_foreground
                            })
                            .child(Icon::new(IconName::Plus).with_size(px(12.))),
                    ),
            )
    }
}
