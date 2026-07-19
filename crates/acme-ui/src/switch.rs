use std::rc::Rc;

use gpui::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, StatefulInteractiveElement as _, Styled as _, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::ActiveTheme;

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// Boolean toggle switch. State is controlled by the caller.
#[derive(IntoElement)]
pub struct Switch {
    id: ElementId,
    checked: bool,
    disabled: bool,
    on_click: Option<ClickHandler>,
}

impl Switch {
    pub fn new(id: impl Into<ElementId>, checked: bool) -> Self {
        Self {
            id: id.into(),
            checked,
            disabled: false,
            on_click: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Switch {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let disabled = self.disabled;
        let handler = self.on_click;
        let track = div()
            .id(self.id)
            .w(px(42.))
            .h(px(24.))
            .px(px(3.))
            .flex()
            .items_center()
            .rounded_full()
            .bg(if self.checked { c.primary } else { c.muted })
            .when(self.checked, |this| this.justify_end())
            .when(!self.checked, |this| this.justify_start())
            .when(!disabled, |this| this.cursor_pointer())
            .child(div().size(px(18.)).rounded_full().bg(c.surface));

        match handler {
            Some(handler) if !disabled => track
                .on_click(move |event, window, cx| handler(event, window, cx))
                .into_any_element(),
            _ => track.into_any_element(),
        }
    }
}
