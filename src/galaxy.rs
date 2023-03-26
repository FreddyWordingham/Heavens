use nalgebra::Vector3;
use rand::Rng;
use std::f64::consts::PI;

use crate::particle::Particle;

/// Collection of matter.
pub struct Galaxy {
    /// Massive particles.
    pub particles: Vec<Particle>,
}

impl Galaxy {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(n: usize, radius: f64) -> Self {
        let mut rng = rand::thread_rng();

        let mut particles = Vec::with_capacity(n);
        for _ in 0..n {
            let theta = rng.gen_range(0.0..2.0 * PI);
            let rho = rng.gen_range(0.0..radius);

            let x = rho * theta.cos();
            let y = rho * theta.sin();
            let z = 0.0;

            particles.push(Particle::new(Vector3::new(x, y, z)));
        }

        Self { particles }
    }
}
