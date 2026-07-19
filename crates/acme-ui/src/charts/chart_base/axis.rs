//! Chart axis component with tick labels and gridlines.

use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, prelude::FluentBuilder, px,
};

use super::scale::Scale;
use crate::ActiveTheme;

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
