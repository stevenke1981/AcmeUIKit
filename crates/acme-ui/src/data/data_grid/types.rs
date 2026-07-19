//! Column and row data types for [`DataGrid`](super::DataGrid).

use gpui::{ElementId, Pixels, SharedString, px};

/// A column definition for [`DataGrid`](super::DataGrid).
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

/// A single row in a [`DataGrid`](super::DataGrid).
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
