use acme_ui::{
    AboutDialog, Accordion, ActiveTheme, Alert, AnnotationLayer, AppMenuBar, AreaChart,
    Autocomplete, Avatar, AvatarGroup, Badge, BarChart, BarEntry, Breadcrumb, Button, Calendar,
    Candlestick, CandlestickChart, Canvas, Card, Carousel, ChartColors, ChartSeries, Checkbox,
    ClickOutsideListener, Collapsible, ColorPicker, Combobox, ComboboxOption, CommandPalette,
    ContextToolbar, Cropper, DataGrid, DataGridColumn, DataGridRow, DatePicker, DateRangePicker,
    Dialog, DiffViewer, Direction, Dock, DockArea, DockPanel, DocumentOutline, DonutChart,
    DragRegion, Drawer, DropZone, EmptyState, ErrorState, Field, FieldShell, FilePicker,
    FindReplace, FocusRing, FocusScope, FocusTrap, Form, FormMessage, Gauge, Grid, Heatmap,
    HeatmapCell, HexViewer, Histogram, HistogramBin, HtmlView, Icon, IconButton, IconName,
    ImageView, InspectorPanel, Kbd, Label, Legend, LegendItem, LegendLayout, Lightbox, LineChart,
    LineNumbers, List, ListItem, LogLevel, LogViewer, Markdown, MarkdownPreview, MaskedInput, Menu,
    MenuItem, ModalBackdrop, MultiSelect, NavigationRail, NavigationView, NumberInput,
    OverlayDepth, Pagination, PanView, PasswordInput, PieChart, PieSlice, PinInput, Popover,
    Progress, PropertyGrid, Radio, RadioGroup, RangeSlider, Rating, Resizable, ResizeHandle,
    RichText, ScatterChart, ScatterPoint, ScatterSeries, ScrollArea, SearchInput, SegmentedControl,
    Select, SelectOption, Separator, SettingsGroup, SettingsPage, SettingsRow, ShortcutManager,
    Sidebar, Size, Skeleton, Slider, SortDirection, Sparkline, Spinner, SplitView, Stack,
    StatusBar, Stepper, StreamingChart, StyledExt, Switch, SystemTray, Table, TableColumn, Tabs,
    Tag, TextInput, Textarea, Theme, ThemeMode, ThumbnailStrip, Tile, TileDirection, Tiles,
    TimePicker, TitleBar, ToggleButton, Tone, Toolbar, Tooltip, Tree, TreeNode, WindowOverlay,
    ZoomView, hsl, render_disabled_overlay, render_loading_overlay, sr_only_label, validators,
};
use gpui::prelude::FluentBuilder;
use gpui::{
    AppContext as _, Context, ElementId, Entity, InteractiveElement as _, IntoElement,
    ParentElement as _, Render, SharedString, StatefulInteractiveElement as _, Styled as _, Window,
    WindowOptions, div, px,
};

