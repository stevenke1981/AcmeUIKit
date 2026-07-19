use std::rc::Rc;

use crate::{ActiveTheme, StyledExt};
use gpui::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div, px,
};

type OutlineClickHandler = Rc<dyn Fn(usize, &ClickEvent, &mut Window, &mut App)>;

/// A single heading entry in the document outline.
pub struct OutlineEntry {
    pub level: usize,
    pub text: SharedString,
}

/// Shows a document heading outline / tree.
///
/// # Example
///
/// ```ignore
/// DocumentOutline::new("outline")
///     .heading(1, "Introduction")
///     .heading(2, "Getting Started")
///     .heading(3, "Installation")
///     .on_click(|index, _event, _window, _cx| {
///         println!("Clicked heading {index}");
///     });
/// ```
#[derive(IntoElement)]
pub struct DocumentOutline {
    id: ElementId,
    headings: Vec<OutlineEntry>,
    on_click: Option<OutlineClickHandler>,
}

impl DocumentOutline {
    /// Create a new [`DocumentOutline`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            headings: Vec::new(),
            on_click: None,
        }
    }

    /// Add a heading at the given level (1–6).
    pub fn heading(mut self, level: usize, text: impl Into<SharedString>) -> Self {
        self.headings.push(OutlineEntry {
            level,
            text: text.into(),
        });
        self
    }

    /// Register a click handler that receives the heading index.
    pub fn on_click(
        mut self,
        handler: impl Fn(usize, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for DocumentOutline {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let handler = self.on_click;
        let id_prefix = match &self.id {
            ElementId::Name(s) => s.to_string(),
            ElementId::Integer(n) => format!("outline-{n}"),
            _ => "outline".to_string(),
        };

        div()
            .id(self.id)
            .v_flex()
            .children(self.headings.into_iter().enumerate().map(|(index, entry)| {
                let indent = px((entry.level as f32) * 12.);
                let is_top = entry.level == 1;
                let fg = if is_top {
                    c.foreground
                } else {
                    c.muted_foreground
                };
                let heading_id = ElementId::Name(format!("{id_prefix}-{index}").into());

                let mut el = div()
                    .id(heading_id)
                    .h(px(26.))
                    .v_flex()
                    .justify_center()
                    .px(px(8.))
                    .ml(indent)
                    .text_color(fg)
                    .cursor_pointer()
                    .hover(|style| style.bg(c.muted))
                    .child(entry.text);

                if let Some(ref h) = handler {
                    let h = h.clone();
                    el = el.on_click(move |event, window, cx| h(index, event, window, cx));
                }

                el
            }))
    }
}
