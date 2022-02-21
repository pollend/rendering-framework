mod buffer;
mod command;
mod desc;
mod device;
mod fence;
mod queue;
mod renderer;
mod types;

pub use buffer::*;

use crate::{
    error::RendererError::VulkanError,
    types::{
        DescriptorType, QueueType, ResourceMemoryUsage, SampleCount, ShaderStage, ShaderStageFlags,
    },
    vulkan::types::{VulkanSupportedFeatures},
    APIType, Buffer, Command, CommandPool, DescriptorIndexMap, Fence, FenceStatus, GPUCommonInfo,
    Queue, RenderContext, RenderTarget, RendererResult, RootSignature, Sampler, Semaphore, Shader,
    SwapChain, Texture,
};
use ash::{prelude::VkResult, vk::CommandPoolResetFlags};
use forge_image_format::ImageFormat;
use gpu_allocator::vulkan::{Allocation, Allocator};
use std::{
    f32::consts::E,
    ffi::{c_void, CString},
    os::{linux::raw::stat, raw::c_float},
    ptr, sync,
    sync::{Arc, Mutex},
};

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
    pub(in crate::vulkan) cmd_pool: ash::vk::CommandPool,
    pub(in crate::vulkan) queue: &'a VulkanQueue,
}

impl<'a> Drop for VulkanCommandPool<'a> {
    fn drop(&mut self) {
        assert_ne!(self.cmd_pool, ash::vk::CommandPool::null());
        unsafe {
            match Arc::get_mut(&mut self.renderer) {
                Some(renderer) => {
                    renderer.device.destroy_command_pool(self.cmd_pool, None);
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
        assert_ne!(self.cmd_pool, ash::vk::CommandPool::null());
        match Arc::get_mut(&mut self.renderer) {
            Some(renderer) => {
                // assert!(renderer.device != ptr::null_mut());
                unsafe {
                    renderer
                        .device
                        .reset_command_pool(self.cmd_pool, CommandPoolResetFlags::empty());
                    // let result = ffi::vk::vkResetCommandPool(renderer.device, self.cmd_pool, 0);
                    // if result != ffi::vk::VkResult_VK_SUCCESS {
                    //     return Err(VulkanError(result));
                    // }
                }
            }
            None => {
                assert!(false, "failed to correctly dispose of command pool");
            }
        }
        Ok(())
    }
}

pub struct VulkanRootSignature {}

impl RootSignature for VulkanRootSignature {}

pub struct VulkanCommand<'a> {
    pub(in crate::vulkan) renderer: Arc<VulkanRenderer>,
    pub(in crate::vulkan) cmd_buf: ash::vk::CommandBuffer,
    pub(in crate::vulkan) active_render_pass: ash::vk::RenderPass,
    pub(in crate::vulkan) bound_pipeline_layout: ash::vk::PipelineLayout,

    pub(in crate::vulkan) pool: &'a VulkanCommandPool<'a>,
}

impl<'a> Drop for VulkanCommand<'a> {
    fn drop(&mut self) {
        assert_ne!(self.cmd_buf, ash::vk::CommandBuffer::null());
        match Arc::get_mut(&mut self.renderer) {
            None => {
                assert!(false, "failed to correctly dispose of command pool");
            }
            Some(renderer) => {
                // assert!(renderer.device != ptr::null_mut());
                assert_ne!(self.pool.cmd_pool, ash::vk::CommandPool::null());
                unsafe {
                    renderer
                        .device
                        .free_command_buffers(self.pool.cmd_pool, [self.cmd_buf].as_slice());
                }
            }
        }
    }
}

pub struct VulkanRenderContext {
    gpu: ash::vk::PhysicalDevice,
    gpu_properties: ash::vk::PhysicalDeviceProperties2,
    common: GPUCommonInfo,
}

impl RenderContext for VulkanRenderContext {}

pub struct VulkanRenderTarget {
    pub(in crate::vulkan) descriptor: ash::vk::ImageView,
    pub(in crate::vulkan) slice_descriptors: Vec<ash::vk::ImageView>,
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
    pub(in crate::vulkan) sampler: ash::vk::Sampler,
}

impl Drop for VulkanSampler {
    fn drop(&mut self) {
        assert_ne!(self.sampler, ash::vk::Sampler::null());
        match Arc::get_mut(&mut self.renderer) {
            None => {
                assert!(false, "failed to correctly dispose of sampler");
            }
            Some(renderer) => {
                // assert!(renderer.device != ptr::null_mut());

                unsafe {
                    renderer.device.destroy_sampler(self.sampler, None);
                    // ffi::vk::vkDestroySampler(renderer.device, self.sampler, ptr::null_mut());
                }
                self.sampler = ash::vk::Sampler::null();
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

    pub(in crate::vulkan) shader_module: Vec<Option<(CString, ash::vk::ShaderModule)>>,
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
                        Some((str, mut shader)) => {
                            unsafe {
                                renderer.device.destroy_shader_module(shader, None);
                                // ffi::vk::vkDestroyShaderModule(renderer.device, shader, ptr::null_mut());
                            }
                        }
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
    pub vk_srv_descriptor: ash::vk::ImageView,
    /// Opaque handle used by shaders for doing read/write operations on the texture
    pub vk_uav_descriptors: Vec<ash::vk::ImageView>,
    /// Opaque handle used by shaders for doing read/write operations on the texture
    pub vk_srv_stencil_descriptor: ash::vk::ImageView,
    /// Native handle of the underlying resource
    pub vk_image: ash::vk::Image,

    pub allocation: Allocation,
    // pub vma_memory: ffi::vk::VmaAllocation,
    pub vk_device_memory: ash::vk::DeviceMemory,

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
    pub(in crate::vulkan) semaphore: ash::vk::Semaphore,
    pub(in crate::vulkan) current_node: u32,
    pub(in crate::vulkan) signaled: bool,
}

impl Drop for VulkanSemaphore {
    fn drop(&mut self) {
        assert_ne!(self.semaphore, ash::vk::Semaphore::null());
        match Arc::get_mut(&mut self.render) {
            Some(renderer) => {
                unsafe {
                    renderer.device.destroy_semaphore(self.semaphore, None);
                }
                self.semaphore = ash::vk::Semaphore::null();
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
    pub(in crate::vulkan) queue: ash::vk::Queue,
    pub(in crate::vulkan) submission_mutex: Mutex<()>,

    pub(in crate::vulkan) family_index: u32,
    pub(in crate::vulkan) queue_index: u32,
    pub(in crate::vulkan) queue_flag: ash::vk::QueueFlags,

    pub(in crate::vulkan) queue_type: QueueType,
    pub(in crate::vulkan) node_index: u32,
}

impl Drop for VulkanQueue {
    fn drop(&mut self) {
        assert_ne!(self.queue, ash::vk::Queue::null());
        let node_index = 0;
        let queue_flags = self.queue_flag;

        match Arc::get_mut(&mut self.render) {
            Some(renderer) => {
                renderer.used_queue_count[node_index as usize][queue_flags.as_raw() as usize] -= 1;
            }
            None => {
                assert!(false, "failed to correctly dispose of fence");
            }
        }
    }
}

pub struct VulkanFence {
    pub(in crate::vulkan) render: Arc<VulkanRenderer>,
    pub(in crate::vulkan) fence: ash::vk::Fence,
    pub(in crate::vulkan) submitted: bool,
}

impl Drop for VulkanFence {
    fn drop(&mut self) {
        assert_ne!(self.fence, ash::vk::Fence::null());
        match Arc::get_mut(&mut self.render) {
            Some(renderer) => {
                unsafe {
                    renderer.device.destroy_fence(self.fence, None);
                }
                self.fence = ash::vk::Fence::null();
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
            match self.render.device.get_fence_status(self.fence) {
                Ok(success) => {
                    if success {
                        render.device.reset_fences([self.fence].as_slice());
                        self.submitted = false;
                        return FenceStatus::Complete;
                    }
                }
                Err(_) => {}
            }
            return FenceStatus::Incomplete;
        }
        return FenceStatus::NotSubmitted;
    }
}

pub struct VulkanPipeline {
    pub(in crate::vulkan)  renderer: Arc<VulkanRenderer>,
    pub(in crate::vulkan)  pipeline: ash::vk::Pipeline, // PipelineType mType;
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
    pub(in crate::vulkan) instance: ash::Instance,
    pub(in crate::vulkan) device: ash::Device,
    pub(in crate::vulkan) swapchain_loader: ash::extensions::khr::Swapchain,
    pub(in crate::vulkan) surface_loader: ash::extensions::khr::Surface,

    pub(in crate::vulkan) features: VulkanSupportedFeatures,

    pub(in crate::vulkan) graphics_queue_family_index: u32,
    pub(in crate::vulkan) transfer_queue_family_index: u32,
    pub(in crate::vulkan) compute_queue_family_index: u32,

    pub(in crate::vulkan) active_gpu: ash::vk::PhysicalDevice,
    pub(in crate::vulkan) active_gpu_properties: ash::vk::PhysicalDeviceProperties,
    pub(in crate::vulkan) active_gpu_common_info: Box<GPUCommonInfo>,
    pub(in crate::vulkan) linked_node_count: u16,

    pub(in crate::vulkan) available_queue_count: Vec<Box<[u32]>>,
    pub(in crate::vulkan) used_queue_count: Vec<Box<[u32]>>,

    pub(in crate::vulkan) me: sync::Weak<VulkanRenderer>,

    pub(in crate::vulkan) allocator: Allocator,
}
