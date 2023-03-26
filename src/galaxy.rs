use nalgebra::Vector3;
use ndarray::Array2;
use rand::Rng;
use std::f64::consts::PI;

use crate::particle::Particle;

/// Collection of matter.
pub struct Galaxy {
    /// Initial radius.
    pub radius: f64,

    /// Massive particles.
    pub stars: Vec<Particle>,
}

impl Galaxy {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(num_stars: usize, radius: f64) -> Self {
        debug_assert!(num_stars > 0);
        debug_assert!(radius > 0.0);

        let mut rng = rand::thread_rng();

        let mut stars = Vec::with_capacity(num_stars);
        for _ in 0..num_stars {
            let theta = rng.gen_range(0.0..2.0 * PI);
            let rho = rng.gen_range(0.0..radius);

            let x = rho * theta.cos();
            let y = rho * theta.sin();
            let z = 0.0;

            stars.push(Particle::new(Vector3::new(x, y, z)));
        }

        Self { radius, stars }
    }

    /// Count the number stars on to a square 2D grid with a given resolution.
    #[inline]
    #[must_use]
    pub fn count(&self, res: usize) -> Array2<u8> {
        debug_assert!(res > 1);

        let delta = 2.0 * self.radius / res as f64;
        let inv_delta = 1.0 / delta;

        let mut grid = Array2::zeros((res, res));
        for star in &self.stars {
            let xi = ((star.pos.x + self.radius) * inv_delta) as usize;
            let yi = ((star.pos.y + self.radius) * inv_delta) as usize;

            if (xi > 0) && (xi < res) && (yi > 0) && (yi < res) {
                grid[[xi, yi]] += 1;
            }
        }

        grid
    }
}
