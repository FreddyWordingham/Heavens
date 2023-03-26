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

    /// Raster the stars on to a 2D grid.
    #[inline]
    #[must_use]
    pub fn raster(&self, res: usize) -> Array2<u8> {
        debug_assert!(res > 0);

        let mut grid = Array2::zeros((res, res));

        for _star in &self.stars {
            let x = 0;
            let y = 0;

            if (x > 0) && (x < res) && (y > 0) && (y < res) {
                grid[[x, y]] = 1;
            }
        }

        grid
    }
}
