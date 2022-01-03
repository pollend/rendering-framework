use crate::configuration::*;
use crate::error::{HalError, HalResult};
use crate::ffi;
use crate::vulkan::*;
use std::{
    ptr,
    rc::{Rc, Weak},
};

pub type HalVKRendererImpl = Rc<VulkanRenderer>;
pub type HalVKPipelineImpl = Rc<HalVKPipeline>;

pub struct HalVKQueue {}

pub struct HalVKBuffer {}

pub struct HalVKTexture {}

pub struct HalVKRenderTarget {}

pub struct HalVKShader {
    shader_module: ffi::vk::VkShaderModule,
}

pub struct HalVKDescriptorSet {}
