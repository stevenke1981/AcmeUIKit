use gpui::{App, IntoElement, ParentElement as _, RenderOnce, Styled as _, Window, div};

use crate::StyledExt;

/// A flex stack layout — horizontal or vertical.
///
/// # Example
///
/// ```ignore
/// Stack::h()
///     .child(div().child("A"))
///     .child(div().child("B"))
/// ```
#[derive(IntoElement)]
pub struct Stack {
    direction: StackDirection,
    gap: Option<Pixels>,
    children: Vec<StackChild>,
}

enum StackChild {
    Element(gpui::AnyElement),
    Spacer,
}

/// Horizontal stack.
pub fn h() -> Stack {
    Stack::horizontal()
}

/// Vertical stack.
pub fn v() -> Stack {
    Stack::vertical()
}

/// Stack layout direction.
pub enum StackDirection {
    Horizontal,
    Vertical,
}

impl Stack {
    /// Creates a horizontal stack.
    pub fn horizontal() -> Self {
        Self {
            direction: StackDirection::Horizontal,
            gap: None,
            children: Vec::new(),
        }
    }

    /// Creates a vertical stack.
    pub fn vertical() -> Self {
        Self {
            direction: StackDirection::Vertical,
            gap: None,
            children: Vec::new(),
        }
    }

    /// Sets the gap between items.
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = Some(gap.into());
        self
    }

    /// Adds a child element.
    pub fn child(mut self, child: impl IntoElement + 'static) -> Self {
        self.children
            .push(StackChild::Element(child.into_any_element()));
        self
    }

    /// Adds a flex spacer.
    pub fn spacer(mut self) -> Self {
        self.children.push(StackChild::Spacer);
        self
    }
}

impl RenderOnce for Stack {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let base = match self.direction {
            StackDirection::Horizontal => div().h_flex(),
            StackDirection::Vertical => div().v_flex(),
        };

        let base = if let Some(gap) = self.gap {
            base.gap(gap)
        } else {
            base
        };

        base.children(self.children.into_iter().map(|child| match child {
            StackChild::Element(el) => el,
            StackChild::Spacer => div().flex_1().into_any_element(),
        }))
    }
}

use gpui::Pixels;
