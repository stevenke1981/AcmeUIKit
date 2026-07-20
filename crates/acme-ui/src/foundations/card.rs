use gpui::{
    AnyElement, App, InteractiveElement as _, IntoElement, ParentElement, RenderOnce, SharedString,
    Styled as _, Window, div,
};

use crate::{ActiveTheme, StyledExt};

/// Visual treatment for a card surface.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CardVariant {
    /// No visible border; use for ordinary grouped content.
    Plain,
    /// A clear one-pixel boundary.
    #[default]
    Outlined,
    /// Raised surface for summaries and dashboards.
    Elevated,
    /// Interactive surface with hover affordance.
    Interactive,
    /// Muted surface for secondary information.
    Muted,
}

/// Surface container with optional heading and arbitrary children.
#[derive(IntoElement, Default)]
pub struct Card {
    title: Option<SharedString>,
    description: Option<SharedString>,
    variant: CardVariant,
    children: Vec<AnyElement>,
}

impl Card {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            variant: CardVariant::Outlined,
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

    /// Sets the card visual variant.
    pub fn with_variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Uses the raised surface treatment.
    pub fn elevated(self) -> Self {
        self.with_variant(CardVariant::Elevated)
    }

    /// Uses the interactive surface treatment.
    pub fn interactive(self) -> Self {
        self.with_variant(CardVariant::Interactive)
    }

    /// Uses the muted surface treatment.
    pub fn muted(self) -> Self {
        self.with_variant(CardVariant::Muted)
    }

    /// Removes the card border.
    pub fn plain(self) -> Self {
        self.with_variant(CardVariant::Plain)
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
        let (background, border) = match self.variant {
            CardVariant::Plain => (c.background, c.background),
            CardVariant::Outlined | CardVariant::Interactive => (c.surface, c.border),
            CardVariant::Elevated => (c.surface, c.border),
            CardVariant::Muted => (c.muted, c.muted),
        };
        let interactive = self.variant == CardVariant::Interactive;
        let mut card = div()
            .w_full()
            .v_flex()
            .gap_4()
            .rounded(theme.radius_lg)
            .border_1()
            .border_color(border)
            .bg(background)
            .p_4();
        if interactive {
            card = card.cursor_pointer().hover(|style| style.bg(c.muted));
        }

        if self.title.is_some() || self.description.is_some() {
            let mut heading = div().v_flex().gap_1();
            if let Some(title) = self.title {
                heading = heading.child(
                    div()
                        .text_color(c.foreground)
                        .text_size(theme.font_sizes.heading)
                        .child(title),
                );
            }
            if let Some(description) = self.description {
                heading = heading.child(
                    div()
                        .text_color(c.muted_foreground)
                        .text_size(theme.font_sizes.caption)
                        .child(description),
                );
            }
            card = card.child(heading);
        }

        card.children(self.children)
    }
}
