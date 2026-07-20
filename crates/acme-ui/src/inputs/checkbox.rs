use std::rc::Rc;

use gpui::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div,
    prelude::FluentBuilder as _,
};

use crate::ActiveTheme;

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// A boolean toggle checkbox with an optional label.
///
/// Stateless — the caller controls `checked` and handles `on_click`.
///
/// # Example
///
/// ```ignore
/// Checkbox::new("agree", true)
///     .label("I agree to the terms")
///     .on_click(|_, _, cx| { /* toggle state */ })
/// ```
#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    checked: bool,
    disabled: bool,
    label: Option<SharedString>,
    on_click: Option<ClickHandler>,
}

impl Checkbox {
    /// Create a new [`Checkbox`] with the given stable `id` and initial `checked` state.
    pub fn new(id: impl Into<ElementId>, checked: bool) -> Self {
        Self {
            id: id.into(),
            checked,
            disabled: false,
            label: None,
            on_click: None,
        }
    }

    /// Set the label text displayed to the right of the checkbox box.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Disable user interaction.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Register a click handler. Fires when the checkbox is clicked (only when enabled).
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Checkbox {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let disabled = self.disabled;
        let handler = self.on_click;

        // ── check-box square ───────────────────────────────────────────────
        let check_size = cx.theme().controls.small;
        let box_el = div()
            .size(check_size)
            .flex()
            .items_center()
            .justify_center()
            .rounded(cx.theme().radius_sm)
            .border_1()
            .border_color(if self.checked { c.primary } else { c.border })
            .bg(if self.checked {
                if disabled { c.muted } else { c.primary }
            } else {
                c.background
            })
            .text_color(if self.checked {
                c.primary_foreground
            } else {
                c.muted_foreground
            })
            .text_size(cx.theme().font_sizes.caption)
            .when(!disabled, |this| {
                this.cursor_pointer().hover(|style| style.bg(c.muted))
            })
            .child(if self.checked { "✓" } else { "" });

        // ── assemble ───────────────────────────────────────────────────────
        let mut row = div()
            .id(self.id)
            .flex()
            .items_center()
            .gap_2()
            .child(box_el);

        if let Some(label) = self.label {
            row = row.child(
                div()
                    .text_color(if disabled {
                        c.muted_foreground
                    } else {
                        c.foreground
                    })
                    .text_size(cx.theme().font_sizes.body)
                    .child(label),
            );
        }

        match handler {
            Some(handler) if !disabled => row
                .on_click(move |event, window, cx| handler(event, window, cx))
                .into_any_element(),
            _ => row.into_any_element(),
        }
    }
}
