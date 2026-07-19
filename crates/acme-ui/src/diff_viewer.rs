use crate::{ActiveTheme, StyledExt};
use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

/// Side-by-side diff display with color-coded added/removed lines.
///
/// # Example
///
/// ```ignore
/// DiffViewer::new("diff")
///     .old_text("line one\nline two")
///     .new_text("line one\nline three\nline four");
/// ```
#[derive(IntoElement)]
pub struct DiffViewer {
    id: ElementId,
    old_text: SharedString,
    new_text: SharedString,
}

impl DiffViewer {
    /// Create a new [`DiffViewer`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            old_text: SharedString::default(),
            new_text: SharedString::default(),
        }
    }

    /// Set the old (left) text.
    pub fn old_text(mut self, text: impl Into<SharedString>) -> Self {
        self.old_text = text.into();
        self
    }

    /// Set the new (right) text.
    pub fn new_text(mut self, text: impl Into<SharedString>) -> Self {
        self.new_text = text.into();
        self
    }
}

impl RenderOnce for DiffViewer {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let old_lines: Vec<&str> = self.old_text.lines().collect();
        let new_lines: Vec<&str> = self.new_text.lines().collect();

        div()
            .id(self.id)
            .h_flex()
            .border_1()
            .border_color(c.border)
            .rounded(px(4.))
            .overflow_hidden()
            .child(
                div()
                    .v_flex()
                    .flex_1()
                    .bg(gpui::hsla(0., 0.5, 0.5, 0.1))
                    .children(old_lines.into_iter().map(|line| {
                        div()
                            .px(px(4.))
                            .text_color(c.foreground)
                            .child(SharedString::from(line))
                    })),
            )
            .child(
                div()
                    .v_flex()
                    .flex_1()
                    .border_l_1()
                    .border_color(c.border)
                    .bg(gpui::hsla(120., 0.5, 0.5, 0.1))
                    .children(new_lines.into_iter().map(|line| {
                        div()
                            .px(px(4.))
                            .text_color(c.foreground)
                            .child(SharedString::from(line))
                    })),
            )
    }
}
