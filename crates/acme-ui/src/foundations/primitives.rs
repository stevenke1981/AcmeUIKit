use gpui::{Pixels, px};

/// Shared component size scale.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Size {
    ExtraSmall,
    Small,
    #[default]
    Medium,
    Large,
}

impl Size {
    pub fn height(self) -> Pixels {
        match self {
            Self::ExtraSmall => px(24.),
            Self::Small => px(28.),
            Self::Medium => px(32.),
            Self::Large => px(38.),
        }
    }

    pub fn horizontal_padding(self) -> Pixels {
        match self {
            Self::ExtraSmall => px(8.),
            Self::Small => px(10.),
            Self::Medium => px(12.),
            Self::Large => px(16.),
        }
    }

    pub fn text_size(self) -> Pixels {
        match self {
            Self::ExtraSmall => px(11.),
            Self::Small => px(12.),
            Self::Medium => px(13.),
            Self::Large => px(14.),
        }
    }
}

/// Semantic component tone.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Tone {
    #[default]
    Neutral,
    Primary,
    Success,
    Warning,
    Danger,
}
