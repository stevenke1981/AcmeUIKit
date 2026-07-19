use std::collections::HashSet;
use std::rc::Rc;

use gpui::{
    App, ClickEvent, Context, ElementId, FocusHandle, InteractiveElement as _, IntoElement,
    KeyDownEvent, ParentElement as _, Pixels, Render, SharedString,
    StatefulInteractiveElement as _, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, SortDirection, StyledExt};

// ── Column ───────────────────────────────────────────────────────────────────

/// A column definition for [`DataGrid`].
///
/// Controls the header label, width, and interactive behaviour (sort,
/// filter, freeze).
#[derive(Clone)]
pub struct DataGridColumn {
    /// Stable element ID for the column.
    pub id: ElementId,
    /// Header label.
    pub title: SharedString,
    /// Column width in pixels.
    pub width: Pixels,
    /// Whether clicking the header toggles sorting for this column.
    pub sortable: bool,
    /// Whether a filter input is shown for this column in the filter row.
    pub filterable: bool,
    /// Whether the column is visible.
    pub visible: bool,
    /// Whether the column stays fixed on the left side.
    pub frozen: bool,
}

impl DataGridColumn {
    /// Creates a new column with the given [`ElementId`] and header title.
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            width: px(120.),
            sortable: false,
            filterable: false,
            visible: true,
            frozen: false,
        }
    }

    /// Sets the column width.
    pub fn width(mut self, width: Pixels) -> Self {
        self.width = width;
        self
    }

    /// Sets whether the column is sortable.
    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    /// Sets whether the column is filterable.
    pub fn filterable(mut self, filterable: bool) -> Self {
        self.filterable = filterable;
        self
    }

    /// Sets whether the column is visible.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Sets whether the column is frozen (fixed on the left).
    pub fn frozen(mut self, frozen: bool) -> Self {
        self.frozen = frozen;
        self
    }
}

// ── Row ──────────────────────────────────────────────────────────────────────

/// A single row in a [`DataGrid`].
#[derive(Clone)]
pub struct DataGridRow {
    /// Cell values in column order.
    pub cells: Vec<SharedString>,
    /// Whether the row is selected.
    pub selected: bool,
    /// Whether the row is expanded (for tree grids).
    pub expanded: bool,
    /// Child rows for hierarchical data.
    pub children: Vec<DataGridRow>,
}

impl DataGridRow {
    /// Creates a new row with the given cell values.
    pub fn new(cells: Vec<SharedString>) -> Self {
        Self {
            cells,
            selected: false,
            expanded: false,
            children: Vec::new(),
        }
    }

    /// Sets the selected state.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Sets the expanded state.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Sets the child rows.
    pub fn children(mut self, children: Vec<DataGridRow>) -> Self {
        self.children = children;
        self
    }
}

// ── Entity ───────────────────────────────────────────────────────────────────

type SortHandler = Rc<dyn Fn(usize, SortDirection, &mut Window, &mut App)>;
type SelectHandler = Rc<dyn Fn(HashSet<usize>, &mut Window, &mut App)>;
type EditHandler = Rc<dyn Fn(usize, usize, &str, &mut Window, &mut App)>;

/// A full-featured, stateful data grid with sorting, filtering, column resize,
/// cell selection, inline editing, keyboard navigation, and frozen columns.
///
/// Stateful (Entity + Render) — the entity owns the sort state, selection,
/// filter text, and editing state.
///
/// # Example
///
/// ```ignore
/// cx.new(|cx| {
///     DataGrid::new("users", cx)
///         .columns(vec![
///             DataGridColumn::new("name", "Name").sortable(true).width(px(150.)),
///             DataGridColumn::new("age", "Age").sortable(true).width(px(80.)),
///             DataGridColumn::new("role", "Role").width(px(150.)),
///         ])
///         .rows(vec![
///             DataGridRow::new(vec!["Alice".into(), "30".into(), "Engineer".into()]),
///             DataGridRow::new(vec!["Bob".into(), "25".into(), "Designer".into()]),
///         ])
///         .on_sort(|col, dir, _, _| { eprintln!("sort col {col} {dir:?}"); })
/// })
/// ```
pub struct DataGrid {
    id: ElementId,
    columns: Vec<DataGridColumn>,
    data: Vec<DataGridRow>,
    sort_state: Vec<(usize, SortDirection)>,
    selected_rows: HashSet<usize>,
    selected_cell: Option<(usize, usize)>,
    editing_cell: Option<(usize, usize)>,
    edit_value: String,
    filter_text: Vec<String>,
    focus_handle: FocusHandle,
    show_filter_row: bool,
    #[allow(dead_code)]
    column_resize: Option<(usize, f32)>,
    loading: bool,
    error: Option<SharedString>,
    on_sort: Option<SortHandler>,
    on_select: Option<SelectHandler>,
    on_edit: Option<EditHandler>,
}

