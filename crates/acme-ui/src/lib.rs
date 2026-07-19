//! Acme UI is a compact, clean-room component starter library for GPUI.

mod primitives;
mod styled;
mod theme;

pub mod badge;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod combobox;
pub mod dialog;
pub mod field;
pub mod icons;
pub mod loading_state;
pub mod menu;
pub mod notification;
pub mod pagination;
pub mod popover;
pub mod progress;
pub mod radio;
pub mod resizable;
pub mod select;
pub mod separator;
pub mod sidebar;
pub mod skeleton;
pub mod switch;
pub mod tabs;
pub mod text_input;
pub mod textarea;
pub mod tooltip;
pub mod virtual_list;

pub use primitives::{Size, Tone};
pub use styled::StyledExt;
pub use theme::{ActiveTheme, FontSizes, Spacing, Theme, ThemeColors, ThemeMode, hsl};

pub use badge::Badge;
pub use button::Button;
pub use card::Card;
pub use checkbox::Checkbox;
pub use combobox::{Combobox, ComboboxOption};
pub use dialog::Dialog;
pub use field::FieldShell;
pub use icons::{Icon, IconName};
pub use loading_state::LoadingState;
pub use loading_state::render_loading_state;
pub use menu::{Menu, MenuItem};
pub use notification::{NotificationLevel, Notifications};
pub use pagination::Pagination;
pub use popover::Popover;
pub use progress::Progress;
pub use radio::{Radio, RadioGroup};
pub use resizable::{Direction, Resizable};
pub use select::{Select, SelectOption};
pub use separator::Separator;
pub use sidebar::Sidebar;
pub use skeleton::Skeleton;
pub use switch::Switch;
pub use tabs::Tabs;
pub use text_input::TextInput;
pub use textarea::Textarea;
pub use tooltip::Tooltip;
pub use virtual_list::VirtualList;

use gpui::App;

/// Initializes Acme UI global state.
pub fn init(cx: &mut App) {
    theme::init(cx);
}
