use gpui::{
    App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div, px,
};

use crate::Size;

/// A user avatar that displays initials.
///
/// # Example
///
/// ```ignore
/// Avatar::new("John Doe")
///     .size(Size::Medium)
/// ```
#[derive(IntoElement)]
pub struct Avatar {
    name: SharedString,
    size: Size,
}

impl Avatar {
    /// Creates a new avatar for the given display name.
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            size: Size::Medium,
        }
    }

    /// Sets the avatar size.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Derives a pastel background color from the name.
    fn background(&self) -> gpui::Hsla {
        let hash = self
            .name
            .as_ref()
            .bytes()
            .fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
        let hue = (hash % 360) as f32;
        gpui::hsla(hue / 360.0, 0.45, 0.55, 1.0)
    }

    fn initials(&self) -> String {
        let parts: Vec<&str> = self.name.split_whitespace().collect();
        match parts.len() {
            0 => "?".to_string(),
            1 => {
                let s = parts[0];
                let first = s
                    .chars()
                    .next()
                    .map(|c| c.to_ascii_uppercase())
                    .unwrap_or('?');
                first.to_string()
            }
            _ => {
                let a = parts[0]
                    .chars()
                    .next()
                    .map(|c| c.to_ascii_uppercase())
                    .unwrap_or('?');
                let b = parts[1]
                    .chars()
                    .next()
                    .map(|c| c.to_ascii_uppercase())
                    .unwrap_or('?');
                format!("{}{}", a, b)
            }
        }
    }
}

impl RenderOnce for Avatar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let display_size = px(match self.size {
            Size::ExtraSmall => 24.,
            Size::Small => 30.,
            Size::Medium => 36.,
            Size::Large => 42.,
        });
        let font_size = px(match self.size {
            Size::ExtraSmall => 10.,
            Size::Small => 12.,
            Size::Medium => 14.,
            Size::Large => 16.,
        });

        div()
            .w(display_size)
            .h(display_size)
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .bg(self.background())
            .text_color(gpui::white())
            .text_size(font_size)
            .font_weight(gpui::FontWeight::BOLD)
            .child(self.initials())
    }
}
