use gpui::{
    App, ElementId, FontWeight, Hsla, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, Styled as _, Window, div, prelude::FluentBuilder, px,
};

use crate::{ActiveTheme, StyledExt};

// ── Scale ──

/// Scale type for mapping data to screen coordinates.
#[derive(Clone, Debug, PartialEq)]
pub enum ScaleType {
    /// Linear scale: maps domain → range linearly.
    Linear,
    /// Band scale: categorical data mapped to evenly-spaced bands.
    Band,
}

/// A scale that maps data values to pixel coordinates.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Scale {
    scale_type: ScaleType,
    domain_min: f64,
    domain_max: f64,
    range_min: f64,
    range_max: f64,
    band_count: usize,
}

impl Default for Scale {
    fn default() -> Self {
        Self {
            scale_type: ScaleType::Linear,
            domain_min: 0.0,
            domain_max: 100.0,
            range_min: 0.0,
            range_max: 1.0,
            band_count: 0,
        }
    }
}

impl Scale {
    /// Creates a new linear scale.
    pub fn linear() -> Self {
        Self::default()
    }

    /// Creates a new band scale for categorical data with `count` categories.
    pub fn band(count: usize) -> Self {
        Self {
            scale_type: ScaleType::Band,
            domain_min: 0.0,
            domain_max: count as f64,
            range_min: 0.0,
            range_max: 1.0,
            band_count: count,
        }
    }

    /// Sets the data domain (min…max values).
    pub fn domain(mut self, min: f64, max: f64) -> Self {
        self.domain_min = min;
        self.domain_max = max;
        self
    }

    /// Sets the output pixel range (start…end).
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.range_min = min;
        self.range_max = max;
        self
    }

    /// Maps a data value to a pixel position.
    pub fn map(&self, value: f64) -> f64 {
        let denom = if self.domain_max == self.domain_min {
            1.0
        } else {
            self.domain_max - self.domain_min
        };
        let t = (value - self.domain_min) / denom;
        self.range_min + t * (self.range_max - self.range_min)
    }

    /// Inverts a pixel position back to a data value.
    pub fn invert(&self, pixel: f64) -> f64 {
        let denom = if self.range_max == self.range_min {
            1.0
        } else {
            self.range_max - self.range_min
        };
        let t = (pixel - self.range_min) / denom;
        self.domain_min + t * (self.domain_max - self.domain_min)
    }

    /// Returns the width of one band for band scales.
    pub fn band_width(&self) -> f64 {
        if self.band_count == 0 {
            return 0.0;
        }
        (self.range_max - self.range_min) / self.band_count as f64
    }

    /// Returns evenly-spaced tick values.
    pub fn ticks(&self, count: usize) -> Vec<f64> {
        let count = count.max(2);
        let step = (self.domain_max - self.domain_min) / (count - 1) as f64;
        (0..count)
            .map(|i| self.domain_min + i as f64 * step)
            .collect()
    }
}

// ── Chart palette ──

/// Provides a default qualitative color palette for chart series.
pub struct ChartColors;

impl ChartColors {
    /// Returns the color at `index` from a 10-color qualitative palette.
    pub fn get(index: usize) -> Hsla {
        // ColorBrewer Set1 palette (trimmed to 10)
        let palette: [Hsla; 10] = [
            Hsla {
                h: 0.0,
                s: 0.6,
                l: 0.5,
                a: 1.0,
            }, // red
            Hsla {
                h: 0.6,
                s: 0.6,
                l: 0.5,
                a: 1.0,
            }, // blue
            Hsla {
                h: 0.3,
                s: 0.6,
                l: 0.5,
                a: 1.0,
            }, // green
            Hsla {
                h: 0.07,
                s: 0.7,
                l: 0.55,
                a: 1.0,
            }, // orange
            Hsla {
                h: 0.8,
                s: 0.5,
                l: 0.55,
                a: 1.0,
            }, // purple
            Hsla {
                h: 0.55,
                s: 0.6,
                l: 0.5,
                a: 1.0,
            }, // teal
            Hsla {
                h: 0.15,
                s: 0.6,
                l: 0.5,
                a: 1.0,
            }, // brown
            Hsla {
                h: 0.9,
                s: 0.5,
                l: 0.5,
                a: 1.0,
            }, // pink
            Hsla {
                h: 0.2,
                s: 0.5,
                l: 0.45,
                a: 1.0,
            }, // olive
            Hsla {
                h: 0.5,
                s: 0.4,
                l: 0.5,
                a: 1.0,
            }, // slate
        ];
        palette[index % palette.len()]
    }
}

