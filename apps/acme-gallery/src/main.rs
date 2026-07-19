use acme_ui::{
    ActiveTheme, Badge, Button, Card, Checkbox, Combobox, ComboboxOption, Dialog, Direction,
    FieldShell, Icon, IconName, Menu, MenuItem, Pagination, Popover, Progress, Radio, RadioGroup,
    Resizable, Select, SelectOption, Separator, Sidebar, Skeleton, StyledExt, Switch, Tabs,
    TextInput, Textarea, Theme, ThemeMode, Tooltip,
};
use gpui::{
    AppContext as _, Context, ElementId, Entity, InteractiveElement as _, IntoElement,
    ParentElement as _, Render, StatefulInteractiveElement as _, Styled as _, Window,
    WindowOptions, div, px,
};

struct Gallery {
    // V1 state
    dark: bool,
    counter: usize,
    switch_enabled: bool,
    progress: f32,
    selected_tab: usize,
    // V2 state
    text_input_entity: Entity<TextInput>,
    checkbox_checked: bool,
    radio_selected: &'static str,
    dialog_open: bool,
    popover_open: bool,
    menu_open: bool,
    menu_selected: Option<usize>,
    #[allow(dead_code)]
    v2_section: &'static str,
    // V2b state
    textarea_entity: Entity<Textarea>,
    combobox_entity: Entity<Combobox>,
    resizable_entity: Entity<Resizable>,
    select_open: bool,
    select_selected: Option<usize>,
    pagination_current: usize,
}

