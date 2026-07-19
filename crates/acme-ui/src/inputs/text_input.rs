use std::rc::Rc;

use gpui::{
    App, Context, ElementId, FocusHandle, InteractiveElement as _, IntoElement, KeyDownEvent,
    ParentElement as _, Render, SharedString, Styled as _, Window, div, px,
};

use crate::ActiveTheme;

type InputHandler = Rc<dyn Fn(&str, &mut Window, &mut App)>;

/// An editable single-line text input with cursor, placeholder, focus ring,
/// and keyboard navigation.
///
/// Stateful (Entity + Render) — the entity owns the text buffer and cursor.
///
/// # Example
///
/// ```ignore
/// cx.new(|cx| {
///     TextInput::new("search", cx)
///         .placeholder("Type to search…")
///         .on_input(|value, _, _| { eprintln!("value: {value}"); })
/// })
/// ```
pub struct TextInput {
    id: ElementId,
    text: String,
    cursor_position: usize,
    focus_handle: FocusHandle,
    placeholder: SharedString,
    disabled: bool,
    on_input: Option<InputHandler>,
}

impl TextInput {
    /// Create a new [`TextInput`] entity.
    ///
    /// `id` is the stable [`ElementId`] for the interactive element.
    /// `cx` is the entity-building context from [`AppContext::new`].
    pub fn new(id: impl Into<ElementId>, cx: &mut Context<Self>) -> Self {
        Self {
            id: id.into(),
            text: String::new(),
            cursor_position: 0,
            focus_handle: cx.focus_handle(),
            placeholder: SharedString::default(),
            disabled: false,
            on_input: None,
        }
    }

    /// Placeholder text shown when the input is empty and not focused.
    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Disable user interaction.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Register a callback that fires on every text change.
    ///
    /// The callback receives the full current text value.
    pub fn on_input(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_input = Some(Rc::new(handler));
        self
    }

    // ── key handling ────────────────────────────────────────────────────────

    fn handle_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut App) {
        if self.disabled {
            return;
        }

        let keystroke = &event.keystroke;
        let chars: Vec<char> = self.text.chars().collect();
        let cursor = self.cursor_position.min(chars.len());

        match keystroke.key.as_str() {
            "backspace" if cursor > 0 => {
                let mut new_chars: Vec<char> = chars.into_iter().collect();
                new_chars.remove(cursor - 1);
                self.text = new_chars.into_iter().collect();
                self.cursor_position = cursor.saturating_sub(1);
                self.fire_input(window, cx);
            }
            "delete" if cursor < chars.len() => {
                let mut new_chars: Vec<char> = chars.into_iter().collect();
                new_chars.remove(cursor);
                self.text = new_chars.into_iter().collect();
                self.fire_input(window, cx);
            }
            "left" => {
                self.cursor_position = cursor.saturating_sub(1);
            }
            "right" => {
                self.cursor_position = (cursor + 1).min(chars.len());
            }
            "home" => {
                self.cursor_position = 0;
            }
            "end" => {
                self.cursor_position = chars.len();
            }
            _ => {
                // Printable character input via key_char
                if let Some(key_char) = &keystroke.key_char {
                    // Only accept non-modifier printable input
                    if !keystroke.modifiers.alt
                        && !keystroke.modifiers.control
                        && !keystroke.modifiers.platform
                    {
                        let mut new_chars: Vec<char> = chars.into_iter().collect();
                        for c in key_char.chars() {
                            new_chars.insert(self.cursor_position, c);
                            self.cursor_position += 1;
                        }
                        self.text = new_chars.into_iter().collect();
                        self.fire_input(window, cx);
                    }
                }
            }
        }
    }

    fn fire_input(&mut self, window: &mut Window, cx: &mut App) {
        if let Some(ref handler) = self.on_input {
            handler(&self.text, window, cx);
        }
    }
}

impl Render for TextInput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;
        let disabled = self.disabled;
        let focused = self.focus_handle.is_focused(window);

        // ── content (text / placeholder / cursor) ──────────────────────────
        let content: gpui::AnyElement = if !self.text.is_empty() {
            // Show text with cursor bar at the correct position
            let chars: Vec<char> = self.text.chars().collect();
            let cursor = self.cursor_position.min(chars.len());

            if focused {
                let before: String = chars[..cursor].iter().collect();
                let after: String = chars[cursor..].iter().collect();
                div()
                    .child(gpui::SharedString::from(before))
                    .child(div().child("|"))
                    .child(gpui::SharedString::from(after))
                    .into_any_element()
            } else {
                div().child(self.text.clone()).into_any_element()
            }
        } else if focused {
            // Empty but focused — show a bare cursor
            div().child("|").into_any_element()
        } else {
            // Empty and unfocused — show placeholder
            div()
                .text_color(c.muted_foreground)
                .child(self.placeholder.clone())
                .into_any_element()
        };

        // ── container ──────────────────────────────────────────────────────
        let mut input = div()
            .id(self.id.clone())
            .track_focus(&self.focus_handle)
            .h(px(36.))
            .px_3()
            .flex()
            .items_center()
            .rounded(cx.theme().radius)
            .border_1()
            .border_color(if focused { c.ring } else { c.border })
            .bg(c.background)
            .text_color(if disabled {
                c.muted_foreground
            } else {
                c.foreground
            })
            .text_size(cx.theme().font_sizes.body)
            .child(content);

        if !disabled {
            input = input.on_key_down(cx.listener(
                move |this: &mut TextInput, event: &KeyDownEvent, window, cx| {
                    this.handle_key(event, window, cx);
                },
            ));
        }

        input.into_any_element()
    }
}
