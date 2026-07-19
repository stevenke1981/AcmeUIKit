use crate::{ActiveTheme, StyledExt};
use gpui::{
    App, ElementId, Hsla, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    Styled as _, Window, div, px,
};

/// Line number gutter for code editors.
///
/// # Example
///
/// ```ignore
/// LineNumbers::new("gutter")
///     .lines(30)
///     .active_line(5);
/// ```
#[derive(IntoElement)]
pub struct LineNumbers {
    id: ElementId,
    line_count: usize,
    active_line: Option<usize>,
}

impl LineNumbers {
    /// Create a new [`LineNumbers`] gutter.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            line_count: 0,
            active_line: None,
        }
    }

    /// Set the total number of lines to display.
    pub fn lines(mut self, count: usize) -> Self {
        self.line_count = count;
        self
    }

    /// Highlight the given 1-indexed line as active.
    pub fn active_line(mut self, ln: usize) -> Self {
        self.active_line = Some(ln);
        self
    }
}

impl RenderOnce for LineNumbers {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        div()
            .id(self.id)
            .v_flex()
            .bg(c.muted)
            .border_r_1()
            .border_color(c.border)
            .w(px(48.))
            .children((1..=self.line_count).map(|i| {
                let is_active = self.active_line == Some(i);
                let mut line = div()
                    .h(px(24.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(c.muted_foreground)
                    .child(i.to_string());
                if is_active {
                    line = line.bg(Hsla {
                        h: c.primary.h,
                        s: c.primary.s,
                        l: c.primary.l,
                        a: 0.1,
                    });
                }
                line
            }))
    }
}
