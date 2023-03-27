use nalgebra::{Point3, Vector3};
use ndarray::Array2;
use rand::Rng;
use std::f64::consts::PI;

use crate::{nbody, Particle};

/// Collection of matter.
pub struct Galaxy {
    /// Strength of gravity.
    pub grav_strength: f64,

    /// Smoothing length.
    pub smoothing_length: f64,

    /// Initial radius.
    pub radius: f64,

    /// Massive particles.
    pub stars: Vec<Particle>,
}

impl Galaxy {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(num_stars: usize, radius: f64, grav_strength: f64, smoothing_length: f64) -> Self {
        debug_assert!(num_stars > 0);
        debug_assert!(radius > 0.0);
        debug_assert!(grav_strength > 0.0);
        debug_assert!(smoothing_length > 0.0);

        let mut rng = rand::thread_rng();

        let mut stars = Vec::with_capacity(num_stars);
        for _ in 0..num_stars {
            let theta = rng.gen_range(0.0..2.0 * PI);
            let rho = rng.gen_range(0.0..radius);

            let x = rho * theta.cos();
            let y = rho * theta.sin();
            let z = 0.0;
            let pos = Point3::new(x, y, z);

            let mut vel = Vector3::zeros();
            vel.x = 1e-9 * -y;
            vel.y = 1e-9 * x;

            stars.push(Particle::new(1.0, pos, vel));
        }

        Self {
            radius,
            stars,
            grav_strength,
            smoothing_length,
        }
    }

    /// Evolve the galaxy in time.
    #[inline]
    pub fn evolve(&mut self, dt: f64) {
        debug_assert!(dt > 0.0);
        nbody::nbody(
            &mut self.stars,
            self.grav_strength,
            self.smoothing_length,
            dt,
        );
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
