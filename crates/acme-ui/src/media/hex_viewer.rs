use crate::{ActiveTheme, StyledExt};
use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

/// A hex dump viewer that displays data as address | hex bytes | ASCII.
///
/// # Example
///
/// ```ignore
/// HexViewer::new("hex")
///     .data(vec![0x48, 0x65, 0x6C, 0x6C, 0x6F])
///     .address(0x1000);
/// ```
#[derive(IntoElement)]
pub struct HexViewer {
    id: ElementId,
    data: Vec<u8>,
    address: usize,
}

impl HexViewer {
    /// Create a new [`HexViewer`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            data: Vec::new(),
            address: 0,
        }
    }

    /// Set the byte data to display.
    pub fn data(mut self, bytes: Vec<u8>) -> Self {
        self.data = bytes;
        self
    }

    /// Set the starting memory address.
    pub fn address(mut self, addr: usize) -> Self {
        self.address = addr;
        self
    }
}

impl RenderOnce for HexViewer {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let bytes_per_row: usize = 16;

        // Build rows of 16 bytes each
        let rows: Vec<Vec<u8>> = self
            .data
            .chunks(bytes_per_row)
            .map(|chunk| chunk.to_vec())
            .collect();

        div()
            .id(self.id)
            .v_flex()
            .children(rows.into_iter().enumerate().map(|(row_idx, row)| {
                let addr = self.address + row_idx * bytes_per_row;

                // Address hex
                let addr_str = format!("{:08X}", addr);

                // Hex bytes (grouped by 4)
                let hex_bytes: String = row
                    .chunks(4)
                    .map(|group| {
                        group
                            .iter()
                            .map(|b| format!("{:02X}", b))
                            .collect::<Vec<_>>()
                            .join(" ")
                    })
                    .collect::<Vec<_>>()
                    .join("  ");

                // Pad hex section to full width
                let expected_hex_len = bytes_per_row * 3 - 1; // 16 bytes * "XX " = 47 chars
                let hex_padded = if hex_bytes.len() < expected_hex_len {
                    let pad = expected_hex_len - hex_bytes.len();
                    format!("{}{}", hex_bytes, " ".repeat(pad))
                } else {
                    hex_bytes
                };

                // ASCII representation
                let ascii_str: String = row
                    .iter()
                    .map(|b| {
                        if b.is_ascii_graphic() || *b == b' ' {
                            *b as char
                        } else {
                            '.'
                        }
                    })
                    .collect();

                div()
                    .h_flex()
                    .gap_2()
                    .px(px(4.))
                    .child(
                        // Address column
                        div()
                            .text_color(c.muted_foreground)
                            .child(SharedString::from(addr_str)),
                    )
                    .child(
                        // Separator
                        div().text_color(c.muted_foreground).child("│"),
                    )
                    .child(
                        // Hex bytes
                        div()
                            .text_color(c.foreground)
                            .child(SharedString::from(hex_padded)),
                    )
                    .child(
                        // Separator
                        div().text_color(c.muted_foreground).child("│"),
                    )
                    .child(
                        // ASCII
                        div()
                            .text_color(c.muted_foreground)
                            .child(SharedString::from(ascii_str)),
                    )
            }))
    }
}
