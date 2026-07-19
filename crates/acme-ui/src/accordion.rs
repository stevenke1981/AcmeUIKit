use gpui::{
    App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div,
};

use crate::{Collapsible, StyledExt};

/// An accordion containing multiple collapsible sections.
///
/// # Example
///
/// ```ignore
/// Accordion::new()
///     .section("Section 1", true, div().child("Content 1"))
///     .section("Section 2", false, div().child("Content 2"))
/// ```
#[derive(IntoElement)]
pub struct Accordion {
    sections: Vec<AccordionSection>,
}

struct AccordionSection {
    title: SharedString,
    open: bool,
    content: Option<gpui::AnyElement>,
}

impl Accordion {
    /// Creates a new accordion.
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }

    /// Adds a section.
    pub fn section(
        mut self,
        title: impl Into<SharedString>,
        open: bool,
        content: impl IntoElement + 'static,
    ) -> Self {
        self.sections.push(AccordionSection {
            title: title.into(),
            open,
            content: Some(content.into_any_element()),
        });
        self
    }
}

impl Default for Accordion {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Accordion {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .v_flex()
            .w_full()
            .children(self.sections.into_iter().map(|section| {
                Collapsible::new(section.title)
                    .open(section.open)
                    .child(section.content.unwrap_or_else(|| div().into_any_element()))
            }))
    }
}
