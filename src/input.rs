use nalgebra::Vector3;
use palette::{Gradient, LinSrgb};
use rand::Rng;

use crate::Parameters;

/// Computed simulation input.
pub struct Input {
    /// Gravitational strength factor.
    gravitational_strength: f32,
    /// Minimum calculation distance between massive particles.
    smoothing_length: f32,
    /// Colour map.
    cmap: Gradient<LinSrgb>,
    /// Positions.
    pos: Vec<Vector3<f32>>,
}

impl Input {
    /// Build an input structure from a parameters object.
    pub fn build(mut rng: impl Rng, params: &Parameters) -> Self {
        let pos = params
            .galaxies
            .iter()
            .map(|galaxy| galaxy.generate(&mut rng, 1000))
            .flatten()
            .collect::<Vec<_>>();

        Input {
            gravitational_strength: params.gravitational_strength,
            smoothing_length: params.smoothing_length,
            cmap: Gradient::new(
                params
                    .cmap
                    .iter()
                    .map(|col| {
                        let (r, g, b) = hex_to_rgb(col);
                        LinSrgb::new(r, g, b)
                    })
                    .collect::<Vec<_>>(),
            ),
            pos,
        }
    }
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
