// describes device features and supported implementations?

use crate::{
    error::{HalError::Unhandled, HalResult},
    renderer::Renderer,
    vulkan::*,
};
use std::ffi::CString;

// pub enum GPUDetailType {
//     Vulkan(HalVkGPUDetail)
// }

pub struct GPUVendorInfo {
    vendor_id: Box<str>,
    model_id: Box<str>,
}

/**
* Implementation details
**/
pub struct GPUCommonInfo {
    // pub imp: GPUDetailType
    vendorPresets: GPUVendorInfo,
}

// impl GPUDetail {
//     // fn all(renderer: &mut Renderer) -> HalResult<Vec<GPUDetail>> {
//     //     match &renderer.imp {
//     //         RendererType::Vulkan(rend) => Ok(VulkanGPUDetail::all_gpus(&rend)),
//     //         _ => Err(Unhandled)
//     //     }
//     // }
// }
