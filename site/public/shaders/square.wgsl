// Vertex shader

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] position: vec2<f32>;
};

struct GlobalUniforms {
    time: f32;
    screen_width: f32;
    screen_height: f32;
    dpi: f32;
    mouse_x: f32;
    mouse_y: f32;
};

[[group(0), binding(0)]]
var<uniform> global_uniforms: GlobalUniforms;

[[stage(vertex)]]
fn vs_main(
    [[builtin(vertex_index)]] in_vertex_index: u32,
    [[builtin(instance_index)]] in_instance_index: u32,
) -> VertexOutput {
    var out: VertexOutput;

    var positions = array<vec2<f32>, 4>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(1.0, 1.0),
    );

    out.position = positions[in_vertex_index];
    out.clip_position = vec4<f32>(out.position * .5, 0.0, 1.0);

    return out;
}

fn smooth_step(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    return t * t * (3.0 - 2.0 * t);
}

// Fragment shader
[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let color = vec3<f32>(189.0, 129.0, 236.0) / 255.0;

    var out = vec4<f32>(color, 1.0);
    return out;
}
