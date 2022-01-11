use std::ffi::{CStr, CString};

// pub union DescImpl {
//     vulkan: VulkanDesc
// }


pub struct VulkanDesc {
    ppInstanceLayers: Vec<CString>,
    ppInstanceExtensions: Vec<CString>,
    ppDeviceExtensions: Vec<CString>,
}

pub enum RenderDescImp {
    Vulkan(VulkanDesc)
}
pub struct RenderDesc {
    pub imp: RenderDescImp
}


trait Descriptors {
    type RenderDesc;
}
