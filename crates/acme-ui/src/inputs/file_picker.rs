use std::rc::Rc;

use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, Button, Icon, IconName, StyledExt};

type BrowseHandler = Rc<dyn Fn(&mut Window, &mut App)>;

/// A file picker showing the selected path with a browse button.
///
/// # Example
///
/// ```ignore
/// FilePicker::new("file")
///     .path("/path/to/file.txt")
/// ```
#[derive(IntoElement)]
pub struct FilePicker {
    id: ElementId,
    path: SharedString,
    placeholder: SharedString,
    button_label: SharedString,
    on_browse: Option<BrowseHandler>,
}

impl FilePicker {
    /// Creates a new file picker.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            path: SharedString::new(""),
            placeholder: SharedString::from("No file selected"),
            button_label: SharedString::from("Browse…"),
            on_browse: None,
        }
    }

    /// Sets the selected file path.
    pub fn path(mut self, p: impl Into<SharedString>) -> Self {
        self.path = p.into();
        self
    }

    /// Sets the browse button label.
    pub fn button(mut self, label: impl Into<SharedString>) -> Self {
        self.button_label = label.into();
        self
    }

    /// Registers a browse click handler.
    pub fn on_browse(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_browse = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for FilePicker {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let is_empty = self.path.is_empty();
        let handler = self.on_browse.clone();

        div()
            .id(self.id)
            .h_flex()
            .items_center()
            .gap_2()
            .h(px(32.))
            .px(px(10.))
            .rounded(theme.radius)
            .bg(c.muted)
            .flex_1()
            .child(Icon::new(IconName::Folder).with_size(px(14.)))
            .child(
                div()
                    .flex_1()
                    .text_size(theme.font_sizes.body)
                    .text_color(if is_empty {
                        c.muted_foreground
                    } else {
                        c.foreground
                    })
                    .child(if is_empty {
                        self.placeholder
                    } else {
                        self.path
                    }),
            )
            .child(
                Button::new("browse")
                    .extra_small()
                    .label(self.button_label)
                    .on_click(move |_event, window, cx| {
                        if let Some(ref h) = handler {
                            h(window, cx);
                        }
                    }),
            )
    }
}
