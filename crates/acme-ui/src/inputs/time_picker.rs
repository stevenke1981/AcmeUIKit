use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

/// A time display with clock icon.
///
/// # Example
///
/// ```ignore
/// TimePicker::new("time")
///     .value("14:30")
/// ```
#[derive(IntoElement)]
pub struct TimePicker {
    id: ElementId,
    value: SharedString,
    placeholder: SharedString,
}

impl TimePicker {
    /// Creates a new time picker.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: SharedString::new(""),
            placeholder: SharedString::from("Select time…"),
        }
    }

    /// Sets the current time value.
    pub fn value(mut self, val: impl Into<SharedString>) -> Self {
        self.value = val.into();
        self
    }

    /// Sets the placeholder text.
    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = text.into();
        self
    }
}

impl RenderOnce for TimePicker {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let is_empty = self.value.is_empty();

        div()
            .id(self.id)
            .h_flex()
            .items_center()
            .gap_2()
            .h(px(32.))
            .px(px(10.))
            .rounded(theme.radius)
            .bg(c.muted)
            .cursor_pointer()
            .child(Icon::new(IconName::Clock).with_size(px(14.)))
            .child(
                div()
                    .text_size(theme.font_sizes.body)
                    .text_color(if is_empty {
                        c.muted_foreground
                    } else {
                        c.foreground
                    })
                    .child(if is_empty {
                        self.placeholder
                    } else {
                        self.value
                    }),
            )
    }
}
