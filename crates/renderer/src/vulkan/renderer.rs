use crate::vulkan::{VulkanPipeline, VulkanRenderer};
use crate::{Renderer, RendererResult, VulkanAPI};

impl Renderer<VulkanAPI> for VulkanRenderer {
    fn init() -> RendererResult<Self> {
        todo!()
    }

    fn add_pipeline() -> VulkanPipeline {
        todo!()
    }

    fn remove_pipeline(pipeline: &mut VulkanPipeline) {
        todo!()
    }
}
