#import bevy_sprite::{
        // https://github.com/bevyengine/bevy/blob/release-0.13.0/crates/bevy_sprite/src/mesh2d/mesh2d_vertex_output.wgsl
        mesh2d_vertex_output::VertexOutput,
        mesh2d_view_bindings::view,
}

struct CustomMaterial {
        color: vec4<f32>,
};

@group(2) @binding(0) var<uniform> material: CustomMaterial;
@group(2) @binding(1) var           texture: texture_2d<f32>;
@group(2) @binding(2) var   texture_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
        return material.color * textureSample(texture, texture_sampler, mesh.uv);
}