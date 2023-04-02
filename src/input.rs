//! Simulation input.

use rand::Rng;
use std::path::Path;

use crate::{render, Camera, Gravity, Parameters};

/// Computed simulation input.
pub struct Input {
    /// Positions stored as a flat array of [x[0], y[0], z[0], x[1], y[1], z[1], ..., x[n-1], y[n-1], z[n-1]]
    pos: Vec<f32>,
    /// Velocities stored as a flat array of [Vx[0], Vy[0], Vz[0], Vx[1], Vy[1], Vz[1], ..., Vx[n-1], Vy[n-1], Vz[n-1]]
    vel: Vec<f32>,
    /// Cameras.
    cameras: Vec<Camera>,
    /// Gravitational force.
    gravity: Gravity,
    /// Sub-steps
    sub_steps: usize,
}

impl Input {
    /// Build an input structure from a parameters object.
    #[inline]
    #[must_use]
    pub fn build(mut rng: impl Rng, params: &Parameters) -> Self {
        let mut pos = Vec::new();
        let mut vel = Vec::new();
        for galaxy in &params.galaxies {
            let (p, v) = galaxy.generate(&mut rng);
            pos.extend(p);
            vel.extend(v);
        }

        let pos = pos
            .iter()
            .map(|p| [p.x, p.y, p.z])
            .flatten()
            .collect::<Vec<_>>();
        let vel = vel
            .iter()
            .map(|v| [v.x, v.y, v.z])
            .flatten()
            .collect::<Vec<_>>();
        let num_particles = pos.len() / 3;

        Input {
            pos,
            vel,
            cameras: params.cameras.clone(),
            gravity: Gravity::new(params.grav_strength, params.smooth_length, num_particles),
            sub_steps: params.sub_steps.max(1),
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
                    .join(format!("camera_{:03}", i))
                    .join(format!("{:06}.png", step_id));
                render::encode(&img)
                    .save(&path)
                    .expect("Failed to save image.");
            })
    }

    /// Evolve the simulation in time.
    #[inline]
    pub fn evolve(&mut self, dt: f32) {
        let f = dt / self.sub_steps as f32;
        for _ in 0..self.sub_steps {
            self.gravity.evolve(&mut self.pos, &mut self.vel, f);
        }
    }
}
