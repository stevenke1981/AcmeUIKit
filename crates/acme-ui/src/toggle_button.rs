use std::rc::Rc;

use gpui::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, StatefulInteractiveElement as _, Styled as _, Window, div,
};

use crate::{ActiveTheme, Size, StyledExt};

/// A toggle button (selected/unselected states).
///
/// # Example
///
/// ```ignore
/// ToggleButton::new("toggle-dark", "Dark Mode")
///     .selected(true)
/// ```
#[allow(clippy::type_complexity)]
#[derive(IntoElement)]
pub struct ToggleButton {
    id: ElementId,
    label: SharedString,
    selected: bool,
    disabled: bool,
    size: Size,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

impl ToggleButton {
    /// Creates a new toggle button.
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            selected: false,
            disabled: false,
            size: Size::Medium,
            on_click: None,
        }
    }

    /// Sets the selected state.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Disables the button.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the button size.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
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

impl RenderOnce for ToggleButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        let bg = if self.selected { c.primary } else { c.surface };
        let text_color = if self.selected {
            c.primary_foreground
        } else {
            c.foreground
        };
        let border_color = if self.selected { c.primary } else { c.border };

        let mut btn = div()
            .id(self.id)
            .h_flex()
            .items_center()
            .px(self.size.horizontal_padding())
            .h(self.size.height())
            .rounded(theme.radius)
            .bg(bg)
            .text_color(text_color)
            .text_size(self.size.text_size())
            .border_1()
            .border_color(border_color)
            .cursor_pointer()
            .child(self.label.clone());

        if self.disabled {
            btn = btn.opacity(0.4);
        } else if let Some(handler) = self.on_click {
            btn = btn.on_click(move |event, window, cx| handler(event, window, cx));
        }

        btn
    }
}

use gpui::SharedString;
