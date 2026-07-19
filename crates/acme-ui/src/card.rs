use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled as _, Window,
    div, px,
};

use crate::{ActiveTheme, StyledExt};

/// Surface container with optional heading and arbitrary children.
#[derive(IntoElement, Default)]
pub struct Card {
    title: Option<SharedString>,
    description: Option<SharedString>,
    children: Vec<AnyElement>,
}

impl Card {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            children: Vec::new(),
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl ParentElement for Card {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Card {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let mut card = div()
            .w_full()
            .v_flex()
            .gap_4()
            .rounded(theme.radius_lg)
            .border_1()
            .border_color(c.border)
            .bg(c.surface)
            .p_4();

        if self.title.is_some() || self.description.is_some() {
            let mut heading = div().v_flex().gap_1();
            if let Some(title) = self.title {
                heading = heading.child(
                    div()
                        .text_color(c.foreground)
                        .text_size(px(16.))
                        .child(title),
                );
            }
            if let Some(description) = self.description {
                heading = heading.child(
                    div()
                        .text_color(c.muted_foreground)
                        .text_size(px(12.))
                        .child(description),
                );
            }
            card = card.child(heading);
        }

        card.children(self.children)
    }
}
