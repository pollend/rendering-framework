mod desc;
mod device;
mod renderer;
mod types;

use crate::{ffi, vulkan::types::{VulkanSupportedFeatures, MAX_QUEUE_FLAGS}, APIType, Buffer, Command, DescriptorIndexMap, Fence, GPUCommonInfo, Queue, RenderContext, RenderTarget, Sampler, Semaphore, Shader, Texture, RootSignature};

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
    type Buffer = VulkanBuffer;

    const CURRENT_API: APIType = APIType::Vulkan;
}


pub struct VulkanRootSignature {}

impl RootSignature for VulkanRootSignature {
}

pub struct VulkanBuffer {}

impl Buffer for VulkanBuffer {}

pub struct VulkanCommand {}

impl Command for VulkanCommand {
    fn begin_cmd(&self) {
        todo!()
    }

    fn end_cmd(&self) {
        todo!()
    }

    fn cmd_bind_render_target(&self) {
        todo!()
    }

    fn cmd_set_shading_rate(&self) {
        todo!()
    }

    fn cmd_set_viewport(&self) {
        todo!()
    }

    fn cmd_set_scissor(&self) {
        todo!()
    }

    fn cmd_set_stencil_reference_value(&self) {
        todo!()
    }

    fn cmd_bind_pipeline(&self) {
        todo!()
    }

    fn cmd_bind_descriptor_set(&self) {
        todo!()
    }

    fn cmd_bind_index_buffer(&self) {
        todo!()
    }

    fn cmd_raw(&self) {
        todo!()
    }

    fn cmd_draw_instanced(&self) {
        todo!()
    }

    fn cmd_draw_indexed(&self) {
        todo!()
    }

    fn cmd_draw_indexed_instanced(&self) {
        todo!()
    }

    fn cmd_dispatch(&self) {
        todo!()
    }

    fn cmd_resource_barrier(&self) {
        todo!()
    }

    fn cmd_update_virtual_texture(&self) {
        todo!()
    }
}

pub struct VulkanRenderContext {
    gpu: ffi::vk::VkPhysicalDevice,
    gpu_properties: ffi::vk::VkPhysicalDeviceProperties2,
    common: GPUCommonInfo,
}

impl RenderContext for VulkanRenderContext {}

pub struct VulkanRenderTarget {}

impl RenderTarget for VulkanRenderTarget {}

pub struct VulkanSampler {}

impl Sampler for VulkanSampler {}

pub struct VulkanDescriptorIndexMap {}

impl DescriptorIndexMap for VulkanDescriptorIndexMap {}

pub struct VulkanShader {}

impl Shader for VulkanShader {}

pub struct VulkanTexture {}

impl Texture for VulkanTexture {}

pub struct VulkanSemaphore {
    semaphore: ffi::vk::VkSemaphore,
    signaled: bool,
}

impl Semaphore for VulkanSemaphore {}

pub struct VulkanQueue {}

impl Queue<VulkanAPI> for VulkanQueue {
    fn submit(&self) {
        todo!()
    }

    fn present(&self) {
        todo!()
    }

    fn wait_idle(&self) {
        todo!()
    }

    fn fence_status(&self) {
        todo!()
    }

    fn wait_fence(&self) {
        todo!()
    }

    fn toggle_v_sync(&self) {
        todo!()
    }
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
