use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// A single annotation note.
pub struct Annotation {
    /// Annotation text content.
    pub text: SharedString,
    /// X position (normalized 0–1 or pixel offset).
    pub x: f32,
    /// Y position (normalized 0–1 or pixel offset).
    pub y: f32,
}

/// Annotation overlay for images.
///
/// Renders a container with annotation badges positioned at
/// the specified coordinates. Each annotation is shown as a
/// small badge with text.
///
/// # Example
///
/// ```ignore
/// AnnotationLayer::new("annotations")
///     .add("Tree", 0.2, 0.3)
///     .add("House", 0.7, 0.6)
/// ```
#[derive(IntoElement)]
pub struct AnnotationLayer {
    id: ElementId,
    annotations: Vec<Annotation>,
}

impl AnnotationLayer {
    /// Creates a new annotation layer with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            annotations: Vec::new(),
        }
    }

    /// Adds an annotation with text and position.
    pub fn add(mut self, text: impl Into<SharedString>, x: f32, y: f32) -> Self {
        self.annotations.push(Annotation {
            text: text.into(),
            x,
            y,
        });
        self
    }
}

impl RenderOnce for AnnotationLayer {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let font_sizes = cx.theme().font_sizes;

        div()
            .id(self.id)
            .v_flex()
            .gap_2()
            .px(px(8.))
            .py(px(8.))
            .children(
                self.annotations
                    .into_iter()
                    .enumerate()
                    .map(move |(i, annotation)| {
                        div()
                            .id(ElementId::Name(SharedString::from(format!(
                                "annotation-{}",
                                i
                            ))))
                            .h_flex()
                            .items_center()
                            .gap_2()
                            .child(div().w(px(6.)).h(px(6.)).rounded_full().bg(c.primary))
                            .child(
                                div()
                                    .px(px(8.))
                                    .py(px(2.))
                                    .bg(c.surface)
                                    .border_1()
                                    .border_color(c.border)
                                    .rounded(px(4.))
                                    .text_size(font_sizes.caption)
                                    .text_color(c.foreground)
                                    .child(annotation.text),
                            )
                            .child(
                                div()
                                    .text_size(font_sizes.caption)
                                    .text_color(c.muted_foreground)
                                    .child(format!("({:.1}, {:.1})", annotation.x, annotation.y)),
                            )
                    }),
            )
    }
}
