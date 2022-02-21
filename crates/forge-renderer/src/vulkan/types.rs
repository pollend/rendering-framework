use crate::{
    types::{
        AddressMode, CompareMode, DescriptorType, FilterType, MipMapMode, QueueType,
        ResourceMemoryUsage, SampleCount,
    },
};
use bitflags::bitflags;
use std::ffi::CStr;
use lazy_static::lazy_static;

#[macro_export]
macro_rules! check_vk_result {
    ($x:expr) => {{
        let result = $x;
        // if result != ffi::vk::VkResult_VK_SUCCESS {
        //     error!("{}: Failed with VKResult: {}", stringify!($x), result);
        //     assert!(false);
        // }
    }};
}

impl QueueType {
    pub fn to_vk_queue(&self) -> ash::vk::QueueFlags {
        match self {
            QueueType::QueueTypeGraphics => ash::vk::QueueFlags::GRAPHICS,
            QueueType::QueueTypeTransfer => ash::vk::QueueFlags::TRANSFER,
            QueueType::QueueTypeCompute => ash::vk::QueueFlags::COMPUTE,
            _ => {
                assert!(false, "invalid Queue Type");
                ash::vk::QueueFlags::GRAPHICS
                    | ash::vk::QueueFlags::TRANSFER
                    | ash::vk::QueueFlags::COMPUTE
            }
        }
    }
}

impl CompareMode {
    pub fn to_comparison_vk(&self) -> ash::vk::CompareOp {
        match self {
            CompareMode::Never => ash::vk::CompareOp::NEVER,
            CompareMode::Less => ash::vk::CompareOp::LESS,
            CompareMode::Equal => ash::vk::CompareOp::EQUAL,
            CompareMode::LeEqual => ash::vk::CompareOp::LESS_OR_EQUAL,
            CompareMode::Greater => ash::vk::CompareOp::GREATER,
            CompareMode::NotEqual => ash::vk::CompareOp::NOT_EQUAL,
            CompareMode::GeEqual => ash::vk::CompareOp::GREATER_OR_EQUAL,
            CompareMode::Always => ash::vk::CompareOp::ALWAYS,
        }
    }
}

impl FilterType {
    pub fn to_vk_filter(&self) -> ash::vk::Filter {
        match self {
            FilterType::Nearest => ash::vk::Filter::NEAREST,
            FilterType::Linear => ash::vk::Filter::LINEAR,
        }
    }
}

impl MipMapMode {
    pub fn to_vk_map_map_mode(&self) -> ash::vk::SamplerMipmapMode {
        match self {
            MipMapMode::Nearest => ash::vk::SamplerMipmapMode::NEAREST,
            MipMapMode::Linear => ash::vk::SamplerMipmapMode::LINEAR,
        }
    }
}

impl DescriptorType {
    pub fn to_vk_usage(&self) -> ash::vk::ImageUsageFlags {
        let mut result: ash::vk::ImageUsageFlags = ash::vk::ImageUsageFlags::empty();
        if self.contains(DescriptorType::DESCRIPTOR_TYPE_TEXTURE) {
            result |= ash::vk::ImageUsageFlags::SAMPLED;
        }
        if self.contains(DescriptorType::DESCRIPTOR_TYPE_RW_TEXTURE) {
            result |= ash::vk::ImageUsageFlags::STORAGE;
        }
        return result;
    }
}

impl SampleCount {
    pub fn to_vk_sample_count(&self) -> ash::vk::SampleCountFlags {
        match self {
            SampleCount::SampleCount1 => ash::vk::SampleCountFlags::TYPE_1,
            SampleCount::SampleCount2 => ash::vk::SampleCountFlags::TYPE_2,
            SampleCount::SampleCount4 => ash::vk::SampleCountFlags::TYPE_4,
            SampleCount::SampleCount8 => ash::vk::SampleCountFlags::TYPE_8,
            SampleCount::SampleCount16 => ash::vk::SampleCountFlags::TYPE_16,
            _ => ash::vk::SampleCountFlags::TYPE_1,
        }
    }
}

impl AddressMode {
    pub fn to_vk_address_mode(&self) -> ash::vk::SamplerAddressMode {
        match self {
            AddressMode::AddressModeMirror => ash::vk::SamplerAddressMode::MIRRORED_REPEAT,
            AddressMode::AddressModeRepeat => ash::vk::SamplerAddressMode::REPEAT,
            AddressMode::AddressModeClampToEdge => ash::vk::SamplerAddressMode::CLAMP_TO_EDGE,
            AddressMode::AddressModeClampToBorder => ash::vk::SamplerAddressMode::CLAMP_TO_BORDER,
        }
    }
}