impl DataGrid {
    // ── constructor ──────────────────────────────────────────────────────────

    /// Creates a new [`DataGrid`] entity.
    ///
    /// `id` is the stable [`ElementId`] for the interactive container element.
    /// `cx` is the entity-building context from [`AppContext::new`].
    pub fn new(id: impl Into<ElementId>, cx: &mut Context<Self>) -> Self {
        Self {
            id: id.into(),
            columns: Vec::new(),
            data: Vec::new(),
            sort_state: Vec::new(),
            selected_rows: HashSet::new(),
            selected_cell: None,
            editing_cell: None,
            edit_value: String::new(),
            filter_text: Vec::new(),
            focus_handle: cx.focus_handle(),
            show_filter_row: false,
            column_resize: None,
            loading: false,
            error: None,
            on_sort: None,
            on_select: None,
            on_edit: None,
        }
    }

    // ── builder methods ──────────────────────────────────────────────────────

    /// Adds a single column.
    pub fn column(mut self, col: DataGridColumn) -> Self {
        self.filter_text.push(String::new());
        self.columns.push(col);
        self
    }

    /// Sets all columns at once.
    pub fn columns(mut self, cols: Vec<DataGridColumn>) -> Self {
        self.filter_text.resize(cols.len(), String::new());
        self.columns = cols;
        self
    }

    /// Adds a single row.
    pub fn row(mut self, r: DataGridRow) -> Self {
        self.data.push(r);
        self
    }

    /// Sets all rows at once.
    pub fn rows(mut self, data: Vec<DataGridRow>) -> Self {
        self.data = data;
        self
    }

    /// Marks a column as sortable (convenience method).
    pub fn sortable(mut self, col_index: usize) -> Self {
        if let Some(col) = self.columns.get_mut(col_index) {
            col.sortable = true;
        }
        self
    }

    /// Shows or hides the filter row.
    pub fn show_filter_row(mut self, show: bool) -> Self {
        self.show_filter_row = show;
        self
    }

    /// Sets the loading state.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Sets the error message.
    pub fn error(mut self, err: impl Into<SharedString>) -> Self {
        self.error = Some(err.into());
        self
    }

