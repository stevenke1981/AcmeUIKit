use gpui::prelude::FluentBuilder as _;
use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

/// A single slide in a carousel.
pub struct CarouselSlide {
    /// Slide label text.
    pub label: SharedString,
    /// Slide placeholder color.
    pub color: gpui::Hsla,
}

/// Image carousel / slideshow.
///
/// # Example
///
/// ```ignore
/// Carousel::new("slideshow")
///     .slide("Sunrise", 30.0, 60.0, 50.0)
///     .slide("Ocean", 200.0, 50.0, 45.0)
///     .current(0)
/// ```
#[derive(IntoElement)]
pub struct Carousel {
    id: ElementId,
    slides: Vec<CarouselSlide>,
    current: usize,
}

impl Carousel {
    /// Creates a new carousel with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            slides: Vec::new(),
            current: 0,
        }
    }

    /// Adds a slide with a label and HSL color (hue 0-360, saturation 0-100, lightness 0-100).
    pub fn slide(mut self, label: impl Into<SharedString>, h: f64, s: f64, l: f64) -> Self {
        self.slides.push(CarouselSlide {
            label: label.into(),
            color: gpui::hsla(h as f32 / 360.0, s as f32 / 100.0, l as f32 / 100.0, 1.0),
        });
        self
    }

    /// Sets the current slide index.
    pub fn current(mut self, current: usize) -> Self {
        self.current = current;
        self
    }
}

impl RenderOnce for Carousel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let font_sizes = cx.theme().font_sizes;
        let slide_count = self.slides.len();
        let current = self.current.min(slide_count.saturating_sub(1));

        // Current slide data
        let slide = self.slides.get(current);

        div()
            .id(self.id)
            .v_flex()
            .w_full()
            .h(px(200.))
            .bg(c.surface)
            .border_1()
            .border_color(c.border)
            .rounded(px(8.))
            .overflow_hidden()
            // Slide content area
            .child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .when_some(slide, |this, s| {
                        this.bg(s.color)
                            .child(
                                div()
                                    .text_size(font_sizes.body)
                                    .text_color(gpui::white())
                                    .child(s.label.clone()),
                            )
                    })
                    .when(slide.is_none(), |this| {
                        this.bg(c.muted)
                            .child(
                                div()
                                    .text_color(c.muted_foreground)
                                    .child("No slides"),
                            )
                    }),
            )
            // Navigation row (arrows + dots)
            .child(
                div()
                    .h_flex()
                    .h(px(36.))
                    .px(px(12.))
                    .bg(c.surface)
                    .border_color(c.border)
                    .justify_between()
                    .child(
                        div()
                            .id(ElementId::Name(SharedString::from("carousel-prev")))
                            .cursor_pointer()
                            .text_color(c.foreground)
                            .child(Icon::new(IconName::ChevronLeft)),
                    )
                    .child(
                        div()
                            .h_flex()
                            .gap_1()
                            .children((0..slide_count).map(move |i| {
                                let is_active = i == current;
                                div()
                                    .id(ElementId::Name(
                                        SharedString::from(format!("carousel-dot-{}", i)),
                                    ))
                                    .w(px(8.))
                                    .h(px(8.))
                                    .rounded_full()
                                    .bg(if is_active { c.primary } else { c.muted })
                                    .cursor_pointer()
                            })),
                    )
                    .child(
                        div()
                            .id(ElementId::Name(SharedString::from("carousel-next")))
                            .cursor_pointer()
                            .text_color(c.foreground)
                            .child(Icon::new(IconName::ChevronRight)),
                    ),
            )
    }
}
