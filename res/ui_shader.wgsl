struct VertexIn {
    @location(0) pos: vec3<f32>,
};

struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec3<f32>,
};


@vertex 
fn vert(
    model: VertexIn
) -> VertexOut {
    var out: VertexOut;

    let position = model.pos * 2.0 - vec3(1.0, 1.0, 0.0);
    let color = vec3<f32>(0.0, model.pos.x, 0.0);

    out.clip_position = vec4<f32>(position, 1.0);
    out.uv = model.pos.xy;
    out.color = color;

    return out;
}

@fragment
fn frag(in: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 0.0);
}