    /// Registers a sort handler.
    ///
    /// The handler receives the column index, the new direction, and context.
    pub fn on_sort(
        mut self,
        handler: impl Fn(usize, SortDirection, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_sort = Some(Rc::new(handler));
        self
    }

    /// Registers a selection handler.
    ///
    /// The handler receives the set of selected row indices.
    pub fn on_select(
        mut self,
        handler: impl Fn(HashSet<usize>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }

    /// Registers a cell edit handler.
    ///
    /// The handler receives (row, col, new_value) when a cell edit is committed.
    pub fn on_edit(
        mut self,
        handler: impl Fn(usize, usize, &str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_edit = Some(Rc::new(handler));
        self
    }

    // ── sort ─────────────────────────────────────────────────────────────────

    /// Toggles the sort direction for a column.
    ///
    /// Cycles through Asc → Desc → None.
    /// When Shift is held (via `multi`), adds to the multi-column sort stack.
    pub fn toggle_sort(&mut self, col_index: usize, multi: bool) {
        if !multi {
            // Single-column sort — replace
            if let Some(pos) = self
                .sort_state
                .iter()
                .position(|&(idx, _)| idx == col_index)
            {
                let (_, dir) = self.sort_state[pos];
                let new_dir = match dir {
                    SortDirection::Asc => SortDirection::Desc,
                    SortDirection::Desc => SortDirection::None,
                    SortDirection::None => SortDirection::Asc,
                };
                if new_dir == SortDirection::None {
                    self.sort_state.clear();
                } else {
                    self.sort_state = vec![(col_index, new_dir)];
                }
            } else {
                self.sort_state = vec![(col_index, SortDirection::Asc)];
            }
        } else {
            // Multi-column sort
            if let Some(pos) = self
                .sort_state
                .iter()
                .position(|&(idx, _)| idx == col_index)
            {
                let (_, dir) = self.sort_state[pos];
                let new_dir = match dir {
                    SortDirection::Asc => SortDirection::Desc,
                    SortDirection::Desc => SortDirection::None,
                    SortDirection::None => SortDirection::Asc,
                };
                if new_dir == SortDirection::None {
                    self.sort_state.remove(pos);
                } else {
                    self.sort_state[pos] = (col_index, new_dir);
                }
            } else {
                self.sort_state.push((col_index, SortDirection::Asc));
            }
        }
    }

    /// Fires the `on_sort` callback with the current primary sort state.
    pub fn fire_sort(&self, window: &mut Window, cx: &mut App) {
        if let Some(ref handler) = self.on_sort {
            if let Some(&(col_idx, dir)) = self.sort_state.first() {
                handler(col_idx, dir, window, cx);
            }
        }
    }

    // ── selection ────────────────────────────────────────────────────────────

    /// Selects or deselects a row.
    ///
    /// * `extend` — if `true`, acts as a toggle (Ctrl-like behaviour).
    /// * `range_extend` — if `true`, extends the selection to the given row
    ///   (Shift-like behaviour; ranges from the last selected row).
    pub fn select_row(&mut self, row_index: usize, extend: bool, range_extend: bool) {
        if range_extend {
            if let Some(&last) = self.selected_rows.iter().max() {
                let from = last.min(row_index);
                let to = last.max(row_index);
                for i in from..=to {
                    self.selected_rows.insert(i);
                }
                return;
            }
        }

        if extend {
            // Toggle single row
            if self.selected_rows.contains(&row_index) {
                self.selected_rows.remove(&row_index);
            } else {
                self.selected_rows.insert(row_index);
            }
        } else {
            // Single selection
            self.selected_rows.clear();
            self.selected_rows.insert(row_index);
        }
    }

    /// Selects a specific cell.
    pub fn select_cell(&mut self, row: usize, col: usize) {
        self.selected_cell = Some((row, col));
        self.selected_rows.clear();
        self.selected_rows.insert(row);
    }

    /// Fires the selection callback.
    pub fn fire_select(&self, window: &mut Window, cx: &mut App) {
        if let Some(ref handler) = self.on_select {
            handler(self.selected_rows.clone(), window, cx);
        }
    }

    // ── editing ───────────────────────────────────────────────────────────────

    /// Starts editing the cell at `(row, col)`.
    pub fn start_edit(&mut self, row: usize, col: usize) {
        if let Some(r) = self.data.get(row) {
            if let Some(val) = r.cells.get(col) {
                self.editing_cell = Some((row, col));
                self.edit_value = val.to_string();
                self.selected_cell = Some((row, col));
            }
        }
    }

    /// Commits the current cell edit.
    pub fn commit_edit(&mut self, window: &mut Window, cx: &mut App) {
        if let Some((row, col)) = self.editing_cell.take() {
            let value = self.edit_value.clone();
            if let Some(r) = self.data.get_mut(row) {
                if let Some(cell) = r.cells.get_mut(col) {
                    *cell = value.clone().into();
                }
            }
            if let Some(ref handler) = self.on_edit {
                handler(row, col, &value, window, cx);
            }
            self.edit_value.clear();
        }
    }

    /// Cancels the current cell edit.
    pub fn cancel_edit(&mut self) {
        self.editing_cell = None;
        self.edit_value.clear();
    }

    // ── keyboard navigation ──────────────────────────────────────────────────

    /// Handles keyboard events for navigation and editing.
    pub fn handle_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut App) {
        let keystroke = &event.keystroke;

        // If editing, handle special keys
        if self.editing_cell.is_some() {
            match keystroke.key.as_str() {
                "enter" => {
                    self.commit_edit(window, cx);
                    // Move down after commit
                    if let Some((row, col)) = self.selected_cell {
                        if row + 1 < self.data.len() {
                            self.select_cell(row + 1, col);
                        }
                    }
                    window.refresh();
                    return;
                }
                "escape" => {
                    self.cancel_edit();
                    window.refresh();
                    return;
                }
                "tab" => {
                    self.commit_edit(window, cx);
                    if let Some((row, col)) = self.selected_cell {
                        if col + 1 < self.columns.len() {
                            self.start_edit(row, col + 1);
                        } else if row + 1 < self.data.len() {
                            self.start_edit(row + 1, 0);
                        }
                    }
                    window.refresh();
                    return;
                }
                _ => {
                    // Let the edit value accumulate input (handled by the
                    // `on_key_down` on the wrapper div — simplified here:
                    // printable characters are appended via `handle_edit_key`.
                    if let Some(ch) = &keystroke.key_char {
                        if !keystroke.modifiers.alt
                            && !keystroke.modifiers.control
                            && !keystroke.modifiers.platform
                        {
                            // Append printable character
                            self.edit_value.push_str(ch);
                            window.refresh();
                            return;
                        }
                    }
                    // Backspace in edit mode
                    if keystroke.key.as_str() == "backspace" {
                        self.edit_value.pop();
                        window.refresh();
                        return;
                    }
                }
            }
            return;
        }

        // Non-editing navigation
        match keystroke.key.as_str() {
            "down" | "arrowdown" => {
                self.move_selection(1, 0);
                window.refresh();
            }
            "up" | "arrowup" => {
                if self.selected_cell.is_some_and(|(r, _)| r > 0) {
                    self.move_selection(-1, 0);
                    window.refresh();
                }
            }
            "left" | "arrowleft" => {
                if self.selected_cell.is_some_and(|(_, c)| c > 0) {
                    self.move_selection(0, -1);
                    window.refresh();
                }
            }
            "right" | "arrowright" => {
                self.move_selection(0, 1);
                window.refresh();
            }
            "enter" => {
                if let Some((row, col)) = self.selected_cell {
                    self.start_edit(row, col);
                    window.refresh();
                }
            }
            "tab" => {
                if let Some((row, col)) = self.selected_cell {
                    if col + 1 < self.columns.len() {
                        self.select_cell(row, col + 1);
                    } else if row + 1 < self.data.len() {
                        self.select_cell(row + 1, 0);
                    }
                    window.refresh();
                }
            }
            _ => {}
        }
    }

    /// Moves the selected cell by the given deltas, clamping to bounds.
    pub fn move_selection(&mut self, delta_row: i32, delta_col: i32) {
        let (r, c) = self.selected_cell.unwrap_or((0, 0));
        let max_row = self.data.len().saturating_sub(1);
        let max_col = self.columns.len().saturating_sub(1);

        let new_row = (r as i32 + delta_row).clamp(0, max_row as i32) as usize;
        let new_col = (c as i32 + delta_col).clamp(0, max_col as i32) as usize;

        self.select_cell(new_row, new_col);
    }

    // ── copy / export ────────────────────────────────────────────────────────

    /// Copies the currently selected data as TSV (tab-separated values).
    ///
    /// Returns an empty string when nothing is selected.
    pub fn copy_selection(&self) -> String {
        if self.selected_rows.is_empty() {
            return String::new();
        }

        let mut lines: Vec<String> = Vec::new();

        // Header row
        let header: Vec<&str> = self
            .columns
            .iter()
            .filter(|c| c.visible)
            .map(|c| c.title.as_ref())
            .collect();
        lines.push(header.join("\t"));

        // Data rows
        for &idx in &self.selected_rows {
            if let Some(row) = self.data.get(idx) {
                let cells: Vec<&str> = row
                    .cells
                    .iter()
                    .enumerate()
                    .filter(|&(ci, _)| self.columns.get(ci).is_some_and(|c| c.visible))
                    .map(|(_, v)| v.as_ref())
                    .collect();
                lines.push(cells.join("\t"));
            }
        }

        lines.join("\n")
    }

    /// Exports all data (filtered) as CSV.
    pub fn export_csv(&self) -> String {
        let indices = self.filtered_indices();

        let mut lines: Vec<String> = Vec::new();

        // Header row
        let header: Vec<&str> = self
            .columns
            .iter()
            .filter(|c| c.visible)
            .map(|c| c.title.as_ref())
            .collect();
        lines.push(header.join(","));

        // Data rows
        for &idx in &indices {
            if let Some(row) = self.data.get(idx) {
                let cells: Vec<&str> = row
                    .cells
                    .iter()
                    .enumerate()
                    .filter(|&(ci, _)| self.columns.get(ci).is_some_and(|c| c.visible))
                    .map(|(_, v)| v.as_ref())
                    .collect();
                lines.push(cells.join(","));
            }
        }

        lines.join("\n")
    }

    // ── filter ───────────────────────────────────────────────────────────────

    /// Sets the filter text for a given column.
    pub fn set_filter(&mut self, col_index: usize, text: String) {
        if col_index < self.filter_text.len() {
            self.filter_text[col_index] = text;
        }
    }

    /// Returns the sort state for inspection.
    pub fn sort_state(&self) -> &[(usize, SortDirection)] {
        &self.sort_state
    }

    // ── internal helpers ─────────────────────────────────────────────────────

    /// Returns the indices of data rows that pass the current filters and sort.
    fn filtered_indices(&self) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..self.data.len()).collect();

        // Apply column filters
        if self.filter_text.iter().any(|f| !f.is_empty()) {
            indices.retain(|&i| {
                self.filter_text
                    .iter()
                    .enumerate()
                    .all(|(col_idx, filter)| {
                        if filter.is_empty() {
                            return true;
                        }
                        self.data[i]
                            .cells
                            .get(col_idx)
                            .is_none_or(|cell| cell.to_lowercase().contains(&filter.to_lowercase()))
                    })
            });
        }

        // Apply sort (primary sort only for now)
        if let Some(&(col_idx, direction)) = self.sort_state.first() {
            if direction != SortDirection::None {
                indices.sort_by(|&a, &b| {
                    let a_val = self.data[a]
                        .cells
                        .get(col_idx)
                        .map(|s| s.as_str())
                        .unwrap_or("");
                    let b_val = self.data[b]
                        .cells
                        .get(col_idx)
                        .map(|s| s.as_str())
                        .unwrap_or("");
                    let cmp = a_val.to_lowercase().cmp(&b_val.to_lowercase());
                    match direction {
                        SortDirection::Asc => cmp,
                        SortDirection::Desc => cmp.reverse(),
                        SortDirection::None => std::cmp::Ordering::Equal,
                    }
                });
            }
        }

        indices
    }

