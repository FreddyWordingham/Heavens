//! Simulation input.

use palette::{Gradient, LinSrgb};
use rand::Rng;
use std::path::Path;

use crate::{render, Camera, Parameters};

/// Computed simulation input.
pub struct Input {
    /// Gravitational strength factor.
    grav_strength: f32,
    /// Minimum calculation distance between massive particles.
    smooth_length: f32,
    /// Colour map.
    cmap: Gradient<LinSrgb>,
    /// Positions stored as a flat array of [x[0], y[0], z[0], x[1], y[1], z[1], ..., x[n-1], y[n-1], z[n-1]]
    pos: Vec<f32>,
    /// Cameras.
    cameras: Vec<Camera>,
}

impl Input {
    /// Build an input structure from a parameters object.
    #[inline]
    #[must_use]
    pub fn build(mut rng: impl Rng, params: &Parameters) -> Self {
        let pos = params
            .galaxies
            .iter()
            .map(|galaxy| galaxy.generate(&mut rng, 1000))
            .flatten()
            .map(|v| [v.x, v.y, v.z])
            .flatten()
            .collect::<Vec<_>>();

        Input {
            grav_strength: params.grav_strength,
            smooth_length: params.smooth_length,
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
            cameras: params.cameras.clone(),
        }
    }

    /// Render.
    #[inline]
    #[must_use]
    pub fn render(&self, output_dir: &Path, step_id: usize) {
        self.cameras
            .iter()
            .map(|camera| camera.render(&self.pos))
            .enumerate()
            .map(|(i, img)| {
                let path = output_dir
                    .join(format!("{}", i))
                    .join(format!("{}.png", step_id));
                let img = render::image(&img, 4.0, &self.cmap);
                render::encode(&img)
                    .save(&path)
                    .expect("Failed to save image.");
            })
            .collect::<Vec<_>>();
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
