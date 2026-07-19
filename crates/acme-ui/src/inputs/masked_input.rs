use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// A masked input for formatted text such as phone numbers or credit cards.
///
/// # Example
///
/// ```ignore
/// MaskedInput::new("phone")
///     .mask("(XXX) XXX-XXXX")
///     .value("5551234567")
/// ```
#[derive(IntoElement)]
pub struct MaskedInput {
    id: ElementId,
    mask: SharedString,
    value: SharedString,
    placeholder: SharedString,
}

impl MaskedInput {
    /// Creates a new masked input.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            mask: SharedString::new(""),
            value: SharedString::new(""),
            placeholder: SharedString::new(""),
        }
    }

    /// Sets the mask pattern (X = digit placeholder).
    pub fn mask(mut self, m: impl Into<SharedString>) -> Self {
        self.mask = m.into();
        self
    }

    /// Sets the raw input value.
    pub fn value(mut self, val: impl Into<SharedString>) -> Self {
        self.value = val.into();
        self
    }

    /// Sets the placeholder text.
    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = text.into();
        self
    }

    fn formatted(&self) -> SharedString {
        if self.value.is_empty() {
            return self.placeholder.clone();
        }
        let mask_chars: Vec<char> = self.mask.chars().collect();
        let val_chars: Vec<char> = self.value.chars().collect();
        let mut out = String::with_capacity(mask_chars.len());
        let mut vi = 0;
        for &mc in &mask_chars {
            if mc == 'X' {
                if vi < val_chars.len() {
                    out.push(val_chars[vi]);
                    vi += 1;
                } else {
                    out.push('_');
                }
            } else {
                out.push(mc);
            }
        }
        SharedString::from(out)
    }
}

impl RenderOnce for MaskedInput {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let display = self.formatted();

        div()
            .id(self.id)
            .h_flex()
            .items_center()
            .h(px(32.))
            .px(px(10.))
            .rounded(theme.radius)
            .bg(c.muted)
            .child(
                div()
                    .text_size(theme.font_sizes.body)
                    .text_color(if display.is_empty() || display == self.placeholder {
                        c.muted_foreground
                    } else {
                        c.foreground
                    })
                    .child(if display.is_empty() {
                        self.placeholder.clone()
                    } else {
                        display
                    }),
            )
    }
}
