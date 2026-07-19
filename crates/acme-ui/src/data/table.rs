use std::rc::Rc;

use gpui::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

// ── Column ────────────────────────────────────────────────────────────────────

/// Sort direction for a table column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    /// No sorting applied.
    None,
    /// Ascending (A → Z, 0 → 9).
    Asc,
    /// Descending (Z → A, 9 → 0).
    Desc,
}

impl SortDirection {
    /// Toggles to the next direction in the cycle: None → Asc → Desc → None.
    pub fn toggle(self) -> Self {
        match self {
            Self::None => Self::Asc,
            Self::Asc => Self::Desc,
            Self::Desc => Self::None,
        }
    }
}

/// A single column definition for a [`Table`].
#[derive(Clone)]
pub struct TableColumn {
    /// Column header label.
    pub label: SharedString,
    /// Optional sort key. When set, the column header becomes clickable and
    /// a sort indicator is shown.
    pub sort_key: Option<SharedString>,
    /// Optional fixed width in pixels.
    pub width: Option<gpui::Pixels>,
}

impl TableColumn {
    /// Creates a new column with the given header label.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            sort_key: None,
            width: None,
        }
    }

    /// Sets an optional sort key, making the header clickable.
    pub fn sortable(mut self, key: impl Into<SharedString>) -> Self {
        self.sort_key = Some(key.into());
        self
    }

    /// Sets an optional fixed width.
    pub fn width(mut self, width: gpui::Pixels) -> Self {
        self.width = Some(width);
        self
    }
}

// ── Table ─────────────────────────────────────────────────────────────────────

type TableSortHandler = Rc<dyn Fn(SharedString, SortDirection, &mut Window, &mut App)>;
type TableRowHandler = Rc<dyn Fn(usize, &ClickEvent, &mut Window, &mut App)>;

/// Sortable data table component.
///
/// Renders a column header row and a set of data rows. The caller manages
/// column definitions, row content, sort state, and click handlers.
///
/// # Example
///
/// ```ignore
/// Table::new("data-table")
///     .columns(vec![
///         TableColumn::new("Name").sortable("name").width(px(120.)),
///         TableColumn::new("Age").sortable("age").width(px(60.)),
///         TableColumn::new("Role"),
///     ])
///     .rows(vec![
///         vec!["Alice".into(), "30".into(), "Engineer".into()],
///         vec!["Bob".into(), "25".into(), "Designer".into()],
///     ])
///     .sort_key(Some("name".into()))
///     .sort_direction(SortDirection::Asc)
///     .on_sort(|key, dir, _window, _cx| {
///         // update sort state
///     })
///     .on_row_click(|index, _event, _window, _cx| {
///         // handle row click
///     })
/// ```
#[derive(IntoElement)]
pub struct Table {
    id: ElementId,
    columns: Vec<TableColumn>,
    rows: Vec<Vec<SharedString>>,
    sort_key: Option<SharedString>,
    sort_direction: SortDirection,
    on_sort: Option<TableSortHandler>,
    on_row_click: Option<TableRowHandler>,
}

