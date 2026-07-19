use std::rc::Rc;

use gpui::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, StatefulInteractiveElement as _, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, Size};

/// An icon-only button.
///
/// # Example
///
/// ```ignore
/// IconButton::new("search-btn", IconName::Search)
///     .size(Size::Small)
/// ```
#[allow(clippy::type_complexity)]
#[derive(IntoElement)]
pub struct IconButton {
    id: ElementId,
    icon: IconName,
    size: Size,
    disabled: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

impl IconButton {
    /// Creates a new icon button.
    pub fn new(id: impl Into<ElementId>, icon: IconName) -> Self {
        Self {
            id: id.into(),
            icon,
            size: Size::Medium,
            disabled: false,
            on_click: None,
        }
    }

    /// Sets the button size.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Disables the button.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Registers a click handler.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for IconButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let icon_size = match self.size {
            Size::ExtraSmall => px(12.),
            Size::Small => px(14.),
            Size::Medium => px(16.),
            Size::Large => px(20.),
        };

        let mut btn = div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .rounded(theme.radius)
            .cursor_pointer()
            .text_color(c.foreground)
            .hover(|style| style.bg(c.muted))
            .child(Icon::new(self.icon).with_size(icon_size));

        let dim = self.size.height();
        btn = btn.w(dim).h(dim);

        if self.disabled {
            btn = btn.opacity(0.4);
        } else if let Some(handler) = self.on_click {
            btn = btn.on_click(move |event, window, cx| handler(event, window, cx));
        }

        btn
    }
}
