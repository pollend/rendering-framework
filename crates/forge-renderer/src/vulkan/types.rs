use crate::{
    ffi,
    types::{
        AddressMode, CompareMode, DescriptorType, FilterType, MipMapMode, QueueType,
        ResourceMemoryUsage, SampleCount,
    },
};
use bitflags::bitflags;

#[macro_export]
macro_rules! check_vk_result {
    ($x:expr) => {{
        let result = $x;
        if result != ffi::vk::VkResult_VK_SUCCESS {
            error!("{}: Failed with VKResult: {}", stringify!($x), result);
            assert!(false);
        }
    }};
}

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

impl CompareMode {
    pub fn to_comparison_vk(&self) -> u32 {
        match self {
            CompareMode::Never => ffi::vk::VkCompareOp_VK_COMPARE_OP_NEVER,
            CompareMode::Less => ffi::vk::VkCompareOp_VK_COMPARE_OP_LESS,
            CompareMode::Equal => ffi::vk::VkCompareOp_VK_COMPARE_OP_EQUAL,
            CompareMode::LeEqual => ffi::vk::VkCompareOp_VK_COMPARE_OP_LESS_OR_EQUAL,
            CompareMode::Greater => ffi::vk::VkCompareOp_VK_COMPARE_OP_GREATER,
            CompareMode::NotEqual => ffi::vk::VkCompareOp_VK_COMPARE_OP_NOT_EQUAL,
            CompareMode::GeEqual => ffi::vk::VkCompareOp_VK_COMPARE_OP_GREATER_OR_EQUAL,
            CompareMode::Always => ffi::vk::VkCompareOp_VK_COMPARE_OP_ALWAYS,
        }
    }
}

impl FilterType {
    pub fn to_vk_filter(&self) -> u32 {
        match self {
            FilterType::Nearest => ffi::vk::VkFilter_VK_FILTER_NEAREST,
            FilterType::Linear => ffi::vk::VkFilter_VK_FILTER_LINEAR,
        }
    }
}

impl MipMapMode {
    pub fn to_vk_map_map_mode(&self) -> u32 {
        match self {
            MipMapMode::Nearest => ffi::vk::VkSamplerMipmapMode_VK_SAMPLER_MIPMAP_MODE_NEAREST,
            MipMapMode::Linear => ffi::vk::VkSamplerMipmapMode_VK_SAMPLER_MIPMAP_MODE_LINEAR,
        }
    }
}

impl DescriptorType {
    pub fn to_vk_usage(&self) -> ffi::vk::VkImageUsageFlags {
        let mut result: ffi::vk::VkImageUsageFlags = 0;
        if self.contains(DescriptorType::DESCRIPTOR_TYPE_TEXTURE) {
            result |= ffi::vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_SAMPLED_BIT;
        }
        if self.contains(DescriptorType::DESCRIPTOR_TYPE_RW_TEXTURE) {
            result |= ffi::vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_STORAGE_BIT;
        }
        return result;
    }
}

impl SampleCount {
    pub fn to_vk_sample_count(&self) -> ffi::vk::VkSampleCountFlagBits {
        match self {
            SampleCount::SampleCount1 => ffi::vk::VkSampleCountFlagBits_VK_SAMPLE_COUNT_1_BIT,
            SampleCount::SampleCount2 => ffi::vk::VkSampleCountFlagBits_VK_SAMPLE_COUNT_2_BIT,
            SampleCount::SampleCount4 => ffi::vk::VkSampleCountFlagBits_VK_SAMPLE_COUNT_4_BIT,
            SampleCount::SampleCount8 => ffi::vk::VkSampleCountFlagBits_VK_SAMPLE_COUNT_8_BIT,
            SampleCount::SampleCount16 => ffi::vk::VkSampleCountFlagBits_VK_SAMPLE_COUNT_16_BIT,
            _ => ffi::vk::VkSampleCountFlagBits_VK_SAMPLE_COUNT_1_BIT,
        }
    }
}

impl ResourceMemoryUsage {
    pub fn to_vma_usage(&self) -> ffi::vk::VmaMemoryUsage {
        match self {
            ResourceMemoryUsage::Unknown => ffi::vk::VmaMemoryUsage_VMA_MEMORY_USAGE_UNKNOWN,
            ResourceMemoryUsage::GpuOnly => ffi::vk::VmaMemoryUsage_VMA_MEMORY_USAGE_GPU_ONLY,
            ResourceMemoryUsage::CpuOnly => ffi::vk::VmaMemoryUsage_VMA_MEMORY_USAGE_CPU_ONLY,
            ResourceMemoryUsage::CpuToGpu => ffi::vk::VmaMemoryUsage_VMA_MEMORY_USAGE_CPU_TO_GPU,
            ResourceMemoryUsage::GpuToCpu => ffi::vk::VmaMemoryUsage_VMA_MEMORY_USAGE_GPU_TO_CPU,
        }
    }
}

