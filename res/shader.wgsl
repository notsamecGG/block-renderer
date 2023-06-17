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


const back_face_rotation = mat3x3<f32>(
    vec3<f32>(-1.0, 0.0,  0.0),
    vec3<f32>( 0.0, 1.0,  0.0),
    vec3<f32>( 0.0, 0.0, -1.0),
);

const left_face_rotation = mat3x3<f32>(
    vec3<f32>(0.0, 0.0, -1.0),
    vec3<f32>(0.0, 1.0,  0.0),
    vec3<f32>(1.0, 0.0,  0.0),
);

const right_face_rotation = mat3x3<f32>(
    vec3<f32>( 0.0, 0.0, 1.0),
    vec3<f32>( 0.0, 1.0, 0.0),
    vec3<f32>(-1.0, 0.0, 0.0),
);

const top_face_rotation = mat3x3<f32>(
    vec3<f32>(1.0,  0.0,  0.0),
    vec3<f32>(0.0,  0.0, -1.0),
    vec3<f32>(0.0,  1.0,  0.0),
);

const bottom_face_rotation = mat3x3<f32>(
    vec3<f32>(1.0,  0.0, 0.0),
    vec3<f32>(0.0,  0.0, 1.0),
    vec3<f32>(0.0, -1.0, 0.0),
);


@vertex 
fn vert(
    model: VertexIn,   
) -> VertexOut {
    var output: VertexOut;

    var position = model.pos - vec3<f32>(0.5, 0.5, -0.5);

    let face_index = model.instance_index % 6u;
    let cube_index = model.instance_index / 6u;

    let x = f32(cube_index % 10u);
    let z = f32(cube_index / 10u) % 10.0;
    let y = f32(cube_index / 100u);

    if (face_index == 1u) {
        position = back_face_rotation * position;  
    } else if (face_index == 2u) {
        position = left_face_rotation * position;  
    } else if (face_index == 3u) {
        position = right_face_rotation * position;  
    } else if (face_index == 4u) {
        position = top_face_rotation * position;  
    } else if (face_index == 5u) {
        position = bottom_face_rotation * position;  
    }

    let spacing = 1.2;
    position += vec3<f32>(1.0 * spacing * x, 1.0 * spacing * y, 1.0 * spacing * z);

    output.clip_position = camera.view_proj * vec4<f32>(position, 1.0);
    output.color = vec3(f32(face_index) / 10.0, 0.0, 0.0);

    return output;
}


@fragment
fn frag(in: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
