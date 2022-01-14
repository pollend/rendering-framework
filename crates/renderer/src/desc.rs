use std::ffi::{CStr, CString};
use crate::Api;
use crate::types::{QueueFlag, QueueType};

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

pub struct  QueueDesc {
    pub queue_type: QueueType,
    pub flag: QueueFlag,
    pub priority: QueuePriority,
    pub node_index: u32
}
