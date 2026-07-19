use std::rc::Rc;

use gpui::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::{ActiveTheme, StyledExt};

type SelectHandler = Rc<dyn Fn(usize, &ClickEvent, &mut Window, &mut App)>;

/// A single option inside a [`Select`].
///
/// Each option has a display label, a backing value, and an optional disabled
/// state.
#[derive(Clone)]
pub struct SelectOption {
    /// Display label shown in the list.
    pub label: SharedString,
    /// Backing value carried by the option.
    pub value: SharedString,
    /// When `true` the option cannot be selected and appears muted.
    pub disabled: bool,
}

impl SelectOption {
    /// Creates a new enabled option with the given label and value.
    pub fn new(label: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: false,
        }
    }

    /// Sets the disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Dropdown select component.
///
/// The caller controls open/close state, the option list, the selected index,
/// and the selection handler. When `open` is `false` only the trigger button is
/// rendered. When `open` is `true` the dropdown list is shown below the trigger.
///
/// # Example
///
/// ```ignore
/// Select::new("color-select")
///     .placeholder("Choose a color…")
///     .options(vec![
///         SelectOption::new("Red", "#ff0000"),
///         SelectOption::new("Green", "#00ff00").disabled(true),
///         SelectOption::new("Blue", "#0000ff"),
///     ])
///     .selected(Some(0))
///     .open(self.select_open)
///     .on_select(|index, _event, _window, _cx| {
///         // handle selection
///     })
/// ```
#[derive(IntoElement)]
pub struct Select {
    id: ElementId,
    placeholder: SharedString,
    options: Vec<SelectOption>,
    selected: Option<usize>,
    open: bool,
    disabled: bool,
    on_select: Option<SelectHandler>,
}

impl Select {
    /// Creates a new select with the given [`ElementId`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            placeholder: SharedString::default(),
            options: Vec::new(),
            selected: None,
            open: false,
            disabled: false,
            on_select: None,
        }
    }

    /// Text shown when no option is selected.
    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Sets the list of [`SelectOption`]s.
    pub fn options(mut self, options: Vec<SelectOption>) -> Self {
        self.options = options;
        self
    }

    /// Marks the option at `index` as the currently selected one.
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = Some(index);
        self
    }

    /// Sets whether the dropdown list is visible.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Disables the entire select.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Registers a click handler that fires when a non-disabled option is
    /// clicked.
    ///
    /// The handler receives the option index, the original click event,
    /// window, and app context.
    pub fn on_select(
        mut self,
        handler: impl Fn(usize, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Select {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let disabled = self.disabled;

        // Determine the trigger label: selected option's label or placeholder.
        let trigger_label = self
            .selected
            .and_then(|idx| self.options.get(idx))
            .map(|opt| opt.label.clone())
            .unwrap_or_else(|| self.placeholder.clone());

        // Build the trigger button.
        let mut trigger = div()
            .id(self.id.clone())
            .h_flex()
            .h(px(36.))
            .px_3()
            .gap_2()
            .rounded(theme.radius)
            .border_1()
            .border_color(c.border)
            .bg(c.surface)
            .text_size(theme.font_sizes.body)
            .text_color(if disabled {
                c.muted_foreground
            } else {
                c.foreground
            })
            .child(div().flex_1().child(trigger_label))
            .child(div().text_color(c.muted_foreground).child("▾"));

        if !disabled {
            trigger = trigger
                .cursor_pointer()
                .hover(|style| style.border_color(c.ring));
        }

        // Wrap the trigger and optional dropdown in a column container.
        div()
            .v_flex()
            .child(trigger)
            .when(self.open, |this| {
                let on_select = self.on_select;

                this.child(
                    div()
                        .v_flex()
                        .mt_1()
                        .py_1()
                        .rounded(theme.radius)
                        .border_1()
                        .border_color(c.border)
                        .bg(c.surface)
                        .min_w(px(180.))
                        .children(self.options.into_iter().enumerate().map(|(index, option)| {
                            let selected = self.selected == Some(index);
                            let opt_disabled = option.disabled || disabled;
                            let handler = on_select.clone();

                            let mut row = div()
                                .id(ElementId::Name(format!("select-option-{index}").into()))
                                .h(px(32.))
                                .px_3()
                                .h_flex()
                                .gap_2()
                                .text_size(theme.font_sizes.body)
                                .text_color(if opt_disabled {
                                    c.muted_foreground
                                } else if selected {
                                    c.primary
                                } else {
                                    c.foreground
                                });

                            if selected {
                                row = row.bg(c.muted);
                            }

                            row = row.child(option.label);

                            if let Some(handler) = handler {
                                if !opt_disabled {
                                    row = row
                                            .cursor_pointer()
                                            .hover(|style| style.bg(c.muted))
                                            .on_click(
                                                move |event: &ClickEvent,
                                                      window: &mut Window,
                                                      cx: &mut App| {
                                                    handler(index, event, window, cx);
                                                },
                                            );
                                }
                            }

                            row.into_any_element()
                        })),
                )
            })
            .into_any_element()
    }
}
