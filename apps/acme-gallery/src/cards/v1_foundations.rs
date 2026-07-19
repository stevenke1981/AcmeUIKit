use crate::Gallery;
use acme_ui::*;
use gpui::{Context, IntoElement, ParentElement as _, Styled as _, div, px};

impl Gallery {
    pub fn buttons_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;

        Card::new()
            .title("Buttons")
            .description("Semantic variants, shared sizes, disabled and selected states")
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_2()
                    .child(Button::new("default").label("Default"))
                    .child(Button::new("primary").primary().label("Primary"))
                    .child(Button::new("secondary").secondary().label("Secondary"))
                    .child(Button::new("danger").danger().label("Danger"))
                    .child(Button::new("ghost").ghost().label("Ghost"))
                    .child(Button::new("disabled").label("Disabled").disabled(true)),
            )
            .child(Separator::new())
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Button::new("count")
                            .primary()
                            .label(format!("Count: {}", self.counter))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.counter += 1;
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("reset")
                            .secondary()
                            .label("Reset")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.counter = 0;
                                cx.notify();
                            })),
                    ),
            )
    }

    pub fn controls_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;
        Card::new()
            .title("Controls")
            .description("Caller-controlled state with immediate entity updates")
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
                                    .child("Notifications"),
                            )
                            .child(
                                div()
                                    .text_color(c.muted_foreground)
                                    .text_size(px(11.))
                                    .child(if self.switch_enabled {
                                        "Enabled"
                                    } else {
                                        "Disabled"
                                    }),
                            ),
                    )
                    .child(Switch::new("notifications", self.switch_enabled).on_click(
                        cx.listener(|this, _, _, cx| {
                            this.switch_enabled = !this.switch_enabled;
                            cx.notify();
                        }),
                    )),
            )
            .child(Separator::new())
            .child(
                div()
                    .v_flex()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_color(c.foreground)
                                    .text_size(px(13.))
                                    .child(format!("Progress: {:.0}%", self.progress)),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap_2()
                                    .child(
                                        Button::new("progress-minus")
                                            .small()
                                            .label("-10")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.progress = (this.progress - 10.).max(0.);
                                                cx.notify();
                                            })),
                                    )
                                    .child(
                                        Button::new("progress-plus").small().label("+10").on_click(
                                            cx.listener(|this, _, _, cx| {
                                                this.progress = (this.progress + 10.).min(100.);
                                                cx.notify();
                                            }),
                                        ),
                                    ),
                            ),
                    )
                    .child(Progress::new(self.progress)),
            )
    }

    pub fn fields_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;
        Card::new()
            .title("Fields and status")
            .description("V1 FieldShell (visual-only) and V2 TextInput (editable)")
            .child(
                div()
                    .grid()
                    .grid_cols(2)
                    .gap_3()
                    .child(
                        FieldShell::new("Project name")
                            .value("Acme Desktop")
                            .helper("Displayed in the window title"),
                    )
                    .child(
                        FieldShell::new("API token")
                            .placeholder("Paste token")
                            .helper("Token is required")
                            .error(true),
                    ),
            )
            .child(Separator::new())
            .child(
                div()
                    .v_flex()
                    .gap_2()
                    .child(
                        div()
                            .text_color(c.foreground)
                            .text_size(px(13.))
                            .child("TextInput (V2):"),
                    )
                    .child(self.text_input_entity.clone()),
            )
            .child(Separator::new())
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(Badge::new("Ready").success())
                    .child(Badge::new("Review").warning())
                    .child(Badge::new("Blocked").danger())
                    .child(Badge::new("Draft")),
            )
    }

    pub fn tabs_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;
        Card::new()
            .title("Tabs and loading placeholders")
            .description("The Gallery owns selected state; Tabs remains a presentation primitive")
            .child(Tabs::new(["Design", "Code", "Tests"]).selected(self.selected_tab))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        Button::new("tab-design")
                            .extra_small()
                            .label("Design")
                            .selected(self.selected_tab == 0)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.selected_tab = 0;
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("tab-code")
                            .extra_small()
                            .label("Code")
                            .selected(self.selected_tab == 1)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.selected_tab = 1;
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("tab-tests")
                            .extra_small()
                            .label("Tests")
                            .selected(self.selected_tab == 2)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.selected_tab = 2;
                                cx.notify();
                            })),
                    ),
            )
            .child(
                div()
                    .v_flex()
                    .gap_2()
                    .child(Skeleton::line())
                    .child(Skeleton::new(px(320.), px(12.)))
                    .child(Skeleton::new(px(180.), px(12.))),
            )
    }
}
