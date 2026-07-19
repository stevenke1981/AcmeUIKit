use crate::Gallery;
use acme_ui::*;
use gpui::{Context, IntoElement, ParentElement as _, Styled as _, div, px};

impl Gallery {
    pub fn notification_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;
        Card::new()
            .title("Notifications (V2)")
            .description("Toast-style notifications with auto-dismiss")
            .child(
                div()
                    .v_flex()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_2()
                            .child(
                                Button::new("notify-info")
                                    .extra_small()
                                    .label("Info")
                                    .on_click(cx.listener(|_this, _, _, cx| {
                                        cx.notify();
                                    })),
                            )
                            .child(
                                Button::new("notify-success")
                                    .extra_small()
                                    .primary()
                                    .label("Success")
                                    .on_click(cx.listener(|_this, _, _, cx| {
                                        cx.notify();
                                    })),
                            )
                            .child(
                                Button::new("notify-warning")
                                    .extra_small()
                                    .secondary()
                                    .label("Warning")
                                    .on_click(cx.listener(|_this, _, _, cx| {
                                        cx.notify();
                                    })),
                            )
                            .child(
                                Button::new("notify-error")
                                    .extra_small()
                                    .danger()
                                    .label("Error")
                                    .on_click(cx.listener(|_this, _, _, cx| {
                                        cx.notify();
                                    })),
                            ),
                    )
                    .child(
                        div()
                            .text_color(c.muted_foreground)
                            .text_size(px(11.))
                            .child(
                                "Notifications require Entity integration ??see V2 docs for setup.",
                            ),
                    ),
            )
    }
}
