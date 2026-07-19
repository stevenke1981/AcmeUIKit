use std::rc::Rc;

use gpui::{
    App, ClickEvent, Context, ElementId, FocusHandle, InteractiveElement as _, IntoElement,
    KeyDownEvent, ParentElement as _, Render, SharedString, StatefulInteractiveElement as _,
    Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

pub mod options;
pub use options::ComboboxOption;

type InputHandler = Rc<dyn Fn(&str, &mut Window, &mut App)>;
type SelectHandler = Rc<dyn Fn(usize, &str, &mut Window, &mut App)>;

/// A combo box that combines a text input with a filterable suggestion dropdown.
///
/// Stateful (Entity + Render) — the entity owns the text buffer, cursor, and
/// dropdown state.
///
/// # Example
///
/// ```ignore
/// cx.new(|cx| {
///     Combobox::new("fruit", cx)
///         .placeholder("Choose a fruit…")
///         .options(vec![
///             ComboboxOption::new("Apple", "apple"),
///             ComboboxOption::new("Banana", "banana"),
///         ])
///         .on_select(|index, value, _, _| {
///             eprintln!("selected #{index}: {value}");
///         })
/// })
/// ```
pub struct Combobox {
    id: ElementId,
    text: String,
    cursor_position: usize,
    focus_handle: FocusHandle,
    placeholder: SharedString,
    disabled: bool,
    options: Vec<ComboboxOption>,
    /// Index into `options` that is currently highlighted by the keyboard.
    highlighted: Option<usize>,
    /// Whether the suggestion dropdown is visible.
    open: bool,
    on_input: Option<InputHandler>,
    on_select: Option<SelectHandler>,
}

impl Combobox {
    /// Create a new [`Combobox`] entity.
    ///
    /// `id` is the stable [`ElementId`] for the interactive input element.
    /// `cx` is the entity-building context from [`AppContext::new`].
    pub fn new(id: impl Into<ElementId>, cx: &mut Context<Self>) -> Self {
        Self {
            id: id.into(),
            text: String::new(),
            cursor_position: 0,
            focus_handle: cx.focus_handle(),
            placeholder: SharedString::default(),
            disabled: false,
            options: Vec::new(),
            highlighted: None,
            open: false,
            on_input: None,
            on_select: None,
        }
    }

    /// Placeholder text shown when the input is empty and unfocused.
    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Disable user interaction.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the full list of [`ComboboxOption`]s available for filtering.
    pub fn options(mut self, items: Vec<ComboboxOption>) -> Self {
        self.options = items;
        self
    }

    /// Register a callback that fires on every text change.
    ///
    /// The callback receives the full current text value.
    pub fn on_input(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_input = Some(Rc::new(handler));
        self
    }

    /// Register a callback that fires when a dropdown option is selected.
    ///
    /// The callback receives the option index and its value string.
    pub fn on_select(
        mut self,
        handler: impl Fn(usize, &str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }

    // ── internal helpers ────────────────────────────────────────────────────

    /// Returns the indices of `options` whose label or value matches the current
    /// input text (case‑insensitive substring).
    fn filtered_indices(&self) -> Vec<usize> {
        if self.text.is_empty() {
            return Vec::new();
        }
        let query = self.text.to_lowercase();
        self.options
            .iter()
            .enumerate()
            .filter(|(_, opt)| {
                opt.label.to_lowercase().contains(&query)
                    || opt.value.to_lowercase().contains(&query)
            })
            .map(|(i, _)| i)
            .collect()
    }

    /// Close the dropdown and clear the highlight.
    fn close_dropdown(&mut self) {
        self.open = false;
        self.highlighted = None;
    }

    /// Re-evaluate the dropdown state based on the current text.
    ///
    /// Opens the dropdown when there are matches, or closes it when there are
    /// none. Resets the highlight if the previously highlighted option is no
    /// longer in the filtered set.
    fn sync_dropdown(&mut self) {
        let filtered = self.filtered_indices();
        if filtered.is_empty() {
            self.close_dropdown();
        } else {
            self.open = true;
            if self.highlighted.is_none_or(|h| !filtered.contains(&h)) {
                self.highlighted = None;
            }
        }
    }

    fn fire_input(&mut self, window: &mut Window, cx: &mut App) {
        if let Some(ref handler) = self.on_input {
            handler(&self.text, window, cx);
        }
    }

    fn fire_select(&mut self, index: usize, window: &mut Window, cx: &mut App) {
        if let Some(ref handler) = self.on_select {
            handler(index, &self.text, window, cx);
        }
    }

    // ── key handling ────────────────────────────────────────────────────────

    fn handle_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut App) {
        if self.disabled {
            return;
        }

        let keystroke = &event.keystroke;
        let filtered = self.filtered_indices();

        // ── Dropdown navigation (only applies when open) ──────────────────
        if self.open && !filtered.is_empty() {
            match keystroke.key.as_str() {
                "down" | "arrowdown" => {
                    let next = match self.highlighted {
                        Some(current) => {
                            if let Some(pos) = filtered.iter().position(|&i| i == current) {
                                if pos + 1 < filtered.len() {
                                    filtered[pos + 1]
                                } else {
                                    filtered[0]
                                }
                            } else {
                                filtered[0]
                            }
                        }
                        None => filtered[0],
                    };
                    self.highlighted = Some(next);
                    return;
                }
                "up" | "arrowup" => {
                    let prev = match self.highlighted {
                        Some(current) => {
                            if let Some(pos) = filtered.iter().position(|&i| i == current) {
                                if pos > 0 {
                                    filtered[pos - 1]
                                } else {
                                    *filtered.last().unwrap_or(&current)
                                }
                            } else {
                                *filtered.last().unwrap_or(&current)
                            }
                        }
                        None => *filtered.last().unwrap_or(&0),
                    };
                    self.highlighted = Some(prev);
                    return;
                }
                "enter" => {
                    if let Some(idx) = self.highlighted {
                        if idx < self.options.len() {
                            let value = self.options[idx].value.clone();
                            self.text = value.to_string();
                            self.cursor_position = self.text.len();
                            self.fire_select(idx, window, cx);
                            self.fire_input(window, cx);
                            self.close_dropdown();
                            return;
                        }
                    }
                }
                "escape" => {
                    self.close_dropdown();
                    return;
                }
                _ => {}
            }
        }

        // ── Text editing keys ─────────────────────────────────────────────
        let chars: Vec<char> = self.text.chars().collect();
        let cursor = self.cursor_position.min(chars.len());

        match keystroke.key.as_str() {
            "backspace" if cursor > 0 => {
                let mut new_chars: Vec<char> = chars.into_iter().collect();
                new_chars.remove(cursor - 1);
                self.text = new_chars.into_iter().collect();
                self.cursor_position = cursor.saturating_sub(1);
                self.fire_input(window, cx);
                self.sync_dropdown();
            }
            "delete" if cursor < chars.len() => {
                let mut new_chars: Vec<char> = chars.into_iter().collect();
                new_chars.remove(cursor);
                self.text = new_chars.into_iter().collect();
                self.fire_input(window, cx);
                self.sync_dropdown();
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
                        self.sync_dropdown();
                    }
                }
            }
        }
    }
}