impl AddressMode {
    pub fn to_vk_address_mode(&self) -> u32 {
        match self {
            AddressMode::AddressModeMirror => {
                ffi::vk::VkSamplerAddressMode_VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT
            }
            AddressMode::AddressModeRepeat => {
                ffi::vk::VkSamplerAddressMode_VK_SAMPLER_ADDRESS_MODE_REPEAT
            }
            AddressMode::AddressModeClampToEdge => {
                ffi::vk::VkSamplerAddressMode_VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE
            }
            AddressMode::AddressModeClampToBorder => {
                ffi::vk::VkSamplerAddressMode_VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER
            }
        }
    }
}

impl DescriptorType {
    pub fn to_vk_buffer_usage(&self, typed: bool) -> ffi::vk::VkBufferUsageFlags {
        let mut result: ffi::vk::VkBufferUsageFlags =
            ffi::vk::VkBufferUsageFlagBits_VK_BUFFER_USAGE_TRANSFER_SRC_BIT;
        if self.contains(DescriptorType::DESCRIPTOR_TYPE_UNIFORM_BUFFER) {
            result |= ffi::vk::VkBufferUsageFlagBits_VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT;
        }
        if self.contains(DescriptorType::DESCRIPTOR_TYPE_RW_BUFFER) {
            result |= ffi::vk::VkBufferUsageFlagBits_VK_BUFFER_USAGE_STORAGE_BUFFER_BIT;
            if typed {
                result |= ffi::vk::VkBufferUsageFlagBits_VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT;
            }
        }
        if self.contains(DescriptorType::DESCRIPTOR_TYPE_BUFFER) {
            result |= ffi::vk::VkBufferUsageFlagBits_VK_BUFFER_USAGE_STORAGE_BUFFER_BIT;
            if typed {
                result |= ffi::vk::VkBufferUsageFlagBits_VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT;
            }
        }

        if self.contains(DescriptorType::DESCRIPTOR_TYPE_INDEX_BUFFER) {
            result |= ffi::vk::VkBufferUsageFlagBits_VK_BUFFER_USAGE_INDEX_BUFFER_BIT;
        }

        if self.contains(DescriptorType::DESCRIPTOR_TYPE_VERTEX_BUFFER) {
            result |= ffi::vk::VkBufferUsageFlagBits_VK_BUFFER_USAGE_VERTEX_BUFFER_BIT;
        }

        if self.contains(DescriptorType::DESCRIPTOR_TYPE_INDIRECT_BUFFER) {
            result |= ffi::vk::VkBufferUsageFlagBits_VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT;
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

pub const GLOBAL_WANTED_DEVICE_EXTENSIONS: &[&[u8]] = &[
    ffi::vk::VK_KHR_SWAPCHAIN_EXTENSION_NAME,
    ffi::vk::VK_KHR_MAINTENANCE1_EXTENSION_NAME,
    ffi::vk::VK_KHR_SHADER_DRAW_PARAMETERS_EXTENSION_NAME,
    ffi::vk::VK_EXT_SHADER_SUBGROUP_BALLOT_EXTENSION_NAME,
    ffi::vk::VK_EXT_SHADER_SUBGROUP_VOTE_EXTENSION_NAME,
    ffi::vk::VK_KHR_DEDICATED_ALLOCATION_EXTENSION_NAME,
    ffi::vk::VK_KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME,
    // Fragment shader interlock extension to be used for ROV type functionality in Vulkan
    ffi::vk::VK_EXT_FRAGMENT_SHADER_INTERLOCK_EXTENSION_NAME,
    /************************************************************************/
    // AMD Specific Extensions
    /************************************************************************/
    ffi::vk::VK_AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME,
    ffi::vk::VK_AMD_SHADER_BALLOT_EXTENSION_NAME,
    ffi::vk::VK_AMD_GCN_SHADER_EXTENSION_NAME,
    /************************************************************************/
    // Multi GPU Extensions
    /************************************************************************/
    ffi::vk::VK_KHR_DEVICE_GROUP_EXTENSION_NAME,
    /************************************************************************/
    // Bindless & None Uniform access Extensions
    /************************************************************************/
    ffi::vk::VK_EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME,
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
    ffi::vk::VK_KHR_BIND_MEMORY_2_EXTENSION_NAME,
    ffi::vk::VK_KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME,
    ffi::vk::VK_KHR_BIND_MEMORY_2_EXTENSION_NAME,
];

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

pub const MAX_QUEUE_FLAGS: u32 = ffi::vk::VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT
    | ffi::vk::VkQueueFlagBits_VK_QUEUE_COMPUTE_BIT
    | ffi::vk::VkQueueFlagBits_VK_QUEUE_TRANSFER_BIT
    | ffi::vk::VkQueueFlagBits_VK_QUEUE_SPARSE_BINDING_BIT
    | ffi::vk::VkQueueFlagBits_VK_QUEUE_PROTECTED_BIT;

pub const VSYNC_PREFERRED_MODE: &[ffi::vk::VkPresentModeKHR] = &[
    ffi::vk::VkPresentModeKHR_VK_PRESENT_MODE_FIFO_RELAXED_KHR,
    ffi::vk::VkPresentModeKHR_VK_PRESENT_MODE_FIFO_KHR,
];

pub const MAX_QUEUE_FAMILIES: u32 = 16;
pub const MAX_QUEUE_COUNT: u32 = 64;
