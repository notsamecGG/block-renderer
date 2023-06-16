struct VertexIn {
    @builtin(instance_index) instance_index: u32,
    @location(0) pos: vec3<f32>,
};

struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@vertex 
fn vert(
    model: VertexIn,   
) -> VertexOut {
    var output: VertexOut;

    // Add 1 unit to each instance
    let position = model.pos + vec3<f32>(1.0, 0.0, 0.0) * f32(model.instance_index);
    output.clip_position = camera.view_proj * vec4<f32>(position, 1.0);
    output.color = vec3(f32(model.instance_index) / 10.0, 0.0, 0.0);

    return output;
}


@fragment
fn frag(in: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
