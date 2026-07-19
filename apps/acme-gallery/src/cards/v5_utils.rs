use crate::Gallery;
use acme_ui::*;
use gpui::{Context, IntoElement, ParentElement as _, Styled as _, div};

impl Gallery {
    pub fn v5_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let _c = cx.theme().colors;
        Card::new()
                    .title("V5 Utility Components")
                    .description("Alert, Tag, Slider, Avatar, Breadcrumb, Stepper, Toolbar, List, Kbd")
                    // Alert
                    .child(Separator::new())
                    .child(div().child("Alert (V5):"))
                    .child(
                        div().v_flex().gap_2()
                            .child(Alert::new("Information message").tone(Tone::Primary))
                            .child(Alert::new("Operation successful!").tone(Tone::Success))
                            .child(Alert::new("Warning: disk space low").tone(Tone::Warning))
                            .child(Alert::new("Error: connection failed").tone(Tone::Danger))
                    )
                    // Tag
                    .child(Separator::new())
                    .child(div().child("Tag (V5):"))
                    .child(
                        div().h_flex().gap_2()
                            .child(Tag::new("Rust"))
                            .child(Tag::new("TypeScript").color(hsl(210., 0.73, 0.48)))
                            .child(Tag::new("Python").color(hsl(120., 0.55, 0.45)))
                            .child(Tag::new("Filter").removable(true))
                    )
                    // Slider
                    .child(Separator::new())
                    .child(div().child("Slider (V5):"))
                    .child(Slider::new("demo-slider").value(0.65).min(0.0).max(1.0))
                    // Avatar
                    .child(Separator::new())
                    .child(div().child("Avatar (V5):"))
                    .child(
                        div().h_flex().gap_3().items_center()
                            .child(Avatar::new("Alice Wang"))
                            .child(Avatar::new("Bob Chen").size(Size::Small))
                            .child(Avatar::new("Charlie Li").size(Size::Large))
                            .child(Avatar::new("Diana"))
                    )
                    // Breadcrumb
                    .child(Separator::new())
                    .child(div().child("Breadcrumb (V5):"))
                    .child(
                        Breadcrumb::new()
                            .item("Home")
                            .item("Products")
                            .item("Widgets")
                    )
                    // Stepper
                    .child(Separator::new())
                    .child(div().child("Stepper (V5):"))
                    .child(
                        Stepper::new()
                            .step("Cart")
                            .step("Shipping")
                            .step("Payment")
                            .step("Confirm")
                            .active_step(2)
                    )
                    // Toolbar
                    .child(Separator::new())
                    .child(div().child("Toolbar (V5):"))
                    .child(
                        Toolbar::new()
                            .child(Button::new("tb-save").extra_small().primary().label("Save"))
                            .child(Button::new("tb-edit").extra_small().secondary().label("Edit"))
                            .separator()
                            .child(Button::new("tb-delete").extra_small().danger().label("Delete"))
                    )
                    // List
                    .child(Separator::new())
                    .child(div().child("List (V5):"))
                    .child(
                        List::new("demo-list")
                            .item(ListItem::new("Apple").description("A sweet red fruit"))
                            .item(ListItem::new("Banana").description("A yellow curved fruit").selected(true))
                            .item(ListItem::new("Cherry").description("A small stone fruit"))
                            .item(ListItem::new("Grapefruit").disabled(true).description("Citrus fruit"))
                    )
                    // Kbd
                    .child(Separator::new())
                    .child(div().child("Kbd / Shortcut (V5):"))
                    .child(
                        div().h_flex().gap_2()
                            .child(Kbd::new("Cmd+K"))
                            .child(Kbd::new("Cmd+Shift+P"))
                            .child(Kbd::new("Ctrl+S"))
                    )
    }
}
