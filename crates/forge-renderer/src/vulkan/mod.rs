mod command;
mod desc;
mod device;
mod fence;
mod queue;
mod renderer;
mod types;
mod buffer;

pub use buffer::*;

use crate::{
    error::RendererError::VulkanError,
    ffi,
    types::{DescriptorType, QueueType, ResourceMemoryUsage, SampleCount},
    vulkan::types::{VulkanSupportedFeatures, MAX_QUEUE_FLAGS},
    APIType, Buffer, Command, CommandPool, DescriptorIndexMap, Fence, FenceStatus, GPUCommonInfo,
    Queue, RenderContext, RenderTarget, RendererResult, RootSignature, Sampler, Semaphore, Shader,
    SwapChain, Texture,
};
use forge_image_format::ImageFormat;
use std::{
    f32::consts::E,
    ffi::c_void,
    os::{linux::raw::stat, raw::c_float},
    ptr, sync,
    sync::{Arc, Mutex},
};
use std::ffi::CString;
use crate::types::{ShaderStage, ShaderStageFlags};

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
    type Command<'a> = VulkanCommand<'a>;
    type CommandPool<'a> = VulkanCommandPool<'a>;
    type Buffer = VulkanBuffer;
    type SwapChain = VulkanSwapChain;

    const CURRENT_API: APIType = APIType::Vulkan;
}

pub struct VulkanSwapChain {}

impl SwapChain for VulkanSwapChain {}

pub struct VulkanCommandPool<'a> {
    pub(in crate::vulkan) renderer: Arc<VulkanRenderer>,
    pub(in crate::vulkan) cmd_pool: ffi::vk::VkCommandPool,
    pub(in crate::vulkan) queue: &'a VulkanQueue,
}

impl<'a> Drop for VulkanCommandPool<'a> {
    fn drop(&mut self) {
        assert!(self.cmd_pool != ptr::null_mut());
        unsafe {
            match Arc::get_mut(&mut self.renderer) {
                Some(renderer) => {

                    assert!(renderer.device != ptr::null_mut());
                    ffi::vk::vkDestroyCommandPool(renderer.device, self.cmd_pool, ptr::null_mut());
                }
                None => {
                    assert!(false, "failed to correctly dispose of command pool");
                }
            }
        }
    }
}

impl<'a> CommandPool for VulkanCommandPool<'a> {
    fn reset(&mut self) -> RendererResult<()> {
        assert!(self.cmd_pool != ptr::null_mut());
        match Arc::get_mut(&mut self.renderer) {
            Some(renderer) => {
                assert!(renderer.device != ptr::null_mut());
                unsafe {
                    let result = ffi::vk::vkResetCommandPool(renderer.device, self.cmd_pool, 0);
                    if result != ffi::vk::VkResult_VK_SUCCESS {
                        return Err(VulkanError(result));
                    }
                }
            }
            None => {
                assert!(false, "failed to correctly dispose of command pool");
            }
        }
        Ok(())
    }
}

pub struct VulkanRootSignature {

}

impl RootSignature for VulkanRootSignature {}

pub struct VulkanCommand<'a> {
    pub(in crate::vulkan) renderer: Arc<VulkanRenderer>,
    pub(in crate::vulkan) cmd_buf: ffi::vk::VkCommandBuffer,
    pub(in crate::vulkan) active_render_pass: ffi::vk::VkRenderPass,
    pub(in crate::vulkan) bound_pipeline_layout: ffi::vk::VkPipelineLayout,

    pub(in crate::vulkan) pool: &'a VulkanCommandPool<'a>,
}

impl<'a> Drop for VulkanCommand<'a> {
    fn drop(&mut self) {
        assert!(self.cmd_buf != ptr::null_mut());
        match Arc::get_mut(&mut self.renderer) {
            None => {
                assert!(false, "failed to correctly dispose of command pool");
            }
            Some(renderer) => {
                assert!(renderer.device != ptr::null_mut());
                assert!(self.pool.cmd_pool != ptr::null_mut());
                unsafe {
                    ffi::vk::vkFreeCommandBuffers(
                        renderer.device,
                        self.pool.cmd_pool,
                        1,
                        &mut self.cmd_buf,
                    );
                    self.cmd_buf = ptr::null_mut();
                }
            }
        }
    }
}

pub struct VulkanRenderContext {
    gpu: ffi::vk::VkPhysicalDevice,
    gpu_properties: ffi::vk::VkPhysicalDeviceProperties2,
    common: GPUCommonInfo,
}

impl RenderContext for VulkanRenderContext {}

