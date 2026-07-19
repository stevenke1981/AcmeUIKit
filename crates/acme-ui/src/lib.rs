//! Acme UI is a compact, clean-room component starter library for GPUI.

mod primitives;
mod styled;
mod theme;

pub mod badge;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod dialog;
pub mod field;
pub mod icons;
pub mod menu;
pub mod notification;
pub mod popover;
pub mod progress;
pub mod radio;
pub mod separator;
pub mod skeleton;
pub mod switch;
pub mod tabs;
pub mod text_input;
pub mod tooltip;

pub use primitives::{Size, Tone};
pub use styled::StyledExt;
pub use theme::{ActiveTheme, Theme, ThemeColors, ThemeMode, hsl};

pub use badge::Badge;
pub use button::Button;
pub use card::Card;
pub use checkbox::Checkbox;
pub use dialog::Dialog;
pub use field::FieldShell;
pub use icons::{Icon, IconName};
pub use menu::{Menu, MenuItem};
pub use notification::{NotificationLevel, Notifications};
pub use popover::Popover;
pub use progress::Progress;
pub use radio::{Radio, RadioGroup};
pub use separator::Separator;
pub use skeleton::Skeleton;
pub use switch::Switch;
pub use tabs::Tabs;
pub use text_input::TextInput;
pub use tooltip::Tooltip;

use gpui::App;

/// Initializes Acme UI global state.
pub fn init(cx: &mut App) {
    theme::init(cx);
}
