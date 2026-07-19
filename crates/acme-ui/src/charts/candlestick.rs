use crate::{ActiveTheme, StyledExt};
use gpui::{
    App, ElementId, Hsla, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

/// A single candlestick data point (OHLC).
#[derive(Clone)]
pub struct Candlestick {
    pub label: SharedString,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
}

impl Candlestick {
    /// Creates a new candlestick with a label and OHLC values.
    pub fn new(label: impl Into<SharedString>, open: f32, high: f32, low: f32, close: f32) -> Self {
        Self {
            label: label.into(),
            open,
            high,
            low,
            close,
        }
    }
}

/// A candlestick chart for financial data.
///
/// Renders OHLC data as candle bodies (open–close) with wicks (high–low).
/// Green candles indicate an upward move (close >= open); red candles
/// indicate a downward move (close < open).
///
/// # Example
///
/// ```ignore
/// CandlestickChart::new("aapl")
///     .height(px(250.))
///     .data(vec![
///         Candlestick::new("Mon", 150.0, 155.0, 148.0, 153.0),
///         Candlestick::new("Tue", 153.0, 158.0, 152.0, 156.0),
///         Candlestick::new("Wed", 156.0, 157.0, 149.0, 151.0),
///     ])
/// ```
#[derive(IntoElement)]
pub struct CandlestickChart {
    id: ElementId,
    data: Vec<Candlestick>,
    height: gpui::Pixels,
}

impl CandlestickChart {
    /// Creates a new candlestick chart with the given `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            data: Vec::new(),
            height: px(200.),
        }
    }

    /// Replaces the candlestick data.
    pub fn data(mut self, data: Vec<Candlestick>) -> Self {
        self.data = data;
        self
    }

    /// Sets the overall chart height in pixels.
    pub fn height(mut self, height: gpui::Pixels) -> Self {
        self.height = height;
        self
    }
}

impl RenderOnce for CandlestickChart {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let h: f32 = self.height.into();
        let count = self.data.len().max(1);

        // Find global min/max across all OHLC values
        let mut global_max = f32::MIN;
        let mut global_min = f32::MAX;
        for d in &self.data {
            if d.high > global_max {
                global_max = d.high;
            }
            if d.low < global_min {
                global_min = d.low;
            }
        }
        let range = (global_max - global_min).max(1.0);

        let candle_w = ((h * 0.8) / count as f32).clamp(6.0, 30.0);
        let pad_x = 10.0;
        let pad_y = 10.0;
        let plot_h = h - pad_y * 2.0;
        let body_min_h = 2.0;

        fn map_y(val: f32, global_min: f32, range: f32, plot_h: f32, pad_y: f32) -> f32 {
            pad_y + plot_h * (1.0 - (val - global_min) / range)
        }

        div()
            .id(self.id)
            .v_flex()
            .w_full()
            .h(self.height)
            .relative()
            .children(self.data.into_iter().enumerate().map(|(i, d)| {
                let x = pad_x + i as f32 * (candle_w + 2.0);
                let y_high = map_y(d.high, global_min, range, plot_h, pad_y);
                let y_low = map_y(d.low, global_min, range, plot_h, pad_y);
                let y_open = map_y(d.open, global_min, range, plot_h, pad_y);
                let y_close = map_y(d.close, global_min, range, plot_h, pad_y);
                let is_up = d.close >= d.open;
                let body_color = if is_up {
                    Hsla {
                        h: 0.3,
                        s: 0.7,
                        l: 0.45,
                        a: 1.0,
                    } // green
                } else {
                    Hsla {
                        h: 0.0,
                        s: 0.7,
                        l: 0.5,
                        a: 1.0,
                    } // red
                };
                let body_top = y_open.min(y_close);
                let body_bottom = y_open.max(y_close);
                let body_h = (body_bottom - body_top).max(body_min_h);
                let wick_w = 2.0;

                let els: Vec<gpui::AnyElement> = vec![
                    // Wick (high–low line)
                    div()
                        .absolute()
                        .left(px(x + candle_w / 2.0 - wick_w / 2.0))
                        .top(px(y_high))
                        .w(px(wick_w))
                        .h(px(y_low - y_high))
                        .bg(body_color)
                        .into_any_element(),
                    // Body (open–close rectangle)
                    div()
                        .absolute()
                        .left(px(x))
                        .top(px(body_top))
                        .w(px(candle_w))
                        .h(px(body_h))
                        .bg(body_color)
                        .rounded(px(1.))
                        .into_any_element(),
                    // Label below the candle
                    div()
                        .absolute()
                        .left(px(x))
                        .top(px(h - pad_y + 2.))
                        .w(px(candle_w))
                        .text_center()
                        .text_size(theme.font_sizes.caption)
                        .text_color(c.muted_foreground)
                        .child(d.label)
                        .into_any_element(),
                ];

                div().children(els).into_any_element()
            }))
            .into_any_element()
    }
}
