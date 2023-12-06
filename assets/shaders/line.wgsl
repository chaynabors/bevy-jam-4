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
    return vec4(
        1.0, 1.0, 1.0,
        1.0 - pow(length((mesh.world_position.xyz - vec3(material.player_position.x, 0.0, material.player_position.y)) * 0.15), 0.05)
    );
}
