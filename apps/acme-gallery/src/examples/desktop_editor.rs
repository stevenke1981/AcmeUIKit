//! Example 3: 桌面編輯器 (Desktop Editor)
//!
//! Demonstrates: TitleBar + AppMenuBar + Dock + PropertyGrid + Canvas + StatusBar + CommandPalette

use acme_ui::{
    ActiveTheme, AppMenuBar, Canvas, CommandPalette, Dock, DockArea, DockPanel, PropertyGrid,
    StatusBar, StyledExt, TitleBar,
};
use gpui::{
    Context, IntoElement, ParentElement as _, Render, SharedString, Styled as _, Window, div, px,
};

pub struct DesktopEditor {
    // Title bar
    title: SharedString,
    subtitle: SharedString,
    // Dock panels
    left_visible: bool,
    right_visible: bool,
    bottom_visible: bool,
    // Property grid
    properties: Vec<(SharedString, SharedString)>,
    // Command palette
    palette_open: bool,
    // Status bar
    status_left: SharedString,
    status_right: SharedString,
}

impl DesktopEditor {
    pub fn new() -> Self {
        Self {
            title: "Untitled Project".into(),
            subtitle: "main.rs — acme-ui".into(),
            left_visible: true,
            right_visible: true,
            bottom_visible: true,
            properties: vec![
                ("Name".into(), "main.rs".into()),
                ("Type".into(), "Rust Source".into()),
                ("Size".into(), "12 KB".into()),
                ("Modified".into(), "2026-07-20".into()),
                ("Lines".into(), "342".into()),
                ("Encoding".into(), "UTF-8".into()),
                ("Language".into(), "Rust".into()),
            ],
            palette_open: false,
            status_left: "Ready".into(),
            status_right: "UTF-8 | Ln 42, Col 15 | Rust".into(),
        }
    }
}

