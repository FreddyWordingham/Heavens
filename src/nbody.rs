//! N-Body calculations.

use nalgebra::Vector3;
use ocl::ProQue;

use crate::Particle;

/// Update the massive particle positions and velocities.
pub fn nbody(particles: &mut [Particle], grav_strength: f64, dt: f64) {
    debug_assert!(grav_strength > 0.0);
    debug_assert!(dt > 0.0);

    let num_particles = particles.len();

    let src = format!(
        "
        __kernel void nbody(__global float* buffer_x, __global float* buffer_y, __global float* buffer_z, __global float* positions_x, __global float* positions_y, __global float* positions_z) {{
            int n = get_global_id(0);

            float x = positions_x[n];
            float y = positions_y[n];
            float z = positions_z[n];

            for (int m = 0; m < {num_particles}; ++m) {{
                if (n == m) {{
                    continue;
                }}

                float dx = positions_x[m] - x;
                float dy = positions_y[m] - y;
                float dz = positions_z[m] - z;

                float r = sqrt((dx * dx) + (dy * dy) + (dz * dz));
                float r_inv = 1.0 / (r * r * r);

                buffer_x[n] += r_inv * dx * {grav_strength};
                buffer_y[n] += r_inv * dy * {grav_strength};
                buffer_z[n] += r_inv * dz * {grav_strength};
            }}
        }}
        ",
        num_particles = num_particles,
        grav_strength = grav_strength,
    );

    let pro_que = ProQue::builder()
        .src(src)
        .dims(particles.len())
        .build()
        .unwrap();
    let gpu_buffer_x = pro_que.create_buffer::<f32>().unwrap();
    let gpu_buffer_y = pro_que.create_buffer::<f32>().unwrap();
    let gpu_buffer_z = pro_que.create_buffer::<f32>().unwrap();

    let positions_x = pro_que.create_buffer::<f32>().unwrap();
    let positions_y = pro_que.create_buffer::<f32>().unwrap();
    let positions_z = pro_que.create_buffer::<f32>().unwrap();
    let mut cpu_buffer_x = vec![0.0f32; gpu_buffer_x.len()];
    let mut cpu_buffer_y = vec![0.0f32; gpu_buffer_y.len()];
    let mut cpu_buffer_z = vec![0.0f32; gpu_buffer_z.len()];
    for (n, particle) in particles.iter().enumerate() {
        cpu_buffer_x[n] = particle.pos.x as f32;
        cpu_buffer_y[n] = particle.pos.y as f32;
        cpu_buffer_z[n] = particle.pos.z as f32;
    }
    positions_x.write(&cpu_buffer_x).enq().unwrap();
    positions_y.write(&cpu_buffer_y).enq().unwrap();
    positions_z.write(&cpu_buffer_z).enq().unwrap();

    let kernel = pro_que
        .kernel_builder("nbody")
        .arg(&gpu_buffer_x)
        .arg(&gpu_buffer_y)
        .arg(&gpu_buffer_z)
        .arg(&positions_x)
        .arg(&positions_y)
        .arg(&positions_z)
        .build()
        .unwrap();
    unsafe { kernel.enq().unwrap() };

    gpu_buffer_x.read(&mut cpu_buffer_x).enq().unwrap();
    gpu_buffer_y.read(&mut cpu_buffer_y).enq().unwrap();
    gpu_buffer_z.read(&mut cpu_buffer_z).enq().unwrap();

    for (n, particle) in particles.iter_mut().enumerate() {
        particle.vel += Vector3::new(
            cpu_buffer_x[n] as f64,
            cpu_buffer_y[n] as f64,
            cpu_buffer_z[n] as f64,
        );
        particle.pos += particle.vel * dt;
    }
}