impl Gallery {
    fn new(cx: &mut Context<Self>) -> Self {
        let text_input_entity =
            cx.new(|cx| TextInput::new("text-input-demo", cx).placeholder("Type something..."));
        let textarea_entity = cx.new(|cx| {
            Textarea::new("textarea-demo", cx)
                .placeholder("Tell us about yourself…")
                .rows(4)
        });
        let combobox_entity = cx.new(|cx| {
            Combobox::new("combobox-demo", cx)
                .placeholder("Search fruit…")
                .options(vec![
                    ComboboxOption::new("Apple", "apple"),
                    ComboboxOption::new("Banana", "banana"),
                    ComboboxOption::new("Cherry", "cherry"),
                    ComboboxOption::new("Grape", "grape"),
                    ComboboxOption::new("Orange", "orange"),
                    ComboboxOption::new("Strawberry", "strawberry"),
                    ComboboxOption::new("Watermelon", "watermelon"),
                ])
        });
        let resizable_entity = cx.new(|cx| {
            Resizable::new("demo-resizable", cx)
                .initial_split(0.5)
                .direction(Direction::Horizontal)
        });
        Self {
            dark: false,
            counter: 0,
            switch_enabled: true,
            progress: 64.,
            selected_tab: 0,
            text_input_entity,
            checkbox_checked: true,
            radio_selected: "option-a",
            dialog_open: false,
            popover_open: false,
            menu_open: false,
            menu_selected: None,
            v2_section: "Inputs",
            textarea_entity,
            combobox_entity,
            resizable_entity,
            select_open: false,
            select_selected: None,
            pagination_current: 1,
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
                    .child("GPUI component kit"),
            )
            .child(Separator::new())
            .child(Badge::new("V2 Components").primary())
            .child(div().h(px(8.)))
            .children(
                [
                    "Overview",
                    "Buttons",
                    "Inputs",
                    "Selection",
                    "Overlays",
                    "Icons",
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
                            .child("V2 — Editable inputs, overlays, menus, icons"),
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

        // ── V1 Foundation cards ──

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

        // ── V2 Selection card ──

        let selection_card = Card::new()
            .title("Selection controls (V2)")
            .description("Checkbox, Switch, Radio — caller-managed state")
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
            );

        // ── V2 Overlays card ──

        let overlays_card = Card::new()
            .title("Overlays (V2)")
            .description("Dialog, Popover, Menu — stateless views controlled by the Gallery")
            .child(
                div()
                    .v_flex()
                    .gap_3()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                Button::new("open-dialog")
                                    .primary()
                                    .small()
                                    .label(if self.dialog_open {
                                        "Close Dialog"
                                    } else {
                                        "Open Dialog"
                                    })
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.dialog_open = !this.dialog_open;
                                        cx.notify();
                                    })),
                            )
                            .child(
                                div()
                                    .text_color(c.muted_foreground)
                                    .text_size(px(11.))
                                    .child(if self.dialog_open {
                                        "Dialog is open"
                                    } else {
                                        "Dialog is closed"
                                    }),
                            ),
                    )
                    .child(
                        Dialog::new()
                            .title("V2 Dialog")
                            .open(self.dialog_open)
                            .on_close(cx.listener(|this, _, _, cx| {
                                this.dialog_open = false;
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .v_flex()
                                    .gap_3()
                                    .p_4()
                                    .child(
                                        div()
                                            .text_color(c.foreground)
                                            .text_size(px(13.))
                                            .child("This is a modal dialog overlay."),
                                    )
                                    .child(
                                        div()
                                            .text_color(c.muted_foreground)
                                            .text_size(px(11.))
                                            .child("Click the close button or the backdrop to dismiss."),
                                    )
                                    .child(
                                        Button::new("dialog-close-btn")
                                            .primary()
                                            .label("OK")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.dialog_open = false;
                                                cx.notify();
                                            })),
                                    ),
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
                                    .child("Popover demo"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        Button::new("toggle-popover")
                                            .small()
                                            .secondary()
                                            .label("Toggle Popover")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.popover_open = !this.popover_open;
                                                cx.notify();
                                            })),
                                    )
                                    .child(Popover::new("demo-popover").open(self.popover_open).child(
                                        div()
                                            .p_4()
                                            .v_flex()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .text_color(c.foreground)
                                                    .text_size(px(13.))
                                                    .child("Popover content"),
                                            )
                                            .child(
                                                div()
                                                    .text_color(c.muted_foreground)
                                                    .text_size(px(11.))
                                                    .child("Inline popover with themed styling."),
                                            ),
                                    )),
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
                                    .child("Menu demo"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        Button::new("toggle-menu")
                                            .small()
                                            .secondary()
                                            .label(if self.menu_open {
                                                "Close Menu"
                                            } else {
                                                "Open Menu"
                                            })
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.menu_open = !this.menu_open;
                                                cx.notify();
                                            })),
                                    )
                                    .child(
                                        div()
                                            .text_color(c.muted_foreground)
                                            .text_size(px(11.))
                                            .child(match self.menu_selected {
                                                Some(i) => format!("Selected: item {}", i),
                                                None => "Nothing selected".into(),
                                            }),
                                    ),
                            )
                            .child(
                                Menu::new("demo-menu")
                                    .open(self.menu_open)
                                    .items(vec![
                                        MenuItem::new("Save").icon("💾"),
                                        MenuItem::new("Edit").icon("✏️"),
                                        MenuItem::new("Delete").disabled(true).icon("🗑️"),
                                    ])
                                    .on_select({
                                        let handle = cx.entity().downgrade();
                                        move |index, _event, _window, cx| {
                                            if let Some(handle) = handle.upgrade() {
                                                handle.update(cx, |this, cx| {
                                                    this.menu_selected = Some(index);
                                                    this.menu_open = false;
                                                    cx.notify();
                                                });
                                            }
                                        }
                                    }),
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
                                    .child("Tooltip demo"),
                            )
                            .child(
                                Tooltip::new("V2 inline caption tooltip")
                                    .child(
                                        Badge::new("Hover me").primary(),
                                    ),
                            ),
                    ),
            );

        // ── V2/V3 New Controls card ──

        let new_controls_card = Card::new()
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
                div().child("Resizable (V3) — drag the divider:")
            )
            .child(
                div().h(px(100.)).w_full()
                    .child(self.resizable_entity.clone())
            );

        // ── V2 Icons card ──

        let icons_card = Card::new()
            .title("Icons (V2)")
            .description("Text-character icons — no SVG dependency")
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
            );

        // ── V2 Notification toast ──

        let notification_card = Card::new()
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
                                "Notifications require Entity integration — see V2 docs for setup.",
                            ),
                    ),
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
                "Acme UI Kit — V2",
                "Editable TextInput, Checkbox, Radio, Dialog, Popover, Menu, Icons, Notifications, and Tooltip.",
                cx,
            ))
            .child(
                div()
                    .grid()
                    .grid_cols(2)
                    .gap_4()
                    // V1 row
                    .child(buttons_card)
                    .child(controls_card)
                    // V1+V2 row
                    .child(fields_card)
                    .child(tabs_card)
                    // V2 row
                    .child(selection_card)
                    .child(overlays_card)
                    // V2/V3 row
                    .child(new_controls_card)
                    // V2 row
                    .child(icons_card)
                    .child(notification_card),
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

fn icon_demo(
    label: &'static str,
    name: IconName,
    c: acme_ui::ThemeColors,
) -> impl gpui::IntoElement {
    use gpui::Styled as _;
    div()
        .v_flex()
        .items_center()
        .gap_1()
        .child(Icon::new(name).with_size(px(18.)))
        .child(
            div()
                .text_size(px(9.))
                .text_color(c.muted_foreground)
                .child(label),
        )
}

fn main() {
    gpui_platform::application().run(move |cx| {
        acme_ui::init(cx);

        cx.spawn(async move |cx| {
            if let Err(error) =
                cx.open_window(WindowOptions::default(), |_, cx| cx.new(Gallery::new))
            {
                eprintln!("failed to open Acme Gallery window: {error:?}");
            }
        })
        .detach();
    });
}
