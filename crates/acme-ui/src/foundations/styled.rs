use gpui::Styled;

/// Small layout extensions used by Acme components.
pub trait StyledExt: Styled + Sized {
    /// Applies horizontal flex layout with vertically centered children.
    fn h_flex(self) -> Self {
        self.flex().flex_row().items_center()
    }

    /// Applies vertical flex layout.
    fn v_flex(self) -> Self {
        self.flex().flex_col()
    }
}

impl<T: Styled> StyledExt for T {}