// ── ChartSeries ──

/// A named data series with color and visibility.
#[derive(Clone)]
pub struct ChartSeries {
    /// Display name.
    pub name: SharedString,
    /// Series color.
    pub color: Hsla,
    /// Whether the series is visible.
    pub visible: bool,
    /// Numeric data points.
    pub data: Vec<f64>,
}

impl ChartSeries {
    /// Creates a new series with the given name.
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            color: Hsla::default(),
            visible: true,
            data: Vec::new(),
        }
    }

    /// Sets the series color.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = color;
        self
    }

    /// Sets the series data.
    pub fn data(mut self, data: Vec<f64>) -> Self {
        self.data = data;
        self
    }

    /// Sets visibility.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}

// ── Legend ──

/// A single legend entry.
#[derive(Clone)]
pub struct LegendItem {
    /// Display name.
    pub name: SharedString,
    /// Color swatch.
    pub color: Hsla,
    /// Whether the series is visible.
    pub visible: bool,
}

impl LegendItem {
    /// Creates a new legend item.
    pub fn new(name: impl Into<SharedString>, color: Hsla) -> Self {
        Self {
            name: name.into(),
            color,
            visible: true,
        }
    }
}

/// Legend layout direction.
#[derive(Clone, Debug, PartialEq)]
pub enum LegendLayout {
    /// Items arranged horizontally.
    Horizontal,
    /// Items arranged vertically.
    Vertical,
}

/// A chart legend component.
///
/// Renders colored swatches with labels for each data series.
#[derive(IntoElement)]
pub struct Legend {
    id: ElementId,
    items: Vec<LegendItem>,
    layout: LegendLayout,
}

impl Legend {
    /// Creates a new legend with the given `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            layout: LegendLayout::Horizontal,
        }
    }

    /// Adds a single legend item.
    pub fn item(mut self, item: LegendItem) -> Self {
        self.items.push(item);
        self
    }

    /// Replaces all legend items.
    pub fn items(mut self, items: Vec<LegendItem>) -> Self {
        self.items = items;
        self
    }

    /// Sets the layout direction.
    pub fn layout(mut self, layout: LegendLayout) -> Self {
        self.layout = layout;
        self
    }
}

impl RenderOnce for Legend {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let is_horizontal = self.layout == LegendLayout::Horizontal;
        let items: Vec<_> = self
            .items
            .into_iter()
            .map(|item| {
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(
                        div()
                            .size(px(10.))
                            .rounded(px(2.))
                            .bg(item.color)
                            .flex_none(),
                    )
                    .child(
                        div()
                            .text_size(theme.font_sizes.caption)
                            .text_color(if item.visible {
                                theme.colors.foreground
                            } else {
                                theme.colors.muted_foreground
                            })
                            .child(item.name),
                    )
                    .into_any_element()
            })
            .collect();

        if is_horizontal {
            div()
                .id(self.id)
                .flex()
                .flex_wrap()
                .gap_3()
                .children(items)
                .into_any_element()
        } else {
            div()
                .id(self.id)
                .v_flex()
                .gap_2()
                .children(items)
                .into_any_element()
        }
    }
}

// ── ChartTooltip ──

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

// ── Crosshair ──

/// A crosshair overlay for chart interaction feedback.
#[derive(IntoElement)]
pub struct Crosshair {
    id: ElementId,
    x: Option<f32>,
    y: Option<f32>,
    color: Hsla,
    size: gpui::Pixels,
}

impl Crosshair {
    /// Creates a new crosshair with the given `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            x: None,
            y: None,
            color: Hsla {
                h: 0.0,
                s: 0.0,
                l: 0.5,
                a: 0.5,
            },
            size: px(8.),
        }
    }

    /// Sets the crosshair position in pixels.
    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }

    /// Sets the crosshair color.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = color;
        self
    }
}

