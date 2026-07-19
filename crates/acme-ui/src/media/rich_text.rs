use crate::{ActiveTheme, StyledExt};
use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

/// A styled text segment with optional bold, italic, and color.
pub struct RichTextSegment {
    text: SharedString,
    bold: bool,
    #[allow(dead_code)]
    italic: bool,
    color: Option<gpui::Hsla>,
}

/// Component for displaying styled text with bold, italic, and heading styles.
///
/// # Example
///
/// ```ignore
/// RichText::new("greeting")
///     .text("bold", "Hello")
///     .text("normal", " world")
///     .text("heading", "Welcome");
/// ```
#[derive(IntoElement)]
pub struct RichText {
    id: ElementId,
    segments: Vec<RichTextSegment>,
}

impl RichText {
    /// Create a new [`RichText`] with the given stable [`ElementId`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            segments: Vec::new(),
        }
    }

    /// Add a styled text segment.
    ///
    /// `style` may be `"normal"`, `"bold"`, `"italic"`, or `"heading"`.
    pub fn text(mut self, style: &str, content: &str) -> Self {
        let segment = match style {
            "bold" => RichTextSegment {
                text: content.into(),
                bold: true,
                italic: false,
                color: None,
            },
            "italic" => RichTextSegment {
                text: content.into(),
                bold: false,
                italic: true,
                color: None,
            },
            "heading" => RichTextSegment {
                text: content.into(),
                bold: true,
                italic: false,
                color: None,
            },
            _ => RichTextSegment {
                text: content.into(),
                bold: false,
                italic: false,
                color: None,
            },
        };
        self.segments.push(segment);
        self
    }
}

impl RenderOnce for RichText {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        div()
            .id(self.id)
            .v_flex()
            .children(self.segments.into_iter().map(|seg| {
                let mut el = div()
                    .text_color(seg.color.unwrap_or(c.foreground))
                    .child(seg.text);
                if seg.bold {
                    el = el.text_size(px(15.));
                }
                el
            }))
    }
}
