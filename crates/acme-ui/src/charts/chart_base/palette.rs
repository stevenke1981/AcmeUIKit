//! Default qualitative color palette for chart series.

use gpui::Hsla;

/// Provides a default qualitative color palette for chart series.
pub struct ChartColors;

impl ChartColors {
    /// Returns the color at `index` from a 10-color qualitative palette.
    pub fn get(index: usize) -> Hsla {
        // ColorBrewer Set1 palette (trimmed to 10)
        let palette: [Hsla; 10] = [
            Hsla {
                h: 0.0,
                s: 0.6,
                l: 0.5,
                a: 1.0,
            }, // red
            Hsla {
                h: 0.6,
                s: 0.6,
                l: 0.5,
                a: 1.0,
            }, // blue
            Hsla {
                h: 0.3,
                s: 0.6,
                l: 0.5,
                a: 1.0,
            }, // green
            Hsla {
                h: 0.07,
                s: 0.7,
                l: 0.55,
                a: 1.0,
            }, // orange
            Hsla {
                h: 0.8,
                s: 0.5,
                l: 0.55,
                a: 1.0,
            }, // purple
            Hsla {
                h: 0.55,
                s: 0.6,
                l: 0.5,
                a: 1.0,
            }, // teal
            Hsla {
                h: 0.15,
                s: 0.6,
                l: 0.5,
                a: 1.0,
            }, // brown
            Hsla {
                h: 0.9,
                s: 0.5,
                l: 0.5,
                a: 1.0,
            }, // pink
            Hsla {
                h: 0.2,
                s: 0.5,
                l: 0.45,
                a: 1.0,
            }, // olive
            Hsla {
                h: 0.5,
                s: 0.4,
                l: 0.5,
                a: 1.0,
            }, // slate
        ];
        palette[index % palette.len()]
    }
}
