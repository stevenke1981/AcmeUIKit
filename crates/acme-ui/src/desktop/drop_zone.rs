use std::rc::Rc;

use gpui::prelude::FluentBuilder as _;
use gpui::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// A file drop target area with icon, label, and hint text.
///
/// # Example
///
/// ```ignore
/// DropZone::new("file-upload")
///     .label("Upload files")
///     .hint("Click or drag files here")
///     .active(false)
///     .on_click(|_event, _window, _cx| { /* handle click */ })
/// ```
#[derive(IntoElement)]
pub struct DropZone {
    id: ElementId,
    label: SharedString,
    hint: SharedString,
    active: bool,
    on_click: Option<ClickHandler>,
}

impl DropZone {
    /// Creates a new drop zone with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: SharedString::default(),
            hint: SharedString::default(),
            active: false,
            on_click: None,
        }
    }

    /// Sets the primary label text.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    /// Sets the hint text shown below the label.
    pub fn hint(mut self, hint: impl Into<SharedString>) -> Self {
        self.hint = hint.into();
        self
    }

    /// Sets whether the zone is in an active (drag-over) state.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Registers a click handler for the zone.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for DropZone {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        let bg = if self.active { c.muted } else { c.background };

        div()
            .id(self.id)
            .v_flex()
            .items_center()
            .justify_center()
            .h(px(100.))
            .w_full()
            .gap_2()
            .rounded(theme.radius)
            .border_1()
            .border_color(c.border)
            .bg(bg)
            .cursor_pointer()
            .when_some(self.on_click, |this, handler| {
                this.on_click(move |event, window, cx| handler(event, window, cx))
            })
            .child(Icon::new(IconName::Folder).with_size(px(24.)))
            .child(div().text_color(c.foreground).child(self.label))
            .child(
                div()
                    .text_size(theme.font_sizes.caption)
                    .text_color(c.muted_foreground)
                    .child(self.hint),
            )
    }
}
