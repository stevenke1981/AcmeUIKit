use std::rc::Rc;

use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    StatefulInteractiveElement as _, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

type RatingHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// A star rating input.
///
/// # Example
///
/// ```ignore
/// Rating::new("score")
///     .max(5)
///     .value(3)
/// ```
#[derive(IntoElement)]
pub struct Rating {
    id: ElementId,
    max: usize,
    value: usize,
    on_change: Option<RatingHandler>,
}

impl Rating {
    /// Creates a new rating.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            max: 5,
            value: 0,
            on_change: None,
        }
    }

    /// Sets the maximum rating value.
    pub fn max(mut self, n: usize) -> Self {
        self.max = n.clamp(1, 10);
        self
    }

    /// Sets the current rating value.
    pub fn value(mut self, v: usize) -> Self {
        self.value = v.min(self.max);
        self
    }

    /// Registers a change handler.
    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Rating {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        let stars: Vec<_> = (0..self.max)
            .map(|i| {
                let filled = i < self.value;
                let handler = self.on_change.clone();
                let star_id = ElementId::from(format!("star-{}", i));
                div()
                    .id(star_id)
                    .cursor_pointer()
                    .text_color(if filled { c.warning } else { c.muted })
                    .on_click(move |_event, window, cx| {
                        if let Some(ref h) = handler {
                            h(i + 1, window, cx);
                        }
                    })
                    .child(Icon::new(IconName::Star).with_size(px(18.)))
            })
            .collect();

        div().id(self.id).h_flex().gap_1().children(stars)
    }
}
