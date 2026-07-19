use crate::{ActiveTheme, ChartColors, StyledExt};
use gpui::{
    App, ElementId, Hsla, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, prelude::FluentBuilder, px,
};

/// A single data point for scatter charts.
#[derive(Clone)]
pub struct ScatterPoint {
    /// X value (data coordinate).
    pub x: f32,
    /// Y value (data coordinate).
    pub y: f32,
    /// Optional label for tooltips or annotations.
    pub label: Option<SharedString>,
}

impl ScatterPoint {
    /// Creates a new scatter point at the given data coordinates.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, label: None }
    }

    /// Sets an optional label for this point.
    pub fn label(mut self, l: impl Into<SharedString>) -> Self {
        self.label = Some(l.into());
        self
    }
}

/// A named series of scatter points with an associated color.
pub struct ScatterSeries {
    /// Display name.
    pub name: SharedString,
    /// Point color.
    pub color: Hsla,
    /// Data points.
    pub points: Vec<ScatterPoint>,
}

impl ScatterSeries {
    /// Creates a new scatter series with the given name and a default palette color.
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            color: ChartColors::get(0),
            points: Vec::new(),
        }
    }

    /// Sets the series color.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = color;
        self
    }

    /// Sets the data points for this series.
    pub fn points(mut self, points: Vec<ScatterPoint>) -> Self {
        self.points = points;
        self
    }
}

/// A scatter plot component.
///
/// Renders points from one or more series on a grid, with optional
/// horizontal and vertical grid lines.
///
/// # Example
///
/// ```ignore
/// ScatterChart::new("correlation")
///     .height(px(200.))
///     .show_grid(true)
///     .series(vec![
///         ScatterSeries::new("Group A")
///             .color(hsl(0.0, 0.6, 0.5))
///             .points(vec![
///                 ScatterPoint::new(10., 20.),
///                 ScatterPoint::new(30., 50.),
///                 ScatterPoint::new(50., 30.),
///             ]),
///     ])
/// ```
#[derive(IntoElement)]
pub struct ScatterChart {
    id: ElementId,
    series: Vec<ScatterSeries>,
    height: gpui::Pixels,
    show_grid: bool,
}

impl ScatterChart {
    /// Creates a new scatter chart with the given `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            series: Vec::new(),
            height: px(200.),
            show_grid: true,
        }
    }

    /// Sets the data series.
    pub fn series(mut self, s: Vec<ScatterSeries>) -> Self {
        self.series = s;
        self
    }

    /// Sets the chart height in pixels.
    pub fn height(mut self, h: gpui::Pixels) -> Self {
        self.height = h;
        self
    }

    /// Toggles horizontal and vertical grid lines.
    pub fn show_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }
}

impl RenderOnce for ScatterChart {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let h: f32 = self.height.into();

        // Find data bounds across all series
        let mut max_x = 100.0f32;
        let mut max_y = 100.0f32;
        for s in &self.series {
            for p in &s.points {
                if p.x > max_x {
                    max_x = p.x;
                }
                if p.y > max_y {
                    max_y = p.y;
                }
            }
        }
        max_x = max_x.max(1.0);
        max_y = max_y.max(1.0);

        let pad = 20.0f32;
        let plot_w = h - pad * 2.0;
        let plot_h = h - pad * 2.0;

        div()
            .id(self.id)
            .v_flex()
            .w_full()
            .h(self.height)
            .relative()
            // Grid lines (both horizontal and vertical)
            .when(self.show_grid, |this| {
                this.children((0..5).flat_map(|i| {
                    let mut els = Vec::new();
                    let t = i as f32 / 4.0;
                    let y = pad + plot_h * (1.0 - t);
                    let x = pad + plot_w * t;
                    els.push(
                        div()
                            .absolute()
                            .left(px(0.))
                            .top(px(y))
                            .w_full()
                            .h(px(1.))
                            .bg(c.muted)
                            .into_any_element(),
                    );
                    els.push(
                        div()
                            .absolute()
                            .top(px(0.))
                            .left(px(x))
                            .w(px(1.))
                            .h_full()
                            .bg(c.muted)
                            .into_any_element(),
                    );
                    els
                }))
            })
            // Data points grouped by series
            .children(self.series.into_iter().map(
                |series| {
                    let color = series.color;
                    div()
                        .children(series.points.into_iter().map(|pt| {
                            let px_x = pad + (pt.x / max_x) * plot_w;
                            let px_y = pad + plot_h * (1.0 - pt.y / max_y);
                            div()
                                .absolute()
                                .left(px(px_x - 4.))
                                .top(px(px_y - 4.))
                                .size(px(8.))
                                .rounded_full()
                                .bg(color)
                                .border_2()
                                .border_color(c.background)
                                .into_any_element()
                        }))
                        .into_any_element()
                },
            ))
            .into_any_element()
    }
}
