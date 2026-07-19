use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

/// A date range picker showing from and to dates.
///
/// # Example
///
/// ```ignore
/// DateRangePicker::new("range")
///     .from("2024-01-01")
///     .to("2024-12-31")
/// ```
#[derive(IntoElement)]
pub struct DateRangePicker {
    id: ElementId,
    from: SharedString,
    to: SharedString,
    placeholder: SharedString,
}

impl DateRangePicker {
    /// Creates a new date range picker.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            from: SharedString::new(""),
            to: SharedString::new(""),
            placeholder: SharedString::from("Select date range…"),
        }
    }

    /// Sets the from date.
    pub fn from(mut self, val: impl Into<SharedString>) -> Self {
        self.from = val.into();
        self
    }

    /// Sets the to date.
    pub fn to(mut self, val: impl Into<SharedString>) -> Self {
        self.to = val.into();
        self
    }
}

impl RenderOnce for DateRangePicker {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let has_from = !self.from.is_empty();
        let has_to = !self.to.is_empty();

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
            .child(Icon::new(IconName::Calendar).with_size(px(14.)))
            .child(
                div()
                    .text_size(theme.font_sizes.body)
                    .text_color(if has_from || has_to {
                        c.foreground
                    } else {
                        c.muted_foreground
                    })
                    .child(if has_from && has_to {
                        SharedString::from(format!("{} – {}", self.from, self.to))
                    } else if has_from {
                        SharedString::from(format!("{} – …", self.from))
                    } else {
                        self.placeholder
                    }),
            )
    }
}
