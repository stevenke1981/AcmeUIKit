use std::rc::Rc;

use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StatefulInteractiveElement as _, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName, StyledExt};

type NavSelectHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// Data for a single navigation item in a [`NavigationRail`].
#[derive(Clone)]
pub struct NavItemData {
    /// Icon displayed for the nav item.
    pub icon: IconName,
    /// Label displayed below the icon.
    pub label: SharedString,
}

impl NavItemData {
    /// Creates a new nav item with the given icon and label.
    pub fn new(icon: IconName, label: impl Into<SharedString>) -> Self {
        Self {
            icon,
            label: label.into(),
        }
    }
}

/// A vertical navigation rail with icon buttons and labels.
///
/// Renders as a narrow vertical strip with a surface background and a right
/// border. Each item is displayed as a column with an icon above a small
/// caption label. The selected item is highlighted with the primary accent.
///
/// # Example
///
/// ```ignore
/// NavigationRail::new()
///     .item(IconName::Folder, "Files")
///     .item(IconName::Search, "Search")
///     .item(IconName::Settings, "Settings")
///     .selected(0)
///     .on_select(|index, _window, _cx| {
///         println!("Nav item {index} selected");
///     })
/// ```
#[derive(IntoElement)]
pub struct NavigationRail {
    id: ElementId,
    items: Vec<NavItemData>,
    selected: usize,
    on_select: Option<NavSelectHandler>,
}

impl NavigationRail {
    /// Creates a new `NavigationRail` with the given [`ElementId`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            selected: 0,
            on_select: None,
        }
    }

    /// Adds a navigation item with the given icon and label.
    pub fn item(mut self, icon: IconName, label: impl Into<SharedString>) -> Self {
        self.items.push(NavItemData::new(icon, label));
        self
    }

    /// Sets the index of the selected item.
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    /// Registers a callback fired when a navigation item is clicked.
    ///
    /// The handler receives the index of the clicked item.
    pub fn on_select(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for NavigationRail {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let caption_size = theme.font_sizes.caption;
        let selected = self.selected;
        let on_select = self.on_select;

        div()
            .id(self.id)
            .v_flex()
            .items_center()
            .gap_4()
            .py(px(16.))
            .bg(c.surface)
            .border_r_1()
            .border_color(c.border)
            .w(px(48.))
            .h_full()
            .children(self.items.into_iter().enumerate().map(|(index, item)| {
                let is_selected = index == selected;
                let handler = on_select.clone();

                let mut el = div()
                    .id(ElementId::Name(format!("nav-item-{index}").into()))
                    .v_flex()
                    .items_center()
                    .gap_1()
                    .cursor_pointer()
                    .w_full()
                    .py(px(8.))
                    .px(px(4.))
                    .rounded(px(6.));

                if is_selected {
                    el = el.bg(c.muted).text_color(c.primary);
                } else {
                    el = el.text_color(c.muted_foreground);
                }
                el = el.hover(|style| style.bg(c.muted));

                if let Some(handler) = handler {
                    el = el.on_click(move |_event, window, cx| handler(index, window, cx));
                }

                el.child(div().child(Icon::new(item.icon).with_size(px(18.))))
                    .child(
                        div()
                            .text_size(caption_size)
                            .text_color(c.muted_foreground)
                            .child(item.label),
                    )
                    .into_any_element()
            }))
    }
}
