use gpui::{
    App, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce, SharedString,
    Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// A single data bar in a [`BarChart`].
#[derive(Clone)]
pub struct BarEntry {
    /// Label shown below the bar.
    pub label: SharedString,
    /// Numeric value (maps to bar height relative to max).
    pub value: f32,
    /// Optional custom color. When `None` the theme primary color is used.
    pub color: Option<gpui::Hsla>,
}

impl BarEntry {
    /// Creates a new bar entry.
    pub fn new(label: impl Into<SharedString>, value: f32) -> Self {
        Self {
            label: label.into(),
            value,
            color: None,
        }
    }

    /// Sets a custom bar color.
    pub fn color(mut self, color: gpui::Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

/// A simple vertical bar chart component.
///
/// Renders a set of labeled bars with heights proportional to their values.
/// The chart has a fixed overall height and optional maximum value.
///
/// # Example
///
/// ```ignore
/// BarChart::new("revenue")
///     .height(px(200.))
///     .max_value(1000.0)
///     .bars(vec![
///         BarEntry::new("Jan", 400.0),
///         BarEntry::new("Feb", 600.0).color(hsl(120., 60., 50.)),
///         BarEntry::new("Mar", 300.0),
///     ])
/// ```
#[derive(IntoElement)]
pub struct BarChart {
    id: gpui::ElementId,
    height: gpui::Pixels,
    max_value: Option<f32>,
    bars: Vec<BarEntry>,
}

impl BarChart {
    /// Creates a new bar chart with the given `id`.
    pub fn new(id: impl Into<gpui::ElementId>) -> Self {
        Self {
            id: id.into(),
            height: px(200.),
            max_value: None,
            bars: Vec::new(),
        }
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

    /// Replaces the bar data.
    pub fn bars(mut self, bars: Vec<BarEntry>) -> Self {
        self.bars = bars;
        self
    }
}

impl RenderOnce for BarChart {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        let max_val = self
            .max_value
            .or_else(|| self.bars.iter().map(|b| b.value).reduce(f32::max))
            .unwrap_or(1.0)
            .max(1.0);

        let bar_area_height: f32 = (self.height - px(32.)).into();

        div().id(self.id).v_flex().w_full().child(
            div()
                .h(self.height)
                .w_full()
                .flex()
                .items_end()
                .gap(px(6.))
                .px_4()
                .py_2()
                .children(self.bars.into_iter().map(|entry| {
                    let ratio = (entry.value / max_val).clamp(0.0, 1.0);
                    let bar_height: gpui::Pixels = px((bar_area_height * ratio).max(4.0));
                    let bar_color = entry.color.unwrap_or(c.primary);

                    div()
                        .v_flex()
                        .flex_1()
                        .items_center()
                        .child(
                            div()
                                .w_full()
                                .h(bar_height)
                                .bg(bar_color)
                                .rounded(theme.radius_sm)
                                .min_h(px(4.)),
                        )
                        .child(
                            div()
                                .text_size(theme.font_sizes.caption)
                                .text_color(c.muted_foreground)
                                .mt_1()
                                .text_center()
                                .child(entry.label),
                        )
                        .into_any_element()
                })),
        )
    }
}
