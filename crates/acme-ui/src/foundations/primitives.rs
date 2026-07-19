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
            Self::Small => px(30.),
            Self::Medium => px(36.),
            Self::Large => px(42.),
        }
    }

    pub fn horizontal_padding(self) -> Pixels {
        match self {
            Self::ExtraSmall => px(8.),
            Self::Small => px(10.),
            Self::Medium => px(14.),
            Self::Large => px(18.),
        }
    }

    pub fn text_size(self) -> Pixels {
        match self {
            Self::ExtraSmall => px(11.),
            Self::Small => px(12.),
            Self::Medium => px(14.),
            Self::Large => px(15.),
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
