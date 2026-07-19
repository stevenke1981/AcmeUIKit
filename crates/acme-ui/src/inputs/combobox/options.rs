//! Option type for [`Combobox`](super::Combobox).

use gpui::SharedString;

/// A single option inside a [`Combobox`](super::Combobox) dropdown.
///
/// Each option pairs a human-readable `label` with the `value` that will be
/// placed into the text input when selected.
#[derive(Clone)]
pub struct ComboboxOption {
    /// Display label shown in the dropdown.
    pub label: SharedString,
    /// Value placed into the text input when this option is selected.
    pub value: SharedString,
}

impl ComboboxOption {
    /// Creates a new [`ComboboxOption`] with the given label and value.
    pub fn new(label: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }
}
