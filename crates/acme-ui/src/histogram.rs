use crate::{ActiveTheme, ChartColors, StyledExt};
use gpui::{
    App, ElementId, Hsla, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

/// A single bin in a histogram.
#[derive(Clone)]
pub struct HistogramBin {
    pub label: SharedString,
    pub value: f32,
    pub color: Option<Hsla>,
}

impl HistogramBin {
    /// Creates a new histogram bin with a label and numeric value.
    pub fn new(label: impl Into<SharedString>, value: f32) -> Self {
        Self {
            label: label.into(),
            value,
            color: None,
        }
    }

    /// Sets a custom color for this bin.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

/// A statistical histogram component.
///
/// Renders binned data as vertical bars with labels below.
/// Bar heights are proportional to their values relative to the
/// maximum value across all bins.
///
/// # Example
///
/// ```ignore
/// Histogram::new("age-dist")
///     .height(px(200.))
///     .bins(vec![
///         HistogramBin::new("0-18", 120.0),
///         HistogramBin::new("19-35", 340.0),
///         HistogramBin::new("36-50", 210.0),
///         HistogramBin::new("51+", 95.0),
///     ])
/// ```
#[derive(IntoElement)]
pub struct Histogram {
    id: ElementId,
    bins: Vec<HistogramBin>,
    height: gpui::Pixels,
    max_value: Option<f32>,
}

impl Histogram {
    /// Creates a new histogram with the given `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            bins: Vec::new(),
            height: px(200.),
            max_value: None,
        }
    }

    /// Replaces the bin data.
    pub fn bins(mut self, bins: Vec<HistogramBin>) -> Self {
        self.bins = bins;
        self
    }

    /// Sets the overall chart height in pixels.
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

impl RenderOnce for Histogram {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        let max_val = self.max_value.unwrap_or_else(|| {
            self.bins
                .iter()
                .map(|b| b.value)
                .reduce(f32::max)
                .unwrap_or(1.0)
                .max(1.0)
        });

        let height_f32: f32 = self.height.into();
        let bar_area_height = (height_f32 - 32.0).max(10.0);

        div()
            .id(self.id)
            .v_flex()
            .w_full()
            .h(self.height)
            .child(
                div()
                    .flex()
                    .items_end()
                    .w_full()
                    .h(px(bar_area_height + 4.))
                    .gap(px(3.))
                    .px_2()
                    .children(self.bins.into_iter().enumerate().map(|(i, bin)| {
                        let ratio = (bin.value / max_val).clamp(0.0, 1.0);
                        let bar_h = px((bar_area_height * ratio).max(4.0));
                        let color = bin.color.unwrap_or_else(|| ChartColors::get(i));

                        div()
                            .v_flex()
                            .flex_1()
                            .items_center()
                            .child(
                                div()
                                    .w_full()
                                    .h(bar_h)
                                    .bg(color)
                                    .rounded(px(2.))
                                    .min_h(px(4.)),
                            )
                            .child(
                                div()
                                    .text_size(theme.font_sizes.caption)
                                    .text_color(c.muted_foreground)
                                    .mt_1()
                                    .child(bin.label),
                            )
                            .into_any_element()
                    })),
            )
            .into_any_element()
    }
}
