use crate::{ActiveTheme, ChartSeries, StyledExt};
use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    Styled as _, Window, div, prelude::FluentBuilder, px,
};

/// An area chart — line chart with a filled region below the line.
///
/// Each series is rendered as colored dots connected by horizontal segments,
/// with a semi-transparent filled area beneath the line.
///
/// # Example
///
/// ```ignore
/// AreaChart::new("traffic")
///     .height(px(200.))
///     .fill_opacity(0.15)
///     .series(vec![
///         ChartSeries::new("Visitors")
///             .color(hsl(0.6, 0.6, 0.5))
///             .data(vec![5., 12., 8., 20., 15.]),
///     ])
///     .show_grid(true)
/// ```
#[derive(IntoElement)]
pub struct AreaChart {
    id: ElementId,
    series: Vec<ChartSeries>,
    height: gpui::Pixels,
    fill_opacity: f32,
    show_grid: bool,
}

impl AreaChart {
    /// Creates a new area chart with the given `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            series: Vec::new(),
            height: px(200.),
            fill_opacity: 0.2,
            show_grid: false,
        }
    }

    /// Sets the data series.
    pub fn series(mut self, series: Vec<ChartSeries>) -> Self {
        self.series = series;
        self
    }

    /// Sets the chart height in pixels.
    pub fn height(mut self, h: gpui::Pixels) -> Self {
        self.height = h;
        self
    }

    /// Sets the fill opacity for the area beneath each series line.
    /// Values range from 0.0 (transparent) to 1.0 (opaque).
    pub fn fill_opacity(mut self, opacity: f32) -> Self {
        self.fill_opacity = opacity;
        self
    }

    /// Toggles horizontal grid lines.
    pub fn show_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }
}

impl RenderOnce for AreaChart {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let h: f32 = self.height.into();
        let fill_alpha = self.fill_opacity;

        // Find global maximum across all visible series (data is Vec<f64>).
        let global_max: f32 = self
            .series
            .iter()
            .filter(|s| s.visible)
            .flat_map(|s| s.data.iter().copied())
            .map(|v| v as f32)
            .reduce(f32::max)
            .unwrap_or(1.0)
            .max(1.0);

        // Max data length for x-axis spacing.
        let max_len = self
            .series
            .iter()
            .map(|s| s.data.len())
            .max()
            .unwrap_or(0)
            .max(2);

        div()
            .id(self.id)
            .v_flex()
            .w_full()
            .h(self.height)
            .relative()
            // Optional grid lines
            .when(self.show_grid, |this| {
                this.children((0..5).map(|i| {
                    let y = h * (1.0 - i as f32 / 4.0);
                    div()
                        .absolute()
                        .left(px(0.))
                        .top(px(y))
                        .w_full()
                        .h(px(1.))
                        .bg(c.muted)
                        .into_any_element()
                }))
            })
            // Render each series
            .children(self.series.into_iter().map(
                move |series| {
                    if !series.visible || series.data.len() < 2 {
                        return div().into_any_element();
                    }

                    let color = series.color;
                    let len = series.data.len();
                    let step_x = h * 0.9 / (len - 1) as f32;
                    let offset_x = h * 0.05;

                    let points: Vec<(f32, f32)> = series
                        .data
                        .iter()
                        .enumerate()
                        .map(|(i, &val)| {
                            let x = offset_x + i as f32 * step_x;
                            let y = h * 0.9 * (1.0 - val as f32 / global_max);
                            (x, y)
                        })
                        .collect();

                    let mut els: Vec<gpui::AnyElement> = Vec::new();

                    // Connecting segments between consecutive points
                    for i in 1..points.len() {
                        let (x1, y1) = points[i - 1];
                        let (x2, y2) = points[i];
                        let mid_y = (y1 + y2) / 2.0;
                        els.push(
                            div()
                                .absolute()
                                .left(px(x1))
                                .top(px(mid_y - 1.))
                                .w(px(x2 - x1))
                                .h(px(2.))
                                .bg(color)
                                .into_any_element(),
                        );
                    }

                    // Filled area — a semi-transparent rectangle covering the plot region
                    els.push(
                        div()
                            .absolute()
                            .left(px(offset_x))
                            .bottom(px(0.))
                            .w(px((max_len - 1) as f32 * step_x))
                            .h(px(h * 0.9))
                            .bg(color.alpha(fill_alpha))
                            .rounded(px(2.))
                            .into_any_element(),
                    );

                    // Data point dots
                    for &(x, y) in &points {
                        els.push(
                            div()
                                .absolute()
                                .left(px(x - 3.))
                                .top(px(y - 3.))
                                .size(px(6.))
                                .rounded_full()
                                .bg(color)
                                .into_any_element(),
                        );
                    }

                    div().children(els).into_any_element()
                },
            ))
            .into_any_element()
    }
}
