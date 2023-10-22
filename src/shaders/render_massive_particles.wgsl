struct Settings {
    display_width: f32,
    display_height: f32,
    pixel_size: f32,
    zoom: f32,

    gravitational_constant: f32,
    time_step: f32,
    smoothing_length: f32,
};


@group(0)
@binding(0)
var<uniform> settings: Settings;

@group(0)
@binding(1)
var<storage, read> massive_positions_and_masses: array<vec4<f32>>;

@group(0)
@binding(2)
var texture: texture_storage_2d<rgba8unorm, read_write>;

@compute
@workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x;

    let position = massive_positions_and_masses[n].xyz;
    let mass = massive_positions_and_masses[n].w;

    let pixel = position_to_pixel(position.x, position.y);
    var colour = vec4<f32>(1.0, 1.0, 1.0, 1.0);

    textureStore(texture, pixel, colour);
}

fn position_to_pixel(x: f32, y: f32) -> vec2<u32> {
    let col = (x * settings.zoom) + settings.display_width * 0.5;
    let row = (y * settings.zoom) + settings.display_height * 0.5;
    return vec2<u32>(u32(col), u32(row));
}
