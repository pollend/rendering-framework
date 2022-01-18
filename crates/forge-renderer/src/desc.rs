use crate::{
    types::{QueueFlag, QueuePriority, QueueType},
    Api,
};
use forge_image_format::ImageFormat;
use std::ffi::{CStr, CString};
use crate::types::{AddressMode, CompareMode, DescriptorType, FilterType, MipMapMode, ResourceMemoryUsage};

pub struct VulkanRenderDesc {
    pub instance_layers: Vec<CString>,
    pub instance_extensions: Vec<CString>,
    pub device_extensions: Vec<CString>,

    /// Flag to specify whether to request all queues from the gpu or just one of each type
    /// This will affect memory usage - Around 200 MB more used if all queues are requested
    pub request_all_available_queues: bool,
}

pub enum RenderDescImp {
    Vulkan(VulkanRenderDesc),
}

pub struct RenderDesc {
    pub imp: RenderDescImp,
}

pub struct CmdPoolDesc<'a, T: Api> {
    pub queue: &'a T::Queue,
    pub transient: bool,
}

pub struct PipelineComputeDesc<'a, T: Api> {
    pub shader: &'a T::Shader,
    pub signature: &'a T::RootSignature
}

pub struct PipelineGraphicsDesc<'a, T: Api> {
    pub shader: &'a T::Shader
}

pub enum PipelineDescType<'a, T:Api> {
    Compute(PipelineComputeDesc<'a, T>),
    Graphics(PipelineGraphicsDesc<'a, T>),
}

pub struct PipelineDesc<'a, T: Api> {
    pub pipeline: PipelineDescType<'a, T>,
    pub name: &'a CString,
}

pub struct RootSignatureDesc<'a, T: Api> {
    shader: &'a Vec<T::Shader>,
}

pub struct SamplerDesc {
    min_filter: FilterType,
    mag_filter: FilterType,
    mode: MipMapMode,
    address_u: AddressMode,
    address_v: AddressMode,
    address_w: AddressMode,
    mip_load_bias: f32,
    max_anisotropy: f32,
    compareFunc: CompareMode

}

pub struct BufferDesc {
    pub size: u64,

    pub debug_name: CString,

    // Flags specifying the suitable usage of this buffer (Uniform buffer, Vertex Buffer, Index Buffer,...)
    pub descriptors: DescriptorType,

    pub format: ImageFormat,

    pub memory_usage: ResourceMemoryUsage
}

pub struct QueueDesc {
    pub queue_type: QueueType,
    pub flag: QueueFlag,
    pub priority: QueuePriority,
    pub node_index: u32,
    pub image_format: ImageFormat
}
