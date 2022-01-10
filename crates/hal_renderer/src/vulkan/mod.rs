pub mod device;
pub mod pipeline;
pub mod renderer;
pub mod shader;
pub mod types;

use crate::{configuration::{ComputePipelineType, RendererConfig, RootSignatureConfig}, error::HalResult, ffi, renderer::{
    MethodTable, PipelineType, RenderAPI, RenderAPI::RendererVulkan, RenderImpl, Renderer,
    RootSignature, RootSignatureImpl,
}};
pub use device::*;
pub use pipeline::*;
pub use renderer::*;
use std::{mem::ManuallyDrop, ptr, sync::Arc};
pub use types::*;
use crate::configuration::PipelineConfig;
use crate::renderer::{Pipeline, PipelineImpl};

const VK_TABLE: MethodTable = MethodTable {
    add_pipeline: vk_new_pipeline,
    remove_pipeline: vk_drop_pipeline,
    add_root_signature: vk_new_root_signature,
    remove_root_signature: vk_remove_root_signature
};

pub fn init_vulkan_renderer(config: &RendererConfig) -> HalResult<Arc<Renderer>> {
    match VulkanRenderer::new(config) {
        Ok(vk) => Ok(Arc::new_cyclic(|me| Renderer {
            imp: RenderImpl {
                vk: ManuallyDrop::new(vk),
            },
            me: me.clone(),
            api: RenderAPI::RendererVulkan,
            call: &VK_TABLE,
        })),

        Err(e) => Err(e),
    }
}

pub fn vk_remove_root_signature(signature: &mut RootSignature) {

}

pub fn vk_new_root_signature(render: &Renderer, config: &RootSignatureConfig) -> RootSignature {
    todo!()
}

pub fn vk_new_pipeline(render: &Renderer, configuration: &PipelineConfig) -> Pipeline {
    return Pipeline {
        imp: PipelineImpl {
            vk: ManuallyDrop::new(VulkanPipeline {
                pipeline: ptr::null_mut(),
            }),
        },
        call: render.call,
        renderer: match render.me.upgrade() {
            None => panic!("unable to promote"),
            Some(r) => r
        }
    };
}

pub fn vk_drop_pipeline(pipeline: &mut Pipeline) {
    unsafe {
        assert!(pipeline.renderer.imp.vk.device != ptr::null_mut());
        assert!(pipeline.imp.vk.pipeline != ptr::null_mut());

        ffi::vk::vkDestroyPipeline(
            pipeline.renderer.imp.vk.device,
            pipeline.imp.vk.pipeline,
            ptr::null_mut(),
        );
        ManuallyDrop::drop(&mut pipeline.imp.vk);
    }
}


fn vk_add_root_signature(render: &mut Renderer, config: &RootSignatureConfig) -> RootSignature {
    let pipeline_type = PipelineType::PipelineNone;

    RootSignature {
        imp: RootSignatureImpl {
            vk: ManuallyDrop::new(VulkanRootSignature {
                pipeline_layout: ptr::null_mut(),
            }),
        },
        pipeline_type: PipelineType::PipelineCompute,
    }
}

fn vk_drop_root_signature(render: &mut Renderer, signature: &mut RootSignature) {}

// pub fn vk_add_pipeline {
//
// }
