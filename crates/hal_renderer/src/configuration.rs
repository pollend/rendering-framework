use crate::{
    renderer::{Sampler, Shader},
    vulkan::*,
};
use std::ffi::CString;

enum FilterType {
    FilterNearest = 0,
    FilterLinear,
}

pub struct VulkanRenderConfiguration {
    pub(crate) instance_layers: Vec<CString>,
    pub(crate) instance_extensions: Vec<CString>,
}

pub enum RendererConfigType {
    Vulkan(VulkanRenderConfiguration),
}

pub struct RendererConfig {
    pub render_type: RendererConfigType,
}

pub struct SamplerConfig {
    minFilter: FilterType,
    magFilter: FilterType,
}

pub struct SamplerPair<'a> {
    name: &'a str,
    sampler: &'a Sampler,
}

pub struct RootSignatureConfig<'a> {
    shaders: Vec<&'a Shader>,
}

pub struct ComputePipelineType {}

pub struct GraphicsPipelineType {}

pub struct RaytracePipelineType {}

pub enum PipelineTypeConfiguration {
    Compute(ComputePipelineType),
    Graphics(GraphicsPipelineType),
    Raytrace(RaytracePipelineType),
}

pub struct PipelineConfig {
    pub pipeline_type: PipelineTypeConfiguration,
    pub name: &'static str,
}
