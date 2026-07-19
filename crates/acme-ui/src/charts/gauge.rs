use gpui::{
    App, ElementId, FontWeight, Hsla, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, Styled as _, Window, div, prelude::FluentBuilder, px,
};

use crate::{ActiveTheme, StyledExt};

/// A radial gauge that displays a value on a colored ring.
///
/// The gauge renders as a circle with a configurable inner hole. The outer
/// ring color changes based on the value relative to min/max thresholds:
///
/// | Range    | Color  |
/// |----------|--------|
/// | < 30%    | Green  |
/// | 30–60%   | Yellow |
/// | 60–85%   | Orange |
/// | > 85%    | Red    |
///
/// The value and an optional label are shown in the center.
///
/// # Example
///
/// ```ignore
/// Gauge::new("cpu")
///     .value(67.0)
///     .min(0.0)
///     .max(100.0)
///     .size(px(120.))
///     .label("CPU %")
///     .show_value(true)
/// ```
#[derive(IntoElement)]
pub struct Gauge {
    id: ElementId,
    value: f32,
    min: f32,
    max: f32,
    size: gpui::Pixels,
    label: Option<SharedString>,
    show_value: bool,
}

impl Gauge {
    /// Creates a new gauge with the given `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: 0.0,
            min: 0.0,
            max: 100.0,
            size: px(120.),
            label: None,
            show_value: true,
        }
    }

    /// Sets the current value.
    pub fn value(mut self, val: f32) -> Self {
        self.value = val;
        self
    }

    /// Sets the minimum value of the gauge range.
    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    /// Sets the maximum value of the gauge range.
    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    /// Sets the outer diameter in pixels.
    pub fn size(mut self, size: gpui::Pixels) -> Self {
        self.size = size;
        self
    }

    /// Sets an optional label shown below the value.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Controls whether the numeric value is shown in the center.
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }
}

impl RenderOnce for Gauge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let range = (self.max - self.min).max(1.0);
        let ratio = ((self.value - self.min) / range).clamp(0.0, 1.0);

        // Threshold-based color selection
        let gauge_color = if ratio < 0.3 {
            Hsla {
                h: 0.3,
                s: 0.6,
                l: 0.5,
                a: 1.0,
            } // green
        } else if ratio < 0.6 {
            Hsla {
                h: 0.12,
                s: 0.7,
                l: 0.55,
                a: 1.0,
            } // yellow
        } else if ratio < 0.85 {
            Hsla {
                h: 0.07,
                s: 0.7,
                l: 0.55,
                a: 1.0,
            } // orange
        } else {
            Hsla {
                h: 0.0,
                s: 0.6,
                l: 0.5,
                a: 1.0,
            } // red
        };

        let size_f32: f32 = self.size.into();
        let hole_size = px(size_f32 * 0.7);

        div()
            .id(self.id)
            .v_flex()
            .items_center()
            .child(
                div()
                    .size(self.size)
                    .rounded_full()
                    .bg(gauge_color.alpha(0.2))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .size(hole_size)
                            .rounded_full()
                            .bg(c.background)
                            .flex()
                            .items_center()
                            .justify_center()
                            .v_flex()
                            .when(self.show_value, |this| {
                                this.child(
                                    div()
                                        .text_size(theme.font_sizes.heading)
                                        .font_weight(FontWeight(700.))
                                        .text_color(gauge_color)
                                        .child(format!("{:.0}", self.value)),
                                )
                            })
                            .when_some(self.label, |this, label| {
                                this.child(
                                    div()
                                        .text_size(theme.font_sizes.caption)
                                        .text_color(c.muted_foreground)
                                        .child(label),
                                )
                            }),
                    ),
            )
            .into_any_element()
    }
}
