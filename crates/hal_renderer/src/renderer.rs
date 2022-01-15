use crate::{
    configuration::*,
    error::{
        HalError,
        HalError::{UnSupportedAPI, UnsupportedDevice},
        HalResult,
    },
    ffi,
    renderer::RenderAPI::{RendererEmpty, RendererVulkan},
    vulkan::{
        init_vulkan_renderer, VulkanBuffer, VulkanDescriptorSet, VulkanPipeline, VulkanQueue,
        VulkanRenderTarget, VulkanRenderer, VulkanRootSignature, VulkanSampler, VulkanShader,
        VulkanTexture,
    },
};

use crate::vulkan::vk_drop_pipeline;
use std::{
    ffi::c_void,
    mem::ManuallyDrop,
    ptr,
    sync::{Arc, Weak},
};
use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};
use crate::stub::{EmptyPipeline, EmptyRootSignature, EmptyShader};

enum FrequencyUpdateCount {
    DescriptorUpdateFreqNone = 0,
    DescriptorUpdateFreqPerFrame,
    DescriptorUpdateFreqPerBatch,
    DescriptorUpdateFreqPerDraw,
    DescriptorUpdateFreqCount,
}

type AddPipeline = fn(render: &Renderer, config: &PipelineConfig) -> Pipeline;
type RemovePipeline = fn(pipeline: &mut Pipeline);

type AddRootSignature = fn(render: &Renderer, config: &RootSignatureConfig) -> RootSignature;
type DropRootSignature = fn(signature: &mut RootSignature);

type AddSampler = fn(render: &mut Renderer, config: &SamplerConfig);
type RemoveSampler = fn(render: &mut Renderer, sampler: &Sampler);

pub struct MethodTable {
    pub(crate) add_pipeline: AddPipeline,
    pub(crate) remove_pipeline: RemovePipeline,
    pub(crate) add_root_signature: AddRootSignature,
    pub(crate) remove_root_signature: DropRootSignature,
}

pub struct DescriptorInfo {
    // imp: DescriptorInfoImpl,
    descriptor_type: u16,
    dim: u8,
    update_frequency: u8,
}

pub union RenderImpl {
    pub(crate) vk: ManuallyDrop<VulkanRenderer>,
}

pub struct Renderer {
    pub(crate) imp: RenderImpl,
    pub(crate) call: &'static MethodTable,
    pub(crate) me: Weak<Renderer>,
    pub(crate) api: RenderAPI,
}

pub union PipelineImpl {
    pub(crate) vk: ManuallyDrop<VulkanPipeline>,
    pub(crate) empty: ManuallyDrop<EmptyPipeline>,
}

pub struct Pipeline{
    pub(crate) imp: PipelineImpl,
    pub(crate) call: &'static MethodTable,
    pub(crate) renderer: Arc<Renderer>,
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        (self.call.remove_pipeline)(self);
    }
}

pub union RootSignatureImpl {
    pub(crate) vk: ManuallyDrop<VulkanRootSignature>,
    pub(crate) empty: ManuallyDrop<EmptyRootSignature>,
}

pub struct RootSignature {
    pub(crate) pipeline_type: PipelineType,
    pub(crate) imp: RootSignatureImpl,
    // pub(crate) renderer: Arc<Renderer>
}

pub union Queue {
    pub(crate) vk: ManuallyDrop<VulkanQueue>,
}

pub union Buffer {
    pub(crate) vk: ManuallyDrop<VulkanBuffer>,
}

pub union Texture {
    pub(crate) vk: ManuallyDrop<VulkanTexture>,
}

pub union RenderTarget {
    pub(crate) vk: ManuallyDrop<VulkanRenderTarget>,
}

pub union ShaderImpl {
    pub(crate) vk: ManuallyDrop<VulkanShader>,
    pub(crate) empty: ManuallyDrop<EmptyShader>,
}

pub struct Shader {
    pub(crate) imp: ShaderImpl,
}

pub union SamplerImpl {
    pub(crate) vk: ManuallyDrop<VulkanSampler>,
}

pub struct Sampler {
    pub(crate) imp: SamplerImpl,
}

pub union DescriptorSet {
    pub(crate) vk: ManuallyDrop<VulkanDescriptorSet>,
}

pub enum PipelineType {
    PipelineNone,
    PipelineCompute,
    PipelineGraphics,
    PipelineRaytrace,
}

#[derive(Debug, Copy, Clone)]
pub enum RenderAPI {
    RendererNone,
    RendererEmpty,
    RendererMetal,
    RendererVulkan,
}

static mut RENDER_API: RenderAPI = RenderAPI::RendererNone;

pub fn get_render_api() -> RenderAPI {
    unsafe { RENDER_API }
}

pub const ENABLED_RENDER_API: &[RenderAPI] = &[RendererVulkan];

fn init_render_api(api: &RenderAPI, config: &RendererConfig) -> HalResult<Arc<Renderer>> {
    match api {
        VulkanRenderer => {
            init_vulkan_renderer(&config);
        }
    }
    Err(UnSupportedAPI)
}



impl Renderer {
    pub fn init_renderer(config: &RendererConfig) -> HalResult<Arc<Renderer>> {
        for api in ENABLED_RENDER_API {
            return match init_render_api(api, config) {
                Ok(result) => {
                    unsafe { RENDER_API = *api };
                    Ok(result)
                }
                Err(e) => Err(e),
            };
        }
        Err(UnsupportedDevice)
    }

    pub fn add_pipeline(&self, config: &PipelineConfig) -> Pipeline {
        (self.call.add_pipeline)(self, config)
    }
}
