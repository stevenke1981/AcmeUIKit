use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Kbd, StyledExt};

/// A single keyboard shortcut entry.
struct ShortcutEntry {
    keys: SharedString,
    description: SharedString,
}

/// A keyboard shortcut reference table.
///
/// # Example
///
/// ```ignore
/// ShortcutManager::new("shortcuts")
///     .shortcut("Ctrl+S", "Save")
///     .shortcut("Ctrl+Z", "Undo")
///     .shortcut("Ctrl+Shift+Z", "Redo")
/// ```
#[derive(IntoElement)]
pub struct ShortcutManager {
    id: ElementId,
    shortcuts: Vec<ShortcutEntry>,
}

impl ShortcutManager {
    /// Creates a new empty shortcut reference table.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            shortcuts: Vec::new(),
        }
    }

    /// Adds a single shortcut entry.
    pub fn shortcut(
        mut self,
        keys: impl Into<SharedString>,
        description: impl Into<SharedString>,
    ) -> Self {
        self.shortcuts.push(ShortcutEntry {
            keys: keys.into(),
            description: description.into(),
        });
        self
    }
}

impl RenderOnce for ShortcutManager {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        let mut table = div()
            .id(self.id)
            .v_flex()
            .px_3()
            .py_2()
            .gap_1()
            .bg(c.surface)
            .rounded(theme.radius)
            .border_1()
            .border_color(c.border);

        for entry in self.shortcuts {
            table = table.child(
                div()
                    .h_flex()
                    .justify_between()
                    .py(px(4.))
                    .child(
                        div()
                            .text_size(theme.font_sizes.body)
                            .text_color(c.foreground)
                            .child(entry.description.clone()),
                    )
                    .child(Kbd::new(entry.keys.clone())),
            );
        }

        table
    }
}
