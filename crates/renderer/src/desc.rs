use std::ffi::{CStr, CString};
use crate::Api;

// pub union DescImpl {
//     vulkan: VulkanDesc
// }


pub struct VulkanRenderDesc {
    pub(crate) instance_layers: Vec<CString>,
    pub(crate) instance_extensions: Vec<CString>,
    device_extensions: Vec<CString>,
}

pub enum RenderDescImp {
    Vulkan(VulkanRenderDesc)
}

pub struct RenderDesc {
    pub imp: RenderDescImp
}

pub struct CmdPoolDesc<'a, T: Api> {
    pub queue: &'a T::Queue,
    pub transient: bool
}