impl DescriptorType {
    pub fn to_vk_buffer_usage(&self, typed: bool) -> ash::vk::BufferUsageFlags {
        let mut result: ash::vk::BufferUsageFlags = ash::vk::BufferUsageFlags::TRANSFER_SRC;

        if self.contains(DescriptorType::DESCRIPTOR_TYPE_UNIFORM_BUFFER) {
            result |= ash::vk::BufferUsageFlags::UNIFORM_BUFFER;
        }
        if self.contains(DescriptorType::DESCRIPTOR_TYPE_RW_BUFFER) {
            result |= ash::vk::BufferUsageFlags::STORAGE_BUFFER;
            if typed {
                result |= ash::vk::BufferUsageFlags::STORAGE_TEXEL_BUFFER;
            }
        }
        if self.contains(DescriptorType::DESCRIPTOR_TYPE_BUFFER) {
            result |= ash::vk::BufferUsageFlags::STORAGE_BUFFER;
            if typed {
                result |= ash::vk::BufferUsageFlags::UNIFORM_TEXEL_BUFFER;
            }
        }

        if self.contains(DescriptorType::DESCRIPTOR_TYPE_INDEX_BUFFER) {
            result |= ash::vk::BufferUsageFlags::INDEX_BUFFER;
        }

        if self.contains(DescriptorType::DESCRIPTOR_TYPE_VERTEX_BUFFER) {
            result |= ash::vk::BufferUsageFlags::VERTEX_BUFFER;
        }

        if self.contains(DescriptorType::DESCRIPTOR_TYPE_INDIRECT_BUFFER) {
            result |= ash::vk::BufferUsageFlags::INDIRECT_BUFFER;
        }
        // #ifdef ENABLE_RAYTRACING
        // if (usage & DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE)
        // {
        //     result |= VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR;
        // }
        // if (usage & DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_BUILD_INPUT)
        // {
        //     result |= VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR;
        // }
        // if (usage & DESCRIPTOR_TYPE_SHADER_DEVICE_ADDRESS)
        // {
        //     result |= VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT;
        // }
        // if (usage & DESCRIPTOR_TYPE_SHADER_BINDING_TABLE)
        // {
        //     result |= VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR;
        // }
        // #endif
        return result;
    }
}

bitflags! {
    pub struct VulkanSupportedFeatures: u32 {
        const NONE = 0x0;
        const RAY_TRACING_SUPPORTED = 0x01;
        const YCBCR_EXTENSION = 0x02;
        const KHR_SPIRV_14_EXTENSION = 0x04;
        const KHR_ACCELERATION_STRUCTURE_EXTENSION = 0x08;
        const KHR_RAY_TRACING_PIPELINE_EXTENSION = 0x10;
        const KHR_RAY_QUERY_EXTENSION = 0x20;
        const AMD_GCN_SHADEREXTENSION = 0x40;
        const AMD_DRAW_INDIRECT_COUNT_EXTENSION = 0x80;
        const DESCRIPTOR_INDEXING_EXTENSION = 0x100;
        const SHADER_FLOAT_CONTROLS_EXTENSION = 0x200;
        const BUFFER_DEVICE_ADDRESS_EXTENSION = 0x400;
        const DEFERRED_HOST_OPERATIONS_EXTENSION = 0x800;
        const DRAW_INDIRECT_COUNT_EXTENSION = 0x1000;
        const DEDICATED_ALLOCATION_EXTENSION = 0x2000;
        const EXTERNAL_MEMORY_EXTENSION = 0x4000;
        const DEBUG_MARKER_SUPPORT = 0x8000;
    }
}


