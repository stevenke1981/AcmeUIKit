use gpui::prelude::FluentBuilder as _;
use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// Stacked overlapping avatars.
///
/// Shows a row of circular avatars with overlapping negative margin.
/// When the number of names exceeds `max_visible`, a "+N" overflow circle is shown.
///
/// # Example
///
/// ```ignore
/// AvatarGroup::new("team")
///     .add("Alice")
///     .add("Bob")
///     .add("Charlie")
///     .max_visible(2)
/// ```
#[derive(IntoElement)]
pub struct AvatarGroup {
    id: ElementId,
    names: Vec<SharedString>,
    max: usize,
}

impl AvatarGroup {
    /// Creates a new avatar group with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            names: Vec::new(),
            max: 3,
        }
    }

    /// Adds an avatar by display name.
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, name: impl Into<SharedString>) -> Self {
        self.names.push(name.into());
        self
    }

    /// Sets the maximum number of visible avatars before overflow.
    pub fn max_visible(mut self, max: usize) -> Self {
        self.max = max.max(1);
        self
    }

    /// Derives a pastel background color from a name.
    fn avatar_color(name: &str) -> gpui::Hsla {
        let hash = name
            .bytes()
            .fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
        let hue = (hash % 360) as f32;
        gpui::hsla(hue / 360.0, 0.45, 0.55, 1.0)
    }

    /// Gets the first letter of a name in uppercase.
    fn initial(name: &str) -> String {
        name.chars()
            .next()
            .map(|c| c.to_ascii_uppercase().to_string())
            .unwrap_or_else(|| "?".to_string())
    }
}

impl RenderOnce for AvatarGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let font_sizes = cx.theme().font_sizes;
        let avatar_size = px(24.);
        let overlap = px(-6.);

        let visible_count = self.names.len().min(self.max);
        let overflow_count = self.names.len().saturating_sub(self.max);

        div()
            .id(self.id)
            .h_flex()
            .children(
                self.names
                    .iter()
                    .take(visible_count)
                    .enumerate()
                    .map(move |(i, name)| {
                        let color = Self::avatar_color(name);
                        let letter = Self::initial(name);
                        div()
                            .id(ElementId::Name(SharedString::from(format!(
                                "avatar-group-{}",
                                i
                            ))))
                            .w(avatar_size)
                            .h(avatar_size)
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_full()
                            .bg(color)
                            .text_color(gpui::white())
                            .text_size(font_sizes.caption)
                            .font_weight(gpui::FontWeight::BOLD)
                            .when(i > 0, |this| this.mx(overlap))
                            .child(letter)
                    }),
            )
            .when(overflow_count > 0, |this| {
                this.child(
                    div()
                        .id(ElementId::Name(SharedString::from("avatar-overflow")))
                        .w(avatar_size)
                        .h(avatar_size)
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .bg(c.muted)
                        .text_color(c.muted_foreground)
                        .text_size(px(10.))
                        .child(format!("+{}", overflow_count)),
                )
            })
    }
}
