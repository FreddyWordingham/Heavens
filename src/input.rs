//! Simulation input.

use rand::Rng;
use std::path::Path;

use crate::{render, Camera, Parameters};

/// Computed simulation input.
pub struct Input {
    /// Gravitational strength factor.
    _grav_strength: f32,
    /// Minimum calculation distance between massive particles.
    _smooth_length: f32,
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
            .map(|galaxy| galaxy.generate(&mut rng))
            .flatten()
            .map(|v| [v.x, v.y, v.z])
            .flatten()
            .collect::<Vec<_>>();

        Input {
            _grav_strength: params.grav_strength,
            _smooth_length: params.smooth_length,
            pos,
            cameras: params.cameras.clone(),
        }
    }

    /// Render.
    #[inline]
    pub fn render(&self, output_dir: &Path, step_id: usize) {
        self.cameras
            .iter()
            .map(|camera| camera.render(&self.pos))
            .enumerate()
            .for_each(|(i, img)| {
                let path = output_dir
                    .join(format!("camera_{}", i))
                    .join(format!("{}.png", step_id));
                render::encode(&img)
                    .save(&path)
                    .expect("Failed to save image.");
            })
    }
}
