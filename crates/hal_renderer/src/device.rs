// describes device features and supported implementations?

use crate::error::HalError::Unhandled;
use crate::error::HalResult;
use crate::renderer::{Renderer, RendererType};
use crate::vulkan::*;

// pub enum GPUDetailType {
//     Vulkan(HalVkGPUDetail)
// }

/**
* Implementation details
**/
pub struct GPUDetail {
    // pub imp: GPUDetailType
}

impl GPUDetail {
    // fn all(renderer: &mut Renderer) -> HalResult<Vec<GPUDetail>> {
    //     match &renderer.imp {
    //         RendererType::Vulkan(rend) => Ok(VulkanGPUDetail::all_gpus(&rend)),
    //         _ => Err(Unhandled)
    //     }
    // }
}


