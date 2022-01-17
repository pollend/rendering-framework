use crate::{
    types::{QueueFlag, QueuePriority, QueueType},
    Api,
};
use std::ffi::{CStr, CString};
use crate::types::DescriptorType;

// pub union DescImpl {
//     vulkan: VulkanDesc
// }

pub struct VulkanRenderDesc {
    pub instance_layers: Vec<CString>,
    pub instance_extensions: Vec<CString>,
    pub device_extensions: Vec<CString>,

    /// Flag to specify whether to request all queues from the gpu or just one of each type
    /// This will affect memory usage - Around 200 MB more used if all queues are requested
    pub request_all_available_queues: bool,
}

pub enum RenderDescImp {
    Vulkan(VulkanRenderDesc),
}

pub struct RenderDesc {
    pub imp: RenderDescImp,
}

pub struct CmdPoolDesc<'a, T: Api> {
    pub queue: &'a T::Queue,
    pub transient: bool,
}

pub struct BufferDesc {
    pub size: u64,

    pub debug_name: CString,

    // Flags specifying the suitable usage of this buffer (Uniform buffer, Vertex Buffer, Index Buffer,...)
    descriptors: DescriptorType
}

pub struct QueueDesc {
    pub queue_type: QueueType,
    pub flag: QueueFlag,
    pub priority: QueuePriority,
    pub node_index: u32,
}