    /// Returns references to filtered and sorted rows.
    pub fn filtered_data(&self) -> Vec<&DataGridRow> {
        self.filtered_indices()
            .into_iter()
            .map(|i| &self.data[i])
            .collect()
    }
}

// ── Render ───────────────────────────────────────────────────────────────────

impl Render for DataGrid {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let id_prefix = match &self.id {
            ElementId::Name(name) => name.to_string(),
            ElementId::Integer(n) => format!("dg-{n}"),
            ElementId::NamedInteger(name, n) => format!("dg-{name}-{n}"),
            _ => format!("dg-{:?}", self.id),
        };
        let c = cx.theme().colors;
        let theme = cx.theme();

        // Ensure filter_text is long enough
        while self.filter_text.len() < self.columns.len() {
            self.filter_text.push(String::new());
        }

        let filtered = self.filtered_indices();
        let _frozen_count = self.columns.iter().filter(|c| c.frozen).count();

        // ── Header ─────────────────────────────────────────────────────────
        let header = {
            let mut header_children: Vec<gpui::AnyElement> = Vec::with_capacity(self.columns.len());

            for (i, col) in self.columns.iter().enumerate() {
                if !col.visible {
                    continue;
                }

                let _is_sorted = self.sort_state.iter().any(|&(idx, _)| idx == i);
                let sort_dir = self
                    .sort_state
                    .iter()
                    .find(|&&(idx, _)| idx == i)
                    .map(|&(_, d)| d)
                    .unwrap_or(SortDirection::None);
                let indicator = match sort_dir {
                    SortDirection::Asc => " \u{25B2}",
                    SortDirection::Desc => " \u{25BC}",
                    SortDirection::None => "",
                };

                let mut header_cell = div()
                    .id(ElementId::Name(format!("{id_prefix}-hdr-{i}").into()))
                    .h_flex()
                    .h_full()
                    .px_2()
                    .gap_1()
                    .text_size(theme.font_sizes.caption)
                    .text_color(c.muted_foreground);

                if col.sortable {
                    let sort_idx = i;
                    let listener =
                        cx.listener(move |this: &mut DataGrid, _ev: &ClickEvent, window, cx| {
                            this.toggle_sort(sort_idx, false);
                            this.fire_sort(window, cx);
                            window.refresh();
                        });
                    header_cell = header_cell
                        .cursor_pointer()
                        .hover(|s| s.bg(c.muted))
                        .on_click(listener)
                        .child(format!("{}{}", col.title, indicator));
                } else {
                    header_cell = header_cell.child(col.title.clone());
                }

                header_cell = header_cell.w(col.width);

                // Resize handle (visual only for now)
                let resize_h = div()
                    .id(ElementId::Name(format!("{id_prefix}-rsz-{i}").into()))
                    .absolute()
                    .right_0()
                    .top_0()
                    .w(px(4.))
                    .h_full()
                    .cursor_col_resize()
                    .hover(|s| s.bg(c.ring));

                header_cell = header_cell.relative().child(resize_h);

                // Frozen column separator
                if col.frozen {
                    header_cell = header_cell.border_r_1().border_color(c.border);
                }

                header_children.push(header_cell.into_any_element());
            }

            div()
                .id(ElementId::Name(format!("{id_prefix}-header-row").into()))
                .h_flex()
                .h(px(36.))
                .bg(c.surface)
                .border_b_1()
                .border_color(c.border)
                .children(header_children)
        };

