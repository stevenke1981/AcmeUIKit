use std::rc::Rc;

use gpui::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::{ActiveTheme, StyledExt};

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// An individual radio button item.
///
/// Stateless — the caller controls `selected` and handles `on_click`.
///
/// # Example
///
/// ```ignore
/// Radio::new("option-a", "a")
///     .selected(true)
///     .on_click(|_, _, _| { /* update selection */ })
/// ```
#[derive(IntoElement)]
pub struct Radio {
    id: ElementId,
    #[allow(dead_code)]
    value: SharedString,
    selected: bool,
    disabled: bool,
    label: Option<SharedString>,
    on_click: Option<ClickHandler>,
}

impl Radio {
    /// Create a new [`Radio`] with the given stable `id` and a `value` identifying this option.
    pub fn new(id: impl Into<ElementId>, value: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            value: value.into(),
            selected: false,
            disabled: false,
            label: None,
            on_click: None,
        }
    }

    /// Set whether this radio is selected.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Disable user interaction.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set an optional label displayed to the right of the radio circle.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Register a click handler. Fires when this radio is clicked (only when enabled).
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Radio {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let disabled = self.disabled;
        let handler = self.on_click;

        let circle_size = px(18.);
        let dot_size = px(10.);

        // ── outer circle ───────────────────────────────────────────────────
        let circle = div()
            .size(circle_size)
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .border_2()
            .border_color(if self.selected {
                c.primary
            } else if disabled {
                c.muted
            } else {
                c.border
            })
            .bg(c.background);

        // ── inner dot (when selected) ──────────────────────────────────────
        let circle = if self.selected {
            circle.child(div().size(dot_size).rounded_full().bg(c.primary))
        } else {
            circle
        };

        let circle = circle.when(!disabled, |this| {
            this.cursor_pointer().hover(|style| style.bg(c.muted))
        });

        // ── assemble row ───────────────────────────────────────────────────
        let mut row = div()
            .id(self.id)
            .flex()
            .items_center()
            .gap_2()
            .child(circle);

        if let Some(label) = self.label {
            row = row.child(
                div()
                    .text_color(if disabled {
                        c.muted_foreground
                    } else {
                        c.foreground
                    })
                    .text_size(px(13.))
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

/// A stateless container that renders a group of [`Radio`] buttons.
///
/// The caller controls the selected value and handles click events.
///
/// # Example
///
/// ```ignore
/// RadioGroup::new()
///     .selected("option-b")
///     .children(
///         Radio::new("opt-a", "a")
///             .label("Option A")
///             .selected(selected == "a")
///             .on_click(cx.listener(|this, _, _, cx| { ... })),
///         Radio::new("opt-b", "b")
///             .label("Option B")
///             .selected(selected == "b")
///             .on_click(cx.listener(|this, _, _, cx| { ... })),
///     )
/// ```
#[derive(IntoElement, Default)]
pub struct RadioGroup {
    selected: SharedString,
    children: Vec<AnyElement>,
}

impl RadioGroup {
    /// Create a new empty [`RadioGroup`].
    pub fn new() -> Self {
        Self {
            selected: SharedString::default(),
            children: Vec::new(),
        }
    }

    /// Set the currently selected value.
    /// Individual [`Radio`] items should still set their own `.selected()`.
    pub fn selected(mut self, selected: impl Into<SharedString>) -> Self {
        self.selected = selected.into();
        self
    }
}

impl gpui::ParentElement for RadioGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for RadioGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().v_flex().gap_2().children(self.children)
    }
}
