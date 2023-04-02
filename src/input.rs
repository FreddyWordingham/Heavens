//! Simulation input.

use rand::Rng;
use std::path::Path;

use crate::{render, Camera, Gravity, Parameters};

/// Computed simulation input.
pub struct Input {
    // /// Gravitational strength factor.
    // grav_strength: f32,
    // /// Minimum calculation distance between massive particles.
    // smooth_length: f32,
    /// Positions stored as a flat array of [x[0], y[0], z[0], x[1], y[1], z[1], ..., x[n-1], y[n-1], z[n-1]]
    pos: Vec<f32>,
    /// Velocities stored as a flat array of [Vx[0], Vy[0], Vz[0], Vx[1], Vy[1], Vz[1], ..., Vx[n-1], Vy[n-1], Vz[n-1]]
    vel: Vec<f32>,
    /// Cameras.
    cameras: Vec<Camera>,
    /// Gravitational force.
    gravity: Gravity,
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
        let num_particles = pos.len() / 3;
        let vel = vec![0.0; num_particles * 3];

        Input {
            pos,
            vel,
            cameras: params.cameras.clone(),
            gravity: Gravity::new(params.grav_strength, params.smooth_length, num_particles),
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

    /// Evolve the simulation in time.
    #[inline]
    pub fn evolve(&mut self, dt: f32) {
        debug_assert!(dt.abs() > 1e-9);
        self.gravity.evolve(&mut self.pos, &mut self.vel, dt);
    }
}
