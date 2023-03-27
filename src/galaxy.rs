use nalgebra::{Point3, Vector3};
use ndarray::Array2;
use palette::{Gradient, LinSrgb};
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};
use std::f32::consts::PI;

use crate::{nbody, Particle};

/// Collection of matter.
pub struct Galaxy {
    /// Strength of gravity.
    pub grav_strength: f32,

    /// Smoothing length.
    pub smoothing_length: f32,

    /// Initial radius.
    pub radius: f32,

    /// Massive particles.
    pub stars: Vec<Particle>,

    /// Colour map.
    pub cmap: Gradient<LinSrgb>,
}

impl Galaxy {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        rng: &mut impl Rng,
        num_stars: usize,
        radius: f32,
        grav_strength: f32,
        smoothing_length: f32,
        cmap: &[String],
    ) -> Self {
        debug_assert!(num_stars > 0);
        debug_assert!(radius > 0.0);
        debug_assert!(grav_strength > 0.0);
        debug_assert!(smoothing_length > 0.0);
        debug_assert!(cmap.len() > 0);

        let mut stars = Vec::with_capacity(num_stars);
        for _ in 0..num_stars {
            let theta = rng.gen_range(0.0..2.0 * PI);
            let rho = rng.gen_range(0.0..1.0f32).sqrt() * radius;

            let x = rho * theta.cos();
            let y = rho * theta.sin();
            let z = 0.0;
            let pos = Point3::new(x, y, z);

            let mut vel = Vector3::zeros();
            vel.x = 0.43e-8 * -y;
            vel.y = 0.43e-8 * x;

            stars.push(Particle::new(1.0, pos, vel));
        }

        let cols: Vec<&str> = cmap.iter().map(|s| s.as_str()).collect();
        let cs: Vec<_> = cols
            .iter()
            .map(|col| {
                let (r, g, b) = hex_to_rgb(col);
                LinSrgb::new(r, g, b)
            })
            .collect();
        let map: Gradient<LinSrgb> = Gradient::new(cs);

        Self {
            radius,
            stars,
            grav_strength,
            smoothing_length,
            cmap: map,
        }
    }

    /// Evolve the galaxy in time.
    #[inline]
    pub fn evolve(&mut self, rng: &mut ThreadRng, dt: f32) {
        debug_assert!(dt > 0.0);

        self.stars.shuffle(rng);

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

        let s = 1.0;

        let delta = s * 2.0 * self.radius / res as f32;
        let inv_delta = 1.0 / delta;

        let mut grid = Array2::zeros((res, res));
        for star in &self.stars {
            let xi = ((star.pos.x + (s * self.radius)) * inv_delta) as usize;
            let yi = ((star.pos.y + (s * self.radius)) * inv_delta) as usize;

            if (xi > 0) && (xi < res) && (yi > 0) && (yi < res) {
                grid[[xi, yi]] += 1;
            }
        }

        grid
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
