//! Camera setup.

use nalgebra::Point2;
use ndarray::{Array2, Array3};
use serde::Deserialize;

use crate::render;
use palette::{Gradient, LinSrgb};

/// Complete information to construct an rendering object.
#[derive(Clone, Debug, Deserialize)]
pub struct Camera {
    /// Centre [x, y].
    pub centre: Point2<f32>,
    /// Scale.
    pub scale: f32,
    /// Resolution.
    pub res: usize,
    /// Normalisation.
    pub norm: f32,
    /// Colour map.
    pub cmap: Vec<String>,
}

impl Camera {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn render(&self, pos: &[f32]) -> Array3<u8> {
        let map = colour_map(&self.cmap);
        let count = self.count(pos);
        render::image(&count.mapv(|v| v as f32), self.norm, &map)
    }

    /// Count the number of particles in the field of view.
    #[inline]
    #[must_use]
    pub fn count(&self, pos: &[f32]) -> Array2<u32> {
        let num_particles = pos.len() / 3;
        let inv_delta = self.res as f32 / self.scale;

        let mut count = Array2::zeros((self.res, self.res));
        for i in 0..num_particles {
            let x = pos[3 * i];
            let y = pos[(3 * i) + 1];

            // NOTE: This has to be done in floating point arithmetic to avoid particles collecting at zero.
            if x <= self.centre.x - (0.5 * self.scale) || y <= self.centre.y - (0.5 * self.scale) {
                continue;
            }

            let xi = ((x + (0.5 * self.scale) - self.centre.x) * inv_delta) as usize;
            let yi = ((y + (0.5 * self.scale) - self.centre.y) * inv_delta) as usize;

            if xi >= self.res || yi >= self.res {
                continue;
            }

            count[(self.res - yi - 1, xi)] += 1;
        }
        count
    }
}

/// Construct a colour map from a list of hex colour strings.
fn colour_map(cols: &[String]) -> Gradient<LinSrgb> {
    let cs: Vec<_> = cols
        .iter()
        .map(|col| {
            let (r, g, b) = hex_to_rgb(col);
            LinSrgb::new(r, g, b)
        })
        .collect();
    Gradient::new(cs)
}

/// Convert a hex colour string to an RGB tuple.
fn hex_to_rgb(hex: &str) -> (f32, f32, f32) {
    let hex = hex.trim_start_matches('#');

    let hex_val: u32 = (u32::from_str_radix(hex, 16).ok()).unwrap();

    let red = ((hex_val >> 16) & 0xFF) as f32 / 255.0;
    let green = ((hex_val >> 8) & 0xFF) as f32 / 255.0;
    let blue = (hex_val & 0xFF) as f32 / 255.0;

    (red, green, blue)
}
