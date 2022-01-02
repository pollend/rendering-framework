use crate::configuration::*;
use crate::error::{HalError, HalResult};
use crate::vulkan::*;
use std::rc::{Rc, Weak};

pub enum RendererType {
    Vulkan(HalVKRenderer),
}

pub struct Renderer {
    me: Weak<Renderer>,
    imp: RendererType,
}

impl Renderer {
    // Renderer::Vulkan(HalVKRenderer::new(&config)?)
    pub fn new(name: &'static str, config: &RendererConfig) -> HalResult<Rc<Renderer>> {
        match &config.render_type {
            RendererConfigType::Vulkan(_) => Ok({
                let imp = HalVKRenderer::new(&config)?;
                Rc::new_cyclic(|self_weak| Renderer {
                    imp: RendererType::Vulkan(imp),
                    me: self_weak.clone(),
                })
            }),
            _ => Err(HalError::Unhandled),
        }

        // Renderer::Vulkan(
    }

    // fn add_pipeline(self, config: &PipelineConfig) -> HalResult<Pipeline> {
    //     match &self {
    //         Renderer::Vulkan(renderer) => Ok(Pipeline::Vulkan(HalVKPipeline::new(&renderer, &config)?))
    //     }
    // }
}

pub enum Pipeline {
    Vulkan(Rc<HalVKPipeline>),
}

pub enum Queue {
    Vulkan(HalVKQueue),
}

pub enum Buffer {
    Vulkan(HalVKBuffer),
}

pub enum Texture {
    Vulkan(HalVKTexture),
}

pub enum RenderTarget {
    Vulkan(HalVKRenderTarget),
}

pub enum Shader {
    Vulkan(HalVKShader),
}

pub enum DescriptorSet {
    Vulkan(HalVKDescriptorSet),
}

pub enum PipelineType {
    PipelineCompute,
    PipelineGraphics,
    PipelineRaytrace,
}

fn test() {
    // let rend = Renderer::new(BackendType::Vulkan);
    // let mv = rend;
    // let desc = PipelineDesc {
    //     pipeline: PipelineDescImpl::ComputeDesc(ComputePipelineDesc {

    //     }),
    //     name: "test"
    // };
    // rend.add_pipeline(&desc);
}

// pub fn add_pipeline(pipeline: &Pipeline) {
//     match pipeline {
//         Pipeline::Vulkan(pipeline) => {}
//         _ => { panic!("unkown pipeline")}
//     }
// }
