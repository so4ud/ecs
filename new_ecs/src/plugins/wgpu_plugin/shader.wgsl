struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,

};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    if model.position.x > 0.1 {
        out.color = vec4<f32>(1.0, 0.0, 0.0, 0.0);
    }
    if model.position.x < -0.1 {
        out.color = vec4<f32>(0.0, 1.0, 0.0, 1.0);
    }
    if model.position.y > 0.1 {
        out.color = vec4<f32>(0.0, 0.0, 1.0, 1.0);
    }
    return out;
}

// Fragment Shader Entry Point
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
  // Return solid red color (RGBA)
  return in.color;
}