        // ── Filter row ──────────────────────────────────────────────────────
        let filter_row = if self.show_filter_row {
            let mut filter_children: Vec<gpui::AnyElement> = Vec::with_capacity(self.columns.len());

            for (i, col) in self.columns.iter().enumerate() {
                if !col.visible {
                    continue;
                }
                let filter_text = self.filter_text.get(i).cloned().unwrap_or_default();
                let display: SharedString = if filter_text.is_empty() {
                    "Filter...".into()
                } else {
                    filter_text.into()
                };
                let flt = div()
                    .id(ElementId::Name(format!("{id_prefix}-flt-{i}").into()))
                    .h_flex()
                    .h_full()
                    .px_1()
                    .items_center()
                    .w(col.width)
                    .text_size(theme.font_sizes.caption)
                    .text_color(c.muted_foreground)
                    .child(display);

                filter_children.push(flt.into_any_element());
            }

            div()
                .id(ElementId::Name(format!("{id_prefix}-filter-row").into()))
                .h_flex()
                .h(px(28.))
                .bg(c.surface)
                .border_b_1()
                .border_color(c.border)
                .children(filter_children)
                .into_any_element()
        } else {
            div().into_any_element()
        };

        // ── Body ────────────────────────────────────────────────────────────
        let body: gpui::AnyElement = if self.loading {
            div()
                .v_flex()
                .items_center()
                .justify_center()
                .py(px(40.))
                .child(
                    div()
                        .text_size(theme.font_sizes.body)
                        .text_color(c.muted_foreground)
                        .child("Loading..."),
                )
                .into_any_element()
        } else if let Some(ref err) = self.error {
            div()
                .v_flex()
                .items_center()
                .justify_center()
                .py(px(40.))
                .child(
                    div()
                        .text_size(theme.font_sizes.body)
                        .text_color(c.danger)
                        .child(err.clone()),
                )
                .into_any_element()
        } else if filtered.is_empty() {
            div()
                .v_flex()
                .items_center()
                .justify_center()
                .py(px(40.))
                .child(
                    div()
                        .text_size(theme.font_sizes.body)
                        .text_color(c.muted_foreground)
                        .child("No data"),
                )
                .into_any_element()
        } else {
            let mut row_elements: Vec<gpui::AnyElement> = Vec::with_capacity(filtered.len());

            for &orig_idx in &filtered {
                let row = &self.data[orig_idx];
                let is_selected = self.selected_rows.contains(&orig_idx);
                let is_editing = self
                    .editing_cell
                    .map(|(r, _)| r == orig_idx)
                    .unwrap_or(false);

                let mut row_div = div()
                    .id(ElementId::Name(
                        format!("{id_prefix}-row-{orig_idx}").into(),
                    ))
                    .h_flex()
                    .h(px(32.))
                    .border_b_1()
                    .border_color(c.border)
                    .text_size(theme.font_sizes.body)
                    .text_color(c.foreground)
                    .bg(if is_selected {
                        c.primary.alpha(0.1)
                    } else {
                        gpui::transparent_black()
                    })
                    .hover(|s| s.bg(c.muted));

                // Row click → select
                let sel_idx = orig_idx;
                let row_listener =
                    cx.listener(move |this: &mut DataGrid, _ev: &ClickEvent, window, cx| {
                        this.select_row(sel_idx, false, false);
                        this.fire_select(window, cx);
                        window.refresh();
                    });
                row_div = row_div.cursor_pointer().on_click(row_listener);

                // Cells
                for (col_idx, col) in self.columns.iter().enumerate() {
                    if !col.visible {
                        continue;
                    }

                    let cell_value = row.cells.get(col_idx).cloned().unwrap_or_default();
                    let is_edit_cell = is_editing
                        && self
                            .editing_cell
                            .map(|(_, c)| c == col_idx)
                            .unwrap_or(false);
                    let is_selected_cell = self.selected_cell == Some((orig_idx, col_idx));

                    let mut cell_div = div()
                        .id(ElementId::Name(
                            format!("{id_prefix}-cell-{orig_idx}-{col_idx}").into(),
                        ))
                        .h_flex()
                        .h_full()
                        .items_center()
                        .px_2()
                        .w(col.width);

                    if is_edit_cell {
                        cell_div = cell_div
                            .bg(c.background)
                            .border_1()
                            .border_color(c.ring)
                            .text_color(c.foreground);
                    } else if is_selected_cell {
                        cell_div = cell_div.border_1().border_color(c.ring);
                    }

                    if is_edit_cell {
                        cell_div = cell_div.child(self.edit_value.clone());
                    } else {
                        cell_div = cell_div.child(cell_value);
                    }

                    row_div = row_div.child(cell_div);
                }

                row_elements.push(row_div.into_any_element());
            }

            div().v_flex().children(row_elements).into_any_element()
        };

        // ── Assemble ───────────────────────────────────────────────────────
        div()
            .id(self.id.clone())
            .track_focus(&self.focus_handle)
            .v_flex()
            .w_full()
            .h_full()
            .child(header)
            .child(filter_row)
            .child(
                div()
                    .id(ElementId::Name(format!("{id_prefix}-body").into()))
                    .flex_1()
                    .child(body),
            )
            .on_key_down(cx.listener(
                move |this: &mut DataGrid, event: &KeyDownEvent, window, cx| {
                    this.handle_key(event, window, cx);
                },
            ))
    }
}
