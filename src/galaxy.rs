//! Particle setup.

use nalgebra::Vector3;
use rand::Rng;
use serde::Deserialize;
use std::f32::consts::PI;

/// Complete information to construct an initial galaxy.
#[derive(Debug, Deserialize)]
pub struct Galaxy {
    /// Number of particles.
    pub num_particles: usize,
    /// Centre.
    pub pos: Vector3<f32>,
    /// Radius.
    pub radius: f32,
}

impl Galaxy {
    /// Generate the mass distribution of the galaxy.
    #[inline]
    #[must_use]
    pub fn generate(&self, mut rng: impl Rng) -> Vec<Vector3<f32>> {
        let mut pos = Vec::with_capacity(self.num_particles);

        for _ in 0..self.num_particles {
            let theta = rng.gen_range(0.0..2.0 * PI);
            let rho = rng.gen_range(0.0..1.0f32).sqrt() * self.radius;

            let x = rho * theta.cos();
            let y = rho * theta.sin();

            let p = Vector3::new(x, y, 0.0);
            pos.push(p + self.pos);
        }

        pos
    }
}
