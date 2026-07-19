use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Icon, IconName};

/// A system tray indicator icon.
///
/// # Example
///
/// ```ignore
/// SystemTray::new("tray-clock")
///     .icon(IconName::Clock)
///     .tooltip("System Clock")
/// ```
#[derive(IntoElement)]
pub struct SystemTray {
    id: ElementId,
    icon: IconName,
    tooltip: SharedString,
}

impl SystemTray {
    /// Creates a new system tray indicator.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            icon: IconName::Menu,
            tooltip: SharedString::default(),
        }
    }

    /// Sets the tray icon.
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = icon;
        self
    }

    /// Sets the tooltip text.
    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = tooltip.into();
        self
    }
}

impl RenderOnce for SystemTray {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;

        div()
            .id(self.id)
            .w(px(20.))
            .h(px(20.))
            .flex()
            .items_center()
            .justify_center()
            .text_color(c.muted_foreground)
            .child(Icon::new(self.icon))
    }
}
