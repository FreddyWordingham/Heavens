//! Gravitational N-body simulation.

use ocl::{Buffer, ProQue};

const SHADER: &str = include_str!("shader.cl");

/// Gravitational force.
pub struct Gravity {
    /// Strength of gravity.
    pub strength: f32,
    /// Smoothing length.
    pub smoothing_length: f32,
    /// OpenCL program.
    pro_que: ProQue,
    /// GPU position buffer.
    gpu_pos: Buffer<f32>,
    /// GPU force buffer.
    gpu_force: Buffer<f32>,
    /// CPU force buffer.
    cpu_force: Vec<f32>,
}

impl Gravity {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(strength: f32, smoothing_length: f32, num_particles: usize) -> Self {
        debug_assert!(strength > 0.0);
        debug_assert!(smoothing_length > 0.0);

        let src = SHADER
            .replace("NUM_PARTICLES", &num_particles.to_string())
            .replace("GRAV_STRENGTH", &strength.to_string())
            .replace("SMOOTH_LENGTH", &smoothing_length.to_string());
        let pro_que = ProQue::builder()
            .src(src)
            .dims(num_particles)
            .build()
            .unwrap();

        let gpu_pos = pro_que
            .buffer_builder::<f32>()
            .len(num_particles * 3)
            .build()
            .unwrap();
        let gpu_force = pro_que
            .buffer_builder::<f32>()
            .len(num_particles * 3)
            .build()
            .unwrap();
        let cpu_force = vec![0.0; num_particles * 3];

        Self {
            strength,
            smoothing_length,
            pro_que,
            gpu_pos,
            gpu_force,
            cpu_force,
        }
    }

    /// Evolve the simulation in time.
    #[inline]
    pub fn evolve(&mut self, pos: &mut [f32], vel: &mut [f32], dt: f32) {
        debug_assert!(dt.abs() > 1e-9);

        self.gpu_pos.write(&*pos).enq().unwrap();
        let kernel = self
            .pro_que
            .kernel_builder("nbody")
            .arg(&self.gpu_force)
            .arg(&self.gpu_pos)
            .build()
            .unwrap();
        unsafe { kernel.enq().unwrap() };

        self.gpu_force.read(&mut self.cpu_force).enq().unwrap();
        if self.cpu_force.iter().any(|&x| x.is_nan()) {
            panic!("NaN detected in force buffer.");
        }

        for i in 0..pos.len() {
            vel[i] += self.cpu_force[i] * dt;
            pos[i] += vel[i] * dt;
        }
    }
}
