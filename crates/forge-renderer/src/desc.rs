use crate::{
    types::{
        AddressMode, CompareMode, DescriptorType, FilterType, MipMapMode, QueueFlag, QueuePriority,
        QueueType, ResourceMemoryUsage,
    },
    Api,
};
use forge_image_format::ImageFormat;
use std::ffi::{CStr, CString};
use raw_window_handle::HasRawWindowHandle;

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

pub struct SwapChainDesc<'a> {
    pub window_handle: &'a HasRawWindowHandle
}

pub struct CmdDesc<'a, T: Api> {
    pub cmd_pool: &'a T::CommandPool<'a>,
    pub secondary: bool
}

pub struct CmdPoolDesc<'a, T: Api> {
    pub queue: &'a T::Queue,
    pub transient: bool,
}

pub struct PipelineComputeDesc<'a, T: Api> {
    pub shader: &'a T::Shader,
    pub signature: &'a T::RootSignature,
}

pub struct PipelineGraphicsDesc<'a, T: Api> {
    pub shader: &'a T::Shader,
}

pub enum PipelineDescType<'a, T: Api> {
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

pub struct SamplerFormatDesc {}

pub struct SamplerDesc {
    pub min_filter: FilterType,
    pub mag_filter: FilterType,
    pub mode: MipMapMode,
    pub address_u: AddressMode,
    pub address_v: AddressMode,
    pub address_w: AddressMode,
    pub mip_load_bias: f32,
    pub max_anisotropy: f32,
    pub compare_func: CompareMode,
    // pub format_conversion: SamplerFormatDesc
}

pub struct BufferDesc {
    pub size: u64,

    pub debug_name: CString,

    // Flags specifying the suitable usage of this buffer (Uniform buffer, Vertex Buffer, Index Buffer,...)
    pub descriptors: DescriptorType,

    pub format: ImageFormat,

    pub memory_usage: ResourceMemoryUsage,
}

pub struct QueueDesc {
    pub queue_type: QueueType,
    pub flag: QueueFlag,
    pub priority: QueuePriority,
    pub node_index: u32,
    pub image_format: ImageFormat,
}

pub struct QueueSubmitDesc<'a,'b, T: Api> {
    pub cmds: Vec<&'a T::Command<'b>>,
    pub signal_fences: Option<&'a mut T::Fence>,
    pub wait_semaphores: Vec<&'a mut T::Semaphore>,
    pub signal_semaphores: Vec<&'a mut T::Semaphore>,
    pub submit_done: bool,
}

pub struct QueuePresentDesc<'a, T: Api> {
    pub wait_semaphores: Vec<&'a mut T::Semaphore>,
    pub index: u8,
    pub submit_done: bool,
}