impl Render for DesktopEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;

        // Build TitleBar callbacks using WeakEntity pattern
        let handle = cx.entity().downgrade();

        let on_minimize = {
            let handle = handle.clone();
            move |_window: &mut Window, cx: &mut gpui::App| {
                if let Some(h) = handle.upgrade() {
                    h.update(cx, |this, cx| {
                        this.status_left = "Minimized".into();
                        cx.notify();
                    });
                }
            }
        };

        let on_maximize = {
            let handle = handle.clone();
            move |_window: &mut Window, cx: &mut gpui::App| {
                if let Some(h) = handle.upgrade() {
                    h.update(cx, |this, cx| {
                        this.status_left = "Maximized".into();
                        cx.notify();
                    });
                }
            }
        };

        let on_close = {
            let handle = handle.clone();
            move |_window: &mut Window, cx: &mut gpui::App| {
                if let Some(h) = handle.upgrade() {
                    h.update(cx, |this, cx| {
                        this.status_left = "Close requested".into();
                        cx.notify();
                    });
                }
            }
        };

        let on_menu_select = {
            let handle = handle.clone();
            move |_index: usize, _window: &mut Window, cx: &mut gpui::App| {
                if let Some(h) = handle.upgrade() {
                    h.update(cx, |this, cx| {
                        this.status_left = format!("Menu item {_index} selected").into();
                        cx.notify();
                    });
                }
            }
        };

        div()
            .flex()
            .flex_col()
            .h_full()
            .bg(c.background)
            // ── Title Bar ──
            .child(
                TitleBar::new("editor-titlebar")
                    .title(self.title.clone())
                    .subtitle(self.subtitle.clone())
                    .on_minimize(on_minimize)
                    .on_maximize(on_maximize)
                    .on_close(on_close),
            )
            // ── Menu Bar ──
            .child(
                AppMenuBar::new()
                    .add("File", &["New", "Open", "Save", "Save As", "—", "Exit"])
                    .add("Edit", &["Undo", "Redo", "—", "Cut", "Copy", "Paste", "Delete"])
                    .add("View", &["Command Palette", "Explorer", "Properties", "—", "Toggle Fullscreen"])
                    .add("Help", &["About", "Documentation", "Check for Updates"])
                    .on_select(on_menu_select),
            )
            // ── Dock (Main Content Area) ──
            .child(
                div()
                    .flex_1()
                    .child(
                        Dock::new("editor-dock")
                            .panels(vec![
                                DockPanel::new(DockArea::Left, "Explorer")
                                    .size(px(250.))
                                    .visible(self.left_visible),
                                DockPanel::new(DockArea::Right, "Properties")
                                    .size(px(260.))
                                    .visible(self.right_visible),
                                DockPanel::new(DockArea::Bottom, "Output")
                                    .size(px(160.))
                                    .visible(self.bottom_visible),
                            ])
                            // Main center: Canvas
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .bg(c.surface)
                                    .child(Canvas::new("main-canvas").size(px(640.), px(420.))),
                            )
                            // Left panel: file tree / project explorer
                            .render_left(
                                div()
                                    .v_flex()
                                    .h_full()
                                    .child(
                                        div()
                                            .h(px(28.))
                                            .flex()
                                            .items_center()
                                            .px_3()
                                            .text_size(px(11.))
                                            .text_color(c.muted_foreground)
                                            .child("PROJECT"),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .px_3()
                                            .py_2()
                                            .v_flex()
                                            .gap_1()
                                            .text_size(px(12.))
                                            .children(
                                                [
                                                    "src/",
                                                    "  main.rs",
                                                    "  lib.rs",
                                                    "  components/",
                                                    "    button.rs",
                                                    "    input.rs",
                                                    "  utils/",
                                                    "    helpers.rs",
                                                    "tests/",
                                                    "Cargo.toml",
                                                    "README.md",
                                                ]
                                                .into_iter()
                                                .map(|line| {
                                                    let is_dir = line.ends_with('/');
                                                    div()
                                                        .h(px(24.))
                                                        .flex()
                                                        .items_center()
                                                        .gap_1()
                                                        .pl(if is_dir { px(0.) } else { px(12.) })
                                                        .text_color(if is_dir {
                                                            c.foreground
                                                        } else {
                                                            c.muted_foreground
                                                        })
                                                        .child(line.trim_end_matches('/'))
                                                }),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .h(px(24.))
                                            .flex()
                                            .items_center()
                                            .px_3()
                                            .border_t_1()
                                            .border_color(c.border)
                                            .text_size(px(10.))
                                            .text_color(c.muted_foreground)
                                            .child("12 files"),
                                    ),
                            )
                            // Right panel: PropertyGrid
                            .render_right(
                                div()
                                    .v_flex()
                                    .h_full()
                                    .child(
                                        div()
                                            .h(px(28.))
                                            .flex()
                                            .items_center()
                                            .px_3()
                                            .text_size(px(11.))
                                            .text_color(c.muted_foreground)
                                            .child("PROPERTIES"),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .px_2()
                                            .py_2()
                                            .child({
                                                let mut pg = PropertyGrid::new();
                                                for (k, v) in &self.properties {
                                                    pg = pg.property(k.clone(), v.clone());
                                                }
                                                pg
                                            }),
                                    ),
                            )
                            // Bottom panel: output / terminal
                            .render_bottom(
                                div()
                                    .v_flex()
                                    .h_full()
                                    .child(
                                        div()
                                            .h(px(28.))
                                            .flex()
                                            .items_center()
                                            .px_3()
                                            .gap_3()
                                            .border_b_1()
                                            .border_color(c.border)
                                            .text_size(px(11.))
                                            .child(div().text_color(c.foreground).child("Output"))
                                            .child(div().text_color(c.muted_foreground).child("Problems"))
                                            .child(div().text_color(c.muted_foreground).child("Terminal")),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .p_3()
                                            .v_flex()
                                            .gap_1()
                                            .text_size(px(11.))
                                            .text_color(c.muted_foreground)
                                            .child(div().child("$ cargo build"))
                                            .child(div().child("   Compiling acme-ui v0.1.0"))
                                            .child(div().child("   Compiling acme-gallery v0.1.0"))
                                            .child(div().text_color(c.success).child("    Finished dev [unoptimized] in 2.34s")),
                                    ),
                            ),
                    ),
            )
            // ── Status Bar ──
            .child(
                StatusBar::new()
                    .left(self.status_left.clone())
                    .right(self.status_right.clone()),
            )
            // ── Command Palette Overlay ──
            .child(CommandPalette::new().open(self.palette_open))
    }
}
