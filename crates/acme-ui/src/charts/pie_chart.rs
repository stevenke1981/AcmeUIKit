use gpui::{
    App, ElementId, Hsla, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, prelude::FluentBuilder, px,
};

use crate::{ActiveTheme, ChartColors, StyledExt};

/// A single slice in a pie chart.
#[derive(Clone)]
pub struct PieSlice {
    label: SharedString,
    value: f32,
    color: Option<Hsla>,
}

impl PieSlice {
    /// Creates a new pie slice with a label and numeric value.
    pub fn new(label: impl Into<SharedString>, value: f32) -> Self {
        Self {
            label: label.into(),
            value,
            color: None,
        }
    }

    /// Sets a custom color for this slice.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

/// Pie chart showing proportional data.
///
/// Renders a circle with colored proportional segments and a legend
/// showing labels, values, and percentages.
///
/// # Example
///
/// ```ignore
/// PieChart::new("sales")
///     .size(px(150.))
///     .slices(vec![
///         PieSlice::new("Product A", 300.0),
///         PieSlice::new("Product B", 200.0).color(hsl(120., 60., 50.)),
///         PieSlice::new("Product C", 100.0),
///     ])
/// ```
#[derive(IntoElement)]
pub struct PieChart {
    id: ElementId,
    slices: Vec<PieSlice>,
    size: gpui::Pixels,
}

impl PieChart {
    /// Creates a new pie chart with the given `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            slices: Vec::new(),
            size: px(150.),
        }
    }

    /// Replaces the slice data.
    pub fn slices(mut self, slices: Vec<PieSlice>) -> Self {
        self.slices = slices;
        self
    }

    /// Sets the diameter of the pie chart in pixels.
    pub fn size(mut self, size: gpui::Pixels) -> Self {
        self.size = size;
        self
    }
}

impl RenderOnce for PieChart {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let total: f32 = self.slices.iter().map(|s| s.value).sum();
        let total = total.max(1.0);
        let circle_size: f32 = self.size.into();

        div()
            .id(self.id)
            .v_flex()
            .items_center()
            .gap_2()
            // Circle with colored proportional segments as stacked bars
            .child(
                div()
                    .size(px(circle_size))
                    .rounded_full()
                    .overflow_hidden()
                    .flex()
                    .flex_row()
                    .children(self.slices.iter().map(|slice| {
                        let ratio = slice.value / total;
                        let width_px = px((circle_size * ratio).max(2.0));
                        let color = slice.color.unwrap_or_else(|| ChartColors::get(0));
                        div()
                            .w(width_px)
                            .h(px(circle_size))
                            .bg(color)
                            .into_any_element()
                    })),
            )
            // Legend with labels, values, and percentages
            .child(
                div()
                    .v_flex()
                    .gap_1()
                    .children(self.slices.into_iter().enumerate().map(
                        |(i, slice)| {
                            let ratio = slice.value / total;
                            let color =
                                slice.color.unwrap_or_else(|| ChartColors::get(i));
                            div()
                                .h_flex()
                                .gap_2()
                                .child(
                                    div()
                                        .size(px(10.))
                                        .rounded(px(2.))
                                        .bg(color)
                                        .flex_none(),
                                )
                                .child(
                                    div()
                                        .text_size(theme.font_sizes.caption)
                                        .text_color(c.foreground)
                                        .child(slice.label),
                                )
                                .child(
                                    div()
                                        .text_size(theme.font_sizes.caption)
                                        .text_color(c.muted_foreground)
                                        .child(format!("{:.0}", slice.value)),
                                )
                                .child(
                                    div()
                                        .text_size(theme.font_sizes.caption)
                                        .text_color(c.muted_foreground)
                                        .child(format!("({:.0}%)", ratio * 100.)),
                                )
                                .into_any_element()
                        },
                    )),
            )
            .into_any_element()
    }
}

