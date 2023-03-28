//! Galaxy input parameters.

use nalgebra::Vector3;
use rand::Rng;
use serde::Deserialize;
use std::f32::consts::PI;

/// Complete information to construct an initial galaxy.
#[derive(Debug, Deserialize)]
pub struct Galaxy {
    /// Centre.
    pub pos: Vector3<f32>,
    /// Radius.
    pub radius: f32,
}

impl Galaxy {
    /// Generate the mass distribution of the galaxy.
    #[inline]
    #[must_use]
    pub fn generate(&self, mut rng: impl Rng, n: usize) -> Vec<Vector3<f32>> {
        let mut pos = Vec::with_capacity(n);

        for _ in 0..n {
            let theta = rng.gen_range(0.0..2.0 * PI);
            let rho = rng.gen_range(0.0..1.0) * self.radius;

            let x = rho * theta.cos();
            let y = rho * theta.sin();

            let p = Vector3::new(x, y, 0.0);
            pos.push(p + self.pos);
        }

        pos
    }
}
