use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    Styled as _, Window, div,
};

use crate::{ActiveTheme, NavigationRail, StyledExt};

/// A top-level layout container with a vertical navigation rail and a content area.
///
/// Renders the provided [`NavigationRail`] on the left and an empty flex-1
/// content area on the right. The content area uses the background color by
/// default so callers can place their own children inside a wrapper layout.
///
/// # Example
///
/// ```ignore
/// NavigationView::new("main-nav")
///     .rail(rail)
/// ```
#[derive(IntoElement)]
pub struct NavigationView {
    id: ElementId,
    rail: Option<NavigationRail>,
}

impl NavigationView {
    /// Creates a new `NavigationView` with the given [`ElementId`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            rail: None,
        }
    }

    /// Sets the navigation rail displayed on the left side.
    pub fn rail(mut self, rail: NavigationRail) -> Self {
        self.rail = Some(rail);
        self
    }
}

impl RenderOnce for NavigationView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;

        div()
            .id(self.id)
            .h_flex()
            .flex_1()
            .h_full()
            .children(self.rail)
            .child(div().flex_1().bg(c.background))
    }
}