/// Donut chart — a pie chart with a hole in the center.
///
/// Renders the same proportional segments as [`PieChart`] but with a
/// configurable inner hole that can display an optional centered label.
///
/// # Example
///
/// ```ignore
/// DonutChart::new("traffic")
///     .size(px(160.))
///     .hole_ratio(0.55)
///     .center_text("Total")
///     .slices(vec![
///         PieSlice::new("Direct", 400.0),
///         PieSlice::new("Referral", 300.0),
///     ])
/// ```
#[derive(IntoElement)]
pub struct DonutChart {
    id: ElementId,
    slices: Vec<PieSlice>,
    size: gpui::Pixels,
    hole_ratio: f32,
    center_text: Option<SharedString>,
}

impl DonutChart {
    /// Creates a new donut chart with the given `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            slices: Vec::new(),
            size: px(150.),
            hole_ratio: 0.6,
            center_text: None,
        }
    }

    /// Replaces the slice data.
    pub fn slices(mut self, slices: Vec<PieSlice>) -> Self {
        self.slices = slices;
        self
    }

    /// Sets the outer diameter in pixels.
    pub fn size(mut self, size: gpui::Pixels) -> Self {
        self.size = size;
        self
    }

    /// Sets the hole-to-outer ratio (clamped to 0.2–0.8).
    pub fn hole_ratio(mut self, ratio: f32) -> Self {
        self.hole_ratio = ratio.clamp(0.2, 0.8);
        self
    }

    /// Sets optional centered text inside the hole.
    pub fn center_text(mut self, text: impl Into<SharedString>) -> Self {
        self.center_text = Some(text.into());
        self
    }
}

impl RenderOnce for DonutChart {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let total: f32 = self.slices.iter().map(|s| s.value).sum();
        let total = total.max(1.0);
        let outer: f32 = self.size.into();
        let hole_f32 = outer * self.hole_ratio;
        let hole = px(hole_f32);
        let offset = px((outer - hole_f32) / 2.);

        div()
            .id(self.id)
            .v_flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .relative()
                    .size(px(outer))
                    // Outer ring (colored segments)
                    .child(
                        div()
                            .size(px(outer))
                            .rounded_full()
                            .overflow_hidden()
                            .flex()
                            .flex_row()
                            .children(self.slices.iter().map(|slice| {
                                let ratio = slice.value / total;
                                let w = px((outer * ratio).max(2.0));
                                let color =
                                    slice.color.unwrap_or_else(|| ChartColors::get(0));
                                div()
                                    .w(w)
                                    .h(px(outer))
                                    .bg(color)
                                    .into_any_element()
                            })),
                    )
                    // Hole (inner circle)
                    .child(
                        div()
                            .absolute()
                            .top(offset)
                            .left(offset)
                            .size(hole)
                            .rounded_full()
                            .bg(c.background)
                            .flex()
                            .items_center()
                            .justify_center()
                            .when_some(self.center_text, |this, text| {
                                this.child(
                                    div()
                                        .text_size(theme.font_sizes.caption)
                                        .text_color(c.foreground)
                                        .child(text),
                                )
                            }),
                    ),
            )
            .child(
                div()
                    .v_flex()
                    .gap_1()
                    .children(self.slices.into_iter().enumerate().map(|(i, slice)| {
                        let ratio = slice.value / total;
                        let color = slice.color.unwrap_or_else(|| ChartColors::get(i));
                        div()
                            .h_flex()
                            .gap_2()
                            .child(div().size(px(10.)).rounded(px(2.)).bg(color).flex_none())
                            .child(
                                div()
                                    .text_size(theme.font_sizes.caption)
                                    .text_color(c.foreground)
                                    .child(slice.label),
                            )
                            .child(
                                div()
                                    .text_size(theme.font_sizes.caption)
                                    .text_color(c.muted_foreground)
                                    .child(format!("{:.0}", slice.value)),
                            )
                            .child(
                                div()
                                    .text_size(theme.font_sizes.caption)
                                    .text_color(c.muted_foreground)
                                    .child(format!("({:.0}%)", ratio * 100.)),
                            )
                            .into_any_element()
                    })),
            )
            .into_any_element()
    }
}
