mod renderer;
mod desc;

use crate::{Api, APIType, DescriptorIndexMap, Fence, ffi, Queue, Renderer, RendererResult, RenderTarget, Sampler, Semaphore, Shader, Texture};

#[derive(Clone)]
pub struct VulkanAPI;

impl crate::Api for VulkanAPI {
    type Renderer = VulkanRenderer;
    type Pipeline = VulkanPipeline;
    type Fence = VulkanFence;
    type Semaphore = VulkanSemaphore;
    type Queue = VulkanQueue;
    type Texture = VulkanTexture;
    type Shader = VulkanShader;
    type RenderTarget = VulkanRenderTarget;
    type DescriptorIndexMap = VulkanDescriptorIndexMap;
    type Sampler = VulkanSampler;

    const api: APIType = APIType::Vulkan;
}

pub struct VulkanRenderTarget {

}

impl RenderTarget for VulkanRenderTarget {

}

pub struct VulkanSampler {

}

impl Sampler for VulkanSampler {

}

pub struct VulkanDescriptorIndexMap {

}

impl DescriptorIndexMap for VulkanDescriptorIndexMap {

}

pub struct VulkanShader {

}

impl Shader for VulkanShader {

}


pub struct VulkanTexture {

}

impl Texture for VulkanTexture {

}

pub struct VulkanSemaphore {

}

impl Semaphore for VulkanSemaphore {

}

pub struct  VulkanQueue {

}

impl Queue for VulkanQueue {

}

pub struct VulkanFence {

}

impl Fence for VulkanFence {

}


pub struct VulkanPipeline {
    pipeline: ffi::vk::VkPipeline
    // PipelineType mType;
    // uint32_t     mShaderStageCount;
    //In DX12 this information is stored in ID3D12StateObject.
    //But for Vulkan we need to store it manually
    // const char** ppShaderStageNames;
}

impl crate::Pipeline for VulkanPipeline {
}

pub struct VulkanRenderer {
    pub(in crate::vulkan) instance: ffi::vk::VkInstance,
    pub(in crate::vulkan) active_gpu: ffi::vk::VkPhysicalDevice,
    pub(in crate::vulkan) device: ffi::vk::VkDevice,
}

