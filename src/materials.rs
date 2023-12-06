use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};

#[derive(Asset, TypePath, Default, AsBindGroup, Debug, Clone)]
pub struct ShipMaterial {
    #[uniform(0)]
    pub player_position: Vec2,
}

impl Material for ShipMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ship.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, Default, AsBindGroup, Debug, Clone)]
pub struct SpaceMaterial {
    #[uniform(0)]
    pub player_position: Vec2,
}

impl Material for SpaceMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/space.wgsl".into()
    }
}

#[derive(Asset, TypePath, Default, AsBindGroup, Debug, Clone)]
pub struct GridMaterial {
    #[uniform(0)]
    pub player_position: Vec2,
}

impl Material for GridMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/line.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}