pub struct VulkanRenderTarget {
    pub(in crate::vulkan) descriptor: ffi::vk::VkImageView,
    pub(in crate::vulkan) slice_descriptors: Vec<ffi::vk::VkImageView>,
    pub(in crate::vulkan) id: u32,

    pub(in crate::vulkan) array_size: u32,
    pub(in crate::vulkan) depth: u32,
    pub(in crate::vulkan) width: u32,
    pub(in crate::vulkan) height: u32,
    pub(in crate::vulkan) descriptors: u32,
    pub(in crate::vulkan) mip_levels: u32,
    pub(in crate::vulkan) sample_quality: u32,
    pub(in crate::vulkan) format: ImageFormat,
    pub(in crate::vulkan) sample_count: SampleCount,
    // vr_multiview: bool,
    // VRFoveatedRendering: bool,
}

impl RenderTarget for VulkanRenderTarget {}

pub struct VulkanSampler {
    pub(in crate::vulkan) renderer: Arc<VulkanRenderer>,
    pub(in crate::vulkan) sampler: ffi::vk::VkSampler,
}

impl Drop for VulkanSampler {
    fn drop(&mut self) {
        assert!(self.sampler != ptr::null_mut());
        match Arc::get_mut(&mut self.renderer) {
            None => {
                assert!(false, "failed to correctly dispose of sampler");
            }
            Some(renderer) => {
                assert!(renderer.device != ptr::null_mut());
                unsafe {
                    ffi::vk::vkDestroySampler(renderer.device, self.sampler, ptr::null_mut());
                }
                self.sampler = ptr::null_mut();
            }
        }
    }
}

impl Sampler for VulkanSampler {}

pub struct VulkanDescriptorIndexMap {}

impl DescriptorIndexMap for VulkanDescriptorIndexMap {}

pub struct VulkanShader {
    pub(in crate::vulkan) render: Arc<VulkanRenderer>,
    pub(in crate::vulkan) stages: ShaderStageFlags,

    pub(in crate::vulkan) shader_module: Vec<Option<(CString, ffi::vk::VkShaderModule)>>,
}

impl Drop for VulkanShader {
    fn drop(&mut self) {
        match Arc::get_mut(&mut self.render) {
            None => {
                assert!(false, "failed to correctly dispose of shader");
            }
            Some(renderer) => {
                for module in &self.shader_module {
                    match module {
                        Some((str, mut shader) ) => {
                            unsafe {
                                ffi::vk::vkDestroyShaderModule(renderer.device, shader, ptr::null_mut());
                            }
                        },
                        None => {}
                    }
                }
            }
        }
    }
}

impl Shader for VulkanShader {}

pub struct VulkanTexture {
    /// Opaque handle used by shaders for doing read/write operations on the texture
    pub vk_srv_descriptor: ffi::vk::VkImageView,
    /// Opaque handle used by shaders for doing read/write operations on the texture
    pub vk_uav_descriptors: Vec<ffi::vk::VkImageView>,
    /// Opaque handle used by shaders for doing read/write operations on the texture
    pub vk_srv_stencil_descriptor: ffi::vk::VkImageView,
    /// Native handle of the underlying resource
    pub vk_image: ffi::vk::VkImage,

    pub vma_memory: ffi::vk::VmaAllocation,
    pub vk_device_memory: ffi::vk::VkDeviceMemory,

    /// Current state of the buffer
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub mip_levels: u32,
    pub array_size_minus_one: u32,
    pub format: ImageFormat,
    // Flags specifying which aspects (COLOR,DEPTH,STENCIL) are included in the pVkImageView
    pub aspect_mask: u32,
    pub node_index: u32,
    // This value will be false if the underlying resource is not owned by the texture (swapchain textures,...)
    pub uav: bool,
    // This value will be false if the underlying resource is not owned by the texture (swapchain textures,...)
    pub own_image: bool,
}

impl Texture for VulkanTexture {}

pub struct VulkanSemaphore {
    pub(in crate::vulkan) render: Arc<VulkanRenderer>,
    pub(in crate::vulkan) semaphore: ffi::vk::VkSemaphore,
    pub(in crate::vulkan) current_node: u32,
    pub(in crate::vulkan) signaled: bool,
}

impl Drop for VulkanSemaphore {
    fn drop(&mut self) {
        assert!(self.semaphore != ptr::null_mut());
        match Arc::get_mut(&mut self.render) {
            Some(renderer) => {
                assert!(renderer.device != ptr::null_mut());
                unsafe {
                    ffi::vk::vkDestroySemaphore(renderer.device, self.semaphore, ptr::null_mut());
                }
                self.semaphore = ptr::null_mut();
            }
            None => {
                assert!(false, "failed to correctly dispose of semaphore");
            }
        }
    }
}

