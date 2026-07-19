use super::helpers::icon_demo;
use crate::Gallery;
use acme_ui::*;
use gpui::{Context, IntoElement, ParentElement as _, Styled as _, div, px};

impl Gallery {
    pub fn selection_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;
        Card::new()
            .title("Selection controls (V2)")
            .description("Checkbox, Switch, Radio ??caller-managed state")
            .child(
                div()
                    .v_flex()
                    .gap_3()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .v_flex()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_color(c.foreground)
                                            .text_size(px(13.))
                                            .child("Enable feature"),
                                    )
                                    .child(
                                        div()
                                            .text_color(c.muted_foreground)
                                            .text_size(px(11.))
                                            .child(if self.checkbox_checked {
                                                "Feature enabled"
                                            } else {
                                                "Feature disabled"
                                            }),
                                    ),
                            )
                            .child(
                                Checkbox::new("feature-checkbox", self.checkbox_checked)
                                    .label("Enable")
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.checkbox_checked = !this.checkbox_checked;
                                        cx.notify();
                                    })),
                            ),
                    )
                    .child(Separator::new())
                    .child(
                        div()
                            .v_flex()
                            .gap_1()
                            .child(
                                div()
                                    .text_color(c.foreground)
                                    .text_size(px(13.))
                                    .child("Radio group"),
                            )
                            .child(
                                div()
                                    .text_color(c.muted_foreground)
                                    .text_size(px(11.))
                                    .child(format!("Selected: {}", self.radio_selected)),
                            )
                            .child(
                                RadioGroup::new()
                                    .child(
                                        Radio::new("radio-a", "option-a")
                                            .label("Option A")
                                            .selected(self.radio_selected == "option-a")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.radio_selected = "option-a";
                                                cx.notify();
                                            })),
                                    )
                                    .child(
                                        Radio::new("radio-b", "option-b")
                                            .label("Option B")
                                            .selected(self.radio_selected == "option-b")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.radio_selected = "option-b";
                                                cx.notify();
                                            })),
                                    )
                                    .child(
                                        Radio::new("radio-c", "option-c")
                                            .label("Option C (disabled)")
                                            .disabled(true)
                                            .selected(self.radio_selected == "option-c")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.radio_selected = "option-c";
                                                cx.notify();
                                            })),
                                    ),
                            ),
                    ),
            )
    }

    pub fn icons_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;
        Card::new()
            .title("Icons (V2)")
            .description("Text-character icons ??no SVG dependency")
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_3()
                    .child(icon_demo("Check", IconName::Check, c))
                    .child(icon_demo("Close", IconName::Close, c))
                    .child(icon_demo("Menu", IconName::Menu, c))
                    .child(icon_demo("ChevronDown", IconName::ChevronDown, c))
                    .child(icon_demo("ChevronRight", IconName::ChevronRight, c))
                    .child(icon_demo("Info", IconName::Info, c))
                    .child(icon_demo("Warning", IconName::Warning, c))
                    .child(icon_demo("Error", IconName::Error, c))
                    .child(icon_demo("Success", IconName::Success, c)),
            )
    }
}
