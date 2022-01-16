use bitflags::bitflags;
use crate::{ffi, types::QueueType};

impl QueueType {
    pub fn to_vk_queue(&self) -> ffi::vk::VkQueueFlagBits {
        match self {
            QueueType::QueueTypeGraphics => ffi::vk::VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT,
            QueueType::QueueTypeTransfer => ffi::vk::VkQueueFlagBits_VK_QUEUE_TRANSFER_BIT,
            QueueType::QueueTypeCompute => ffi::vk::VkQueueFlagBits_VK_QUEUE_COMPUTE_BIT,
            _ => {
                assert!(false, "invalid Queue Type");
                ffi::vk::VkQueueFlagBits_VK_QUEUE_FLAG_BITS_MAX_ENUM
            }
        }
    }
}

bitflags! {
    pub struct VulkanSupportedFeatures: u32 {
        const None = 0x0;
        const RaytracingSupported = 0x01;
        const YCbCrExtension = 0x02;
        const KHRSpirv14Extension = 0x04;
        const KHRAccelerationStructureExtension = 0x08;
        const KHRRayTracingPipelineExtension = 0x10;
        const KHRRayQueryExtension = 0x20;
        const AMDGCNShaderExtension = 0x40;
        const AMDDrawIndirectCountExtension = 0x80;
        const DescriptorIndexingExtension = 0x100;
        const ShaderFloatControlsExtension = 0x200;
        const BufferDeviceAddressExtension = 0x400;
        const DeferredHostOperationsExtension = 0x800;
        const DrawIndirectCountExtension = 0x1000;
        const DedicatedAllocationExtension = 0x2000;
        const ExternalMemoryExtension = 0x4000;
        const DebugMarkerSupport = 0x8000;
    }
}


pub const GLOBAL_INSTANCE_EXTENSIONS: &[&[u8]] = &[
    ffi::vk::VK_KHR_SURFACE_EXTENSION_NAME,
    #[cfg(feature = "vulkan_sys/vulkan-win32")]
        ffi::vk::VK_KHR_WIN32_SURFACE_EXTENSION_NAME,
    #[cfg(feature = "vulkan_sys/vulkan-xlib")]
        ffi::vk::VK_KHR_XLIB_SURFACE_EXTENSION_NAME,
    #[cfg(feature = "vulkan_sys/vulkan-ggp")]
        ffi::vk::VK_GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME,
    #[cfg(feature = "vulkan_sys/vulkan-vi")]
        ffi::vk::VK_NN_VI_SURFACE_EXTENSION_NAME,
    // #ifdef ENABLE_DEBUG_UTILS_EXTENSION
    // VK_EXT_DEBUG_UTILS_EXTENSION_NAME,
    // #else
    // VK_EXT_DEBUG_REPORT_EXTENSION_NAME,
    // #endif
    ffi::vk::VK_NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME,
    // To legally use HDR formats
    ffi::vk::VK_EXT_SWAPCHAIN_COLOR_SPACE_EXTENSION_NAME,
    // /************************************************************************/
    // // Multi GPU Extensions
    // /************************************************************************/
    // #if VK_KHR_device_group_creation
    // VK_KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME,
    // #endif
    /************************************************************************/
    // VR Extensions
    /************************************************************************/
    ffi::vk::VK_KHR_DISPLAY_EXTENSION_NAME,
    ffi::vk::VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME,
];
