use acme_ui::{
    ActiveTheme, Badge, Button, Card, FieldShell, Progress, Separator, Skeleton, StyledExt, Switch,
    Tabs, Theme, ThemeMode,
};
use gpui::{
    AppContext as _, Context, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    Render, StatefulInteractiveElement as _, Styled as _, Window, WindowOptions, div, px,
};

struct Gallery {
    dark: bool,
    counter: usize,
    switch_enabled: bool,
    progress: f32,
    selected_tab: usize,
}

impl Gallery {
    fn new() -> Self {
        Self {
            dark: false,
            counter: 0,
            switch_enabled: true,
            progress: 64.,
            selected_tab: 0,
        }
    }

    fn section_title(
        title: &'static str,
        description: &'static str,
        cx: &gpui::App,
    ) -> impl IntoElement {
        let c = cx.theme().colors;
        div()
            .v_flex()
            .gap_1()
            .child(
                div()
                    .text_size(px(18.))
                    .text_color(c.foreground)
                    .child(title),
            )
            .child(
                div()
                    .text_size(px(12.))
                    .text_color(c.muted_foreground)
                    .child(description),
            )
    }
}

impl Render for Gallery {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;

        let sidebar = div()
            .w(px(220.))
            .h_full()
            .v_flex()
            .gap_2()
            .border_r_1()
            .border_color(c.border)
            .bg(c.surface)
            .p_4()
            .child(
                div()
                    .text_size(px(18.))
                    .text_color(c.foreground)
                    .child("Acme UI"),
            )
            .child(
                div()
                    .text_size(px(11.))
                    .text_color(c.muted_foreground)
                    .child("GPUI component starter"),
            )
            .child(Separator::new())
            .child(Badge::new("V1 Foundation").primary())
            .child(div().h(px(8.)))
            .children(
                [
                    "Overview", "Buttons", "Inputs", "Status", "Layout", "Roadmap",
                ]
                .into_iter()
                .enumerate()
                .map(|(index, label)| {
                    div()
                        .h(px(32.))
                        .px_3()
                        .flex()
                        .items_center()
                        .rounded(cx.theme().radius)
                        .bg(if index == 0 { c.muted } else { c.surface })
                        .text_color(if index == 0 {
                            c.foreground
                        } else {
                            c.muted_foreground
                        })
                        .text_size(px(12.))
                        .child(label)
                }),
            );

        let header = div()
            .w_full()
            .h(px(64.))
            .flex()
            .items_center()
            .justify_between()
            .border_b_1()
            .border_color(c.border)
            .px_6()
            .child(
                div()
                    .v_flex()
                    .gap_1()
                    .child(
                        div()
                            .text_size(px(16.))
                            .text_color(c.foreground)
                            .child("Component Gallery"),
                    )
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(c.muted_foreground)
                            .child("Clean-room Rust + GPUI starter"),
                    ),
            )
            .child(
                Button::new("theme-toggle")
                    .secondary()
                    .small()
                    .label(if self.dark {
                        "Switch to Light"
                    } else {
                        "Switch to Dark"
                    })
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.dark = !this.dark;
                        Theme::set_mode(
                            if this.dark {
                                ThemeMode::Dark
                            } else {
                                ThemeMode::Light
                            },
                            window,
                            cx,
                        );
                        cx.notify();
                    })),
            );

        let buttons_card =
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
                        .child(Button::new("reset").secondary().label("Reset").on_click(
                            cx.listener(|this, _, _, cx| {
                                this.counter = 0;
                                cx.notify();
                            }),
                        )),
                );

        let controls_card = Card::new()
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
            );

        let fields_card = Card::new()
            .title("Fields and status")
            .description("V1 FieldShell is visual-only; editable input is scheduled for V2")
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
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(Badge::new("Ready").success())
                    .child(Badge::new("Review").warning())
                    .child(Badge::new("Blocked").danger())
                    .child(Badge::new("Draft")),
            );

        let tabs_card = Card::new()
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
            );

        let content = div()
            .id(ElementId::Name("content".into()))
            .w_full()
            .flex_1()
            .overflow_y_scroll()
            .v_flex()
            .gap_5()
            .p_6()
            .bg(c.background)
            .child(Self::section_title(
                "Foundation components",
                "A deliberately small V1 that can be extended safely into a complete desktop UI kit.",
                cx,
            ))
            .child(
                div()
                    .grid()
                    .grid_cols(2)
                    .gap_4()
                    .child(buttons_card)
                    .child(controls_card)
                    .child(fields_card)
                    .child(tabs_card),
            );

        div()
            .size_full()
            .flex()
            .bg(c.background)
            .text_color(c.foreground)
            .child(sidebar)
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .v_flex()
                    .child(header)
                    .child(content),
            )
    }
}

fn main() {
    gpui_platform::application().run(move |cx| {
        acme_ui::init(cx);

        cx.spawn(async move |cx| {
            if let Err(error) =
                cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| Gallery::new()))
            {
                eprintln!("failed to open Acme Gallery window: {error:?}");
            }
        })
        .detach();
    });
}
