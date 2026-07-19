use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

/// Image cropping frame.
///
/// Renders a bordered area with an image placeholder and a centred
/// crop overlay frame. The aspect ratio is displayed as a label.
///
/// # Example
///
/// ```ignore
/// Cropper::new("crop-tool")
///     .label("Select crop area")
///     .aspect_ratio(16.0 / 9.0)
/// ```
#[derive(IntoElement)]
pub struct Cropper {
    id: ElementId,
    label: SharedString,
    aspect_ratio: f64,
}

impl Cropper {
    /// Creates a new cropper with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: SharedString::default(),
            aspect_ratio: 1.0,
        }
    }

    /// Sets the label text.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    /// Sets the crop aspect ratio (width / height).
    pub fn aspect_ratio(mut self, ratio: f64) -> Self {
        self.aspect_ratio = ratio;
        self
    }
}

impl RenderOnce for Cropper {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let font_sizes = cx.theme().font_sizes;

        let ratio_text = SharedString::from(format!("{:.2}:1", self.aspect_ratio));

        div()
            .id(self.id)
            .v_flex()
            .w_full()
            .h(px(300.))
            .border_1()
            .border_color(c.border)
            .rounded(px(6.))
            .overflow_hidden()
            .bg(c.surface)
            .child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(c.muted)
                    .child(
                        div()
                            .v_flex()
                            .items_center()
                            .child(
                                // Crop overlay frame: centered box with distinct border
                                div()
                                    .w(px(200.))
                                    .h(px(150.))
                                    .border_1()
                                    .border_color(c.primary)
                                    .rounded(px(2.))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(c.primary_foreground)
                                    .text_size(font_sizes.caption)
                                    .bg(gpui::hsla(
                                        0.,
                                        0.,
                                        1.,
                                        0.1,
                                    ))
                                    .child(Icon::new(IconName::Folder).with_size(px(20.))),
                            ),
                    ),
            )
            // Aspect ratio label and info
            .child(
                div()
                    .h_flex()
                    .h(px(28.))
                    .px(px(8.))
                    .bg(c.surface)
                    .border_color(c.border)
                    .gap_2()
                    .child(
                        div()
                            .text_size(font_sizes.caption)
                            .text_color(c.muted_foreground)
                            .child(ratio_text),
                    )
                    .child(
                        div()
                            .text_size(font_sizes.caption)
                            .text_color(c.muted_foreground)
                            .child(self.label),
                    ),
            )
    }
}
