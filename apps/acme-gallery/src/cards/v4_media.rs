use crate::Gallery;
use acme_ui::*;
use gpui::{Context, IntoElement, ParentElement as _, Styled as _, div, px};

impl Gallery {
    pub fn v4_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;
        Card::new()
                    .title("V4 Rich Content")
                    .description("Settings, BarChart, Markdown, Dock, Tiles")
                    // Settings
                    .child(Separator::new())
                    .child(div().child("Settings (V4):"))
                    .child(
                        SettingsPage::new("demo-settings").groups(vec![
                            SettingsGroup::new("appearance", "Appearance")
                                .description("Customise the display")
                                .row(
                                    SettingsRow::new("theme", "Theme")
                                        .description("Light or dark mode")
                                        .control(
                                            Button::new("theme-btn-inner")
                                                .extra_small()
                                                .secondary()
                                                .label("Toggle"),
                                        ),
                                )
                                .row(
                                    SettingsRow::new("lang", "Language")
                                        .description("UI language"),
                                ),
                            SettingsGroup::new("account", "Account")
                                .description("Your profile settings")
                                .row(
                                    SettingsRow::new("email", "Email")
                                        .description("Primary email address"),
                                ),
                        ]),
                    )
                    // BarChart
                    .child(Separator::new())
                    .child(div().child("BarChart (V4):"))
                    .child(
                        BarChart::new("demo-chart")
                            .height(px(160.))
                            .max_value(100.0)
                            .bars(vec![
                                BarEntry::new("Mon", 45.0),
                                BarEntry::new("Tue", 72.0).color(hsl(142., 0.71, 0.38)),
                                BarEntry::new("Wed", 60.0),
                                BarEntry::new("Thu", 88.0).color(hsl(0., 0.72, 0.51)),
                                BarEntry::new("Fri", 34.0),
                            ]),
                    )
                    // Markdown
                    .child(Separator::new())
                    .child(div().child("Markdown (V4):"))
                    .child(
                        Markdown::new("demo-markdown")
                            .text("# Welcome\n\nThis is **bold** text and `inline code`.\n\n- Item one\n- Item two\n\n## Subheading\nMore *italic* text here."),
                    )
                    // Dock (simplified demo)
                    .child(Separator::new())
                    .child(div().child("Dock Layout (V4):"))
                    .child(
                        div().h(px(180.)).w_full()
                            .child(
                                Dock::new("demo-dock")
                                    .panels(vec![
                                        DockPanel::new(DockArea::Left, "Explorer").size(px(120.)),
                                        DockPanel::new(DockArea::Bottom, "Terminal").size(px(60.)),
                                    ])
                                    .render_left(div().p_2().child("Explorer Panel").into_any_element())
                                    .render_bottom(div().p_2().child("Terminal Output").into_any_element())
                                    .child(div().p_4().child("Main Editor Area"))
                            )
                    )
                    // Tiles
                    .child(Separator::new())
                    .child(div().child("Tiles Layout (V4):"))
                    .child(
                        div().h(px(120.)).w_full()
                            .child(
                                Tiles::new("demo-tiles")
                                    .direction(TileDirection::Horizontal)
                                    .tile(Tile::new().fixed(px(100.)).child(
                                        div().p_2().bg(c.surface).child("Sidebar"),
                                    ))
                                    .tile(Tile::new().child(
                                        div().p_2().child("Content"),
                                    )),
                            ),
                    )
    }
}