impl Semaphore for VulkanSemaphore {}

pub struct VulkanQueue {
    pub(in crate::vulkan) render: Arc<VulkanRenderer>,
    pub(in crate::vulkan) queue: ffi::vk::VkQueue,
    pub(in crate::vulkan) submission_mutex: Mutex<()>,

    pub(in crate::vulkan) family_index: u32,
    pub(in crate::vulkan) queue_index: u32,
    pub(in crate::vulkan) queue_flag: ffi::vk::VkQueueFlags,

    pub(in crate::vulkan) queue_type: QueueType,
    pub(in crate::vulkan) node_index: u32,
}

impl Drop for VulkanQueue {
    fn drop(&mut self) {
        assert!(self.queue != ptr::null_mut());
        let node_index = 0;
        let queue_flags = self.queue_flag;

        match Arc::get_mut(&mut self.render) {
            Some(renderer) => {
                renderer.used_queue_count[node_index as usize][queue_flags as usize] -= 1;
            }
            None => {
                assert!(false, "failed to correctly dispose of fence");
            }
        }
    }
}

pub struct VulkanFence {
    pub(in crate::vulkan) render: Arc<VulkanRenderer>,
    pub(in crate::vulkan) fence: ffi::vk::VkFence,
    pub(in crate::vulkan) submitted: bool,
}

impl Drop for VulkanFence {
    fn drop(&mut self) {
        assert!(self.fence != ptr::null_mut());
        match Arc::get_mut(&mut self.render) {
            Some(renderer) => {
                assert!(renderer.device != ptr::null_mut());
                unsafe {
                    ffi::vk::vkDestroyFence(renderer.device, self.fence, ptr::null_mut());
                }
                self.fence = ptr::null_mut();
            }
            None => {
                assert!(false, "failed to correctly dispose of fence");
            }
        }
    }
}

impl Fence<VulkanAPI> for VulkanFence {
    unsafe fn status(&mut self, render: &VulkanRenderer) -> FenceStatus {
        if self.submitted {
            let result = ffi::vk::vkGetFenceStatus(render.device, self.fence);
            if result == ffi::vk::VkResult_VK_SUCCESS {
                ffi::vk::vkResetFences(render.device, 1, &mut self.fence);
                self.submitted = false;
                return FenceStatus::Complete;
            }
            return FenceStatus::Incomplete;
        }
        return FenceStatus::NotSubmitted;
    }
}

pub struct VulkanPipeline {
    renderer: Arc<VulkanRenderer>,
    pipeline: ffi::vk::VkPipeline, // PipelineType mType;
                                   // uint32_t     mShaderStageCount;
                                   //In DX12 this information is stored in ID3D12StateObject.
                                   //But for Vulkan we need to store it manually
                                   // const char** ppShaderStageNames;
}

impl Drop for VulkanPipeline {
    fn drop(&mut self) {
        todo!()
    }
}

impl crate::Pipeline for VulkanPipeline {}

pub struct VulkanRenderer {
    pub(in crate::vulkan) entry: ash::Entry,
    pub(in crate::vulkan) instance: Option<ash::Instance>,
    pub(in crate::vulkan) device: Option<ash::vk::Device>,
    pub(in crate::vulkan) features: VulkanSupportedFeatures,

    pub(in crate::vulkan) graphics_queue_family_index: u32,
    pub(in crate::vulkan) transfer_queue_family_index: u32,
    pub(in crate::vulkan) compute_queue_family_index: u32,

    pub(in crate::vulkan) active_gpu: Option<ash::vk::PhysicalDevice>,
    pub(in crate::vulkan) active_gpu_properties: Option<ash::vk::PhysicalDeviceProperties>,
    pub(in crate::vulkan) active_gpu_common_info: Option<Box<GPUCommonInfo>>,
    pub(in crate::vulkan) linked_node_count: u16,

    pub(in crate::vulkan) available_queue_count: Vec<[u32; MAX_QUEUE_FLAGS as usize]>,
    pub(in crate::vulkan) used_queue_count: Vec<[u32; MAX_QUEUE_FLAGS as usize]>,

    pub(in crate::vulkan) me: sync::Weak<VulkanRenderer>,

    pub(in crate::vulkan) vma_allocator: Option<vk_mem::Allocator>,
}
