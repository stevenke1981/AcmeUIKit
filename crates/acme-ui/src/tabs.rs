use gpui::{
    App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div, px,
};

use crate::ActiveTheme;

/// Presentational tab strip. The parent application controls the selected index.
#[derive(IntoElement)]
pub struct Tabs {
    labels: Vec<SharedString>,
    selected: usize,
}

impl Tabs {
    pub fn new<I, S>(labels: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<SharedString>,
    {
        Self {
            labels: labels.into_iter().map(Into::into).collect(),
            selected: 0,
        }
    }

    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = selected;
        self
    }
}

impl RenderOnce for Tabs {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        div()
            .w_full()
            .flex()
            .gap_1()
            .rounded(theme.radius)
            .bg(c.muted)
            .p_1()
            .children(self.labels.into_iter().enumerate().map(|(index, label)| {
                let selected = index == self.selected;
                div()
                    .h(px(30.))
                    .px_3()
                    .flex()
                    .items_center()
                    .rounded(theme.radius_sm)
                    .bg(if selected { c.surface } else { c.muted })
                    .text_color(if selected {
                        c.foreground
                    } else {
                        c.muted_foreground
                    })
                    .text_size(px(12.))
                    .child(label)
            }))
    }
}
