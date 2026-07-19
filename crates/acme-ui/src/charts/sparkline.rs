use gpui::{
    App, ElementId, Hsla, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    Styled as _, Window, div, px,
};

use crate::{ChartColors, StyledExt};

/// A small inline sparkline chart showing a data trend as columns.
///
/// Renders a compact row of proportional-height bars. Useful for embedding
/// trend visualisations inside tables, cards, or labels.
///
/// # Example
///
/// ```ignore
/// Sparkline::new("revenue-trend")
///     .data(vec![10.0, 45.0, 30.0, 70.0, 55.0, 90.0])
///     .height(px(32.))
///     .color(hsl(220., 80., 55.))
/// ```
#[derive(IntoElement)]
pub struct Sparkline {
    id: ElementId,
    data: Vec<f32>,
    color: Option<Hsla>,
    height: gpui::Pixels,
    max_value: Option<f32>,
}

impl Sparkline {
    /// Creates a new sparkline with the given `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            data: Vec::new(),
            color: None,
            height: px(32.),
            max_value: None,
        }
    }

    /// Replaces the data points.
    pub fn data(mut self, data: Vec<f32>) -> Self {
        self.data = data;
        self
    }

    /// Sets a custom bar color. When `None`, the theme primary color is used.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the overall height of the sparkline in pixels.
    pub fn height(mut self, height: gpui::Pixels) -> Self {
        self.height = height;
        self
    }

    /// Sets a fixed maximum value for the Y axis. When `None`, the maximum
    /// is derived from the data.
    pub fn max_value(mut self, max: f32) -> Self {
        self.max_value = Some(max);
        self
    }
}

impl RenderOnce for Sparkline {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let stroke = self.color.unwrap_or_else(|| ChartColors::get(0));

        let max_val = self
            .max_value
            .or_else(|| self.data.iter().cloned().reduce(f32::max))
            .unwrap_or(1.0)
            .max(0.001);

        let count = self.data.len().max(2);
        let bar_width = px((200.0 / count as f32).max(3.0));
        let height_f32: f32 = self.height.into();

        div()
            .id(self.id)
            .h_flex()
            .items_end()
            .gap(px(1.))
            .h(self.height)
            .children(self.data.into_iter().map(|val| {
                let ratio = (val / max_val).clamp(0.0, 1.0);
                let bar_h = px((height_f32 * ratio).max(2.0));
                div()
                    .w(bar_width)
                    .h(bar_h)
                    .bg(stroke)
                    .rounded(px(1.))
                    .into_any_element()
            }))
            .into_any_element()
    }
}
