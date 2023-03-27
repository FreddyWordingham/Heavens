//! N-Body calculations.

use nalgebra::Vector3;
use ocl::ProQue;

use crate::Particle;

/// Calculate the force between all particles.
pub fn nbody(particles: &mut [Particle], dt: f64) {
    let src = format!(
        "
        __kernel void nbody(__global float* buffer, float dt) {{
            int n = get_global_id(0);
            int i = n * 3;
            buffer[i + 0] = 1.0 * dt;
            buffer[i + 1] = 1.0 * dt;
            buffer[i + 2] = 1.0 * dt;
        }}
        "
    );

    let pro_que = ProQue::builder()
        .src(src)
        .dims(particles.len())
        .build()
        .unwrap();
    let gpu_buffer = pro_que.create_buffer::<f32>().unwrap();
    let kernel = pro_que
        .kernel_builder("mandelbrot")
        .arg(&gpu_buffer)
        .arg(dt as f32)
        .build()
        .unwrap();
    unsafe { kernel.enq().unwrap() };

    let mut cpu_buffer = vec![0.0f32; gpu_buffer.len()];
    gpu_buffer.read(&mut cpu_buffer).enq().unwrap();

    for (n, particle) in particles.iter_mut().enumerate() {
        let i = n * 3;
        particle.vel += Vector3::new(
            cpu_buffer[i] as f64,
            cpu_buffer[i + 1] as f64,
            cpu_buffer[i + 2] as f64,
        );
        particle.pos += particle.vel * dt;
    }
}
