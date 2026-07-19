use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Separator, StyledExt};

/// A single item within an inspector section.
struct InspectorItem {
    label: SharedString,
    value: SharedString,
}

/// A titled section containing inspector items.
struct InspectorSection {
    title: SharedString,
    items: Vec<InspectorItem>,
}

/// A right-side property inspector panel with labeled sections.
///
/// # Example
///
/// ```ignore
/// InspectorPanel::new("props")
///     .section("Geometry", &[("Width", "400px"), ("Height", "300px")])
///     .section("Appearance", &[("Opacity", "1.0"), ("Radius", "8px")])
/// ```
#[derive(IntoElement)]
pub struct InspectorPanel {
    id: ElementId,
    sections: Vec<InspectorSection>,
}

impl InspectorPanel {
    /// Creates a new empty inspector panel.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            sections: Vec::new(),
        }
    }

    /// Adds a section with the given title and label-value rows.
    pub fn section(mut self, title: impl Into<SharedString>, items: &[(&str, &str)]) -> Self {
        let section = InspectorSection {
            title: title.into(),
            items: items
                .iter()
                .map(|(label, value)| InspectorItem {
                    label: SharedString::from(*label),
                    value: SharedString::from(*value),
                })
                .collect(),
        };
        self.sections.push(section);
        self
    }
}

impl RenderOnce for InspectorPanel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        let mut panel = div()
            .id(self.id)
            .v_flex()
            .w(px(240.))
            .h_full()
            .bg(c.surface)
            .border_l_1()
            .border_color(c.border);

        for (i, section) in self.sections.into_iter().enumerate() {
            if i > 0 {
                panel = panel.child(Separator::new());
            }

            // Section header
            panel = panel.child(
                div()
                    .px_2()
                    .py_1()
                    .text_size(theme.font_sizes.caption)
                    .text_color(c.muted_foreground)
                    .child(section.title.clone()),
            );

            // Section items
            for item in section.items {
                panel = panel.child(
                    div()
                        .h_flex()
                        .px_2()
                        .py(px(4.))
                        .child(
                            div()
                                .flex_1()
                                .text_size(theme.font_sizes.body)
                                .text_color(c.muted_foreground)
                                .child(item.label.clone()),
                        )
                        .child(
                            div()
                                .text_size(theme.font_sizes.body)
                                .text_color(c.foreground)
                                .child(item.value.clone()),
                        ),
                );
            }
        }

        panel
    }
}
