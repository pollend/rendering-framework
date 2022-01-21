mod command;
mod desc;
mod device;
mod queue;
mod renderer;
mod types;

use crate::{
    ffi,
    types::QueueType,
    vulkan::types::{VulkanSupportedFeatures, MAX_QUEUE_FLAGS},
    APIType, Buffer, Command, CommandPool, DescriptorIndexMap, Fence, GPUCommonInfo, Queue,
    RenderContext, RenderTarget, RootSignature, Sampler, Semaphore, Shader, Texture,
};
use std::{os::raw::c_float, sync::Mutex};

#[derive(Clone)]
pub struct VulkanAPI;

impl crate::Api for VulkanAPI {
    type RenderContext = VulkanRenderContext;
    type Renderer = VulkanRenderer;
    type RootSignature = VulkanRootSignature;
    type Pipeline = VulkanPipeline;
    type Fence = VulkanFence;
    type Semaphore = VulkanSemaphore;
    type Queue = VulkanQueue;
    type Texture = VulkanTexture;
    type Shader = VulkanShader;
    type RenderTarget = VulkanRenderTarget;
    type DescriptorIndexMap = VulkanDescriptorIndexMap;
    type Sampler = VulkanSampler;
    type Command = VulkanCommand;
    type CommandPool = VulkanCommandPool;
    type Buffer = VulkanBuffer;

    const CURRENT_API: APIType = APIType::Vulkan;
}

pub struct VulkanCommandPool {
    pub(in crate::vulkan) cmd_pool: ffi::vk::VkCommandPool,
}

impl CommandPool for VulkanCommandPool {}

pub struct VulkanRootSignature {}

impl RootSignature for VulkanRootSignature {}

pub struct VulkanBuffer {}

impl Buffer for VulkanBuffer {}

pub struct VulkanCommand {
    cmd_buf: ffi::vk::VkCommandBuffer,
    active_render_pass: ffi::vk::VkRenderPass,
    bound_pipeline_layout: ffi::vk::VkPipelineLayout,
}

pub struct VulkanRenderContext {
    gpu: ffi::vk::VkPhysicalDevice,
    gpu_properties: ffi::vk::VkPhysicalDeviceProperties2,
    common: GPUCommonInfo,
}

impl RenderContext for VulkanRenderContext {}

pub struct VulkanRenderTarget {}

impl RenderTarget for VulkanRenderTarget {}

pub struct VulkanSampler {
    sampler: ffi::vk::VkSampler,
}

impl Sampler for VulkanSampler {}

pub struct VulkanDescriptorIndexMap {}

impl DescriptorIndexMap for VulkanDescriptorIndexMap {}

pub struct VulkanShader {}

impl Shader for VulkanShader {}

pub struct VulkanTexture {}

impl Texture for VulkanTexture {}

pub struct VulkanSemaphore {
    pub semaphore: ffi::vk::VkSemaphore,
    pub current_node: u32,
    pub signaled: bool,
}

impl Semaphore for VulkanSemaphore {}

pub struct VulkanQueue {
    pub(in crate::vulkan) queue: ffi::vk::VkQueue,
    pub(in crate::vulkan) submission_mutex: Mutex<()>,

    // timestamp_period: f32,
    pub(in crate::vulkan) family_index: u32,
    pub(in crate::vulkan) queue_index: u32,
    pub(in crate::vulkan) queue_flag: ffi::vk::VkQueueFlags,

    pub(in crate::vulkan) queue_type: QueueType,
}

pub struct VulkanFence {
    pub(in crate::vulkan) fence: ffi::vk::VkFence,
    pub(in crate::vulkan) submitted: bool,
}

impl Fence for VulkanFence {}

pub struct VulkanPipeline {
    pipeline: ffi::vk::VkPipeline, // PipelineType mType;
                                   // uint32_t     mShaderStageCount;
                                   //In DX12 this information is stored in ID3D12StateObject.
                                   //But for Vulkan we need to store it manually
                                   // const char** ppShaderStageNames;
}

impl crate::Pipeline for VulkanPipeline {}

pub struct VulkanRenderer {
    pub(in crate::vulkan) instance: ffi::vk::VkInstance,
    pub(in crate::vulkan) device: ffi::vk::VkDevice,
    pub(in crate::vulkan) features: VulkanSupportedFeatures,

    pub(in crate::vulkan) graphics_queue_family_index: u32,
    pub(in crate::vulkan) transfer_queue_family_index: u32,
    pub(in crate::vulkan) compute_queue_family_index: u32,

    pub(in crate::vulkan) active_gpu: ffi::vk::VkPhysicalDevice,
    pub(in crate::vulkan) active_gpu_properties: Option<ffi::vk::VkPhysicalDeviceProperties>,
    pub(in crate::vulkan) active_gpu_common_info: Option<Box<GPUCommonInfo>>,
    pub(in crate::vulkan) linked_node_count: u16,

    pub(in crate::vulkan) available_queue_count: Vec<[u32; MAX_QUEUE_FLAGS as usize]>,
    pub(in crate::vulkan) used_queue_count: Vec<[u32; MAX_QUEUE_FLAGS as usize]>,
}
