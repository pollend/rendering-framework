#![feature(associated_type_defaults)]

use std::ffi::CStr;
use std::rc::Rc;
use std::sync::Arc;
use crate::desc::{CmdPoolDesc, RenderDesc};
use crate::error::RendererResult;
use crate::vulkan::VulkanAPI;

mod vulkan;
mod types;
mod error;
mod desc;

pub mod ffi {
    pub use vulkan_sys as vk;
}

pub struct GPUCommonInfo {

}

pub enum APIType {
    None,
    Vulkan
}

pub trait Api: Clone + Sized {
    type RenderContext: RenderContext;
    type Renderer: Renderer<Self>;
    type Pipeline: Pipeline;
    type Fence: Fence;
    type Semaphore: Semaphore;
    type Queue: Queue;
    type Texture: Texture;
    type Shader: Shader;
    type RenderTarget: RenderTarget;
    type DescriptorIndexMap: DescriptorIndexMap;
    type Sampler: Sampler;

    const CURRENT_API: APIType;
}

pub trait Renderer<A: Api> : Sized {
    fn init(name: &CStr, desc: &RenderDesc) -> RendererResult<A::Renderer>;

    fn add_pipeline(&self) -> A::Pipeline;
    fn drop_pipeline(&self,pipeline: &mut A::Pipeline);

    fn add_fence(&self) -> RendererResult<A::Fence>;
    fn drop_fence(&self, fence: &mut A::Fence);

    // semaphore
    fn add_semaphore(&self) -> RendererResult<A::Semaphore>;
    fn drop_semaphore(&self, semaphore: &mut A::Semaphore);

    fn add_swap_chain(&self);
    fn drop_swap_chain(&self);

    // command pool functions
    fn add_cmd_pool(&self, desc: &CmdPoolDesc<A>);
    fn drop_cmd_pool(&self);
    fn add_cmd(&self);
    fn drop_cmd(&self);

    fn add_render_target(&self) -> RendererResult<A::RenderTarget>;
    fn remove_render_target(&self, target: &mut A::RenderTarget);

    fn add_root_signature(&self);
    fn remove_root_signature();

    // command buffer functions
    fn reset_cmd_pool(&self);
    fn begin_cmd(&self);
    fn end_cmd(&self);
}

pub trait RenderContext {
    // fn info(&self) -> &GPUCommonInfo;

}

pub trait Texture {

}

pub trait  Shader {

}

pub trait Queue {

}

pub trait Sampler {

}

pub trait DescriptorIndexMap {

}

pub trait RenderTarget {

}

pub trait Semaphore {

}

pub trait Fence {

}

pub trait Pipeline {

}

// struct Example<A: Api> {
//     render: A::Renderer
// }

// impl<A: Api> Example<A> {
//     fn init() {
//         match A::api {
//             APIType::Vulkan => {

//             }
//             _ => {}
//         }
//     }
// }

// pub fn example() {
//     let a: Example<VulkanAPI>;
// }

