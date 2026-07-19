use crate::{ActiveTheme, StyledExt};
use gpui::{
    App, ElementId, FontWeight, Hsla, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, Styled as _, Window, div, prelude::FluentBuilder, px,
};

/// A streaming chart showing recent data points in a scrolling viewport.
///
/// Renders data as dots connected by horizontal segments, simulating a
/// right-to-left scrolling line chart. The latest value can be displayed
/// as an overlay label.
///
/// Since this is a `RenderOnce` (stateless) component, it renders a
/// snapshot of the current data. For live updates the parent entity
/// must re-render with new data.
///
/// # Example
///
/// ```ignore
/// StreamingChart::new("cpu")
///     .data(vec![10.0, 45.0, 30.0, 70.0, 55.0, 90.0])
///     .height(px(120.))
///     .color(hsl(220., 80., 55.))
///     .show_latest_value(true)
/// ```
#[derive(IntoElement)]
pub struct StreamingChart {
    id: ElementId,
    data: Vec<f32>,
    height: gpui::Pixels,
    max_points: usize,
    color: Option<Hsla>,
    show_latest_value: bool,
}

impl StreamingChart {
    /// Creates a new streaming chart with the given `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            data: Vec::new(),
            height: px(100.),
            max_points: 50,
            color: None,
            show_latest_value: true,
        }
    }

    /// Replaces the data points. The chart shows only the most recent
    /// `max_points` values, scrolling older points off the left edge.
    pub fn data(mut self, data: Vec<f32>) -> Self {
        self.data = data;
        self
    }

    /// Sets the overall chart height in pixels.
    pub fn height(mut self, height: gpui::Pixels) -> Self {
        self.height = height;
        self
    }

    /// Sets the maximum number of visible points. Older points are
    /// discarded from the left. Defaults to 50.
    pub fn max_points(mut self, max: usize) -> Self {
        self.max_points = max;
        self
    }

    /// Sets the line and dot color. When `None`, the theme primary
    /// colour is used.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    /// When `true`, overlays the most recent data value in the top-right
    /// corner. Defaults to `true`.
    pub fn show_latest_value(mut self, show: bool) -> Self {
        self.show_latest_value = show;
        self
    }
}

impl RenderOnce for StreamingChart {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let stroke = self.color.unwrap_or(c.primary);

        let max_val = self
            .data
            .iter()
            .cloned()
            .reduce(f32::max)
            .unwrap_or(1.0)
            .max(1.0);

        let h_f32: f32 = self.height.into();
        let plot_w = h_f32 * 0.95;
        let plot_h = h_f32 * 0.85;
        let pad_left = 5.0;
        let pad_top = 5.0;

        // Take the latest max_points values, preserving chronological order
        let points: Vec<f32> = self
            .data
            .iter()
            .rev()
            .take(self.max_points)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();

        let count = points.len().max(2);
        let step_x = plot_w / (count - 1) as f32;

        div()
            .id(self.id)
            .v_flex()
            .w_full()
            .h(self.height)
            .relative()
            .overflow_hidden()
            .child(
                div()
                    .absolute()
                    .left(px(pad_left))
                    .top(px(pad_top))
                    .w(px(plot_w))
                    .h(px(plot_h))
                    .relative()
                    // Data point dots
                    .children(points.iter().enumerate().map(|(i, &val)| {
                        let x = i as f32 * step_x;
                        let y = plot_h * (1.0 - val / max_val);

                        div()
                            .absolute()
                            .left(px(x - 2.))
                            .top(px(y - 2.))
                            .size(px(4.))
                            .rounded_full()
                            .bg(stroke)
                            .into_any_element()
                    }))
                    // Horizontal connecting segments between consecutive points
                    .children((1..points.len()).map(|i| {
                        let x1 = (i - 1) as f32 * step_x;
                        let y1 = plot_h * (1.0 - points[i - 1] / max_val);
                        let x2 = i as f32 * step_x;
                        let y2 = plot_h * (1.0 - points[i] / max_val);
                        let mid_y = (y1 + y2) / 2.0;
                        let dx = x2 - x1;

                        div()
                            .absolute()
                            .left(px(x1))
                            .top(px(mid_y - 1.))
                            .w(px(dx))
                            .h(px(2.))
                            .bg(stroke)
                            .into_any_element()
                    })),
            )
            // Latest value overlay
            .when(self.show_latest_value && !points.is_empty(), |this| {
                let last_val = points[points.len() - 1];

                this.child(
                    div()
                        .absolute()
                        .right(px(4.))
                        .top(px(2.))
                        .text_size(theme.font_sizes.caption)
                        .font_weight(FontWeight(600.))
                        .text_color(stroke)
                        .child(format!("{:.1}", last_val)),
                )
            })
            .into_any_element()
    }
}
