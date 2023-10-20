@group(0)
@binding(0)
var<storage, read_write> massive_positions: array<vec4<f32>>;

@compute
@workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x;

    let dt = 0.01;

    massive_positions[n].x = massive_positions[n].x + dt;
    massive_positions[n].y = massive_positions[n].y + dt;
    massive_positions[n].z = massive_positions[n].z + 0.0;
}
