struct Settings {
    display_width: f32,
    display_height: f32,
    pixel_size: f32,
    zoom: f32,

    gravitational_constant: f32,
    time_step: f32,
    smoothing_length: f32,

    ghost_mass: f32,
    ghost_stack_visible_limit: f32,

    blur_radius: f32,
};

@group(0)
@binding(0)
var<uniform> settings: Settings;

@group(0)
@binding(1)
var<storage, read> massive_velocities_and_masses: array<vec4<f32>>;

@group(0)
@binding(2)
var<storage, read_write> massive_positions_and_masses: array<vec4<f32>>;

@compute
@workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x;

    let vx = massive_velocities_and_masses[n].x;
    let vy = massive_velocities_and_masses[n].y;
    let vz = massive_velocities_and_masses[n].z;

    massive_positions_and_masses[n].x += vx * settings.time_step;
    massive_positions_and_masses[n].y += vy * settings.time_step;
    massive_positions_and_masses[n].z += vz * settings.time_step;
}
