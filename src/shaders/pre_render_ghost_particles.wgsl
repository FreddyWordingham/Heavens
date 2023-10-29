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

    mvp_xx: f32,
    mvp_xy: f32,
    mvp_xz: f32,
    mvp_xw: f32,
    mvp_yx: f32,
    mvp_yy: f32,
    mvp_yz: f32,
    mvp_yw: f32,
    mvp_zx: f32,
    mvp_zy: f32,
    mvp_zz: f32,
    mvp_zw: f32,
    mvp_wx: f32,
    mvp_wy: f32,
    mvp_wz: f32,
    mvp_ww: f32,
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
    let kind = ghost_positions_and_kinds[n].w;

    let mvp = mat4x4<f32>(vec4<f32>(settings.mvp_xx, settings.mvp_xy, settings.mvp_xz, settings.mvp_xw), vec4<f32>(settings.mvp_yx, settings.mvp_yy, settings.mvp_yz, settings.mvp_yw), vec4<f32>(settings.mvp_zx, settings.mvp_zy, settings.mvp_zz, settings.mvp_zw), vec4<f32>(settings.mvp_wx, settings.mvp_wy, settings.mvp_wz, settings.mvp_ww));
    let projected_pos = mvp * vec4<f32>(position.x, position.y, position.z, 1.0);

    let pixel = position_to_pixel(projected_pos.x, projected_pos.y);
    var prev_colour = textureLoad(texture, pixel);

    let a = 1.0 / settings.ghost_stack_visible_limit;

    var colour = vec3<f32>(1.0, 1.0, 1.0);
    if kind <= 1.0 {
        colour = vec3<f32>(1.0, 0.0, 0.0);
    } else if kind <= 2.0 {
        colour = vec3<f32>(0.0, 1.0, 0.0);
    } else if kind <= 3.0 {
        colour = vec3<f32>(0.0, 0.0, 1.0);
    } else if kind <= 4.0 {
        colour = vec3<f32>(0.0, 1.0, 1.0);
    } else if kind <= 5.0 {
        colour = vec3<f32>(1.0, 0.0, 1.0);
    } else if kind <= 6.0 {
        colour = vec3<f32>(1.0, 1.0, 0.0);
    }

    textureStore(texture, pixel, prev_colour + vec4<f32>(colour, 1.0));
}

fn position_to_pixel(x: f32, y: f32) -> vec2<i32> {
    let col = (x * settings.zoom) + settings.display_width * 0.5;
    let row = (y * settings.zoom) + settings.display_height * 0.5;
    return vec2<i32>(i32(col + settings.camera_x), i32(row + settings.camera_y));
}
