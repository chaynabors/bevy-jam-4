#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput
}

struct Material {
    player_position: vec2<f32>,
};

@group(1) @binding(0) var<uniform> material: Material;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return vec4(mesh.color.xyz, 0.66);
}
