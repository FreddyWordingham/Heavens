// Vertex shader
struct VertexInput {
    @location(0) position_and_mass: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) colour: vec3<f32>,
};

@vertex
fn vs_main(particle: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    var position = vec3<f32>(particle.position_and_mass.x, particle.position_and_mass.y, particle.position_and_mass.z);
    var mass = particle.position_and_mass.w;
    out.clip_position = vec4<f32>(position.x, position.y, position.z, 1.0);
    out.colour = vec3<f32>(1.0, 1.0, 1.0);
    return out;
}




// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.colour, 1.0);
}

 

 
