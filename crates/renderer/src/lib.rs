use std::rc::Rc;
use std::sync::Arc;
use crate::error::RendererResult;
use crate::vulkan::VulkanAPI;

mod vulkan;
mod types;
mod error;
mod desc;

pub mod ffi {
    pub use vulkan_sys as vk;
}


pub enum APIType {
    None,
    Vulkan
}


trait Api: Clone + Sized{
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

    const api: APIType;
}

trait Renderer<A: Api> : Sized {
    fn init() -> RendererResult<A::Renderer>;

    fn add_pipeline() -> A::Pipeline;
    fn remove_pipeline(pipeline: &mut A::Pipeline);
}

trait Texture {

}

trait  Shader {

}

trait Queue {

}

trait Sampler {

}

trait DescriptorIndexMap {

}

trait RenderTarget {

}

trait Semaphore {

}

trait Fence {

}

trait Pipeline {

}

struct Example<A: Api> {
    render: A::Renderer
}

impl<A: Api> Example<A> {
    fn init() {
        match A::api {
            APIType::Vulkan => {

            }
            _ => {}
        }
    }
}

pub fn example() {
    let a: Example<VulkanAPI>;
}

