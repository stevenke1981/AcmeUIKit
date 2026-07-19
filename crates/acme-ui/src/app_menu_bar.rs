use std::rc::Rc;

use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StatefulInteractiveElement as _, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

type MenuSelectHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// Data for a single menu item in the application menu bar.
#[derive(Clone)]
pub struct MenuItemData {
    /// Display label, e.g. "File", "Edit".
    pub label: SharedString,
    /// Sub-menu item labels (currently rendered as a hint only).
    pub children: Vec<SharedString>,
}

impl MenuItemData {
    /// Creates a new menu item with the given label and sub-menu children.
    pub fn new(label: impl Into<SharedString>, children: Vec<impl Into<SharedString>>) -> Self {
        Self {
            label: label.into(),
            children: children.into_iter().map(|c| c.into()).collect(),
        }
    }
}

/// A horizontal application menu bar with labeled items and dropdown hints.
///
/// Renders as a slim surface bar with a bottom border. Each item shows its
/// label followed by a chevron-down icon. Items are clickable and fire the
/// `on_select` callback with the item index.
///
/// # Example
///
/// ```ignore
/// AppMenuBar::new()
///     .add("File", &["New", "Open", "Save", "Exit"])
///     .add("Edit", &["Undo", "Redo", "Cut", "Copy"])
///     .add("View", &["Zoom In", "Zoom Out", "Full Screen"])
///     .on_select(|index, _window, _cx| {
///         println!("Menu {index} clicked");
///     })
/// ```
#[derive(IntoElement, Default)]
pub struct AppMenuBar {
    items: Vec<MenuItemData>,
    on_select: Option<MenuSelectHandler>,
}

impl AppMenuBar {
    /// Creates a new empty `AppMenuBar`.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            on_select: None,
        }
    }

    /// Adds a menu item with the given label and sub-menu children labels.
    pub fn add(mut self, label: &str, children: &[&str]) -> Self {
        self.items.push(MenuItemData::new(
            label,
            children.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
        ));
        self
    }

    /// Registers a callback fired when a menu item is clicked.
    ///
    /// The handler receives the index of the clicked item.
    pub fn on_select(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for AppMenuBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let on_select = self.on_select;

        div()
            .h_flex()
            .items_center()
            .h(px(28.))
            .px(px(4.))
            .gap_3()
            .bg(c.surface)
            .border_b_1()
            .border_color(c.border)
            .children(self.items.into_iter().enumerate().map(|(index, item)| {
                let handler = on_select.clone();

                let mut el = div()
                    .id(ElementId::Name(format!("menu-{index}").into()))
                    .h_flex()
                    .items_center()
                    .gap_1()
                    .px(px(12.))
                    .py(px(2.))
                    .rounded(px(4.))
                    .cursor_pointer()
                    .text_size(theme.font_sizes.body)
                    .text_color(c.foreground)
                    .hover(|style| style.bg(c.muted))
                    .child(item.label)
                    .child(Icon::new(IconName::ChevronDown).with_size(px(10.)));

                if let Some(handler) = handler {
                    el = el.on_click(move |_event, window, cx| handler(index, window, cx));
                }

                el.into_any_element()
            }))
    }
}