impl Table {
    /// Creates a new table with the given [`ElementId`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            columns: Vec::new(),
            rows: Vec::new(),
            sort_key: None,
            sort_direction: SortDirection::None,
            on_sort: None,
            on_row_click: None,
        }
    }

    /// Sets the table column definitions.
    pub fn columns(mut self, columns: Vec<TableColumn>) -> Self {
        self.columns = columns;
        self
    }

    /// Sets the data rows (each row is a list of cell values matching column
    /// order).
    pub fn rows(mut self, rows: Vec<Vec<SharedString>>) -> Self {
        self.rows = rows;
        self
    }

    /// Sets the currently active sort column key.
    pub fn sort_key(mut self, key: Option<SharedString>) -> Self {
        self.sort_key = key;
        self
    }

    /// Sets the current sort direction.
    pub fn sort_direction(mut self, direction: SortDirection) -> Self {
        self.sort_direction = direction;
        self
    }

    /// Registers a handler for column-header sort clicks.
    ///
    /// The handler receives the column sort key and the *new* direction that
    /// should be applied.
    pub fn on_sort(
        mut self,
        handler: impl Fn(SharedString, SortDirection, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_sort = Some(Rc::new(handler));
        self
    }

    /// Registers a click handler for data rows.
    ///
    /// The handler receives the zero‑based row index.
    pub fn on_row_click(
        mut self,
        handler: impl Fn(usize, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_row_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Table {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        div()
            .id(self.id)
            .v_flex()
            .w_full()
            .child(
                // Header row
                div()
                    .h_flex()
                    .h(px(36.))
                    .bg(c.surface)
                    .border_b_1()
                    .border_color(c.border)
                    .gap_0()
                    .children(self.columns.iter().map(|col| {
                        let is_sorted = self.sort_key.as_deref() == col.sort_key.as_deref();
                        let indicator = if is_sorted {
                            match self.sort_direction {
                                SortDirection::Asc => " ▲",
                                SortDirection::Desc => " ▼",
                                SortDirection::None => "",
                            }
                        } else {
                            ""
                        };

                        let style = div()
                            .id(ElementId::Name(format!("table-col-{}", col.label).into()))
                            .h_flex()
                            .h_full()
                            .px_3()
                            .gap_1()
                            .text_size(theme.font_sizes.caption)
                            .text_color(c.muted_foreground);

                        let style = if col.sort_key.is_some() {
                            let handler = self.on_sort.clone();
                            let sort_key = col.sort_key.clone().unwrap_or_default();

                            style
                                .cursor_pointer()
                                .hover(|style| style.text_color(c.foreground))
                                .on_click(
                                    move |_event: &ClickEvent,
                                          window: &mut Window,
                                          cx: &mut App| {
                                        if let Some(ref handler) = handler {
                                            let current_dir = if is_sorted {
                                                self.sort_direction
                                            } else {
                                                SortDirection::None
                                            };
                                            handler(
                                                sort_key.clone(),
                                                current_dir.toggle(),
                                                window,
                                                cx,
                                            );
                                        }
                                    },
                                )
                        } else {
                            style
                        };

                        let mut cell = style.child(format!("{}{}", col.label, indicator));

                        if let Some(w) = col.width {
                            cell = cell.w(w);
                        } else {
                            cell = cell.flex_1();
                        }

                        cell.into_any_element()
                    })),
            )
            .child(
                div()
                    .v_flex()
                    .children(self.rows.into_iter().enumerate().map(|(row_idx, row)| {
                        let row_handler = self.on_row_click.clone();

                        let mut row_style = div()
                            .id(ElementId::Name(format!("table-row-{row_idx}").into()))
                            .h_flex()
                            .h(px(32.))
                            .gap_0()
                            .border_b_1()
                            .border_color(c.border)
                            .text_size(theme.font_sizes.body)
                            .text_color(c.foreground);

                        if let Some(ref handler) = row_handler {
                            let handler = handler.clone();
                            row_style = row_style
                                .cursor_pointer()
                                .hover(|style| style.bg(c.muted))
                                .on_click(
                                    move |event: &ClickEvent, window: &mut Window, cx: &mut App| {
                                        handler(row_idx, event, window, cx);
                                    },
                                );
                        }

                        row_style
                            .children(row.into_iter().enumerate().map(|(col_idx, value)| {
                                let col = self.columns.get(col_idx);
                                let mut cell = div().h_flex().h_full().px_3().child(value);

                                if let Some(w) = col.and_then(|c| c.width) {
                                    cell = cell.w(w);
                                } else {
                                    cell = cell.flex_1();
                                }

                                cell.into_any_element()
                            }))
                            .into_any_element()
                    })),
            )
    }
}
