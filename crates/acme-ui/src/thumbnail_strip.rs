use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

/// A single thumbnail item.
pub struct ThumbnailItem {
    /// The label text displayed on the thumbnail.
    pub label: SharedString,
}

/// Horizontal strip of thumbnails.
///
/// Renders a horizontal scrollable strip of bordered thumbnail
/// placeholders. The selected item is highlighted with a primary ring.
///
/// # Example
///
/// ```ignore
/// ThumbnailStrip::new("gallery-thumbs")
///     .item("Photo 1")
///     .item("Photo 2")
///     .item("Photo 3")
///     .selected(1)
/// ```
#[derive(IntoElement)]
pub struct ThumbnailStrip {
    id: ElementId,
    items: Vec<ThumbnailItem>,
    selected: Option<usize>,
}

impl ThumbnailStrip {
    /// Creates a new thumbnail strip with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            selected: None,
        }
    }

    /// Adds a thumbnail item with the given label.
    pub fn item(mut self, label: impl Into<SharedString>) -> Self {
        self.items.push(ThumbnailItem {
            label: label.into(),
        });
        self
    }

    /// Sets the selected item index.
    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = Some(selected);
        self
    }
}

impl RenderOnce for ThumbnailStrip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let font_sizes = cx.theme().font_sizes;

        div()
            .id(self.id)
            .h_flex()
            .gap_2()
            .overflow_hidden()
            .px(px(8.))
            .py(px(8.))
            .children(self.items.into_iter().enumerate().map(move |(i, item)| {
                let is_selected = self.selected == Some(i);

                let mut thumb = div()
                    .id(ElementId::Name(SharedString::from(format!("thumb-{}", i))))
                    .v_flex()
                    .items_center()
                    .justify_center()
                    .w(px(60.))
                    .h(px(48.))
                    .rounded(px(4.))
                    .bg(c.muted)
                    .border_1()
                    .overflow_hidden()
                    .text_color(c.muted_foreground)
                    .text_size(font_sizes.caption)
                    .child(item.label);

                if is_selected {
                    thumb = thumb.border_color(c.ring);
                } else {
                    thumb = thumb.border_color(c.border);
                }

                thumb
            }))
    }
}
