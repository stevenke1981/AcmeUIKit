//! Acme UI is a compact, clean-room component starter library for GPUI.

mod primitives;
mod styled;
mod theme;

pub mod badge;
pub mod button;
pub mod card;
pub mod field;
pub mod progress;
pub mod separator;
pub mod skeleton;
pub mod switch;
pub mod tabs;

pub use primitives::{Size, Tone};
pub use styled::StyledExt;
pub use theme::{ActiveTheme, Theme, ThemeColors, ThemeMode, hsl};

pub use badge::Badge;
pub use button::Button;
pub use card::Card;
pub use field::FieldShell;
pub use progress::Progress;
pub use separator::Separator;
pub use skeleton::Skeleton;
pub use switch::Switch;
pub use tabs::Tabs;

use gpui::App;

/// Initializes Acme UI global state.
pub fn init(cx: &mut App) {
    theme::init(cx);
}
