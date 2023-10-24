struct Settings {
    display_width: f32,
    display_height: f32,
    pixel_size: f32,

    zoom: f32,
    camera_x: f32,
    camera_y: f32,

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
var texture: texture_storage_2d<rgba8unorm, read_write>;

@compute
@workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x;

    let position = ghost_positions_and_kinds[n].xyz;
    let mass = ghost_positions_and_kinds[n].w;

    let pixel = position_to_pixel(position.x, position.y);
    var colour = textureLoad(texture, pixel);

    let a = 1.0 / settings.ghost_stack_visible_limit;
    textureStore(texture, pixel, colour + vec4<f32>(a, a, a, 1.0));
}

fn position_to_pixel(x: f32, y: f32) -> vec2<i32> {
    let col = (x * settings.zoom) + settings.display_width * 0.5;
    let row = (y * settings.zoom) + settings.display_height * 0.5;
    return vec2<i32>(i32(col + settings.camera_x), i32(row + settings.camera_y));
}
