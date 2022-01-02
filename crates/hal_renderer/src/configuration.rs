use crate::vulkan::*;

pub enum RendererConfigType<'config> {
    Vulkan(HalVKRenderConfiguration<'config>),
}

pub struct RendererConfig<'config> {
    pub render_type: RendererConfigType<'config>,
}

pub struct ComputePipelineType {}

pub struct GraphicsPipelineType {}

pub struct RaytracePipelineType {}

pub enum PipelineType {
    Compute(ComputePipelineType),
    Graphics(GraphicsPipelineType),
    Raytrace(RaytracePipelineType),
}

pub struct PipelineConfig {
    pub pipeline_type: PipelineType,
    pub name: &'static str,
}