mod examples;

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
    // V3 state
    tree_selected: Option<usize>,
    table_sort_key: Option<SharedString>,
    table_sort_dir: SortDirection,
    // V9 state
    data_grid_entity: Entity<DataGrid>,
    // P2 Infrastructure state
    states_loading: bool,
    states_disabled: bool,
    focus_trap_active: bool,
    modal_backdrop_open: bool,
    // Real-world examples
    settings_center: Entity<examples::settings_center::SettingsCenter>,
    file_manager: Entity<examples::file_manager::FileManager>,
    desktop_editor: Entity<examples::desktop_editor::DesktopEditor>,
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
        let data_grid_entity = cx.new(|cx| {
            DataGrid::new("dg-demo", cx)
                .column(
                    DataGridColumn::new("col-name", "Name")
                        .width(px(150.))
                        .sortable(true)
                        .frozen(true),
                )
                .column(
                    DataGridColumn::new("col-type", "Type")
                        .width(px(100.))
                        .sortable(true),
                )
                .column(
                    DataGridColumn::new("col-size", "Size")
                        .width(px(80.))
                        .sortable(true),
                )
                .column(
                    DataGridColumn::new("col-date", "Modified")
                        .width(px(120.))
                        .sortable(true),
                )
                .rows(vec![
                    DataGridRow::new(vec![
                        "README.md".into(),
                        "Markdown".into(),
                        "2 KB".into(),
                        "2024-12-01".into(),
                    ]),
                    DataGridRow::new(vec![
                        "src".into(),
                        "Directory".into(),
                        "--".into(),
                        "2024-12-15".into(),
                    ]),
                    DataGridRow::new(vec![
                        "Cargo.toml".into(),
                        "Config".into(),
                        "1 KB".into(),
                        "2024-11-20".into(),
                    ]),
                    DataGridRow::new(vec![
                        "main.rs".into(),
                        "Rust".into(),
                        "8 KB".into(),
                        "2024-12-10".into(),
                    ]),
                ])
                .show_filter_row(true)
        });
        let settings_center = cx.new(|_cx| examples::settings_center::SettingsCenter::new());
        let file_manager = cx.new(examples::file_manager::FileManager::new);
        let desktop_editor = cx.new(|_cx| examples::desktop_editor::DesktopEditor::new());
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
            data_grid_entity,
            states_loading: false,
            states_disabled: false,

            focus_trap_active: false,
            modal_backdrop_open: false,
            select_open: false,
            select_selected: None,
            pagination_current: 1,
            tree_selected: None,
            table_sort_key: None,
            table_sort_dir: SortDirection::None,
            settings_center,
            file_manager,
            desktop_editor,
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
                            .child("V2–V10 + P2 Charts + Infrastructure"),
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

        // ── V3 Tree + Form + Table card ──

        let v3_card = Card::new()
            .title("V3 Components")
            .description("Tree, Form+Validation, Sortable Table")
            // Tree
            .child(Separator::new())
            .child(div().child("Tree (V3) — click a node:"))
            .child(
                div()
                    .child(match self.tree_selected {
                        Some(i) => format!("Selected node: {i}"),
                        None => "No node selected".to_string(),
                    })
                    .text_size(px(11.))
                    .text_color(c.muted_foreground),
            )
            .child(Tree::new("demo-tree")
                .nodes(vec![
                    TreeNode::new("src", 0).children(vec![
                        TreeNode::new("main.rs", 1),
                        TreeNode::new("lib.rs", 2),
                        TreeNode::new("components", 3).children(vec![
                            TreeNode::new("button.rs", 4),
                            TreeNode::new("card.rs", 5),
                        ]),
                    ]),
                    TreeNode::new("Cargo.toml", 6),
                    TreeNode::new("README.md", 7),
                ])
                .expanded(&[0, 3])
                .on_click({
                    let handle = cx.entity().downgrade();
                    move |index, _event, _window, cx| {
                        if let Some(handle) = handle.upgrade() {
                            handle.update(cx, |this, cx| {
                                this.tree_selected = Some(index);
                                cx.notify();
                            });
                        }
                    }
                })
            )
            // Form
            .child(Separator::new())
            .child(div().child("Form + Validation (V3):"))
            .child(
                Form::new("demo-form")
                    .submit_label("Register")
                    .fields(vec![
                        Field::new("form-name", "Name")
                            .value("John")
                            .rule(validators::required())
                            .rule(validators::min_length(2)),
                        Field::new("form-email", "Email")
                            .value("john@example")
                            .rule(validators::email()),
                        Field::new("form-bio", "Bio")
                            .value("")
                            .rule(validators::required())
                            .helper("Tell us about yourself"),
                    ])
                    .on_submit({
                        let handle = cx.entity().downgrade();
                        move |_event, _window, cx| {
                            if let Some(handle) = handle.upgrade() {
                                handle.update(cx, |_this, cx| {
                                    cx.notify();
                                });
                            }
                        }
                    }),
            )
            // Table
            .child(Separator::new())
            .child(div().child("Sortable Table (V3):"))
            .child(
                Table::new("demo-table")
                    .columns(vec![
                        TableColumn::new("Name").sortable("name").width(px(120.)),
                        TableColumn::new("Role").sortable("role").width(px(100.)),
                        TableColumn::new("Email"),
                    ])
                    .rows(vec![
                        vec!["Alice".into(), "Engineer".into(), "alice@acme.dev".into()],
                        vec!["Bob".into(), "Designer".into(), "bob@acme.dev".into()],
                        vec!["Charlie".into(), "Manager".into(), "charlie@acme.dev".into()],
                    ])
                    .sort_key(self.table_sort_key.clone())
                    .sort_direction(self.table_sort_dir)
                    .on_sort({
                        let handle = cx.entity().downgrade();
                        move |key, dir, _window, cx| {
                            if let Some(handle) = handle.upgrade() {
                                handle.update(cx, |this, cx| {
                                    this.table_sort_key = Some(key);
                                    this.table_sort_dir = dir;
                                    cx.notify();
                                });
                            }
                        }
                    })
                    .on_row_click({
                        let handle = cx.entity().downgrade();
                        move |_index, _event, _window, cx| {
                            if let Some(handle) = handle.upgrade() {
                                handle.update(cx, |_this, cx| {
                                    cx.notify();
                                });
                            }
                        }
                    }),
            );

        // ── V4 Rich Content card ──

        let v4_card = Card::new()
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
            );

        // ── V5 Utils card ──

        let v5_card = Card::new()
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
                    .child(Kbd::new("⌘K"))
                    .child(Kbd::new("⌘⇧P"))
                    .child(Kbd::new("Ctrl+S"))
            );

        // ── V6 More Components card ──

        let v6_card = Card::new()
            .title("V6 More Components")
            .description("Label, ScrollArea, Stack, Grid, IconButton, ToggleButton, SegmentedControl, Spinner, Collapsible, Accordion, Drawer, CommandPalette, EmptyState, ErrorState, StatusBar, NumberInput, SearchInput, DatePicker, Calendar, ColorPicker, PropertyGrid")
            // Label
            .child(Separator::new())
            .child(div().child("Label (V6):"))
            .child(Label::new("Username").size(Size::Small))
            .child(Label::new("Full Name"))
            .child(Label::new("Section Heading").size(Size::Large))
            // Stack + Grid
            .child(Separator::new())
            .child(div().child("Stack + Grid (V6):"))
            .child(Stack::horizontal().gap(px(4.)).child(Button::new("s1").extra_small().label("A")).child(Button::new("s2").extra_small().label("B")).spacer().child(Button::new("s3").extra_small().label("C")))
            .child(Grid::new().cols(3).gap(px(4.)).child(Badge::new("A")).child(Badge::new("B")).child(Badge::new("C")).child(Badge::new("D")).child(Badge::new("E")))
            // ScrollArea
            .child(Separator::new())
            .child(div().child("ScrollArea (V6):"))
            .child(ScrollArea::new("demo-scroll").child(div().h(px(60.)).child("Scrollable content with overflow hidden")))
            // IconButton
            .child(Separator::new())
            .child(div().child("IconButton (V6):"))
            .child(div().h_flex().gap_2().child(IconButton::new("ib-search", IconName::Search)).child(IconButton::new("ib-settings", IconName::Settings)))
            // ToggleButton + SegmentedControl
            .child(Separator::new())
            .child(div().child("ToggleButton + SegmentedControl (V6):"))
            .child(div().h_flex().gap_2().child(ToggleButton::new("tb-dark", "Dark").selected(false)).child(ToggleButton::new("tb-light", "Light").selected(true)))
            .child(SegmentedControl::new("view-mode").items(&["List", "Grid", "Split"]).selected(1))
            // Spinner
            .child(Separator::new())
            .child(div().child("Spinner (V6):"))
            .child(div().h_flex().gap_3().child(Spinner::new().size(Size::Small)).child(Spinner::new()).child(Spinner::new().size(Size::Large)))
            // Collapsible + Accordion
            .child(Separator::new())
            .child(div().child("Collapsible + Accordion (V6):"))
            .child(Collapsible::new("Details").open(true).child(div().child("Hidden content here")))
            .child(Accordion::new().section("Section 1", true, div().child("Content 1")).section("Section 2", false, div().child("Content 2")))
            // Drawer
            .child(Separator::new())
            .child(div().child("Drawer + CommandPalette (V6) — need Entity: show placeholder:"))
            .child(div().h_flex().gap_2().child(Drawer::new("Settings").open(true).child(div().child("Drawer content"))).child(CommandPalette::new().open(true)))
            // EmptyState
            .child(Separator::new())
            .child(div().child("EmptyState (V6):"))
            .child(EmptyState::new("No items found").icon(IconName::Search).description("Try adjusting your filters"))
            // ErrorState
            .child(Separator::new())
            .child(div().child("ErrorState (V6):"))
            .child(ErrorState::new("Connection lost").message("Check your network and try again").retry("Retry"))
            // StatusBar
            .child(Separator::new())
            .child(div().child("StatusBar (V6):"))
            .child(StatusBar::new().left("Ready").right("Line 42, Col 8"))
            // NumberInput + SearchInput
            .child(Separator::new())
            .child(div().child("NumberInput + SearchInput (V6):"))
            .child(div().h_flex().gap_3().child(NumberInput::new("demo-num").value(5).min(0).max(10)).child(SearchInput::new("demo-search").placeholder("Search…")))
            // DatePicker + Calendar
            .child(Separator::new())
            .child(div().child("DatePicker + Calendar (V6):"))
            .child(DatePicker::new("2024-12-25"))
            .child(Calendar::new("December 2024"))
            // ColorPicker
            .child(Separator::new())
            .child(div().child("ColorPicker (V6):"))
            .child(ColorPicker::new("Primary", hsl(217., 0.91, 0.59)))
            // PropertyGrid
            .child(Separator::new())
            .child(div().child("PropertyGrid (V6):"))
            .child(PropertyGrid::new().property("Name", "Acme App").property("Version", "2.1.0").property("Author", "Acme Corp"));

        // ── V7 P1 Input & Selection card ──

        let v7_card = Card::new()
            .title("V7 P1 Inputs & Selection")
            .description("PasswordInput, MaskedInput, PinInput, TimePicker, DateRangePicker, FilePicker, MultiSelect, RangeSlider, Rating, FormMessage, Autocomplete")
            // PasswordInput
            .child(Separator::new())
            .child(div().child("PasswordInput (V7):"))
            .child(PasswordInput::new("pwd-demo").placeholder("Enter password"))
            // MaskedInput
            .child(Separator::new())
            .child(div().child("MaskedInput (V7):"))
            .child(MaskedInput::new("mask-phone").mask("(XXX) XXX-XXXX").value("5551234567"))
            // PinInput
            .child(Separator::new())
            .child(div().child("PinInput (V7):"))
            .child(PinInput::new("pin-demo").digits(6).value("123"))
            // TimePicker
            .child(Separator::new())
            .child(div().child("TimePicker (V7):"))
            .child(TimePicker::new("time-demo").value("14:30"))
            // DateRangePicker
            .child(Separator::new())
            .child(div().child("DateRangePicker (V7):"))
            .child(DateRangePicker::new("range-demo").from("2024-01-01").to("2024-12-31"))
            // FilePicker
            .child(Separator::new())
            .child(div().child("FilePicker (V7):"))
            .child(FilePicker::new("file-demo").path("C:/project/src/main.rs"))
            // MultiSelect
            .child(Separator::new())
            .child(div().child("MultiSelect (V7):"))
            .child(MultiSelect::new("ms-demo").items(&["Rust", "Go", "Python", "TypeScript"]).selected_items(&["Rust", "Go"]))
            // RangeSlider
            .child(Separator::new())
            .child(div().child("RangeSlider (V7):"))
            .child(RangeSlider::new("range-slider").min(0.).max(100.).low(20.).high(80.))
            // Rating
            .child(Separator::new())
            .child(div().child("Rating (V7):"))
            .child(Rating::new("score").max(5).value(3))
            // FormMessage
            .child(Separator::new())
            .child(div().child("FormMessage (V7):"))
            .child(FormMessage::new("err-msg").tone(Tone::Danger).message("This field is required"))
            .child(FormMessage::new("warn-msg").tone(Tone::Warning).message("Please review before submitting"))
            .child(FormMessage::new("info-msg").tone(Tone::Primary).message("All changes saved"))
            // Autocomplete
            .child(Separator::new())
            .child(div().child("Autocomplete (V7):"))
            .child(Autocomplete::new("ac-demo").value("New").suggestions(&["New York", "London", "Tokyo", "Paris"]));

        // ── V8 Desktop Shell card ──

        let v8_card = Card::new()
            .title("V8 Desktop Shell")
            .description("TitleBar, WindowControls, AppMenuBar, NavigationRail, NavigationView, SplitView, InspectorPanel, ContextToolbar, ShortcutManager, SystemTray, FocusRing, FocusScope, DragRegion, DropZone, ResizeHandle, WindowOverlay, AboutDialog")
            // TitleBar
            .child(Separator::new())
            .child(div().child("TitleBar (V8):"))
            .child(TitleBar::new("title-bar").title("My App").subtitle("v1.0.0"))
            // AppMenuBar
            .child(Separator::new())
            .child(div().child("AppMenuBar (V8):"))
            .child(AppMenuBar::new().add("File", &["New", "Open", "Save"]).add("Edit", &["Undo", "Redo"]).add("Help", &["About"]))
            // NavigationRail + NavigationView
            .child(Separator::new())
            .child(div().child("NavigationRail + NavigationView (V8):"))
            .child(NavigationView::new("nav-view").rail(NavigationRail::new("nav-rail").item(IconName::Menu, "Home").item(IconName::Settings, "Settings").item(IconName::User, "Profile").selected(0)))
            // SplitView
            .child(Separator::new())
            .child(div().child("SplitView (V8):"))
            .child(SplitView::new("split-view").ratio(0.4).left_label("Left Panel").right_label("Right Panel"))
            // InspectorPanel
            .child(Separator::new())
            .child(div().child("InspectorPanel (V8):"))
            .child(InspectorPanel::new("inspector").section("Properties", &[("Width", "800"), ("Height", "600"), ("Opacity", "1.0")]))
            // ContextToolbar
            .child(Separator::new())
            .child(div().child("ContextToolbar (V8):"))
            .child(ContextToolbar::new("ctx-bar").items(&["Bold", "Italic", "Underline"]))
            // ShortcutManager
            .child(Separator::new())
            .child(div().child("ShortcutManager (V8):"))
            .child(ShortcutManager::new("shortcuts").shortcut("Ctrl+S", "Save").shortcut("Ctrl+Z", "Undo").shortcut("Ctrl+Shift+Z", "Redo"))
            // SystemTray
            .child(Separator::new())
            .child(div().child("SystemTray (V8):"))
            .child(div().h_flex().gap_2().child(SystemTray::new("tray-1").icon(IconName::Settings)).child(SystemTray::new("tray-2").icon(IconName::User)))
            // FocusRing + FocusScope
            .child(Separator::new())
            .child(div().child("FocusRing + FocusScope (V8):"))
            .child(FocusScope::new("focus-scope").label("Focus Group"))
            .child(div().child(FocusRing::new("ring-1").label("Input 1").focused(true)))
            .child(div().child(FocusRing::new("ring-2").label("Input 2")))
            // DragRegion
            .child(Separator::new())
            .child(div().child("DragRegion (V8):"))
            .child(DragRegion::new("drag-region").label("Drag me"))
            // DropZone
            .child(Separator::new())
            .child(div().child("DropZone (V8):"))
            .child(DropZone::new("drop-zone").label("Drop files here").hint("Supports PNG, JPG, SVG"))
            // ResizeHandle
            .child(Separator::new())
            .child(div().child("ResizeHandle (V8):"))
            .child(ResizeHandle::new("resize-grip"))
            // WindowOverlay
            .child(Separator::new())
            .child(div().child("WindowOverlay (V8) — always visible in demo:"))
            .child(WindowOverlay::new("overlay").title("Modal Title"))
            // AboutDialog
            .child(Separator::new())
            .child(div().child("AboutDialog (V8):"))
            .child(AboutDialog::new("about").app_name("Acme UI Kit").version("v1.0.0").description("A GPUI component library"));

        // ── V9 DataGrid card ──

        let v9_card = Card::new()
            .title("V9 DataGrid")
            .description("Entity-based data grid with sort, filter, selection, edit, keyboard nav, CSV export")
            .child(Separator::new())
            .child(div().child("DataGrid (V9) — Entity-based:"))
            .child(self.data_grid_entity.clone());

        // ── V10 Content & Media card ──

        let v10_card = Card::new()
            .title("V10 Content & Media")
            .description("RichText, HtmlView, LineNumbers, DiffViewer, MarkdownPreview, DocumentOutline, FindReplace, LogViewer, HexViewer, ImageView, AvatarGroup, Carousel, Lightbox, Canvas, ZoomView, PanView, ThumbnailStrip, Cropper, AnnotationLayer")
            // RichText
            .child(Separator::new())
            .child(div().child("RichText (V10):"))
            .child(RichText::new("rt-demo").text("bold", "Bold text").text("italic", "Italic text").text("normal", "Plain text"))
            // HtmlView
            .child(Separator::new())
            .child(div().child("HtmlView (V10):"))
            .child(HtmlView::new("html-demo").html("<h1>Hello</h1><p>World</p>"))
            // LineNumbers
            .child(Separator::new())
            .child(div().child("LineNumbers (V10):"))
            .child(LineNumbers::new("ln-demo").lines(15).active_line(3))
            // DiffViewer
            .child(Separator::new())
            .child(div().child("DiffViewer (V10):"))
            .child(DiffViewer::new("diff-demo").old_text("Hello World").new_text("Hello GPUI"))
            // MarkdownPreview
            .child(Separator::new())
            .child(div().child("MarkdownPreview (V10):"))
            .child(MarkdownPreview::new("md-demo").markdown("# Title\n**bold** and *italic*"))
            // DocumentOutline
            .child(Separator::new())
            .child(div().child("DocumentOutline (V10):"))
            .child(DocumentOutline::new("outline").heading(1, "Introduction").heading(2, "Getting Started").heading(2, "Advanced"))
            // FindReplace
            .child(Separator::new())
            .child(div().child("FindReplace (V10):"))
            .child(FindReplace::new("fr-demo").find_text("search").replace_text("replace").matches(3, 1))
            // LogViewer
            .child(Separator::new())
            .child(div().child("LogViewer (V10):"))
            .child(LogViewer::new("log-demo").entry(LogLevel::Info, "10:00:00", "Application started").entry(LogLevel::Warn, "10:00:05", "Deprecated API used").entry(LogLevel::Error, "10:01:00", "Connection failed"))
            // HexViewer
            .child(Separator::new())
            .child(div().child("HexViewer (V10):"))
            .child(HexViewer::new("hex-demo").address(0x1000).data(vec![0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x00, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21]))
            // ImageView
            .child(Separator::new())
            .child(div().child("ImageView (V10):"))
            .child(ImageView::new("img-demo").src("placeholder.png").alt("Sample image").width(px(120.)).height(px(80.)))
            // AvatarGroup
            .child(Separator::new())
            .child(div().child("AvatarGroup (V10):"))
            .child(AvatarGroup::new("av-group").add("Alice").add("Bob").add("Charlie").add("Diana").max_visible(3))
            // Carousel
            .child(Separator::new())
            .child(div().child("Carousel (V10):"))
            .child(Carousel::new("car-demo").slide("Slide 1", 0.6, 0.7, 0.5).slide("Slide 2", 0.3, 0.8, 0.6).slide("Slide 3", 0.5, 0.6, 0.7).current(0))
            // Lightbox
            .child(Separator::new())
            .child(div().child("Lightbox (V10):"))
            .child(Lightbox::new("lb-demo").src("image.png").caption("Sample image"))
            // Canvas
            .child(Separator::new())
            .child(div().child("Canvas (V10):"))
            .child(Canvas::new("canvas-demo").size(px(150.), px(80.)))
            // ZoomView
            .child(Separator::new())
            .child(div().child("ZoomView (V10):"))
            .child(ZoomView::new("zoom-demo").zoom(1.5).label("Content"))
            // PanView
            .child(Separator::new())
            .child(div().child("PanView (V10):"))
            .child(PanView::new("pan-demo").offset(10., 20.).label("Pannable content"))
            // ThumbnailStrip
            .child(Separator::new())
            .child(div().child("ThumbnailStrip (V10):"))
            .child(ThumbnailStrip::new("strip-demo").item("Frame 1").item("Frame 2").item("Frame 3").item("Frame 4").selected(1))
            // Cropper
            .child(Separator::new())
            .child(div().child("Cropper (V10):"))
            .child(Cropper::new("crop-demo").label("Crop area").aspect_ratio(16. / 9.))
            // AnnotationLayer
            .child(Separator::new())
            .child(div().child("AnnotationLayer (V10):"))
            .child(AnnotationLayer::new("ann-demo").add("Note 1", 20., 10.).add("Note 2", 100., 40.));

        // ── P2 Infrastructure: States card ──

        let states_card = Card::new()
            .title("P2 States")
            .description("Loading overlay, disabled state, validation, AriaLabel, sr-only label, StateStyling trait")
            // Loading overlay
            .child(Separator::new())
            .child(div().child("Loading Overlay:"))
            .child(
                Button::new("toggle-loading")
                    .extra_small()
                    .secondary()
                    .label(if self.states_loading { "Hide" } else { "Show" })
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.states_loading = !this.states_loading;
                        cx.notify();
                    })),
            )
            .child(
                if self.states_loading {
                    render_loading_overlay(cx, Some("Loading...".into()))
                } else {
                    render_loading_overlay(cx, None)
                },
            )
            // Disabled overlay
            .child(Separator::new())
            .child(div().child("Disabled Overlay:"))
            .child(
                Button::new("toggle-disabled")
                    .extra_small()
                    .secondary()
                    .label(if self.states_disabled { "Enabled" } else { "Disabled" })
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.states_disabled = !this.states_disabled;
                        cx.notify();
                    })),
            )
            .child(render_disabled_overlay())
            // Validation border
            .child(Separator::new())
            .child(div().child("Validation border color:"))
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(
                        div()
                            .rounded(cx.theme().radius)
                            .px_3()
                            .py_1()
                            .border_1()
                            .border_color(c.success)
                            .child("Valid"),
                    )
                    .child(
                        div()
                            .rounded(cx.theme().radius)
                            .px_3()
                            .py_1()
                            .border_1()
                            .border_color(c.danger)
                            .child("Invalid"),
                    ),
            )
            // State styling (disabled vs normal buttons)
            .child(Separator::new())
            .child(div().child("State styling:"))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        Button::new("styling-normal")
                            .extra_small()
                            .label("Normal"),
                    )
                    .child(
                        Button::new("styling-disabled")
                            .extra_small()
                            .label("Disabled")
                            .disabled(true),
                    )
                    .child(
                        Button::new("styling-danger")
                            .extra_small()
                            .danger()
                            .label("Danger"),
                    )
                    .child(
                        Button::new("styling-primary")
                            .extra_small()
                            .primary()
                            .label("Primary"),
                    ),
            )
            // sr-only label
            .child(Separator::new())
            .child(div().child("SR-only label (inspect DOM):"))
            .child(sr_only_label("Hidden screen reader text"));

        // ── P2 Infrastructure: Accessibility card ──

        let reduced_motion = false; // prefers_reduced_motion requires Window + App

        let access_card = Card::new()
            .title("P2 Accessibility")
            .description("AriaRole, AriaAttrs, reduced-motion query, focus ring style")
            // AriaRole
            .child(Separator::new())
            .child(div().child("AriaRole variants:"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_2()
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .bg(c.surface)
                            .rounded(cx.theme().radius)
                            .text_size(px(11.))
                            .child("Button"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .bg(c.surface)
                            .rounded(cx.theme().radius)
                            .text_size(px(11.))
                            .child("Dialog"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .bg(c.surface)
                            .rounded(cx.theme().radius)
                            .text_size(px(11.))
                            .child("Alert"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .bg(c.surface)
                            .rounded(cx.theme().radius)
                            .text_size(px(11.))
                            .child("TabPanel"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .bg(c.surface)
                            .rounded(cx.theme().radius)
                            .text_size(px(11.))
                            .child("Tooltip"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .bg(c.surface)
                            .rounded(cx.theme().radius)
                            .text_size(px(11.))
                            .child("Menu"),
                    ),
            )
            // AriaAttrs builder
            .child(Separator::new())
            .child(div().child("AriaAttrs builder (API reference):"))
            .child(
                div()
                    .v_flex()
                    .gap_1()
                    .text_size(px(11.))
                    .text_color(c.muted_foreground)
                    .child("40+ ARIA roles: Button, Dialog, Alert, TabPanel, Tooltip, Menu, …")
                    .child("17 ARIA attributes: label, describedby, expanded, pressed, …"),
            )
            // Reduced motion
            .child(Separator::new())
            .child(div().child("Reduced motion preference:"))
            .child(
                div()
                    .text_size(px(11.))
                    .text_color(if reduced_motion { c.warning } else { c.success })
                    .child(if reduced_motion {
                        "Reduced motion requested"
                    } else {
                        "No reduced motion preference"
                    }),
            )
            // Focus ring style
            .child(Separator::new())
            .child(div().child("Focus ring style:"))
            .child(
                div()
                    .text_size(px(11.))
                    .text_color(c.muted_foreground)
                    .child("Use focus_ring_style(cx) to get (width, color) at runtime"),
            );

        // ── P2 Infrastructure: Focus card ──

        let focus_card = Card::new()
            .title("P2 Focus")
            .description("FocusTrap, RovingTabIndex, keyboard handlers, DefaultCancelButtons")
            // FocusTrap
            .child(Separator::new())
            .child(div().child("FocusTrap:"))
            .child(
                Button::new("toggle-focustrap")
                    .extra_small()
                    .secondary()
                    .label(if self.focus_trap_active {
                        "Trap Active"
                    } else {
                        "Trap Inactive"
                    })
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.focus_trap_active = !this.focus_trap_active;
                        cx.notify();
                    })),
            )
            .child(
                FocusTrap::new("demo-focus-trap")
                    .active(self.focus_trap_active)
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .p_2()
                            .bg(c.surface)
                            .rounded(cx.theme().radius)
                            .child(Button::new("ft-btn-1").extra_small().label("Btn 1"))
                            .child(Button::new("ft-btn-2").extra_small().label("Btn 2"))
                            .child(Button::new("ft-btn-3").extra_small().label("Btn 3")),
                    ),
            )
            // RovingTabIndex
            .child(Separator::new())
            .child(div().child("RovingTabIndex demo:"))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded(cx.theme().radius)
                            .bg(c.surface)
                            .border_1()
                            .border_color(c.border)
                            .child("Item A"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded(cx.theme().radius)
                            .bg(c.surface)
                            .border_1()
                            .border_color(c.ring)
                            .child("Item B (focused)"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded(cx.theme().radius)
                            .bg(c.surface)
                            .border_1()
                            .border_color(c.border)
                            .child("Item C"),
                    ),
            )
            // Keyboard handlers
            .child(Separator::new())
            .child(div().child("Keyboard handlers:"))
            .child(
                div()
                    .v_flex()
                    .gap_2()
                    .child(
                        Button::new("demo-escape")
                            .extra_small()
                            .secondary()
                            .label("Press Escape → callback")
                            .on_click(cx.listener(|_, _, _, _| {})),
                    )
                    .child(
                        div()
                            .text_size(px(10.))
                            .text_color(c.muted_foreground)
                            .child("escape_close_handler() / arrow_key_nav_handler() for on_key_down"),
                    ),
            )
            // DefaultCancelButtons
            .child(Separator::new())
            .child(div().child("DefaultCancelButtons:"))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(Button::new("dflt-ok").extra_small().primary().label("OK"))
                    .child(Button::new("dflt-cancel").extra_small().secondary().label("Cancel")),
            );

        // ── P2 Infrastructure: Overlay card ──

        let overlay_card = Card::new()
            .title("P2 Overlay")
            .description("ModalBackdrop, OverlayDepth, AutoPositioner, ClickOutsideListener, FocusRestore")
            // ModalBackdrop
            .child(Separator::new())
            .child(div().child("ModalBackdrop:"))
            .child(
                Button::new("toggle-backdrop")
                    .extra_small()
                    .secondary()
                    .label(if self.modal_backdrop_open {
                        "Hide Backdrop"
                    } else {
                        "Show Backdrop"
                    })
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.modal_backdrop_open = !this.modal_backdrop_open;
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .h(px(80.))
                    .relative()
                    .bg(c.surface)
                    .rounded(cx.theme().radius)
                    .child(
                        div().p_2().text_size(px(11.)).text_color(c.muted_foreground)
                            .child(if self.modal_backdrop_open {
                                "Backdrop visible"
                            } else {
                                "Click 'Show Backdrop' above"
                            }),
                    )
                    .when(self.modal_backdrop_open, |this| {
                        this.child(ModalBackdrop::new("demo-backdrop").depth(OverlayDepth::Modal))
                    }),
            )
            // OverlayDepth
            .child(Separator::new())
            .child(div().child("OverlayDepth z-index layers:"))
            .child(
                div()
                    .v_flex()
                    .gap_1()
                    .text_size(px(11.))
                    .text_color(c.muted_foreground)
                    .child("Popover=100  Drawer=200  Dialog=300  Modal=400  Toast=500  DragDrop=600"),
            )
            // AutoPositioner
            .child(Separator::new())
            .child(div().child("AutoPositioner placements:"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_2()
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .bg(c.surface)
                            .rounded(cx.theme().radius)
                            .child("Bottom"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .bg(c.surface)
                            .rounded(cx.theme().radius)
                            .child("Top"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .bg(c.surface)
                            .rounded(cx.theme().radius)
                            .child("Left"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .bg(c.surface)
                            .rounded(cx.theme().radius)
                            .child("Right"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .bg(c.surface)
                            .rounded(cx.theme().radius)
                            .child("BottomStart"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .bg(c.surface)
                            .rounded(cx.theme().radius)
                            .child("TopEnd"),
                    ),
            )
            // ClickOutsideListener
            .child(Separator::new())
            .child(div().child("ClickOutsideListener:"))
            .child(
                ClickOutsideListener::new("demo-outside")
                    .child(
                        div()
                            .bg(c.surface)
                            .rounded(cx.theme().radius)
                            .p_3()
                            .text_size(px(11.))
                            .text_color(c.muted_foreground)
                            .child("Click outside this box"),
                    ),
            )
            // FocusRestore
            .child(Separator::new())
            .child(div().child("FocusRestore:"))
            .child(
                div()
                    .v_flex()
                    .gap_1()
                    .text_size(px(11.))
                    .text_color(c.muted_foreground)
                    .child("FocusRestore saves and restores focus for overlay show/hide lifecycle")
                    .child("Used internally by Dialog, Drawer, Popover"),
            );

        let p2_charts_card = Card::new()
            .title("P2 Charts")
            .description("BarChart, LineChart, AreaChart, PieChart, DonutChart, ScatterChart, Gauge, Sparkline, Histogram, Heatmap, CandlestickChart, StreamingChart")
            // BarChart (existing)
            .child(Separator::new())
            .child(div().child("BarChart (V2):"))
            .child(BarChart::new("bc1").height(px(120.)).bars(vec![
                BarEntry::new("A", 30.0).color(ChartColors::get(0)),
                BarEntry::new("B", 50.0).color(ChartColors::get(1)),
                BarEntry::new("C", 40.0).color(ChartColors::get(2)),
                BarEntry::new("D", 70.0).color(ChartColors::get(3)),
            ]))
            // LineChart
            .child(Separator::new())
            .child(div().child("LineChart:"))
            .child(LineChart::new("lc1").height(px(100.)).show_dots(true).series(vec![
                ChartSeries::new("Series 1").color(ChartColors::get(0)).data(vec![10.0, 25.0, 15.0, 40.0, 30.0, 55.0, 45.0]),
                ChartSeries::new("Series 2").color(ChartColors::get(1)).data(vec![20.0, 15.0, 35.0, 25.0, 45.0, 35.0, 50.0]),
            ]))
            // AreaChart
            .child(Separator::new())
            .child(div().child("AreaChart:"))
            .child(AreaChart::new("ac1").height(px(100.)).fill_opacity(0.2).series(vec![
                ChartSeries::new("Series").color(ChartColors::get(2)).data(vec![10.0, 30.0, 20.0, 50.0, 35.0, 60.0, 45.0]),
            ]))
            // PieChart
            .child(Separator::new())
            .child(div().child("PieChart:"))
            .child(PieChart::new("pc1").size(px(120.)).slices(vec![
                PieSlice::new("Red", 30.0).color(ChartColors::get(0)),
                PieSlice::new("Blue", 25.0).color(ChartColors::get(1)),
                PieSlice::new("Green", 20.0).color(ChartColors::get(2)),
                PieSlice::new("Orange", 15.0).color(ChartColors::get(3)),
                PieSlice::new("Purple", 10.0).color(ChartColors::get(4)),
            ]))
            // DonutChart
            .child(Separator::new())
            .child(div().child("DonutChart:"))
            .child(DonutChart::new("dc1").size(px(120.)).hole_ratio(0.6).center_text("Total").slices(vec![
                PieSlice::new("A", 40.0).color(ChartColors::get(0)),
                PieSlice::new("B", 30.0).color(ChartColors::get(1)),
                PieSlice::new("C", 20.0).color(ChartColors::get(2)),
                PieSlice::new("D", 10.0).color(ChartColors::get(3)),
            ]))
            // ScatterChart
            .child(Separator::new())
            .child(div().child("ScatterChart:"))
            .child(ScatterChart::new("sc1").height(px(100.)).show_grid(true).series(vec![
                ScatterSeries::new("Series A").color(ChartColors::get(0))
                    .points(vec![
                        ScatterPoint::new(10.0, 20.0), ScatterPoint::new(30.0, 50.0),
                        ScatterPoint::new(50.0, 30.0), ScatterPoint::new(70.0, 80.0),
                        ScatterPoint::new(90.0, 45.0),
                    ]),
                ScatterSeries::new("Series B").color(ChartColors::get(1))
                    .points(vec![
                        ScatterPoint::new(15.0, 60.0), ScatterPoint::new(35.0, 25.0),
                        ScatterPoint::new(55.0, 70.0), ScatterPoint::new(75.0, 35.0),
                        ScatterPoint::new(85.0, 90.0),
                    ]),
            ]))
            // Gauge
            .child(Separator::new())
            .child(div().child("Gauge:"))
            .child(div().h_flex().gap_3().child(Gauge::new("g1").value(25.0).size(px(80.)).label("Low")).child(Gauge::new("g2").value(55.0).size(px(80.)).label("Mid")).child(Gauge::new("g3").value(85.0).size(px(80.)).label("High")))
            // Sparkline
            .child(Separator::new())
            .child(div().child("Sparkline:"))
            .child(Sparkline::new("sl1").data(vec![3.0, 8.0, 5.0, 12.0, 7.0, 15.0, 9.0, 18.0, 11.0, 22.0, 14.0, 20.0]).height(px(40.)).color(ChartColors::get(2)))
            // Histogram
            .child(Separator::new())
            .child(div().child("Histogram:"))
            .child(Histogram::new("h1").height(px(100.)).bins(vec![
                HistogramBin::new("0-10", 5.0).color(ChartColors::get(0)),
                HistogramBin::new("10-20", 12.0).color(ChartColors::get(1)),
                HistogramBin::new("20-30", 18.0).color(ChartColors::get(2)),
                HistogramBin::new("30-40", 25.0).color(ChartColors::get(3)),
                HistogramBin::new("40-50", 15.0).color(ChartColors::get(4)),
                HistogramBin::new("50-60", 8.0).color(ChartColors::get(5)),
            ]))
            // Heatmap
            .child(Separator::new())
            .child(div().child("Heatmap:"))
            .child(Heatmap::new("hm1").cell_size(px(24.)).col_labels(vec!["Mon", "Tue", "Wed", "Thu", "Fri"]).rows(vec![
                vec![
                    HeatmapCell::new(0.2), HeatmapCell::new(0.5), HeatmapCell::new(0.8), HeatmapCell::new(0.3), HeatmapCell::new(0.1),
                ],
                vec![
                    HeatmapCell::new(0.7), HeatmapCell::new(0.9), HeatmapCell::new(0.4), HeatmapCell::new(0.6), HeatmapCell::new(0.2),
                ],
                vec![
                    HeatmapCell::new(0.3), HeatmapCell::new(0.6), HeatmapCell::new(1.0), HeatmapCell::new(0.5), HeatmapCell::new(0.4),
                ],
            ]))
            // CandlestickChart
            .child(Separator::new())
            .child(div().child("CandlestickChart:"))
            .child(CandlestickChart::new("cc1").height(px(100.)).data(vec![
                Candlestick::new("Mon", 25.0, 35.0, 20.0, 30.0),
                Candlestick::new("Tue", 30.0, 45.0, 28.0, 42.0),
                Candlestick::new("Wed", 42.0, 48.0, 38.0, 40.0),
                Candlestick::new("Thu", 40.0, 55.0, 38.0, 50.0),
                Candlestick::new("Fri", 50.0, 60.0, 45.0, 48.0),
            ]))
            // StreamingChart
            .child(Separator::new())
            .child(div().child("StreamingChart:"))
            .child(StreamingChart::new("st1").height(px(60.)).color(ChartColors::get(3)).data(vec![
                10.0, 12.0, 8.0, 15.0, 18.0, 14.0, 20.0, 22.0, 19.0, 25.0,
                28.0, 24.0, 30.0, 27.0, 32.0, 35.0, 30.0, 38.0, 42.0, 40.0,
            ]).show_latest_value(true))
            // Legend
            .child(Separator::new())
            .child(div().child("Legend:"))
            .child(Legend::new("leg1").items(vec![
                LegendItem::new("Series A", ChartColors::get(0)),
                LegendItem::new("Series B", ChartColors::get(1)),
                LegendItem::new("Series C", ChartColors::get(2)),
                LegendItem::new("Series D", ChartColors::get(3)),
            ]).layout(LegendLayout::Horizontal));

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
                    // V3 row
                    .child(v3_card)
                    // V4 row
                    .child(v4_card)
                    // V5 row
                    .child(v5_card)
                    // V6 row
                    .child(v6_card)
                    // V7 row
                    .child(v7_card)
                    // V8 row
                    .child(v8_card)
                    // V9 row
                    .child(v9_card)
                    // V10 row
                    .child(v10_card)
                    // P2 Infrastructure row
                    .child(states_card)
                    // P2 Infrastructure row
                    .child(access_card)
                    // P2 Infrastructure row
                    .child(focus_card)
                    // P2 Infrastructure row
                    .child(overlay_card)
                    // P2 Charts row
                    .child(p2_charts_card)
                    // V2 row
                    .child(icons_card)
                    .child(notification_card)
                    // ── Real-world Example: Settings Center ──
                    .child(
                        div().child(
                            Card::new()
                                .title("Settings Center")
                                .description(
                                    "Real-world example: Sidebar navigation + SettingsPage groups + Switch toggles + Dialog confirmation",
                                )
                                .child(
                                    div()
                                        .h(px(480.))
                                        .child(self.settings_center.clone()),
                                ),
                        ),
                    )
                    // ── Real-world Example: File Manager ──
                    .child(
                        div().child(
                            Card::new()
                                .title("File / Asset Manager")
                                .description(
                                    "Real-world example: Tree folder navigation + DataGrid file listing + ThumbnailStrip preview + ContextMenu + DropZone",
                                )
                                .child(
                                    div()
                                        .h(px(520.))
                                        .child(self.file_manager.clone()),
                                ),
                        ),
                    )
                    // ── Real-world Example: Desktop Editor ──
                    .child(
                        div().child(
                            Card::new()
                                .title("Desktop Editor")
                                .description(
                                    "Real-world example: TitleBar + AppMenuBar + Dock w/ panels + Canvas + PropertyGrid + StatusBar + CommandPalette",
                                )
                                .child(
                                    div()
                                        .h(px(560.))
                                        .child(self.desktop_editor.clone()),
                                ),
                        ),
                    ),
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
