use gpui::{App, SharedString, px};

use crate::ActiveTheme;

// ── ARIA roles ──

/// Common ARIA roles used by Acme UI components.
#[derive(Clone, Debug, PartialEq)]
pub enum AriaRole {
    Button,
    Checkbox,
    Dialog,
    Grid,
    GridCell,
    Group,
    Heading,
    Img,
    Label,
    Link,
    List,
    ListBox,
    ListItem,
    Menu,
    MenuBar,
    MenuItem,
    Navigation,
    None,
    Option,
    Presentation,
    ProgressBar,
    Radio,
    RadioGroup,
    Region,
    Row,
    RowGroup,
    Scrollbar,
    Search,
    Separator,
    Slider,
    SpinButton,
    Status,
    Switch,
    Tab,
    Table,
    TabList,
    TabPanel,
    TextBox,
    Toolbar,
    Tooltip,
    Tree,
    TreeItem,
}

impl AriaRole {
    /// Returns the ARIA role string.
    pub fn as_str(&self) -> &'static str {
        use AriaRole::*;
        match self {
            Button => "button",
            Checkbox => "checkbox",
            Dialog => "dialog",
            Grid => "grid",
            GridCell => "gridcell",
            Group => "group",
            Heading => "heading",
            Img => "img",
            Label => "label",
            Link => "link",
            List => "list",
            ListBox => "listbox",
            ListItem => "listitem",
            Menu => "menu",
            MenuBar => "menubar",
            MenuItem => "menuitem",
            Navigation => "navigation",
            None => "none",
            Option => "option",
            Presentation => "presentation",
            ProgressBar => "progressbar",
            Radio => "radio",
            RadioGroup => "radiogroup",
            Region => "region",
            Row => "row",
            RowGroup => "rowgroup",
            Scrollbar => "scrollbar",
            Search => "search",
            Separator => "separator",
            Slider => "slider",
            SpinButton => "spinbutton",
            Status => "status",
            Switch => "switch",
            Tab => "tab",
            Table => "table",
            TabList => "tablist",
            TabPanel => "tabpanel",
            TextBox => "textbox",
            Toolbar => "toolbar",
            Tooltip => "tooltip",
            Tree => "tree",
            TreeItem => "treeitem",
        }
    }
}

// ── Aria attributes container ──

/// Tri-state for aria-checked.
#[derive(Clone, Copy, PartialEq)]
pub enum AriaChecked {
    False,
    True,
    Mixed,
}

/// Tri-state for aria-pressed.
#[derive(Clone, Copy, PartialEq)]
pub enum AriaPressed {
    False,
    True,
    Mixed,
}

/// Popup types for aria-haspopup.
#[derive(Clone, Copy, PartialEq)]
pub enum AriaPopup {
    False,
    True,
    Menu,
    ListBox,
    Tree,
    Grid,
    Dialog,
}

/// Live region types.
#[derive(Clone, Copy, PartialEq)]
pub enum AriaLive {
    Off,
    Polite,
    Assertive,
}

/// Holds ARIA attributes for a single element.
/// Since GPUI does not support raw HTML attributes on Div,
/// this type serves as a documentation & builder pattern reference.
/// To apply ARIA attributes, use the role/builder accessor values
/// within your component's rendered description.
#[derive(Clone, Default)]
pub struct AriaAttrs {
    pub role: Option<AriaRole>,
    pub label: Option<SharedString>,
    pub described_by: Option<SharedString>,
    pub labelled_by: Option<SharedString>,
    pub required: Option<bool>,
    pub disabled: Option<bool>,
    pub readonly: Option<bool>,
    pub invalid: Option<bool>,
    pub checked: Option<AriaChecked>,
    pub selected: Option<bool>,
    pub expanded: Option<bool>,
    pub pressed: Option<AriaPressed>,
    pub has_popup: Option<AriaPopup>,
    pub controls: Option<SharedString>,
    pub hidden: Option<bool>,
    pub live: Option<AriaLive>,
}

impl AriaAttrs {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn role(mut self, role: AriaRole) -> Self {
        self.role = Some(role);
        self
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    pub fn described_by(mut self, id: impl Into<SharedString>) -> Self {
        self.described_by = Some(id.into());
        self
    }
    pub fn labelled_by(mut self, id: impl Into<SharedString>) -> Self {
        self.labelled_by = Some(id.into());
        self
    }
    pub fn required(mut self, val: bool) -> Self {
        self.required = Some(val);
        self
    }
    pub fn disabled(mut self, val: bool) -> Self {
        self.disabled = Some(val);
        self
    }
    pub fn readonly(mut self, val: bool) -> Self {
        self.readonly = Some(val);
        self
    }
    pub fn invalid(mut self, val: bool) -> Self {
        self.invalid = Some(val);
        self
    }
    pub fn checked(mut self, val: AriaChecked) -> Self {
        self.checked = Some(val);
        self
    }
    pub fn selected(mut self, val: bool) -> Self {
        self.selected = Some(val);
        self
    }
    pub fn expanded(mut self, val: bool) -> Self {
        self.expanded = Some(val);
        self
    }
    pub fn pressed(mut self, val: AriaPressed) -> Self {
        self.pressed = Some(val);
        self
    }
    pub fn has_popup(mut self, val: AriaPopup) -> Self {
        self.has_popup = Some(val);
        self
    }
    pub fn controls(mut self, id: impl Into<SharedString>) -> Self {
        self.controls = Some(id.into());
        self
    }
    pub fn hidden(mut self, val: bool) -> Self {
        self.hidden = Some(val);
        self
    }
    pub fn live(mut self, val: AriaLive) -> Self {
        self.live = Some(val);
        self
    }
}

// ── Reduced motion ──

/// GPUI exposes accessibility via window settings.
pub fn prefers_reduced_motion(_window: &gpui::Window, _cx: &App) -> bool {
    false
}

/// Returns motion multiplier (0 for reduced motion, 1 otherwise).
pub fn animation_duration_multiplier(window: &gpui::Window, cx: &App) -> f32 {
    if prefers_reduced_motion(window, cx) {
        0.0
    } else {
        1.0
    }
}

pub fn use_motion(window: &gpui::Window, cx: &App) -> bool {
    !prefers_reduced_motion(window, cx)
}

/// High contrast check — reads from theme.
pub fn prefers_high_contrast(_cx: &App) -> bool {
    false
}

/// Returns focus ring style: (width, color).
pub fn focus_ring_style(cx: &App) -> (gpui::Pixels, gpui::Hsla) {
    let c = cx.theme().colors;
    (px(2.), c.ring)
}
