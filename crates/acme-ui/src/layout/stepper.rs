use gpui::{
    App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::{ActiveTheme, StyledExt};

/// A step progress indicator.
///
/// # Example
///
/// ```ignore
/// Stepper::new()
///     .step("Cart")
///     .step("Shipping")
///     .step("Payment")
///     .active_step(1)
/// ```
#[derive(IntoElement)]
pub struct Stepper {
    steps: Vec<SharedString>,
    active: usize,
}

impl Stepper {
    /// Creates a new stepper.
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            active: 0,
        }
    }

    /// Adds a step label.
    pub fn step(mut self, label: impl Into<SharedString>) -> Self {
        self.steps.push(label.into());
        self
    }

    /// Sets the active step index (0-based).
    pub fn active_step(mut self, index: usize) -> Self {
        self.active = index;
        self
    }
}

impl Default for Stepper {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Stepper {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let total = self.steps.len();

        div()
            .h_flex()
            .items_center()
            .w_full()
            .children(self.steps.into_iter().enumerate().map(|(i, label)| {
                let is_completed = i < self.active;
                let is_active = i == self.active;
                let is_pending = i > self.active;

                div()
                    .h_flex()
                    .items_center()
                    .when(i + 1 < total, |this| this.flex_1())
                    .child(
                        div()
                            .v_flex()
                            .items_center()
                            .gap_1()
                            .child(
                                div()
                                    .w(px(20.))
                                    .h(px(20.))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded_full()
                                    .border_2()
                                    .when(is_active, |this| this.border_color(c.primary))
                                    .when(is_completed, |this| {
                                        this.bg(c.primary).border_color(c.primary)
                                    })
                                    .when(is_pending, |this| this.border_color(c.muted))
                                    .child(if is_completed {
                                        div()
                                            .text_color(gpui::white())
                                            .text_size(px(11.))
                                            .child("✓")
                                            .into_any_element()
                                    } else {
                                        div()
                                            .text_size(px(11.))
                                            .text_color(if is_active {
                                                c.primary
                                            } else {
                                                c.muted_foreground
                                            })
                                            .child(format!("{}", i + 1))
                                            .into_any_element()
                                    }),
                            )
                            .child(
                                div()
                                    .text_size(theme.font_sizes.caption)
                                    .text_color(if is_active {
                                        c.foreground
                                    } else {
                                        c.muted_foreground
                                    })
                                    .child(label),
                            ),
                    )
                    .when(i + 1 < total, |this| {
                        this.child(
                            div()
                                .flex_1()
                                .h(px(2.))
                                .mx(px(4.))
                                .rounded(px(1.))
                                .mt(px(-20.))
                                .bg(if is_completed { c.primary } else { c.muted }),
                        )
                    })
            }))
    }
}