impl Render for Combobox {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;
        let disabled = self.disabled;
        let focused = self.focus_handle.is_focused(window);
        let filtered = self.filtered_indices();

        // Derive a stable string prefix from the element id so that child
        // element ids are unique per combobox instance.
        let id_prefix = match &self.id {
            ElementId::Name(name) => name.to_string(),
            _ => "combobox".to_string(),
        };

        // ── sync dropdown state (defensive — render may be called after
        //    model changes that didn't go through handle_key) ──────────────
        if self.open && filtered.is_empty() {
            self.close_dropdown();
        }

        // ── input content (text / placeholder / cursor) ─────────────────────
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

        // ── input area ──────────────────────────────────────────────────────
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
                move |this: &mut Combobox, event: &KeyDownEvent, window, cx| {
                    this.handle_key(event, window, cx);
                },
            ));
        }

        // ── dropdown panel ──────────────────────────────────────────────────
        let dropdown: gpui::AnyElement = if self.open && !filtered.is_empty() {
            let theme = cx.theme();

            // Build items outside the div builder to keep borrows manageable.
            let mut items: Vec<gpui::AnyElement> = Vec::with_capacity(filtered.len());

            for &opt_idx in filtered.iter() {
                let option = &self.options[opt_idx];
                let highlighted = self.highlighted == Some(opt_idx);

                let item_id = ElementId::Name(format!("{id_prefix}-option-{opt_idx}").into());

                let mut row = div()
                    .id(item_id)
                    .h(px(32.))
                    .px_2()
                    .flex()
                    .items_center()
                    .text_size(cx.theme().font_sizes.body)
                    .text_color(if highlighted {
                        c.primary_foreground
                    } else {
                        c.foreground
                    });

                if highlighted {
                    row = row.bg(c.primary);
                }

                row = row.child(option.label.clone());

                // Click handler — uses cx.listener to mutate the entity.
                let val = option.value.clone();
                let listener =
                    cx.listener(move |this: &mut Combobox, _: &ClickEvent, window, cx| {
                        this.text = val.to_string();
                        this.cursor_position = this.text.len();
                        this.fire_select(opt_idx, window, cx);
                        this.fire_input(window, cx);
                        this.close_dropdown();
                        window.refresh();
                    });

                row = row
                    .cursor_pointer()
                    .hover(|style| style.bg(c.muted))
                    .on_click(listener);

                items.push(row.into_any_element());
            }

            div()
                .v_flex()
                .py_1()
                .rounded(theme.radius)
                .border_1()
                .border_color(c.border)
                .bg(c.surface)
                .min_w(px(160.))
                .children(items)
                .into_any_element()
        } else {
            div().into_any_element()
        };

        // ── container ───────────────────────────────────────────────────────
        div()
            .v_flex()
            .child(input)
            .child(dropdown)
            .into_any_element()
    }
}
