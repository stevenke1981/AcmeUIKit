use crate::{ActiveTheme, StyledExt};
use gpui::{
    App, ElementId, Hsla, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, prelude::FluentBuilder, px,
};

/// A single cell in a heatmap.
#[derive(Clone)]
pub struct HeatmapCell {
    pub value: f32,
    pub label: Option<SharedString>,
    pub color: Option<Hsla>,
}

impl HeatmapCell {
    /// Creates a new heatmap cell with a numeric value.
    pub fn new(value: f32) -> Self {
        Self {
            value,
            label: None,
            color: None,
        }
    }

    /// Sets an optional display label for this cell.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets an explicit color for this cell. When `None` the color is
    /// interpolated from blue (low) to red (high).
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

/// A grid heatmap component.
///
/// Renders a matrix of colored cells where colour intensity represents
/// data density. Supports optional row and column labels as well as
/// custom cell sizes.
///
/// # Example
///
/// ```ignore
/// Heatmap::new("activity")
///     .cell_size(px(28.))
///     .row_labels(vec!["Mon", "Tue", "Wed"])
///     .col_labels(vec!["9am", "10am", "11am"])
///     .rows(vec![
///         vec![
///             HeatmapCell::new(0.3),
///             HeatmapCell::new(0.8).label("8"),
///             HeatmapCell::new(0.5),
///         ],
///         vec![
///             HeatmapCell::new(0.1),
///             HeatmapCell::new(0.6),
///             HeatmapCell::new(0.9).label("9"),
///         ],
///     ])
/// ```
#[derive(IntoElement)]
pub struct Heatmap {
    id: ElementId,
    rows: Vec<Vec<HeatmapCell>>,
    row_labels: Vec<SharedString>,
    col_labels: Vec<SharedString>,
    cell_size: gpui::Pixels,
    max_value: Option<f32>,
}

impl Heatmap {
    /// Creates a new heatmap with the given `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            rows: Vec::new(),
            row_labels: Vec::new(),
            col_labels: Vec::new(),
            cell_size: px(30.),
            max_value: None,
        }
    }

    /// Replaces the row data (a 2D grid of cells).
    pub fn rows(mut self, rows: Vec<Vec<HeatmapCell>>) -> Self {
        self.rows = rows;
        self
    }

    /// Sets the row labels (displayed on the left side).
    pub fn row_labels(mut self, labels: Vec<impl Into<SharedString>>) -> Self {
        self.row_labels = labels.into_iter().map(|x| x.into()).collect();
        self
    }

    /// Sets the column labels (displayed at the top).
    pub fn col_labels(mut self, labels: Vec<impl Into<SharedString>>) -> Self {
        self.col_labels = labels.into_iter().map(|x| x.into()).collect();
        self
    }

    /// Sets the size of each cell in pixels (width and height).
    pub fn cell_size(mut self, size: gpui::Pixels) -> Self {
        self.cell_size = size;
        self
    }

    /// Sets a fixed maximum value for colour scaling. When `None`, the
    /// maximum is derived from the data.
    pub fn max_value(mut self, max: f32) -> Self {
        self.max_value = Some(max);
        self
    }
}

impl RenderOnce for Heatmap {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;
        let cell = self.cell_size;

        let max_val = self.max_value.unwrap_or_else(|| {
            self.rows
                .iter()
                .flat_map(|r| r.iter().map(|cell| cell.value))
                .reduce(f32::max)
                .unwrap_or(1.0)
                .max(1.0)
        });

        let has_row_labels = !self.row_labels.is_empty();
        let has_col_labels = !self.col_labels.is_empty();

        div()
            .id(self.id)
            .v_flex()
            // Column labels
            .when(has_col_labels, |this| {
                this.child(
                    div()
                        .h_flex()
                        .gap(px(2.))
                        .child(
                            div()
                                .when(has_row_labels, |this| this.w(cell))
                                .into_any_element(),
                        )
                        .children(self.col_labels.iter().map(|lbl| {
                            div()
                                .w(cell)
                                .text_center()
                                .text_size(theme.font_sizes.caption)
                                .text_color(c.muted_foreground)
                                .child(lbl.clone())
                                .into_any_element()
                        })),
                )
            })
            // Rows
            .children(self.rows.into_iter().enumerate().map(|(ri, row)| {
                div()
                    .h_flex()
                    .gap(px(2.))
                    .mt_1()
                    .when(has_row_labels && ri < self.row_labels.len(), |this| {
                        this.child(
                            div()
                                .w(cell)
                                .h(cell)
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(theme.font_sizes.caption)
                                .text_color(c.muted_foreground)
                                .child(self.row_labels[ri].clone()),
                        )
                    })
                    .children(row.into_iter().map(|cell_data| {
                        let ratio = (cell_data.value / max_val).clamp(0.0, 1.0);
                        // Interpolate from blue (high hue) to red (low hue)
                        let hue = 0.6 - ratio * 0.6;
                        let cell_color = cell_data.color.unwrap_or(Hsla {
                            h: hue,
                            s: 0.6,
                            l: 0.3 + 0.4 * ratio,
                            a: 1.0,
                        });

                        div()
                            .size(cell)
                            .rounded(px(3.))
                            .bg(cell_color)
                            .flex()
                            .items_center()
                            .justify_center()
                            .when_some(cell_data.label, |this, lbl| {
                                this.child(
                                    div()
                                        .text_size(theme.font_sizes.caption)
                                        .text_color(Hsla {
                                            h: 0.0,
                                            s: 0.0,
                                            l: 1.0,
                                            a: 1.0,
                                        })
                                        .child(lbl),
                                )
                            })
                            .into_any_element()
                    }))
                    .into_any_element()
            }))
            .into_any_element()
    }
}
