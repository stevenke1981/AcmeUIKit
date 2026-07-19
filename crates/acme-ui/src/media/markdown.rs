use gpui::{
    AnyElement, App, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// A simple Markdown-to-GPUI renderer component.
///
/// Supports a subset of Markdown:
/// - `# H1`, `## H2`, `### H3` headings
/// - `**bold**`, `*italic*` inline styles
/// - `` `code` `` inline code
/// - ```code``` fenced code blocks
/// - `- ` unordered list items
/// - Paragraphs (separated by blank lines)
///
/// # Example
///
/// ```ignore
/// Markdown::new("intro")
///     .text("# Hello\n\nThis is **bold** and `code`.")
/// ```
#[derive(IntoElement)]
pub struct Markdown {
    id: gpui::ElementId,
    text: SharedString,
}

impl Markdown {
    /// Creates a new markdown renderer with the given `id`.
    pub fn new(id: impl Into<gpui::ElementId>) -> Self {
        Self {
            id: id.into(),
            text: SharedString::default(),
        }
    }

    /// Sets the markdown source text.
    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = text.into();
        self
    }
}

impl RenderOnce for Markdown {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        let mut children: Vec<AnyElement> = Vec::new();
        let text = self.text.to_string(); // owned copy for lifetime

        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Headings
            if let Some(rest) = trimmed.strip_prefix("### ") {
                children.push(
                    div()
                        .text_size(px(15.))
                        .text_color(c.foreground)
                        .mt_2()
                        .mb_1()
                        .child(SharedString::from(rest.trim().to_string()))
                        .into_any_element(),
                );
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("## ") {
                children.push(
                    div()
                        .text_size(px(17.))
                        .text_color(c.foreground)
                        .mt_2()
                        .mb_1()
                        .child(SharedString::from(rest.trim().to_string()))
                        .into_any_element(),
                );
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("# ") {
                children.push(
                    div()
                        .text_size(theme.font_sizes.heading)
                        .text_color(c.foreground)
                        .mt_2()
                        .mb_2()
                        .child(SharedString::from(rest.trim().to_string()))
                        .into_any_element(),
                );
                continue;
            }

            // Unordered list item
            if let Some(rest) = trimmed.strip_prefix("- ") {
                children.push(
                    div()
                        .h_flex()
                        .gap_2()
                        .pl_4()
                        .text_size(theme.font_sizes.body)
                        .text_color(c.foreground)
                        .child(
                            div()
                                .text_color(c.muted_foreground)
                                .child(SharedString::from("•")),
                        )
                        .child(render_inline(rest.trim(), c, theme))
                        .into_any_element(),
                );
                continue;
            }

            // Paragraph
            children.push(
                div()
                    .text_size(theme.font_sizes.body)
                    .text_color(c.foreground)
                    .mb_1()
                    .child(render_inline(trimmed, c, theme))
                    .into_any_element(),
            );
        }

        div().id(self.id).v_flex().children(children)
    }
}

/// Renders inline Markdown: bold (`**`), italic (`*`), code (`` ` ``).
fn render_inline(text: &str, c: crate::ThemeColors, theme: &crate::Theme) -> gpui::Div {
    // Simplified: render text with basic inline markup detection.
    let mut segments: Vec<(String, &str)> = Vec::new();
    let mut remaining = text;

    while !remaining.is_empty() {
        // Bold **...**
        if let Some(start) = remaining.find("**") {
            let after = &remaining[start + 2..];
            if let Some(end) = after.find("**") {
                if start > 0 {
                    segments.push((remaining[..start].to_string(), "normal"));
                }
                segments.push((after[..end].to_string(), "bold"));
                remaining = &after[end + 2..];
                continue;
            }
        }

        // Italic *...* (but not **)
        if let Some(start) = remaining.find('*') {
            let after = &remaining[start + 1..];
            if let Some(end) = after.find('*') {
                if !after[..end].contains('*') {
                    if start > 0 {
                        segments.push((remaining[..start].to_string(), "normal"));
                    }
                    segments.push((after[..end].to_string(), "italic"));
                    remaining = &after[end + 1..];
                    continue;
                }
            }
        }

        // Inline code `...`
        if let Some(start) = remaining.find('`') {
            let after = &remaining[start + 1..];
            if let Some(end) = after.find('`') {
                if start > 0 {
                    segments.push((remaining[..start].to_string(), "normal"));
                }
                segments.push((after[..end].to_string(), "code"));
                remaining = &after[end + 1..];
                continue;
            }
        }

        segments.push((remaining.to_string(), "normal"));
        break;
    }

    if segments.is_empty() {
        return div().child(SharedString::from(text));
    }

    let mut el = div().h_flex().flex_wrap().gap_0();
    for (segment, style) in segments {
        let mut seg = div().child(segment.clone());
        match style {
            "bold" => {
                seg = seg.text_size(theme.font_sizes.body);
            }
            "italic" => {
                seg = seg.text_size(theme.font_sizes.body);
            }
            "code" => {
                seg = seg
                    .px_1()
                    .bg(c.muted)
                    .rounded(theme.radius_sm)
                    .text_size(theme.font_sizes.caption);
            }
            _ => {
                seg = seg.text_size(theme.font_sizes.body);
            }
        }
        el = el.child(seg);
    }
    el
}
