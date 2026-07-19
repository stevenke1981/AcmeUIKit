use std::rc::Rc;

use gpui::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

type ClickHandler = Rc<dyn Fn(usize, &ClickEvent, &mut Window, &mut App)>;

/// A single item inside a [`Menu`].
///
/// Each item has a label, an optional disabled state, and an optional text-based icon.
#[derive(Clone)]
pub struct MenuItem {
    /// Display label.
    pub label: SharedString,
    /// When `true` the item cannot be selected and appears muted.
    pub disabled: bool,
    /// Optional text-based icon character (e.g. `"✓"`, `"✕"`).
    pub icon: Option<SharedString>,
}

impl MenuItem {
    /// Creates a new enabled item with the given label.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            disabled: false,
            icon: None,
        }
    }

    /// Sets the disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Attaches a text-based icon (e.g. `"✓"`, `"✕"`).
    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Dropdown menu component.
///
/// The caller controls open state, the item list, the selected index, and the
/// selection handler. When `open` is `false` nothing is rendered.
///
/// # Example
///
/// ```ignore
/// Menu::new("file-menu")
///     .open(self.menu_open)
///     .items(vec![
///         MenuItem::new("Open").icon("📂"),
///         MenuItem::new("Save").disabled(true),
///         MenuItem::new("Exit"),
///     ])
///     .selected(Some(0))
///     .on_select(|event, window, cx| {
///         // handle selection
///     })
/// ```
#[derive(IntoElement)]
pub struct Menu {
    id: ElementId,
    open: bool,
    items: Vec<MenuItem>,
    selected: Option<usize>,
    on_select: Option<ClickHandler>,
}

impl Menu {
    /// Creates a new menu with the given [`ElementId`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            items: Vec::new(),
            selected: None,
            on_select: None,
        }
    }

    /// Sets whether the dropdown is visible.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Sets the list of [`MenuItem`]s.
    pub fn items(mut self, items: Vec<MenuItem>) -> Self {
        self.items = items;
        self
    }

    /// Marks the item at `selected` as the currently selected one.
    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = Some(selected);
        self
    }

    /// Registers a click handler that fires when a non-disabled item is clicked.
    ///
    /// The handler receives the item index, the original click event, window,
    /// and app context.
    pub fn on_select(
        mut self,
        handler: impl Fn(usize, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Menu {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        if !self.open {
            return div().into_any_element();
        }

        let theme = cx.theme();
        let c = theme.colors;
        let on_select = self.on_select;

        div()
            .id(self.id)
            .v_flex()
            .py_1()
            .rounded(theme.radius)
            .border_1()
            .border_color(c.border)
            .bg(c.surface)
            .min_w(px(160.))
            .children(self.items.into_iter().enumerate().map(|(index, item)| {
                let selected = self.selected == Some(index);
                let disabled = item.disabled;
                let handler = on_select.clone();

                let mut row = div()
                    .id(ElementId::Name(format!("menu-item-{index}").into()))
                    .h(px(32.))
                    .px_2()
                    .flex()
                    .items_center()
                    .gap_2()
                    .text_size(cx.theme().font_sizes.body)
                    .text_color(if disabled {
                        c.muted_foreground
                    } else if selected {
                        c.primary
                    } else {
                        c.foreground
                    });

                if selected {
                    row = row.bg(c.muted);
                }

                if let Some(icon) = item.icon {
                    row = row.child(div().w(px(16.)).child(icon));
                }

                row = row.child(item.label);

                if let Some(handler) = handler {
                    if !disabled {
                        row = row
                            .cursor_pointer()
                            .hover(|style| style.bg(c.muted))
                            .on_click(
                                move |event: &ClickEvent, window: &mut Window, cx: &mut App| {
                                    handler(index, event, window, cx);
                                },
                            );
                    }
                }

                row.into_any_element()
            }))
            .into_any_element()
    }
}
