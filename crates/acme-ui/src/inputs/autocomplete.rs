use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// An autocomplete input showing filtered suggestions.
///
/// # Example
///
/// ```ignore
/// Autocomplete::new("city")
///     .suggestions(&["New York", "London", "Tokyo"])
///     .value("New")
/// ```
#[derive(IntoElement)]
pub struct Autocomplete {
    id: ElementId,
    value: SharedString,
    suggestions: Vec<SharedString>,
    placeholder: SharedString,
}

impl Autocomplete {
    /// Creates a new autocomplete.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: SharedString::new(""),
            suggestions: Vec::new(),
            placeholder: SharedString::from("Type to search…"),
        }
    }

    /// Sets the current input value.
    pub fn value(mut self, v: impl Into<SharedString>) -> Self {
        self.value = v.into();
        self
    }

    /// Sets the suggestion list.
    pub fn suggestions(mut self, items: &[&str]) -> Self {
        self.suggestions = items.iter().map(|s| SharedString::from(*s)).collect();
        self
    }

    /// Sets the placeholder text.
    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = text.into();
        self
    }
}

impl RenderOnce for Autocomplete {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        let filtered: Vec<&SharedString> = if self.value.is_empty() {
            self.suggestions.iter().collect()
        } else {
            let lower = self.value.to_lowercase();
            self.suggestions
                .iter()
                .filter(|s| s.to_lowercase().contains(&lower))
                .collect()
        };

        div()
            .id(self.id)
            .v_flex()
            .gap_1()
            .child(
                div()
                    .h_flex()
                    .items_center()
                    .h(px(32.))
                    .px(px(10.))
                    .rounded(theme.radius)
                    .bg(c.muted)
                    .child(
                        div()
                            .text_size(theme.font_sizes.body)
                            .text_color(if self.value.is_empty() {
                                c.muted_foreground
                            } else {
                                c.foreground
                            })
                            .child(if self.value.is_empty() {
                                self.placeholder
                            } else {
                                self.value
                            }),
                    ),
            )
            .child(
                div()
                    .v_flex()
                    .rounded(theme.radius)
                    .bg(c.background)
                    .border_1()
                    .border_color(c.border)
                    .overflow_hidden()
                    .children(filtered.into_iter().map(|s| {
                        div()
                            .h_flex()
                            .items_center()
                            .h(px(28.))
                            .px(px(8.))
                            .text_size(theme.font_sizes.body)
                            .text_color(c.foreground)
                            .child(s.clone())
                    })),
            )
    }
}
