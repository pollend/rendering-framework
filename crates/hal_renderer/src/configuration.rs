use crate::vulkan::*;

pub enum RendererConfigType {
    Vulkan(VulkanRenderConfiguration),
}

pub struct RendererConfig {
    pub render_type: RendererConfigType,
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
