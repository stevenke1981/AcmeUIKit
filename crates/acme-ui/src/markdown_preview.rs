use crate::{ActiveTheme, StyledExt};
use gpui::{
    AnyElement, App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, Styled as _, Window, div, px,
};

/// Renders Markdown as simple formatted text with `# ` heading and `**bold**` support.
///
/// # Example
///
/// ```ignore
/// MarkdownPreview::new("preview")
///     .markdown("# Title\n\nThis is **bold** text.");
/// ```
#[derive(IntoElement)]
pub struct MarkdownPreview {
    id: ElementId,
    markdown: SharedString,
}

impl MarkdownPreview {
    /// Create a new [`MarkdownPreview`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            markdown: SharedString::default(),
        }
    }

    /// Set the Markdown source text.
    pub fn markdown(mut self, text: impl Into<SharedString>) -> Self {
        self.markdown = text.into();
        self
    }
}

impl RenderOnce for MarkdownPreview {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let text = self.markdown.to_string();

        let children: Vec<AnyElement> = text
            .lines()
            .map(|line| {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    return div().h(px(4.)).into_any_element();
                }

                // Check for `# ` heading
                if let Some(rest) = trimmed.strip_prefix("# ") {
                    return div()
                        .text_size(px(18.))
                        .text_color(c.foreground)
                        .child(SharedString::from(rest.to_string()))
                        .into_any_element();
                }

                // Check for `## ` sub-heading
                if let Some(rest) = trimmed.strip_prefix("## ") {
                    return div()
                        .text_size(px(15.))
                        .text_color(c.foreground)
                        .child(SharedString::from(rest.to_string()))
                        .into_any_element();
                }

                // Inline bold detection `**...**`
                let segments = render_inline_bold(trimmed, c.foreground);
                if segments.len() == 1 {
                    // No bold found — return plain line
                    div()
                        .text_color(c.foreground)
                        .child(SharedString::from(trimmed))
                        .into_any_element()
                } else {
                    div().h_flex().children(segments).into_any_element()
                }
            })
            .collect();

        div().id(self.id).v_flex().children(children)
    }
}

/// Splits text on `**bold**` markers and returns styled segments.
fn render_inline_bold(text: &str, fg: gpui::Hsla) -> Vec<AnyElement> {
    let mut segments: Vec<AnyElement> = Vec::new();
    let mut remaining = text;
    let mut found = false;

    while !remaining.is_empty() {
        if let Some(start) = remaining.find("**") {
            let after_start = &remaining[start + 2..];
            if let Some(end) = after_start.find("**") {
                found = true;
                // Text before bold
                if start > 0 {
                    segments.push(
                        div()
                            .text_color(fg)
                            .child(SharedString::from(remaining[..start].to_string()))
                            .into_any_element(),
                    );
                }
                // Bold text
                segments.push(
                    div()
                        .text_size(px(15.))
                        .text_color(fg)
                        .child(SharedString::from(after_start[..end].to_string()))
                        .into_any_element(),
                );
                remaining = &after_start[end + 2..];
                continue;
            }
        }
        // No (more) bold markers — emit remaining text
        segments.push(
            div()
                .text_color(fg)
                .child(SharedString::from(remaining.to_string()))
                .into_any_element(),
        );
        break;
    }

    if !found {
        segments.clear();
    }
    segments
}
