struct Uniforms {  
    mvp: mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> unifroms: Uniforms; 
@group(0) @binding(1) var mySampler: sampler; 
@group(0) @binding(2) var myTexture: texture_2d<f32>; 

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,

};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position =  unifroms.mvp * vec4f(model.position, 1.0); 
    out.uv = vec2<f32>(model.uv); 
    return out;
}

// Fragment Shader Entry Point
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
  let color = textureSample( myTexture,  mySampler, in.uv);

  return color;
}