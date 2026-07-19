use crate::Gallery;
use acme_ui::*;
use gpui::{Context, IntoElement, ParentElement as _, Styled as _, div, px};

impl Gallery {
    pub fn new_controls_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;
        Card::new()
                    .title("New V2/V3 Controls")
                    .description("Textarea, Select, Combobox, Pagination, Sidebar, Resizable")
                    // Textarea row
                    .child(Separator::new())
                    .child(div().child("Textarea (V2)"))
                    .child(self.textarea_entity.clone())
                    // Select row
                    .child(Separator::new())
                    .child(div().child("Select (V2)"))
                    .child(
                        div().flex().gap_2().items_center()
                            .child(
                                Button::new("toggle-select")
                                    .small().secondary()
                                    .label(if self.select_open { "Close" } else { "Open" })
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.select_open = !this.select_open;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Select::new("demo-select")
                                    .placeholder("Choose one...")
                                    .options(vec![
                                        SelectOption::new("Option A", "a"),
                                        SelectOption::new("Option B", "b"),
                                        SelectOption::new("Option C", "c"),
                                    ])
                                    .selected(self.select_selected.unwrap_or(99))
                                    .open(self.select_open)
                                    .on_select({
                                        let handle = cx.entity().downgrade();
                                        move |index, _event, _window, cx| {
                                            if let Some(handle) = handle.upgrade() {
                                                handle.update(cx, |this, cx| {
                                                    this.select_selected = Some(index);
                                                    this.select_open = false;
                                                    cx.notify();
                                                });
                                            }
                                        }
                                    })
                            )
                    )
                    // Combobox row
                    .child(Separator::new())
                    .child(div().child("Combobox (V2)"))
                    .child(self.combobox_entity.clone())
                    // Pagination row
                    .child(Separator::new())
                    .child(div().child("Pagination (V3)"))
                    .child(
                        Pagination::new("demo-pagination")
                            .current(self.pagination_current)
                            .total(10)
                            .on_page_change({
                                let handle = cx.entity().downgrade();
                                move |page, _event, _window, cx| {
                                    if let Some(handle) = handle.upgrade() {
                                        handle.update(cx, |this, cx| {
                                            this.pagination_current = page;
                                            cx.notify();
                                        });
                                    }
                                }
                            })
                    )
                    // Sidebar + Resizable row
                    .child(Separator::new())
                    .child(div().child("Sidebar (V3) + Resizable (V3)"))
                    .child(
                        div().h(px(200.)).flex().w_full()
                            .child(
                                Sidebar::new("demo-sidebar")
                                    .title("Navigation")
                                    .width(px(160.))
                                    .child(div().child("Item 1"))
                                    .child(div().child("Item 2"))
                                    .child(div().child("Item 3"))
                            )
                            .child(
                                div().flex_1().bg(c.surface)
                                    .child(" Main Content ")
                            )
                    )
                    .child(Separator::new())
                    .child(
                        div().child("Resizable (V3) ??drag the divider:")
                    )
                    .child(
                        div().h(px(100.)).w_full()
                            .child(self.resizable_entity.clone())
                    )
    }
}
