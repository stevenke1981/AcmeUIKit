use crate::{ActiveTheme, ChartSeries, StyledExt};
use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    Styled as _, Window, div, prelude::FluentBuilder, px,
};

/// A simple line chart component with one or more data series.
///
/// Each series is rendered as colored dots connected by horizontal segments
/// approximating a line, with colors from the chart palette.
///
/// # Example
///
/// ```ignore
/// LineChart::new("sales")
///     .height(px(200.))
///     .series(vec![
///         ChartSeries::new("Revenue")
///             .color(hsl(0., 60., 50.))
///             .data(vec![10., 25., 18., 40., 35.]),
///     ])
///     .show_dots(true)
///     .show_grid(true)
/// ```
#[derive(IntoElement)]
pub struct LineChart {
    id: ElementId,
    series: Vec<ChartSeries>,
    height: gpui::Pixels,
    show_dots: bool,
    show_grid: bool,
}

impl LineChart {
    /// Creates a new line chart with the given `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            series: Vec::new(),
            height: px(200.),
            show_dots: true,
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

    /// Toggles dot rendering at each data point.
    pub fn show_dots(mut self, show: bool) -> Self {
        self.show_dots = show;
        self
    }

    /// Toggles horizontal grid lines.
    pub fn show_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }
}

impl RenderOnce for LineChart {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let h: f32 = self.height.into();

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

                    let mut children: Vec<gpui::AnyElement> = Vec::new();

                    // Connecting segments between consecutive points
                    for i in 1..points.len() {
                        let (x1, y1) = points[i - 1];
                        let (x2, y2) = points[i];
                        let dx = x2 - x1;
                        let dy = y2 - y1;
                        let seg_len = ((dx * dx + dy * dy) as f64).sqrt() as f32;

                        if seg_len > 2.0 {
                            let mid_y = (y1 + y2) / 2.0;
                            children.push(
                                div()
                                    .absolute()
                                    .left(px(x1))
                                    .top(px(mid_y - 1.0))
                                    .w(px(seg_len))
                                    .h(px(2.))
                                    .bg(color)
                                    .into_any_element(),
                            );
                        }
                    }

                    // Data point dots
                    if self.show_dots {
                        for &(x, y) in &points {
                            children.push(
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
                    }

                    div().children(children).into_any_element()
                },
            ))
            .into_any_element()
    }
}
