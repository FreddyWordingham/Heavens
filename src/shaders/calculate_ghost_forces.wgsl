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
var<storage, read> ghost_positions_and_kinds: array<vec4<f32>>;

@group(0)
@binding(2)
var<storage, read> massive_positions_and_masses: array<vec4<f32>>;

@group(0)
@binding(3)
var<storage, read_write> ghost_forces_and_kinds: array<vec4<f32>>;

@compute
@workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x;

    let num_massive_bodies = arrayLength(&massive_positions_and_masses);

    let p0x = ghost_positions_and_kinds[n].x;
    let p0y = ghost_positions_and_kinds[n].y;
    let p0z = ghost_positions_and_kinds[n].z;

    var total_force = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    for (var i = 0u; i < num_massive_bodies; i = i + 1u) {
        if i == n {
            continue;
        }

        let p1x = massive_positions_and_masses[i].x;
        let p1y = massive_positions_and_masses[i].y;
        let p1z = massive_positions_and_masses[i].z;
        let m1 = massive_positions_and_masses[i].w;

        let dx = p1x - p0x;
        let dy = p1y - p0y;
        let dz = p1z - p0z;

        let r2 = (dx * dx + dy * dy + dz * dz) + (settings.smoothing_length * settings.smoothing_length);
        let r = sqrt(r2);
        let f = (settings.gravitational_constant * settings.ghost_mass * m1) / r2;

        total_force.x = total_force.x + (f * dx / r);
        total_force.y = total_force.y + (f * dy / r);
        total_force.z = total_force.z + (f * dz / r);
    }

    ghost_forces_and_kinds[n] = total_force;
}
