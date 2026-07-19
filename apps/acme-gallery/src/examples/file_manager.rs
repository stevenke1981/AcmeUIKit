//! Example 2: 檔案／資產管理器 (File/Asset Manager)
//!
//! Demonstrates: Tree + DataGrid + ThumbnailStrip + Menu + DropZone

use std::rc::Rc;

use acme_ui::{
    ActiveTheme, DataGrid, DataGridColumn, DataGridRow, DropZone, StyledExt, ThumbnailStrip, Tree,
    TreeNode,
};
use gpui::{
    AppContext as _, Context, Entity, IntoElement, ParentElement as _, Render, SharedString,
    Styled as _, Window, div, px,
};

pub struct FileManager {
    // Tree
    folder_tree: Vec<TreeNode>,
    expanded: Rc<Vec<usize>>,
    selected_folder: usize,
    // DataGrid (child entity)
    data_grid: Entity<DataGrid>,
    // Thumbnails
    thumbnails: Vec<SharedString>,
    selected_thumbnail: Option<usize>,
    // Context menu
    #[allow(dead_code)]
    context_menu_open: bool,
    // Drop zone
    drop_active: bool,
    // File data
    current_folder: SharedString,
}

impl FileManager {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let data_grid = cx.new(|cx| {
            DataGrid::new("fm-datagrid", cx)
                .column(
                    DataGridColumn::new("name", "Name")
                        .width(px(220.))
                        .sortable(true)
                        .filterable(true),
                )
                .column(
                    DataGridColumn::new("type", "Type")
                        .width(px(100.))
                        .sortable(true),
                )
                .column(
                    DataGridColumn::new("size", "Size")
                        .width(px(90.))
                        .sortable(true),
                )
                .column(
                    DataGridColumn::new("modified", "Modified")
                        .width(px(140.))
                        .sortable(true),
                )
                .rows(vec![
                    DataGridRow::new(vec![
                        "document.pdf".into(),
                        "PDF".into(),
                        "2.4 MB".into(),
                        "2026-07-19 14:32".into(),
                    ]),
                    DataGridRow::new(vec![
                        "photo.jpeg".into(),
                        "Image".into(),
                        "1.1 MB".into(),
                        "2026-07-19 10:15".into(),
                    ]),
                    DataGridRow::new(vec![
                        "data.json".into(),
                        "JSON".into(),
                        "48 KB".into(),
                        "2026-07-18 22:40".into(),
                    ]),
                    DataGridRow::new(vec![
                        "main.rs".into(),
                        "Rust".into(),
                        "12 KB".into(),
                        "2026-07-18 16:00".into(),
                    ]),
                    DataGridRow::new(vec![
                        "logo.png".into(),
                        "PNG".into(),
                        "256 KB".into(),
                        "2026-07-17 09:30".into(),
                    ]),
                    DataGridRow::new(vec![
                        "notes.txt".into(),
                        "Text".into(),
                        "4 KB".into(),
                        "2026-07-16 11:20".into(),
                    ]),
                    DataGridRow::new(vec![
                        "budget.xlsx".into(),
                        "Spreadsheet".into(),
                        "84 KB".into(),
                        "2026-07-15 15:45".into(),
                    ]),
                    DataGridRow::new(vec![
                        "presentation.pptx".into(),
                        "Slides".into(),
                        "3.8 MB".into(),
                        "2026-07-14 08:00".into(),
                    ]),
                    DataGridRow::new(vec![
                        "archive.zip".into(),
                        "Archive".into(),
                        "15 MB".into(),
                        "2026-07-13 19:10".into(),
                    ]),
                ])
                .show_filter_row(true)
        });

        Self {
            folder_tree: vec![
                TreeNode::new("Documents", 0).children(vec![
                    TreeNode::new("Work", 1).children(vec![
                        TreeNode::new("Projects", 2),
                        TreeNode::new("Reports", 3),
                    ]),
                    TreeNode::new("Personal", 4).children(vec![
                        TreeNode::new("Photos", 5),
                        TreeNode::new("Finance", 6),
                    ]),
                ]),
                TreeNode::new("Downloads", 7).children(vec![
                    TreeNode::new("Applications", 8),
                    TreeNode::new("Archives", 9),
                ]),
                TreeNode::new("Images", 10),
                TreeNode::new("Music", 11).disabled(true),
            ],
            expanded: Rc::new(vec![0, 1, 4]),
            selected_folder: 0,
            data_grid,
            thumbnails: vec![
                "photo.jpeg".into(),
                "logo.png".into(),
                "screenshot.png".into(),
                "banner.jpg".into(),
                "icon.svg".into(),
            ],
            selected_thumbnail: Some(0),
            context_menu_open: false,
            drop_active: false,
            current_folder: "Documents / Work / Projects".into(),
        }
    }
}

