use crate::configuration::*;
use crate::error::{HalError, HalResult};
use crate::ffi;
use crate::vulkan::*;
use std::{
    ptr,
    rc::{Rc, Weak},
};

pub struct HalVKPipeline {
    pipeline: ffi::vk::VkPipeline,
    me: Weak<HalVKPipeline>,
    // ffi::vulkan::VkPipeline   pVkPipeline;
    // PipelineType mType;
    // uint32_t     mShaderStageCount;
}

impl HalVKPipeline {
    pub fn new(
        renderer: &HalVKRendererImpl,
        config: &PipelineConfig,
    ) -> HalResult<Rc<HalVKPipeline>> {
        match &config.pipeline_type {
            PipelineType::Compute(compute_desc) => {}
            PipelineType::Graphics(graphics_desc) => {}
            PipelineType::Raytrace(raytrace_desc) => {}
        }
        Ok(Rc::new_cyclic(|self_weak| HalVKPipeline {
            pipeline: ptr::null_mut(),
            me: self_weak.clone(),
        }))
    }
}
