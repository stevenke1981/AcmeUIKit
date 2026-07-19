//! Acme UI is a compact, clean-room component starter library for GPUI.

mod primitives;
mod styled;
mod theme;

pub mod alert;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod chart;
pub mod checkbox;
pub mod combobox;
pub mod dialog;
pub mod dock;
pub mod field;
pub mod form;
pub mod icons;
pub mod kbd;
pub mod list;
pub mod loading_state;
pub mod markdown;
pub mod menu;
pub mod notification;
pub mod pagination;
pub mod popover;
pub mod progress;
pub mod radio;
pub mod resizable;
pub mod select;
pub mod separator;
pub mod settings;
pub mod sidebar;
pub mod skeleton;
pub mod slider;
pub mod stepper;
pub mod switch;
pub mod table;
pub mod tabs;
pub mod tag;
pub mod text_input;
pub mod textarea;
pub mod tiles;
pub mod toolbar;
pub mod tooltip;
pub mod tree;
pub mod virtual_list;

pub use primitives::{Size, Tone};
pub use styled::StyledExt;
pub use theme::{ActiveTheme, FontSizes, Spacing, Theme, ThemeColors, ThemeMode, hsl};

pub use alert::Alert;
pub use avatar::Avatar;
pub use badge::Badge;
pub use breadcrumb::Breadcrumb;
pub use button::Button;
pub use card::Card;
pub use chart::{BarChart, BarEntry};
pub use checkbox::Checkbox;
pub use combobox::{Combobox, ComboboxOption};
pub use dialog::Dialog;
pub use dock::{Dock, DockArea, DockPanel};
pub use field::FieldShell;
pub use form::validators;
pub use form::{Field, Form};
pub use icons::{Icon, IconName};
pub use kbd::Kbd;
pub use list::{List, ListItem};
pub use loading_state::LoadingState;
pub use loading_state::render_loading_state;
pub use markdown::Markdown;
pub use menu::{Menu, MenuItem};
pub use notification::{NotificationLevel, Notifications};
pub use pagination::Pagination;
pub use popover::Popover;
pub use progress::Progress;
pub use radio::{Radio, RadioGroup};
pub use resizable::{Direction, Resizable};
pub use select::{Select, SelectOption};
pub use separator::Separator;
pub use settings::{SettingsGroup, SettingsPage, SettingsRow};
pub use sidebar::Sidebar;
pub use skeleton::Skeleton;
pub use slider::Slider;
pub use stepper::Stepper;
pub use switch::Switch;
pub use table::{SortDirection, Table, TableColumn};
pub use tabs::Tabs;
pub use tag::Tag;
pub use text_input::TextInput;
pub use textarea::Textarea;
pub use tiles::{Tile, TileDirection, Tiles};
pub use toolbar::Toolbar;
pub use tooltip::Tooltip;
pub use tree::{Tree, TreeNode};
pub use virtual_list::VirtualList;

use gpui::App;

/// Initializes Acme UI global state.
pub fn init(cx: &mut App) {
    theme::init(cx);
}
