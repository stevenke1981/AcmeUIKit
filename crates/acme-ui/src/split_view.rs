use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// A resizable split panel with left and right sections and a divider.
///
/// # Example
///
/// ```ignore
/// SplitView::new("my-split")
///     .ratio(0.6)
///     .left_label("Editor")
///     .right_label("Preview")
/// ```
#[derive(IntoElement)]
pub struct SplitView {
    id: ElementId,
    ratio: f64,
    left_label: SharedString,
    right_label: SharedString,
}

impl SplitView {
    /// Creates a new split view with an equilibrated default ratio.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            ratio: 0.5,
            left_label: SharedString::default(),
            right_label: SharedString::default(),
        }
    }

    /// Sets the left/right split ratio (0.0 – 1.0).
    pub fn ratio(mut self, ratio: f64) -> Self {
        self.ratio = ratio.clamp(0.0, 1.0);
        self
    }

    /// Sets the label displayed in the left panel.
    pub fn left_label(mut self, label: impl Into<SharedString>) -> Self {
        self.left_label = label.into();
        self
    }

    /// Sets the label displayed in the right panel.
    pub fn right_label(mut self, label: impl Into<SharedString>) -> Self {
        self.right_label = label.into();
        self
    }
}

impl RenderOnce for SplitView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        div()
            .id(self.id)
            .h_flex()
            .w_full()
            .h(px(200.))
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .h_flex()
                    .items_center()
                    .justify_center()
                    .bg(c.surface)
                    .border_1()
                    .border_color(c.border)
                    .child(self.left_label.clone()),
            )
            .child(div().w(px(4.)).h_full().bg(c.border).cursor_col_resize())
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .h_flex()
                    .items_center()
                    .justify_center()
                    .bg(c.background)
                    .border_1()
                    .border_color(c.border)
                    .child(self.right_label.clone()),
            )
    }
}
