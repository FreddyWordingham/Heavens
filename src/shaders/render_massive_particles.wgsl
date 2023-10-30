struct Settings {
    display_width: f32,
    display_height: f32,
    pixel_size: f32,

    gravitational_constant: f32,
    time_step: f32,
    smoothing_length: f32,

    ghost_mass: f32,
    ghost_stack_visible_limit: f32,

    blur_radius: f32,
};

struct Camera {
    mvp: mat4x4<f32>,
    zoom: f32,
};

@group(0)
@binding(0)
var<uniform> settings: Settings;

@group(0)
@binding(1)
var<uniform> camera: Camera;

@group(0)
@binding(2)
var<storage, read> massive_positions_and_masses: array<vec4<f32>>;

@group(0)
@binding(3)
var texture: texture_storage_2d<rgba8unorm, read_write>;

@compute
@workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x;

    let position = massive_positions_and_masses[n].xyz;
    let mass = massive_positions_and_masses[n].w;

    let clip_space_pos = camera.mvp * vec4<f32>(position.x / camera.zoom, position.y / camera.zoom, position.z / camera.zoom, 1.0);

    let pixel = position_to_pixel(clip_space_pos.xyz);
    var colour = vec4<f32>(1.0, 1.0, 1.0, 1.0);

    textureStore(texture, pixel, colour);
}

fn position_to_pixel(clip_space_pos: vec3<f32>) -> vec2<i32> {
    let col = (clip_space_pos.x + 1.0) * 0.5 * settings.display_width;
    let row = (clip_space_pos.y + 1.0) * 0.5 * settings.display_height;
    return vec2<i32>(i32(col), i32(row));
}
