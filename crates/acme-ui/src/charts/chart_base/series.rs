//! Named data series with color and visibility.

use gpui::{Hsla, SharedString};

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
