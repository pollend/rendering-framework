use crate::{
    types::{
        AddressMode, CompareMode, DescriptorType, FilterType, MipMapMode, QueueFlag, QueuePriority,
        QueueType, ResourceMemoryUsage,
    },
    Api,
};
use forge_image_format::ImageFormat;
use raw_window_handle::HasRawWindowHandle;
use std::ffi::{c_void, CStr, CString};
use crate::types::{IndirectArgumentType, ResourceState, SampleCount, TextureCreationFlags};

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

pub struct SwapChainDesc<'a, T: Api> {
    pub presentation_queues: Vec<&'a T::Queue>,
    pub image_count: u32,
    pub width: u32,
    pub height: u32,
    pub color_format: ImageFormat,
    pub enable_vsync: bool, // pub window_handle: &impl raw_window_handle::HasRawWindowHandle
}

pub struct CmdDesc<'a, T: Api> {
    pub cmd_pool: &'a T::CommandPool<'a>,
    pub secondary: bool,
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

pub struct QueueSubmitDesc<'a, 'b, T: Api> {
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

pub struct TextureDesc {
    // Pointer to native texture handle if the texture does not own underlying resource
    pub native_handle: *mut c_void,
    // Debug name used in gpu profile
    pub name: CString,
    // Texture creation flags (decides memory allocation strategy, sharing access,...)
    pub flags: TextureCreationFlags,
    // Width
    pub width: u32,
    // Height
    pub height: u32,
    // Depth (Should be 1 if not a mType is not TEXTURE_TYPE_3D)
    pub depth: u32,
    // Texture array size (Should be 1 if texture is not a texture array or cubemap)
    pub array_size: u32,
    // Number of mip levels
    pub mip_levels: u32,
    // Number of multisamples per pixel (currently Textures created with mUsage TEXTURE_USAGE_SAMPLED_IMAGE only support SAMPLE_COUNT_1)
    pub sample_count: SampleCount,
    // The image quality level. The higher the quality, the lower the performance. The valid range is between zero and the value appropriate for mSampleCount
    pub sample_quality: u32,
    //  image format
    pub format: ImageFormat,
    // What state will the texture get created in
    pub start_state: ResourceState,
    // Descriptor creation
    pub descriptors: DescriptorType,
    // GPU indices to share this texture
    pub shared_node_indices: Vec<u32>,
    // GPU which will own this texture
    pub node_index: u32
}

pub struct RenderTargetDesc {
    // Texture creation flags (decides memory allocation strategy, sharing access,...)
    pub flags: TextureCreationFlags,
    // width
    pub width: u32,
    // height
    pub height: u32,
    // Depth (Should be 1 if not a mType is not TEXTURE_TYPE_3D)
    pub depth: u32,
    // Texture array size (Should be 1 if texture is not a texture array or cubemap)
    pub array_size: u32,
    // Number of mip levels
    pub mip_levels: u32,
    // MSAA
    pub sample_count: SampleCount,
    // Internal image format
    pub format: ImageFormat,
    // What state will the texture get created in
    pub start_state: ResourceState,
    // ICB draw type
    pub argument_type: IndirectArgumentType,
    // clear value
    // The image quality level. The higher the quality, the lower the performance. The valid range is between zero and the value appropriate for mSampleCount
    pub sample_quality: u32,
    // Descriptor creation
    pub descriptors: DescriptorType,
    pub native_handle: *mut c_void,
    // Debug name used in gpu profile
    pub name: CString,
    // GPU indices to share this texture
    pub shared_node_indices: Vec<u32>,
    // GPU which will own this texture
    pub node_index: u32
}
