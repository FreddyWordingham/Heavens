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
var read_texture: texture_storage_2d<rgba8unorm, read>;

@group(0)
@binding(2)
var write_texture: texture_storage_2d<rgba8unorm, read_write>;

@compute
@workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;

    var sum: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    var weight_sum: f32 = 0.0;
    let radius: i32 = i32(settings.blur_radius);

    for (var dx: i32 = -radius; dx <= radius; dx = dx + 1) {
        let blur_x: u32 = u32(i32(x) + dx);

        if blur_x < u32(settings.display_height) {
            let weight = exp(-f32(dx * dx) / (2.0 * settings.blur_radius * settings.blur_radius));
            let texel = textureLoad(read_texture, vec2<i32>(i32(blur_x), i32(y)));
            sum += texel * weight;
            weight_sum += weight;
        }
    }

    let final_color: vec4<f32> = sum / weight_sum;
    textureStore(write_texture, vec2<i32>(i32(x), i32(y)), final_color);
}
