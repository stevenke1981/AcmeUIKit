//! Chart tooltip overlay component.

use gpui::{
    App, ElementId, FontWeight, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, Styled as _, Window, div, prelude::FluentBuilder,
};

use crate::{ActiveTheme, StyledExt};

/// A simple chart tooltip overlay component.
///
/// Renders a positioned box with a title and key–value rows.
#[derive(IntoElement)]
pub struct ChartTooltip {
    id: ElementId,
    title: Option<SharedString>,
    rows: Vec<(SharedString, SharedString)>,
    visible: bool,
}

impl ChartTooltip {
    /// Creates a new tooltip with the given `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            title: None,
            rows: Vec::new(),
            visible: false,
        }
    }

    /// Sets the tooltip title.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Adds a data row (label, value).
    pub fn row(mut self, label: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        self.rows.push((label.into(), value.into()));
        self
    }

    /// Sets visibility.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}

impl RenderOnce for ChartTooltip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        if !self.visible {
            return div().id(self.id).into_any_element();
        }

        let theme = cx.theme();
        let c = theme.colors;

        div()
            .id(self.id)
            .absolute()
            .bg(c.background)
            .border_1()
            .border_color(c.border)
            .rounded(theme.radius)
            .shadow_lg()
            .px_3()
            .py_2()
            .child(
                div()
                    .v_flex()
                    .gap_1()
                    .child(div().when_some(self.title, |this, title| {
                        this.child(
                            div()
                                .text_size(theme.font_sizes.caption)
                                .font_weight(FontWeight(600.))
                                .text_color(c.foreground)
                                .mb_1()
                                .child(title),
                        )
                    }))
                    .children(self.rows.into_iter().map(|(label, value)| {
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .text_size(theme.font_sizes.caption)
                                    .text_color(c.muted_foreground)
                                    .child(label),
                            )
                            .child(
                                div()
                                    .text_size(theme.font_sizes.caption)
                                    .text_color(c.foreground)
                                    .child(value),
                            )
                            .into_any_element()
                    })),
            )
            .into_any_element()
    }
}
