use std::rc::Rc;

use gpui::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

type SliderHandler = Rc<dyn Fn(f32, &ClickEvent, &mut Window, &mut App)>;

/// A numeric slider with track, fill, and value label.
///
/// The caller manages the current value and receives change events.
///
/// # Example
///
/// ```ignore
/// Slider::new("volume")
///     .value(0.5)
///     .min(0.0)
///     .max(1.0)
///     .step(0.1)
///     .on_change(|value, _event, _window, _cx| { })
/// ```
#[derive(IntoElement)]
pub struct Slider {
    id: ElementId,
    value: f32,
    min: f32,
    max: f32,
    step: f32,
    on_change: Option<SliderHandler>,
}

/// Fixed track width for consistent layout.
const TRACK_WIDTH: f32 = 200.0;

impl Slider {
    /// Creates a new slider with the given [`ElementId`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: 0.0,
            min: 0.0,
            max: 1.0,
            step: 0.01,
            on_change: None,
        }
    }

    /// Sets the current slider value.
    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    /// Sets the minimum value.
    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    /// Sets the maximum value.
    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    /// Sets the step increment.
    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    /// Registers a change handler fired when the track is clicked.
    pub fn on_change(
        mut self,
        handler: impl Fn(f32, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Slider {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        let range = (self.max - self.min).max(0.01);
        let pct = ((self.value - self.min) / range * 100.0).clamp(0.0, 100.0);
        let fill_w = (pct / 100.0 * TRACK_WIDTH).max(0.0);

        div()
            .id(self.id)
            .v_flex()
            .w(px(TRACK_WIDTH))
            .gap_1()
            // Track + fill
            .child(
                div()
                    .relative()
                    .h(px(8.))
                    .w_full()
                    .rounded(px(4.))
                    .bg(c.muted)
                    .overflow_hidden()
                    .child(
                        div()
                            .h_full()
                            .w(px(fill_w))
                            .bg(c.primary)
                            .rounded(px(4.)),
                    )
                    .child(
                        // Thumb indicator
                        div()
                            .absolute()
                            .left(px((fill_w - 8.0).max(0.0)))
                            .top(px(-4.))
                            .w(px(16.))
                            .h(px(16.))
                            .rounded_full()
                            .bg(c.primary)
                            .border_2()
                            .border_color(c.surface),
                    ),
            )
            // Value label
            .child(
                div()
                    .h_flex()
                    .justify_between()
                    .w_full()
                    .child(
                        div()
                            .text_size(theme.font_sizes.caption)
                            .text_color(c.muted_foreground)
                            .child(format!("{:.1}", self.min)),
                    )
                    .child(
                        div()
                            .text_size(theme.font_sizes.body)
                            .text_color(c.foreground)
                            .child(format!("{:.1}", self.value)),
                    )
                    .child(
                        div()
                            .text_size(theme.font_sizes.caption)
                            .text_color(c.muted_foreground)
                            .child(format!("{:.1}", self.max)),
                    ),
            )
    }
}