lazy_static! {
    pub static ref MAX_QUEUE_FLAGS: ash::vk::QueueFlags = {
        (ash::vk::QueueFlags::GRAPHICS
            | ash::vk::QueueFlags::COMPUTE
            | ash::vk::QueueFlags::TRANSFER
            | ash::vk::QueueFlags::SPARSE_BINDING
            | ash::vk::QueueFlags::PROTECTED)
    };

    pub static ref GLOBAL_INSTANCE_EXTENSIONS: Vec<&'static CStr> = {
        Vec::from([
            ash::vk::KhrSurfaceProtectedCapabilitiesFn::name(),
            ash::vk::KhrXlibSurfaceFn::name(),
            // #[cfg(feature = "vulkan_sys/vulkan-xlib")]
            // ffi::vk::VK_KHR_XLIB_SURFACE_EXTENSION_NAME,
            ash::vk::NvExternalMemoryCapabilitiesFn::name(),
            // To legally use HDR formats
            ash::vk::ExtSwapchainColorspaceFn::name(),
            //
            // #[cfg(feature = "vulkan_sys/vulkan-win32")]
            // ffi::vk::VK_KHR_WIN32_SURFACE_EXTENSION_NAME,
            // #[cfg(feature = "vulkan_sys/vulkan-ggp")]
            // ffi::vk::VK_GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME,
            // #[cfg(feature = "vulkan_sys/vulkan-vi")]
            // ffi::vk::VK_NN_VI_SURFACE_EXTENSION_NAME,
            // // #ifdef ENABLE_DEBUG_UTILS_EXTENSION
            // // VK_EXT_DEBUG_UTILS_EXTENSION_NAME,
            // // #else
            // // VK_EXT_DEBUG_REPORT_EXTENSION_NAME,
            // // #endif
            // ffi::vk::VK_NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME,
            // // To legally use HDR formats
            // ffi::vk::VK_EXT_SWAPCHAIN_COLOR_SPACE_EXTENSION_NAME,
            // // /************************************************************************/
            // // // Multi GPU Extensions
            // // /************************************************************************/
            // // #if VK_KHR_device_group_creation
            // // VK_KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME,
            // // #endif
            // /************************************************************************/
            // // VR Extensions
            // /************************************************************************/
            // ffi::vk::VK_KHR_DISPLAY_EXTENSION_NAME,
            // ffi::vk::VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME,
        ])
    };

    pub static ref GLOBAL_WANTED_DEVICE_EXTENSIONS: Vec<&'static CStr> = {
          Vec::from([ash::extensions::khr::Swapchain::name(),
            ash::extensions::khr::Maintenance1::name(),
            ash::vk::KhrShaderDrawParametersFn::name(),
            ash::vk::ExtShaderSubgroupBallotFn::name(),
            ash::vk::ExtShaderSubgroupVoteFn::name(),
            ash::vk::KhrDedicatedAllocationFn::name(),
            ash::vk::KhrGetMemoryRequirements2Fn::name(),
            // Fragment shader interlock extension to be used for ROV type functionality in Vulkan
            ash::vk::ExtFragmentShaderInterlockFn::name(),
            /************************************************************************/
            // AMD Specific Extensions
            /************************************************************************/
            ash::vk::AmdDrawIndirectCountFn::name(),
            ash::vk::AmdShaderBallotFn::name(),
            ash::vk::AmdGcnShaderFn::name(),
            /************************************************************************/
            // Multi GPU Extensions
            /************************************************************************/
            ash::vk::KhrDeviceGroupFn::name(),
            /************************************************************************/
            // Bindless & None Uniform access Extensions
            /************************************************************************/
            ash::vk::ExtDescriptorIndexingFn::name(),
            /************************************************************************/
            // Raytracing
            /************************************************************************/
            // #ifdef ENABLE_RAYTRACING
            //     ffi::vk::VK_KHR_SHADER_FLOAT_CONTROLS_EXTENSION_NAME,
            //     ffi::vk::VK_KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME,
            //     ffi::vk::VK_KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME,
            //
            //     ffi::vk::VK_KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME,
            //     ffi::vk::VK_KHR_SPIRV_1_4_EXTENSION_NAME,
            //     ffi::vk::VK_KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME,
            //
            //     ffi::vk::VK_KHR_RAY_QUERY_EXTENSION_NAME,
            // #endif

            /************************************************************************/
            // YCbCr format support
            /************************************************************************/
            ash::vk::KhrBindMemory2Fn::name(),
            ash::vk::KhrSamplerYcbcrConversionFn::name(),
        ])
    };
}

// pub const VSYNC_PREFERRED_MODE: &[ffi::vk::VkPresentModeKHR] = &[
//     ffi::vk::VkPresentModeKHR_VK_PRESENT_MODE_FIFO_RELAXED_KHR,
//     ffi::vk::VkPresentModeKHR_VK_PRESENT_MODE_FIFO_KHR,
// ];

pub const MAX_QUEUE_FAMILIES: u32 = 16;
pub const MAX_QUEUE_COUNT: u32 = 64;
