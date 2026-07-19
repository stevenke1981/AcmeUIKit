//! Scale and ScaleType for mapping data to pixel coordinates.

/// Scale type for mapping data to screen coordinates.
#[derive(Clone, Debug, PartialEq)]
pub enum ScaleType {
    /// Linear scale: maps domain → range linearly.
    Linear,
    /// Band scale: categorical data mapped to evenly-spaced bands.
    Band,
}

/// A scale that maps data values to pixel coordinates.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Scale {
    scale_type: ScaleType,
    domain_min: f64,
    domain_max: f64,
    range_min: f64,
    range_max: f64,
    band_count: usize,
}

impl Default for Scale {
    fn default() -> Self {
        Self {
            scale_type: ScaleType::Linear,
            domain_min: 0.0,
            domain_max: 100.0,
            range_min: 0.0,
            range_max: 1.0,
            band_count: 0,
        }
    }
}

impl Scale {
    /// Creates a new linear scale.
    pub fn linear() -> Self {
        Self::default()
    }

    /// Creates a new band scale for categorical data with `count` categories.
    pub fn band(count: usize) -> Self {
        Self {
            scale_type: ScaleType::Band,
            domain_min: 0.0,
            domain_max: count as f64,
            range_min: 0.0,
            range_max: 1.0,
            band_count: count,
        }
    }

    /// Sets the data domain (min…max values).
    pub fn domain(mut self, min: f64, max: f64) -> Self {
        self.domain_min = min;
        self.domain_max = max;
        self
    }

    /// Sets the output pixel range (start…end).
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.range_min = min;
        self.range_max = max;
        self
    }

    /// Maps a data value to a pixel position.
    pub fn map(&self, value: f64) -> f64 {
        let denom = if self.domain_max == self.domain_min {
            1.0
        } else {
            self.domain_max - self.domain_min
        };
        let t = (value - self.domain_min) / denom;
        self.range_min + t * (self.range_max - self.range_min)
    }

    /// Inverts a pixel position back to a data value.
    pub fn invert(&self, pixel: f64) -> f64 {
        let denom = if self.range_max == self.range_min {
            1.0
        } else {
            self.range_max - self.range_min
        };
        let t = (pixel - self.range_min) / denom;
        self.domain_min + t * (self.domain_max - self.domain_min)
    }

    /// Returns the width of one band for band scales.
    pub fn band_width(&self) -> f64 {
        if self.band_count == 0 {
            return 0.0;
        }
        (self.range_max - self.range_min) / self.band_count as f64
    }

    /// Returns evenly-spaced tick values.
    pub fn ticks(&self, count: usize) -> Vec<f64> {
        let count = count.max(2);
        let step = (self.domain_max - self.domain_min) / (count - 1) as f64;
        (0..count)
            .map(|i| self.domain_min + i as f64 * step)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_scale_map() {
        let s = Scale::linear().domain(0.0, 100.0).range(0.0, 500.0);
        assert!((s.map(50.0) - 250.0).abs() < 1e-9);
    }

    #[test]
    fn linear_scale_invert() {
        let s = Scale::linear().domain(0.0, 100.0).range(0.0, 500.0);
        assert!((s.invert(250.0) - 50.0).abs() < 1e-9);
    }

    #[test]
    fn band_scale_width() {
        let s = Scale::band(5).range(0.0, 500.0);
        assert!((s.band_width() - 100.0).abs() < 1e-9);
    }
}
