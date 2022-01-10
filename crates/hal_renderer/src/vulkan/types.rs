use crate::{
    configuration::*,
    error::{HalError, HalResult},
    ffi,
    vulkan::*,
};
use std::{
    ptr,
    rc::{Rc, Weak},
};

pub struct VulkanPipeline {
    pub(in crate::vulkan) pipeline: ffi::vk::VkPipeline,
    // ffi::vulkan::VkPipeline   pVkPipeline;
    // PipelineType mType;
    // uint32_t     mShaderStageCount;
}

pub struct VulkanRootSignature {
    pub(crate) pipeline_layout: ffi::vk::VkPipelineLayout,
}

pub struct VulkanSampler {
    sampler: ffi::vk::VkSampler,
    samplerYcbcrConversion: ffi::vk::VkSamplerYcbcrConversion,
    samplerYcbcrConversionInfo: ffi::vk::VkSamplerYcbcrConversionInfo,
}

pub struct VulkanQueue {}

pub struct VulkanBuffer {}

pub struct VulkanTexture {}

pub struct VulkanRenderTarget {}

pub struct VulkanShader {
    shader_module: ffi::vk::VkShaderModule,
}

pub struct VulkanDescriptorSet {}
