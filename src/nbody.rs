//! N-Body calculations.

use nalgebra::Vector3;
use ocl::ProQue;

use crate::Particle;

/// Update the massive particle positions and velocities.
pub fn nbody(particles: &mut [Particle], grav_strength: f64, smoothing_length: f64, dt: f64) {
    debug_assert!(grav_strength > 0.0);
    debug_assert!(dt > 0.0);

    let num_particles = particles.len();

    let src = format!(
        "
        __kernel void nbody(__global float* buffer, __global float* positions) {{
            int n = get_global_id(0);

            float x = positions[(n * 3) + 0];
            float y = positions[(n * 3) + 1];
            float z = positions[(n * 3) + 2];

            for (int m = 0; m < {num_particles}; ++m) {{
                if (n == m) {{
                    continue;
                }}

                float dx = positions[(m * 3) + 0] - x;
                float dy = positions[(m * 3) + 1] - y;
                float dz = positions[(m * 3) + 2] - z;

                float r = max(sqrt((dx * dx) + (dy * dy) + (dz * dz)), (float){smoothing_length});
                float r_inv = 1.0 / (r * r * r);

                buffer[(n * 3) + 0] += r_inv * dx * {grav_strength};
                buffer[(n * 3) + 1] += r_inv * dy * {grav_strength};
                buffer[(n * 3) + 2] += r_inv * dz * {grav_strength};
            }}
        }}
        ",
        num_particles = num_particles,
        grav_strength = grav_strength as f32,
    );

    let pro_que = ProQue::builder()
        .src(src)
        .dims(particles.len())
        .build()
        .unwrap();
    let gpu_buffer = pro_que
        .buffer_builder::<f32>()
        .len(particles.len() * 3)
        .build()
        .unwrap();

    let positions = pro_que
        .buffer_builder::<f32>()
        .len(particles.len() * 3)
        .build()
        .unwrap();
    let mut cpu_buffer = vec![0.0f32; gpu_buffer.len()];
    for (n, particle) in particles.iter().enumerate() {
        cpu_buffer[(n * 3) + 0] = particle.pos.x as f32;
        cpu_buffer[(n * 3) + 1] = particle.pos.y as f32;
        cpu_buffer[(n * 3) + 2] = particle.pos.z as f32;
    }
    positions.write(&cpu_buffer).enq().unwrap();

    let kernel = pro_que
        .kernel_builder("nbody")
        .arg(&gpu_buffer)
        .arg(&positions)
        .build()
        .unwrap();
    unsafe { kernel.enq().unwrap() };

    gpu_buffer.read(&mut cpu_buffer).enq().unwrap();

    for (n, particle) in particles.iter_mut().enumerate() {
        particle.vel += Vector3::new(
            cpu_buffer[(n * 3) + 0] as f64,
            cpu_buffer[(n * 3) + 1] as f64,
            cpu_buffer[(n * 3) + 2] as f64,
        );
        particle.pos += particle.vel * dt;
    }
}
