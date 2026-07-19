use std::rc::Rc;

use gpui::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::{ActiveTheme, StyledExt};

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// A styled list container.
///
/// # Example
///
/// ```ignore
/// List::new("items")
///     .item(ListItem::new("Apple").on_click(|_e, _w, _c| println!("selected")))
///     .item(ListItem::new("Banana"))
/// ```
#[derive(IntoElement)]
pub struct List {
    id: ElementId,
    children: Vec<ListItem>,
}

/// A single list item.
pub struct ListItem {
    label: SharedString,
    description: Option<SharedString>,
    selected: bool,
    disabled: bool,
    on_click: Option<ClickHandler>,
}

impl List {
    /// Creates a new list with the given [`ElementId`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
        }
    }

    /// Adds a list item.
    pub fn item(mut self, item: ListItem) -> Self {
        self.children.push(item);
        self
    }
}

impl ListItem {
    /// Creates a new list item.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            description: None,
            selected: false,
            disabled: false,
            on_click: None,
        }
    }

    /// Sets a description sub-text.
    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Marks the item as selected.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Disables the item.
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

impl RenderOnce for List {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let total = self.children.len();

        div()
            .id(self.id)
            .v_flex()
            .w_full()
            .rounded(px(6.))
            .border_1()
            .border_color(c.border)
            .overflow_hidden()
            .children(self.children.into_iter().enumerate().map(move |(i, item)| {
                let is_last = i == total - 1;
                let bg = if item.selected {
                    c.surface
                } else {
                    gpui::transparent_black()
                };

                let row_id = ElementId::Name(format!("list-item-{}", i).into());

                let mut row = div()
                    .id(row_id)
                    .h_flex()
                    .items_center()
                    .w_full()
                    .px(px(12.))
                    .py(px(10.))
                    .bg(bg)
                    .child(
                        div()
                            .v_flex()
                            .flex_1()
                            .child(
                                div()
                                    .text_size(theme.font_sizes.body)
                                    .text_color(if item.disabled {
                                        c.muted_foreground
                                    } else {
                                        c.foreground
                                    })
                                    .child(item.label.clone()),
                            )
                            .when_some(item.description, |this, desc| {
                                this.child(
                                    div()
                                        .text_size(theme.font_sizes.caption)
                                        .text_color(c.muted_foreground)
                                        .child(desc),
                                )
                            }),
                    );

                if !is_last {
                    row = row.child(div().w_full().h(px(1.)).bg(c.border));
                }

                if !item.disabled {
                    let handler = item.on_click.clone();
                    row = row.on_click(move |event, window, cx| {
                        if let Some(ref h) = handler {
                            h(event, window, cx);
                        }
                    });
                }

                row
            }))
    }
}
