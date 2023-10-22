@group(0)
@binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;

@compute
@workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;

    let pixel = vec2<u32>(x, y);

    var new_colour = textureLoad(texture, pixel);

    if x == y {
        new_colour = vec4<f32>(1.0, 0.0, 0.0, 1.0);
    }

    // let new_colour = vec4<f32>(1.0 - prev_colour.r, 1.0 - prev_colour.g, 1.0 - prev_colour.b, 1.0);
    // let new_colour = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    textureStore(texture, pixel, new_colour);
}
