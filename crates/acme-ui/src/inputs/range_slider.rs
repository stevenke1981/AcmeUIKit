use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, Pixels, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// A range slider with min and max values.
///
/// # Example
///
/// ```ignore
/// RangeSlider::new("range")
///     .min(0.)
///     .max(100.)
///     .low(25.)
///     .high(75.)
/// ```
#[derive(IntoElement)]
pub struct RangeSlider {
    id: ElementId,
    min: f64,
    max: f64,
    low: f64,
    high: f64,
}

impl RangeSlider {
    /// Creates a new range slider.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            min: 0.,
            max: 100.,
            low: 25.,
            high: 75.,
        }
    }

    /// Sets the minimum value.
    pub fn min(mut self, v: f64) -> Self {
        self.min = v;
        self
    }

    /// Sets the maximum value.
    pub fn max(mut self, v: f64) -> Self {
        self.max = v;
        self
    }

    /// Sets the low (left) value.
    pub fn low(mut self, v: f64) -> Self {
        self.low = v.max(self.min);
        self
    }

    /// Sets the high (right) value.
    pub fn high(mut self, v: f64) -> Self {
        self.high = v.min(self.max);
        self
    }

    fn pct(&self, val: f64) -> f64 {
        if (self.max - self.min).abs() < f64::EPSILON {
            0.
        } else {
            (val - self.min) / (self.max - self.min)
        }
    }
}

impl RenderOnce for RangeSlider {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let track_w: f64 = 200.;
        let track_h: Pixels = px(4.);
        let _thumb_size: f64 = 14.;
        let low_pct = self.pct(self.low);
        let high_pct = self.pct(self.high);
        let range_pct = high_pct - low_pct;

        div()
            .id(self.id)
            .v_flex()
            .gap_2()
            .child(
                div()
                    .h_flex()
                    .justify_between()
                    .text_size(theme.font_sizes.caption)
                    .text_color(c.muted_foreground)
                    .child(SharedString::from(format!("{}", self.low as i64)))
                    .child(SharedString::from(format!("{}", self.high as i64))),
            )
            .child(
                div().h(track_h).rounded(px(2.)).bg(c.muted).child(
                    div()
                        .h(track_h)
                        .rounded(px(2.))
                        .bg(c.primary)
                        .w(px((range_pct * track_w) as f32))
                        .mx(px((low_pct * track_w) as f32)),
                ),
            )
    }
}