impl Render for FileManager {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;

        // Build tree on_click handler using WeakEntity pattern
        let tree_handle = cx.entity().downgrade();
        let tree_on_click = move |index: usize,
                                  _event: &gpui::ClickEvent,
                                  _window: &mut Window,
                                  cx: &mut gpui::App| {
            if let Some(handle) = tree_handle.upgrade() {
                handle.update(cx, |this, cx| {
                    this.selected_folder = index;
                    cx.notify();
                });
            }
        };

        // Build drop zone on_click handler using cx.listener (works for Fn(&ClickEvent, &mut Window, &mut App))
        let drop_on_click = cx.listener(|this, _event: &gpui::ClickEvent, _window, cx| {
            this.drop_active = !this.drop_active;
            cx.notify();
        });

        div()
            .flex()
            .h_full()
            // ── Left: Folder Tree ──
            .child(
                div()
                    .w(px(240.))
                    .h_full()
                    .v_flex()
                    .border_r_1()
                    .border_color(c.border)
                    .bg(c.surface)
                    .child(
                        div()
                            .h(px(40.))
                            .flex()
                            .items_center()
                            .px_3()
                            .border_b_1()
                            .border_color(c.border)
                            .text_size(px(13.))
                            .text_color(c.foreground)
                            .child("Explorer"),
                    )
                    .child(
                        div()
                            .flex_1()
                            .px_2()
                            .py_2()
                            .child(
                                Tree::new("folder-tree")
                                    .nodes(self.folder_tree.clone())
                                    .expanded(&self.expanded)
                                    .on_click(tree_on_click),
                            ),
                    )
                    .child(
                        div()
                            .h(px(32.))
                            .flex()
                            .items_center()
                            .px_3()
                            .border_t_1()
                            .border_color(c.border)
                            .text_size(px(11.))
                            .text_color(c.muted_foreground)
                            .child(format!("{} items", 12)),
                    ),
            )
            // ── Right: Content Area ──
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    // Breadcrumb bar
                    .child(
                        div()
                            .h(px(36.))
                            .flex()
                            .items_center()
                            .px_3()
                            .gap_1()
                            .border_b_1()
                            .border_color(c.border)
                            .text_size(px(11.))
                            .text_color(c.muted_foreground)
                            .child(div().text_color(c.primary).cursor_pointer().child("Home"))
                            .child(div().child("/"))
                            .child(self.current_folder.clone()),
                    )
                    // DataGrid
                    .child(div().flex_1().child(self.data_grid.clone()))
                    // Thumbnail strip
                    .child(
                        div()
                            .h(px(100.))
                            .v_flex()
                            .border_t_1()
                            .border_color(c.border)
                            .bg(c.surface)
                            .child(
                                div()
                                    .h(px(24.))
                                    .flex()
                                    .items_center()
                                    .px_3()
                                    .text_size(px(10.))
                                    .text_color(c.muted_foreground)
                                    .child("Preview"),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .px_2()
                                    .flex()
                                    .items_center()
                                    .child({
                                        let mut strip = ThumbnailStrip::new("preview-strip");
                                        for t in &self.thumbnails {
                                            strip = strip.item(t.clone());
                                        }
                                        strip.selected(self.selected_thumbnail.unwrap_or(0))
                                    }),
                            ),
                    )
                    // Drop zone
                    .child(
                        div()
                            .h(px(100.))
                            .border_t_1()
                            .border_color(c.border)
                            .p_2()
                            .child(
                                DropZone::new("file-dropzone")
                                    .label("Upload Files")
                                    .hint("Drag & drop files anywhere on this panel")
                                    .active(self.drop_active)
                                    .on_click(drop_on_click),
                            ),
                    ),
            )
    }
}