impl RenderOnce for Crosshair {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let size_f32: f32 = self.size.into();

        if let (Some(x), Some(y)) = (self.x, self.y) {
            div()
                .id(self.id)
                .absolute()
                .left(px(x - 0.5))
                .top(px(0.))
                .w(px(1.))
                .h_full()
                .bg(self.color)
                .child(
                    div()
                        .absolute()
                        .left(px(-size_f32 / 2. + 0.5))
                        .top(px(y - size_f32 / 2.))
                        .size(self.size)
                        .rounded_full()
                        .border_2()
                        .border_color(self.color)
                        .bg(cx.theme().colors.background.alpha(0.5)),
                )
                .into_any_element()
        } else {
            div().id(self.id).into_any_element()
        }
    }
}

// ── Axis ──

/// Axis orientation.
#[derive(Clone, Debug, PartialEq)]
pub enum AxisOrientation {
    /// X-axis at the bottom.
    Bottom,
    /// Y-axis on the left.
    Left,
    /// X-axis at the top.
    Top,
    /// Y-axis on the right.
    Right,
}

/// A chart axis component with optional tick labels and gridlines.
#[derive(IntoElement)]
pub struct Axis {
    id: ElementId,
    scale: Scale,
    orientation: AxisOrientation,
    tick_count: usize,
    show_gridlines: bool,
    label: Option<SharedString>,
    /// Optional format function for tick labels.
    format_fn: Option<fn(f64) -> SharedString>,
}

impl Axis {
    /// Creates a new axis with the given `id` and `scale`.
    pub fn new(id: impl Into<ElementId>, scale: Scale) -> Self {
        Self {
            id: id.into(),
            scale,
            orientation: AxisOrientation::Bottom,
            tick_count: 5,
            show_gridlines: false,
            label: None,
            format_fn: None,
        }
    }

    /// Sets the axis orientation.
    pub fn orientation(mut self, orientation: AxisOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the number of tick marks.
    pub fn tick_count(mut self, count: usize) -> Self {
        self.tick_count = count;
        self
    }

    /// Toggles gridline rendering.
    pub fn show_gridlines(mut self, show: bool) -> Self {
        self.show_gridlines = show;
        self
    }

    /// Sets the axis label.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets a custom format function for tick labels.
    pub fn format(mut self, format_fn: fn(f64) -> SharedString) -> Self {
        self.format_fn = Some(format_fn);
        self
    }
}

impl RenderOnce for Axis {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let ticks = self.scale.ticks(self.tick_count);

        let is_horizontal = matches!(
            self.orientation,
            AxisOrientation::Bottom | AxisOrientation::Top
        );

        let tick_els: Vec<_> = ticks
            .iter()
            .map(|&val| {
                let pos = self.scale.map(val);
                let label = match self.format_fn {
                    Some(fmt) => fmt(val),
                    None => format_float(val).into(),
                };

                if is_horizontal {
                    div()
                        .absolute()
                        .left(px(pos as f32 - 20.))
                        .w(px(40.))
                        .text_center()
                        .child(
                            div()
                                .text_size(theme.font_sizes.caption)
                                .text_color(c.muted_foreground)
                                .child(label),
                        )
                        .into_any_element()
                } else {
                    div()
                        .absolute()
                        .top(px(pos as f32 - 8.))
                        .h(px(16.))
                        .w_full()
                        .child(
                            div()
                                .text_size(theme.font_sizes.caption)
                                .text_color(c.muted_foreground)
                                .text_right()
                                .pr_1()
                                .child(label),
                        )
                        .into_any_element()
                }
            })
            .collect();

        div()
            .id(self.id)
            .relative()
            .w_full()
            .h_full()
            .children(tick_els)
            .when_some(self.label, |this, label| {
                this.child(
                    div()
                        .absolute()
                        .text_size(theme.font_sizes.caption)
                        .text_color(c.muted_foreground)
                        .child(label),
                )
            })
            .into_any_element()
    }
}

fn format_float(val: f64) -> String {
    if val.fract().abs() < 1e-6 {
        format!("{:.0}", val)
    } else if val.fract().abs() * 10.0 < 1.0 {
        format!("{:.2}", val)
    } else {
        format!("{:.1}", val)
    }
}
