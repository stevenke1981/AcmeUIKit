use std::rc::Rc;

use gpui::{
    App, Context, ElementId, FocusHandle, InteractiveElement as _, IntoElement, KeyDownEvent,
    ParentElement as _, Render, SharedString, StatefulInteractiveElement as _, Styled as _, Window,
    div, px,
};

use crate::{ActiveTheme, StyledExt};

type InputHandler = Rc<dyn Fn(&str, &mut Window, &mut App)>;

/// An editable multi-line text area with cursor, placeholder, focus ring,
/// scrolling, and keyboard navigation.
///
/// Stateful (Entity + Render) — the entity owns the text buffer and cursor.
///
/// # Example
///
/// ```ignore
/// cx.new(|cx| {
///     Textarea::new("bio", cx)
///         .placeholder("Tell us about yourself…")
///         .rows(5)
///         .on_input(|value, _, _| { eprintln!("value: {value}"); })
/// })
/// ```
pub struct Textarea {
    id: ElementId,
    text: String,
    cursor_position: usize,
    focus_handle: FocusHandle,
    placeholder: SharedString,
    disabled: bool,
    rows: usize,
    on_input: Option<InputHandler>,
}

impl Textarea {
    /// Create a new [`Textarea`] entity.
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
            rows: 3,
            on_input: None,
        }
    }

    /// Placeholder text shown when the textarea is empty and not focused.
    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Disable user interaction.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the visible number of rows (minimum height). Defaults to 3.
    ///
    /// The minimum value is clamped to 1.
    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows.max(1);
        self
    }

    /// Register a callback that fires on every text change.
    ///
    /// The callback receives the full current text value.
    pub fn on_input(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_input = Some(Rc::new(handler));
        self
    }

    // ── line helpers ──────────────────────────────────────────────────────

    /// Returns the character index of the start of each line.
    fn line_starts(text: &str) -> Vec<usize> {
        let mut starts = vec![0];
        for (char_idx, c) in text.chars().enumerate() {
            if c == '\n' {
                starts.push(char_idx + 1);
            }
        }
        starts
    }

    /// Returns `(line_index, line_start_char, line_end_char_exclusive)` for
    /// the line containing the given character index.
    fn line_at(text: &str, cursor: usize, lines: &[usize]) -> (usize, usize, usize) {
        let total_chars = text.chars().count();
        let cursor = cursor.min(total_chars);
        let mut line_idx = 0;
        for (i, &start) in lines.iter().enumerate() {
            if cursor >= start {
                line_idx = i;
            }
        }
        let line_start = lines[line_idx];
        let line_end = if line_idx + 1 < lines.len() {
            // Exclude the '\n' character at the end of this line
            lines[line_idx + 1].saturating_sub(1)
        } else {
            total_chars
        };
        (line_idx, line_start, line_end)
    }

    // ── key handling ──────────────────────────────────────────────────────

    fn handle_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut App) {
        if self.disabled {
            return;
        }

        let keystroke = &event.keystroke;
        let chars: Vec<char> = self.text.chars().collect();
        let cursor = self.cursor_position.min(chars.len());
        let lines = Self::line_starts(&self.text);

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
            "up" => {
                let (line_idx, line_start, _line_end) = Self::line_at(&self.text, cursor, &lines);
                if line_idx > 0 {
                    let prev_line_start = lines[line_idx - 1];
                    let prev_line_end = if line_idx < lines.len() {
                        lines[line_idx].saturating_sub(1)
                    } else {
                        chars.len()
                    };
                    let col = cursor - line_start;
                    let prev_line_len = prev_line_end - prev_line_start;
                    let new_col = col.min(prev_line_len);
                    self.cursor_position = prev_line_start + new_col;
                } else {
                    self.cursor_position = 0;
                }
            }
            "down" => {
                let (line_idx, line_start, _line_end) = Self::line_at(&self.text, cursor, &lines);
                if line_idx + 1 < lines.len() {
                    let next_line_start = lines[line_idx + 1];
                    let next_line_end = if line_idx + 2 < lines.len() {
                        lines[line_idx + 2].saturating_sub(1)
                    } else {
                        chars.len()
                    };
                    let col = cursor - line_start;
                    let next_line_len = next_line_end - next_line_start;
                    let new_col = col.min(next_line_len);
                    self.cursor_position = next_line_start + new_col;
                } else {
                    self.cursor_position = chars.len();
                }
            }
            "enter" => {
                let mut new_chars: Vec<char> = chars.into_iter().collect();
                new_chars.insert(cursor, '\n');
                self.text = new_chars.into_iter().collect();
                self.cursor_position = cursor + 1;
                self.fire_input(window, cx);
            }
            "home" => {
                let (_idx, line_start, _) = Self::line_at(&self.text, cursor, &lines);
                self.cursor_position = line_start;
            }
            "end" => {
                let (_idx, _line_start, line_end) = Self::line_at(&self.text, cursor, &lines);
                self.cursor_position = line_end;
            }
            _ => {
                // Printable character input via key_char
                if let Some(key_char) = &keystroke.key_char {
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

impl Render for Textarea {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;
        let disabled = self.disabled;
        let focused = self.focus_handle.is_focused(window);

        // Minimum height based on row count: ~20 px per row + 12 px vertical padding.
        let min_height = px(self.rows as f32 * 20. + 12.);

        // ── content (text / placeholder / cursor) ──────────────────────────
        let content: gpui::AnyElement = if !self.text.is_empty() {
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
        let mut el = div()
            .id(self.id.clone())
            .track_focus(&self.focus_handle)
            .min_h(min_height)
            .w_full()
            .px_3()
            .py_1p5()
            .v_flex()
            .overflow_y_scroll()
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
            .line_height(px(20.))
            .whitespace_normal()
            .child(content);

        if !disabled {
            el = el.on_key_down(cx.listener(
                move |this: &mut Textarea, event: &KeyDownEvent, window, cx| {
                    this.handle_key(event, window, cx);
                },
            ));
        }

        el.into_any_element()
    }
}
