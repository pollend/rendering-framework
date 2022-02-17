use crate::{check_vk_result, desc::{CmdDesc, RenderDescImp}, error::{RendererError, RendererError::VulkanError}, ffi, ffi::vk, types::{
    BufferCreationFlag, CompareMode, DescriptorType, FilterType, MipMapMode, QueueType,
    ResourceMemoryUsage, ResourceState, SampleCount, TextureCreationFlags,
}, vulkan::{
    device::VulkanGPUInfo,
    types::{
        VulkanSupportedFeatures, GLOBAL_INSTANCE_EXTENSIONS, GLOBAL_WANTED_DEVICE_EXTENSIONS,
        MAX_QUEUE_COUNT, MAX_QUEUE_FAMILIES, MAX_QUEUE_FLAGS,
    },
    VulkanCommand, VulkanCommandPool, VulkanPipeline, VulkanQueue,
    VulkanRenderTarget, VulkanRenderer, VulkanSampler, VulkanSemaphore, VulkanSwapChain,
    VulkanTexture,
}, BufferDesc, CmdPoolDesc, GPUCommonInfo, QueueDesc, RenderDesc, RenderTargetDesc, Renderer, RendererResult, RootSignatureDesc, SamplerDesc, SwapChainDesc, TextureDesc, VulkanAPI, BinaryShaderDesc};
use forge_image_format::{ImageFormat, ImageFormat::UNDEFINED};
use forge_math::round_up;
use log::{error, info, log, warn};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::{
    borrow::{Borrow, BorrowMut},
    cmp::{max, min, Ordering},
    collections::HashSet,
    ffi::{c_void, CStr, CString},
    mem,
    mem::MaybeUninit,
    os::raw::c_char,
    ptr,
    sync::{Arc, Mutex},
    thread::sleep,
    u32,
};
use std::ops::DerefMut;
use ash::{Entry, Instance};
use ash::vk::{BufferUsageFlags, ColorSpaceKHR, DeviceMemory, DeviceSize, FormatFeatureFlags, ImageViewCreateInfo, SharingMode, SurfaceFormatKHR};
use gpu_allocator::vulkan::{Allocator, AllocatorCreateDesc};
use winit::event::VirtualKeyCode::N;
use crate::desc::BinaryShaderStageDesc;
use crate::types::{ShaderStage, ShaderStageFlags};
use crate::vulkan::buffer::VulkanBuffer;
use crate::vulkan::{VulkanRootSignature, VulkanShader};
use gpu_allocator::vulkan::*;
use gpu_allocator::MemoryLocation;

use spirv_cross::{spirv, glsl};
use spirv_cross::spirv::{Ast, ShaderResources};

struct MemoryRequirementResults<'a> {
    requirement: ash::vk::MemoryRequirementsBuilder<'a>,
    planar_offset: Vec<u64>,
}


fn util_get_planar_vk_image_memory_requirement(
    device: &ash::Device,
    image: ash::vk::Image,
    num_planes: u32,
) -> MemoryRequirementResults {
    let mut results = MemoryRequirementResults {
        requirement: ash::vk::MemoryRequirements::builder(),
        planar_offset: vec![],
    };

    let mut mem_dedicated_req = ash::vk::MemoryDedicatedRequirements::builder();
    let mut mem_req_2 =
        ash::vk::MemoryRequirements2::builder()
            .push_next(&mut mem_dedicated_req);

    let mut image_plane_mem_req_info = ash::vk::ImagePlaneMemoryRequirementsInfo::builder();
    let mut image_plane_mem_req_info2 = ash::vk::ImageMemoryRequirementsInfo2::builder()
        .image(image)
        .push_next(&mut image_plane_mem_req_info);

    unsafe {
        for i in 0..num_planes {
            image_plane_mem_req_info.plane_aspect = ash::vk::ImageAspectFlags::PLANE_0 << i;;
            device.get_image_memory_requirements2(&image_plane_mem_req_info2, &mut mem_req_2);

            results.planar_offset.push(results.requirement.size);
            results.requirement =
                    results.requirement.alignment(max(
                        mem_req_2.memory_requirements.alignment,
                        results.requirement.alignment,
                    ))
                    .size(results.requirement.size + round_up(
                        mem_req_2.memory_requirements.size,
                        mem_req_2.memory_requirements.alignment,
                    ))
                    .memory_type_bits(results.requirement.memory_type_bits | mem_req_2.memory_requirements.memory_type_bits);
        }
    }

    results
}

fn util_get_memory_type(
    type_bits: u32,
    memory_properties: &ash::vk::PhysicalDeviceMemoryProperties,
    properties: ash::vk::MemoryPropertyFlags,
) -> Option<u32> {
    let mut type_bits_test = type_bits;
    for i in 0..memory_properties.memoryTypeCount {
        if (type_bits_test & 1) == 1 {
            if memory_properties.memory_types[i as usize].property_flags.contains(properties) {
                return Some(i);
            }
            // if (memory_properties.memoryTypes[i as usize].propertyFlags & properties) == properties {
            //     return Some(i);
            // }
        }
        type_bits_test >>= 1;
    }
    return None;
}

struct QueueFamilyResult {
    properties: ash::vk::QueueFamilyProperties,
    family_index: u32,
    queue_index: u32,
}

fn util_vk_image_usage_to_format_features(
    usage: ash::vk::ImageUsageFlags,
) -> ash::vk::FormatFeatureFlags {

    let mut result: ash::vk::FormatFeatureFlags  = ash::vk::FormatFeatureFlags::empty();
    if usage.contains(ash::vk::ImageUsageFlags::SAMPLED) {
        result |= ash::vk::FormatFeatureFlags::SAMPLED_IMAGE;
    }
    if usage.contains(ash::vk::ImageUsageFlags::STORAGE)
    {
        result |= ash::vk::FormatFeatureFlags::STORAGE_IMAGE;
    }
    if usage.contains(ash::vk::ImageUsageFlags::COLOR_ATTACHMENT)
    {
        result |= ash::vk::FormatFeatureFlags::COLOR_ATTACHMENT;
    }
    if  usage.contains(ash::vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
    {
        result |= ash::vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT;
    }
    return result;
}

unsafe fn util_find_queue_family_index(
    renderer: &VulkanRenderer,
    node_index: u32,
    queue_type: QueueType,
) -> QueueFamilyResult {
    let mut queue_family_index: u32 = u32::MAX;
    let mut queue_index: u32 = u32::MAX;
    let required_flags = queue_type.to_vk_queue();
    let mut found = false;

    let mut family_properties =  renderer.instance.get_physical_device_queue_family_properties(renderer.active_gpu);
    let mut min_queue_flag: u32 = u32::MAX;
    for (i, property) in family_properties.iter().enumerate() {
        let queue_flags = property.queueFlags;
        let is_graphics_queue = (queue_flags & ffi::vk::VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT) > 0;
        let filter_flags = queue_flags & required_flags;
        if queue_type == QueueType::QueueTypeGraphics && is_graphics_queue {
            found = true;
            queue_family_index = i as u32;
            queue_index = 0;
            break;
        }
        if (queue_flags & required_flags) > 0
            && (queue_flags & !required_flags) == 0
            && renderer.used_queue_count[node_index as usize][queue_flags as usize]
                < renderer.available_queue_count[node_index as usize][queue_flags as usize]
        {
            found = true;
            queue_family_index = i as u32;
            queue_index = renderer.used_queue_count[node_index as usize][queue_flags as usize];
            break;
        }
        if filter_flags > 0
            && ((queue_flags - filter_flags) < min_queue_flag)
            && !is_graphics_queue
            && renderer.used_queue_count[node_index as usize][queue_flags as usize]
                < renderer.available_queue_count[node_index as usize][queue_flags as usize]
        {
            found = true;
            min_queue_flag = queue_flags - filter_flags;
            queue_family_index = i as u32;
            queue_index = renderer.used_queue_count[node_index as usize][queue_flags as usize];
            break;
        }
    }

    if !found {
        for (i, property) in family_properties.iter().enumerate() {
            let queue_flags = property.queueFlags;
            if queue_flags & required_flags > 0
                && renderer.used_queue_count[node_index as usize][queue_flags as usize]
                    < renderer.available_queue_count[node_index as usize][queue_flags as usize]
            {
                found = true;
                queue_family_index = i as u32;
                queue_index = renderer.used_queue_count[node_index as usize][queue_flags as usize];
                break;
            }
        }
    }

    if !found {
        found = true;
        queue_family_index = 0;
        queue_index = 0;
        error!(
            "Could not find queue of type {}. Using default queue",
            queue_type as u32
        );
    }

    QueueFamilyResult {
        properties: family_properties[queue_family_index as usize],
        family_index: queue_family_index,
        queue_index: queue_index,
    }
}

unsafe fn create_instance(entry: &mut ash::Entry, mut wanted_instance_extensions: HashSet<CString>, mut wanted_instance_layers: HashSet<CString> ) -> RendererResult<ash::Instance> {
    // layers: Vec<const *c_char> = Vec::new();
    let application_name = CString::new("3DEngine").expect("CString::new failed");
    let engine_name = CString::new("3DEngine").expect("CString::new failed");

    let _loaded_extension: Vec<CString> = vec![];

    let layer_properties = entry.enumerate_instance_layer_properties().unwrap();
    let ext_properties = entry.enumerate_instance_extension_properties().unwrap();


    for layer_property in &layer_properties {
        info!(
            "vkinstance-layer: {}",
            CStr::from_ptr(layer_property.layer_name.as_ptr()).to_string_lossy()
        );
    }

    for ext_property in &ext_properties {
        info!(
            "vkinstance-ext: {}",
            CStr::from_ptr(ext_property.extension_name.as_ptr()).to_string_lossy()
        );
    }

    let app_create_info = ash::vk::ApplicationInfo::builder()
        .application_name(&application_name.as_c_str())
        .engine_version(0)
        .application_version(0)
        .engine_name(&engine_name.as_c_str())
        .api_version(0)
        .build();

    wanted_instance_layers.retain(move |layer| {
        for layer_property in &layer_properties {
            let layer_name = CStr::from_ptr(layer_property.layerName.as_ptr());
            if layer_name.eq(layer.as_c_str()) {
                return true;
            }
        }
        warn!("vkinstance-layer-missing: {}", layer.to_string_lossy());
        return false;
    });

    // Layer extensions
    for target_layer in &wanted_instance_layers {
        let mut layer_target_ext_count: u32 = 0;
        renderer.entry.fp_v1_0().enumerate_instance_extension_properties(target_layer.as_ptr(),
                                                                         &mut layer_target_ext_count,
                                                                         ptr::null_mut()
        );

        let mut layer_target_ext: Vec<ash::vk::ExtensionProperties> =
            vec![ash::vk::ExtensionProperties::default(); ext_count as usize];
        renderer.entry.fp_v1_0().enumerate_instance_extension_properties(target_layer.as_ptr(),
                                                                         &mut layer_target_ext_count,
                                                                         layer_target_ext.as_mut_ptr()
        );

        wanted_instance_extensions.retain(move |layer| {
            for ext_property in &layer_target_ext {
                if ext_property.extension_name.eq(layer) {
                    return true
                }

            }
            return false;
        });
    }


    // Standalone extensions
    wanted_instance_extensions.retain(move |layer| {
        for ext_property in &ext_properties {
            let extension_name = CStr::from_ptr(ext_property.extensionName.as_ptr());
            if extension_name.eq(layer) {
                return true;
            }
        }
        return false;
    });

    let mut enabled_layers: Vec<*const c_char> = Vec::with_capacity(wanted_instance_layers.len());
    let mut enabled_extensions: Vec<*const c_char> =
        Vec::with_capacity(wanted_instance_extensions.len());
    for layer in &wanted_instance_layers {
        enabled_layers.push(layer.as_ptr());
    }

    for ext in &wanted_instance_extensions {
        enabled_extensions.push(ext.as_ptr());
    }

    // enabled_layers.push()
    let mut instance_info = ash::vk::InstanceCreateInfo::builder()
        .application_info(&app_create_info)
        .enabled_layer_names(&enabled_layers.as_slice())
        .enabled_extension_names(&enabled_extensions.as_slice())
        .build();

    Ok(entry.create_instance(&instance_info, None).unwrap())
}
//
// unsafe fn init_device(instance: &mut ash::Instance, active_gpu: ash::vk::PhysicalDevice, desc: &RenderDesc) -> RendererResult<()> {
//
//     let mut ext = instance.enumerate_device_extension_properties(active_gpu).unwrap();
//     let mut layers = instance.enumerate_device_layer_properties(active_gpu).unwrap();
//
//     for layer in &layers {
//         let layer_name = CStr::from_ptr(layer.layer_name.as_ptr());
//         info!("vkdevice-layer: {}", layer_name.to_string_lossy());
//     }
//
//     for ext in &ext {
//         let ext_name = CStr::from_ptr(ext.extension_name.as_ptr());
//         info!("vkdevice-ext: {}", ext_name.to_string_lossy());
//     }
//
//     let mut device_extension_cache: Vec<CString> = Vec::new();
//     let mut extension_count: u32 = 0;
//     let mut dedicated_allocation_extension: bool = false;
//     let mut memory_req2extension: bool = false;
//     let mut external_memory_extension: bool = false;
//     let mut fragment_shader_interlock_extension: bool = false;
//     ffi::vk::vkEnumerateDeviceExtensionProperties(
//         renderer.active_gpu,
//         ptr::null_mut(),
//         &mut extension_count,
//         ptr::null_mut(),
//     );
//     if extension_count > 0 {
//         let mut properties: Vec<ffi::vk::VkExtensionProperties> =
//             vec![MaybeUninit::zeroed().assume_init(); extension_count as usize];
//         let empty_ext = Vec::new();
//         let user_extensions: &Vec<CString> = match &desc.imp {
//             RenderDescImp::Vulkan(des) => &des.device_extensions,
//             _ => &empty_ext,
//         };
//
//         let mut wanted_device_extensions: Vec<&CStr> =
//             Vec::with_capacity(GLOBAL_WANTED_DEVICE_EXTENSIONS.len() + user_extensions.len());
//         for extension in GLOBAL_WANTED_DEVICE_EXTENSIONS {
//             wanted_device_extensions.push(CStr::from_bytes_with_nul_unchecked(extension));
//         }
//         for extension in user_extensions {
//             wanted_device_extensions.push(extension);
//         }
//         renderer.instance.unwrap().enumerate_device_extension_properties(renderer.active_gpu.unwrap());
//         ffi::vk::vkEnumerateDeviceExtensionProperties(
//             renderer.active_gpu,
//             ptr::null_mut(),
//             &mut extension_count,
//             properties.as_mut_ptr(),
//         );
//         for property in &properties {
//             let current_property_name = CStr::from_ptr(property.extensionName.as_ptr());
//             for wanted_extension in &wanted_device_extensions {
//                 if current_property_name.cmp(wanted_extension) == Ordering::Equal {
//                     device_extension_cache.push(CString::from(*wanted_extension));
//                     if wanted_extension.cmp(&ash::vk::KhrDedicatedAllocationFn::name()) == Ordering::Equal
//                     {
//                         // dedicated_allocation_extension = true;
//                     }
//
//
//                     if wanted_extension.cmp(&ash::vk::KhrGetMemoryRequirements2Fn::name()) == Ordering::Equal
//                     {
//                         // memory_req2extension = true;
//                     }
//
//                     if wanted_extension.cmp(&ash::vk::KhrExternalMemoryFn::name()) == Ordering::Equal
//                     {
//                         // external_memory_extension = true;
//                     }
//                     // #if defined(VK_USE_PLATFORM_WIN32_KHR)
//                     // if (strcmp(wantedDeviceExtensions[k], VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME) == 0)
//                     // externalMemoryWin32Extension = true;
//                     // #endif
//                     if wanted_extension.cmp(&ash::vk::KhrDrawIndirectCountFn::name()) == Ordering::Equal
//                     {
//                         renderer.features |= VulkanSupportedFeatures::DRAW_INDIRECT_COUNT_EXTENSION;
//                     }
//
//
//                     if wanted_extension.cmp(&ash::vk::AmdDrawIndirectCountFn::name()) == Ordering::Equal
//                     {
//
//                         renderer.features |=
//                             VulkanSupportedFeatures::AMD_DRAW_INDIRECT_COUNT_EXTENSION;
//                     }
//
//                     if wanted_extension.cmp(&ash::vk::AmdGcnShaderFn::name()) == Ordering::Equal
//                     {
//                         renderer.features |= VulkanSupportedFeatures::AMD_GCN_SHADEREXTENSION;
//                     }
//
//                     // TODO: rayracing
//                     // // KHRONOS VULKAN RAY TRACING
//                     // uint32_t khrRaytracingSupported = 1;
//                     //
//                     // if (strcmp(wantedDeviceExtensions[k], VK_KHR_SHADER_FLOAT_CONTROLS_EXTENSION_NAME) == 0)
//                     // pRenderer->mVulkan.mShaderFloatControlsExtension = 1;
//                     // khrRaytracingSupported &= pRenderer->mVulkan.mShaderFloatControlsExtension;
//                     //
//                     // if (strcmp(wantedDeviceExtensions[k], VK_KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME) == 0)
//                     // pRenderer->mVulkan.mBufferDeviceAddressExtension = 1;
//                     // khrRaytracingSupported &= pRenderer->mVulkan.mBufferDeviceAddressExtension;
//                     //
//                     // if (strcmp(wantedDeviceExtensions[k], VK_KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME) == 0)
//                     // pRenderer->mVulkan.mDeferredHostOperationsExtension = 1;
//                     // khrRaytracingSupported &= pRenderer->mVulkan.mDeferredHostOperationsExtension;
//                     //
//                     // if (strcmp(wantedDeviceExtensions[k], VK_KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME) == 0)
//                     // pRenderer->mVulkan.mKHRAccelerationStructureExtension = 1;
//                     // khrRaytracingSupported &= pRenderer->mVulkan.mKHRAccelerationStructureExtension;
//                     //
//                     // if (strcmp(wantedDeviceExtensions[k], VK_KHR_SPIRV_1_4_EXTENSION_NAME) == 0)
//                     // pRenderer->mVulkan.mKHRSpirv14Extension = 1;
//                     // khrRaytracingSupported &= pRenderer->mVulkan.mKHRSpirv14Extension;
//                     //
//                     // if (strcmp(wantedDeviceExtensions[k], VK_KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME) == 0)
//                     // pRenderer->mVulkan.mKHRRayTracingPipelineExtension = 1;
//                     // khrRaytracingSupported &= pRenderer->mVulkan.mKHRRayTracingPipelineExtension;
//                     //
//                     // if (khrRaytracingSupported)
//                     // pRenderer->mVulkan.mRaytracingSupported = 1;
//                     //
//                     // if (strcmp(wantedDeviceExtensions[k], VK_KHR_RAY_QUERY_EXTENSION_NAME) == 0)
//                     // pRenderer->mVulkan.mKHRRayQueryExtension = 1;
//
//                     if wanted_extension.cmp(&ash::vk::KhrSamplerYcbcrConversionFn::name()) == Ordering::Equal
//                     {
//                         renderer.features |= VulkanSupportedFeatures::YCBCR_EXTENSION;
//                     }
//                     if wanted_extension.cmp(&ash::vk::ExtFragmentShaderInterlockFn::name()) == Ordering::Equal
//                     {
//                         fragment_shader_interlock_extension = true;
//                     }
//                     break;
//                 }
//             }
//         }
//     }
//     let mut physical_features = ash::vk::PhysicalDeviceFeatures2KHR::builder();
//     // let mut gpu_features2: ffi::vk::VkPhysicalDeviceFeatures2KHR =
//     //     MaybeUninit::zeroed().assume_init();
//     // gpu_features2.sType = ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR;
//     //
//     // let mut base: *mut ffi::vk::VkBaseOutStructure = (&mut gpu_features2 as *mut _) as *mut _;
//     //
//     // unsafe fn next_to_chain<T>(
//     //     base: *mut *mut ffi::vk::VkBaseOutStructure,
//     //     next: &mut T,
//     //     condition: bool,
//     // ) {
//     //     if condition {
//     //         (**base).pNext = (next as *mut T) as *mut ffi::vk::VkBaseOutStructure;
//     //         *base = (**base).pNext;
//     //     }
//     // }
//
//     // add extensions
//     let mut fragment_shader_interlock_features =
//         ash::vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT::builder();
//     if fragment_shader_interlock_extension {
//         physical_features = physical_features.push_next(&mut fragment_shader_interlock_features.deref_mut());
//     }
//
//     // let mut fragment_shader_interlock_features: ffi::vk::VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT = MaybeUninit::zeroed().assume_init();
//     // fragment_shader_interlock_features.sType =  ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT;
//     // next_to_chain(
//     //     &mut base,
//     //     &mut fragment_shader_interlock_features,
//     //     fragment_shader_interlock_extension,
//     // );
//
//     // let mut descriptor_indexing_features: ffi::vk::VkPhysicalDeviceDescriptorIndexingFeaturesEXT =
//     //     MaybeUninit::zeroed().assume_init();
//     // descriptor_indexing_features.sType =
//     //     ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT;
//     // next_to_chain(
//     //     &mut base,
//     //     &mut descriptor_indexing_features,
//     //     renderer
//     //         .features
//     //         .contains(VulkanSupportedFeatures::DESCRIPTOR_INDEXING_EXTENSION),
//     // );
//
//     let mut descriptor_indexing_features = ash::vk::PhysicalDeviceDescriptorIndexingFeaturesEXT::builder();
//     if renderer
//         .features
//         .contains(VulkanSupportedFeatures::DESCRIPTOR_INDEXING_EXTENSION) {
//         physical_features = physical_features.push_next(&mut descriptor_indexing_features.deref_mut());
//     }
//
//     let mut ycbcr_features = ash::vk::PhysicalDeviceSamplerYcbcrConversionFeatures::builder();
//     if renderer
//         .features
//         .contains(VulkanSupportedFeatures::YCBCR_EXTENSION) {
//         physical_features = physical_features.push_next(&mut ycbcr_features.deref_mut());
//     }
//
//     // let mut ycbcr_features: ffi::vk::VkPhysicalDeviceSamplerYcbcrConversionFeatures =
//     //     MaybeUninit::zeroed().assume_init();
//     // ycbcr_features.sType = ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES;
//     // next_to_chain(
//     //     &mut base,
//     //     &mut ycbcr_features,
//     //     renderer
//     //         .features
//     //         .contains(VulkanSupportedFeatures::YCBCR_EXTENSION),
//     // );
//
//
//     // let mut enabled_buffer_device_address_features: ffi::vk::VkPhysicalDeviceBufferDeviceAddressFeatures = MaybeUninit::zeroed().assume_init();
//     // enabled_buffer_device_address_features.sType =
//     //     ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES;
//     // next_to_chain(
//     //     &mut base,
//     //     &mut enabled_buffer_device_address_features,
//     //     renderer
//     //         .features
//     //         .contains(VulkanSupportedFeatures::BUFFER_DEVICE_ADDRESS_EXTENSION),
//     // );
//     let mut enabled_buffer_device_address_features = ash::vk::PhysicalDeviceBufferDeviceAddressFeatures::builder();
//     if renderer
//         .features
//         .contains(VulkanSupportedFeatures::BUFFER_DEVICE_ADDRESS_EXTENSION) {
//         physical_features = physical_features.push_next(&mut enabled_buffer_device_address_features.deref_mut());
//     }
//
//     // let mut enabled_ray_tracing_pipeline_features: ffi::vk::VkPhysicalDeviceRayTracingPipelineFeaturesKHR = MaybeUninit::zeroed().assume_init();
//     // enabled_ray_tracing_pipeline_features.sType = ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR;
//     // next_to_chain(&mut base, &mut enabled_ray_tracing_pipeline_features, true);
//
//     // let mut enabled_acceleration_structure_features: ffi::vk::VkPhysicalDeviceAccelerationStructureFeaturesKHR = MaybeUninit::zeroed().assume_init();
//     // enabled_acceleration_structure_features.sType = ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR;
//     // next_to_chain(
//     //     &mut base,
//     //     &mut enabled_acceleration_structure_features,
//     //     renderer
//     //         .features
//     //         .contains(VulkanSupportedFeatures::KHR_ACCELERATION_STRUCTURE_EXTENSION),
//     // );
//
//     let mut enabled_acceleration_structure_features = ash::vk::PhysicalDeviceAccelerationStructureFeaturesKHR::builder();
//     if  renderer
//         .features
//         .contains(VulkanSupportedFeatures::KHR_ACCELERATION_STRUCTURE_EXTENSION) {
//         physical_features = physical_features.push_next(&mut enabled_acceleration_structure_features.deref_mut());
//     }
//
//     // let mut enabled_ray_query_features: ffi::vk::VkPhysicalDeviceRayQueryFeaturesKHR = MaybeUninit::zeroed().assume_init();
//     // enabled_ray_query_features.sType = ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR;
//     // next_to_chain(&mut base, &mut enabled_ray_query_features, true);
//
//     renderer.instance.unwrap().get_physical_device_features2(renderer.active_gpu.unwrap(), &mut physical_features.deref_mut());
//
//     // ffi::vk::vkGetPhysicalDeviceFeatures2(renderer.active_gpu, &mut gpu_features2);
//
//     // Get queue family properties
//     let mut queue_family_count = 0;
//     ffi::vk::vkGetPhysicalDeviceQueueFamilyProperties(
//         renderer.active_gpu,
//         &mut queue_family_count,
//         ptr::null_mut(),
//     );
//     let mut queue_family_properties: Vec<ffi::vk::VkQueueFamilyProperties> =
//         vec![MaybeUninit::zeroed().assume_init(); queue_family_count as usize];
//     ffi::vk::vkGetPhysicalDeviceQueueFamilyProperties(
//         renderer.active_gpu,
//         &mut queue_family_count,
//         queue_family_properties.as_mut_ptr(),
//     );
//
//     // need a queue_priority for each queue in the queue family we create
//     let mut queue_family_priorities: [[f32; MAX_QUEUE_COUNT as usize]; MAX_QUEUE_FAMILIES as usize] =
//         [[0.0; MAX_QUEUE_COUNT as usize]; MAX_QUEUE_FAMILIES as usize];
//     let mut queue_create_infos: Vec<ash::vk::DeviceQueueCreateInfo> =
//         Vec::with_capacity(queue_family_count as usize);
//     renderer.available_queue_count.resize(
//         renderer.linked_node_count as usize,
//         [0; MAX_QUEUE_FLAGS as usize],
//     );
//     renderer.used_queue_count.resize(
//         renderer.linked_node_count as usize,
//         [0; MAX_QUEUE_FLAGS as usize],
//     );
//
//     for (queue_index, &property) in queue_family_properties.iter().enumerate() {
//         let mut queue_count = property.queueCount;
//         if queue_count > 0 {
//             // Request only one queue of each type if mRequestAllAvailableQueues is not set to true
//             if queue_count > 1
//                 && !(match &desc.imp {
//                     RenderDescImp::Vulkan(imp) => imp.request_all_available_queues,
//                     _ => false,
//                 })
//             {
//                 queue_count = 1;
//             }
//             assert!(queue_count <= MAX_QUEUE_COUNT);
//             queue_count = min(queue_count, MAX_QUEUE_COUNT);
//
//             let mut create_info = ash::vk::DeviceQueueCreateInfo::builder()
//                 .queue_family_index(queue_index as u32)
//                 .queue_priorities(&queue_family_priorities[queue_index][0 .. queue_count]);
//
//             // let create_info = ffi::vk::VkDeviceQueueCreateInfo {
//             //     sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
//             //     pNext: ptr::null_mut(),
//             //     flags: 0,
//             //     queueFamilyIndex: queue_index as u32,
//             //     queueCount: queue_count,
//             //     pQueuePriorities: queue_family_priorities[queue_index].as_mut_ptr(),
//             // };
//             queue_create_infos.push(create_info.build());
//
//             for i in 0..renderer.linked_node_count {
//                 renderer.available_queue_count[i as usize][property.queueFlags as usize] =
//                     queue_count;
//             }
//         }
//     }
//
//     let extension_result: Vec<*const c_char> = device_extension_cache
//         .into_iter()
//         .map(|st| st.as_ptr())
//         .collect();
//     let mut create_info = ash::vk::DeviceCreateInfo::builder()
//         .queue_create_infos(queue_create_infos.as_slice())
//         .enabled_extension_names(extension_result.as_slice())
//         .push_next(gpu_features2);
//
//     // let mut create_info = ffi::vk::VkDeviceCreateInfo {
//     //     sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
//     //     pNext: (&mut gpu_features2 as *mut _) as *mut c_void,
//     //     flags: 0,
//     //     queueCreateInfoCount: queue_create_infos.len() as u32,
//     //     pQueueCreateInfos: queue_create_infos.as_mut_ptr(),
//     //     enabledLayerCount: 0,
//     //     ppEnabledLayerNames: ptr::null_mut(),
//     //     enabledExtensionCount: extension_result.len() as u32,
//     //     ppEnabledExtensionNames: extension_result.as_ptr(),
//     //     pEnabledFeatures: ptr::null_mut(),
//     // };
//
//     // instance.create_device()
//     let result = ffi::vk::vkCreateDevice(
//         renderer.active_gpu,
//         &mut create_info,
//         ptr::null_mut(),
//         &mut renderer.device,
//     );
//     if result != ffi::vk::VkResult_VK_SUCCESS {
//         return Err(VulkanError(result));
//     }
//     Ok()
// }


pub unsafe fn wanted_extensions(desc: &RenderDesc) -> (HashSet<CString>, HashSet<CString> ){
    let mut wanted_instance_extensions: HashSet<CString> = HashSet::new();
    let mut wanted_instance_layers: HashSet<CString> = HashSet::new();
    for ext in GLOBAL_INSTANCE_EXTENSIONS {
        wanted_instance_extensions.insert(CString::from(ext));
    }
    match &desc.imp {
        RenderDescImp::Vulkan(vk) => {
            for extension in &vk.instance_extensions {
                wanted_instance_extensions.insert(extension.clone());
            }

            for layer in &vk.instance_layers {
                wanted_instance_layers.insert(layer.clone());
            }
        }
        _ => panic!("invalid configuration"),
    }
    return (wanted_instance_layers, wanted_instance_extensions)
}

impl Renderer<VulkanAPI> for VulkanRenderer {
    unsafe fn init(_name: &CStr, desc: &RenderDesc) -> RendererResult<Arc<VulkanRenderer>> {
        let mut entry = ash::Entry::load()?;
        // initialize instance
        let mut wanted_instance_extensions: HashSet<CString> = HashSet::new();
        let mut wanted_instance_layers: HashSet<CString> = HashSet::new();
        for ext in GLOBAL_INSTANCE_EXTENSIONS {
            wanted_instance_extensions.insert(CString::from(ext));
        }
        match &desc.imp {
            RenderDescImp::Vulkan(vk) => {
                for extension in &vk.instance_extensions {
                    wanted_instance_extensions.insert(extension.clone());
                }

                for layer in &vk.instance_layers {
                    wanted_instance_layers.insert(layer.clone());
                }
            }
            _ => panic!("invalid configuration"),
        }
        let mut instance = create_instance(&mut entry, wanted_instance_extensions, wanted_instance_layers)?;

        let devices = VulkanGPUInfo::all(&instance);
        let gpu = VulkanGPUInfo::select_best_gpu(&instance, &devices)?;
        assert!(gpu.get_device() != ptr::null_mut());

        let mut active_gpu = gpu.get_device();
        let active_gpu_properties = gpu.get_device_properties().clone();
        let mut supported_features = VulkanSupportedFeatures::empty();

        let user_extensions: &Vec<CString> = match &desc.imp {
            RenderDescImp::Vulkan(des) => &des.device_extensions,
            _ => &empty_ext,
        };
        let mut wanted_device_extensions: Vec<&CStr> =
            Vec::with_capacity(GLOBAL_WANTED_DEVICE_EXTENSIONS.len() + user_extensions.len());
        for extension in GLOBAL_WANTED_DEVICE_EXTENSIONS {
            wanted_device_extensions.push(extension);
        }
        for extension in user_extensions {
            wanted_device_extensions.push(extension);
        }

        let mut device_extension_cache: Vec<CString> = Vec::new();
        let mut extension_count: u32 = 0;
        let mut dedicated_allocation_extension: bool = false;
        let mut memory_req2extension: bool = false;
        let mut external_memory_extension: bool = false;
        let mut fragment_shader_interlock_extension: bool = false;

        let mut ext = instance.enumerate_device_extension_properties(active_gpu).unwrap();
        let mut layers = instance.enumerate_device_layer_properties(active_gpu).unwrap();

        for layer in &layers {
            let layer_name = CStr::from_ptr(layer.layer_name.as_ptr());
            info!("vkdevice-layer: {}", layer_name.to_string_lossy());
        }

        for ext in &ext {
            let ext_name = CStr::from_ptr(ext.extension_name.as_ptr());
            info!("vkdevice-ext: {}", ext_name.to_string_lossy());
        }

        let mut features = VulkanSupportedFeatures::empty();

        for property in &ext {
            let current_property_name = CStr::from_ptr(property.extension_name.as_ptr());
            for wanted_extension in &wanted_device_extensions {
                if current_property_name.cmp(wanted_extension) == Ordering::Equal {
                    device_extension_cache.push(CString::from(*wanted_extension));
                    if wanted_extension.cmp(&ash::vk::KhrDedicatedAllocationFn::name()) == Ordering::Equal
                    {
                        // dedicated_allocation_extension = true;
                    }


                    if wanted_extension.cmp(&ash::vk::KhrGetMemoryRequirements2Fn::name()) == Ordering::Equal
                    {
                        // memory_req2extension = true;
                    }

                    if wanted_extension.cmp(&ash::vk::KhrExternalMemoryFn::name()) == Ordering::Equal
                    {
                        // external_memory_extension = true;
                    }
                    // #if defined(VK_USE_PLATFORM_WIN32_KHR)
                    // if (strcmp(wantedDeviceExtensions[k], VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME) == 0)
                    // externalMemoryWin32Extension = true;
                    // #endif
                    if wanted_extension.cmp(&ash::vk::KhrDrawIndirectCountFn::name()) == Ordering::Equal
                    {
                        features |= VulkanSupportedFeatures::DRAW_INDIRECT_COUNT_EXTENSION;
                    }


                    if wanted_extension.cmp(&ash::vk::AmdDrawIndirectCountFn::name()) == Ordering::Equal
                    {

                        features |= VulkanSupportedFeatures::AMD_DRAW_INDIRECT_COUNT_EXTENSION;
                    }

                    if wanted_extension.cmp(&ash::vk::AmdGcnShaderFn::name()) == Ordering::Equal
                    {
                        features |= VulkanSupportedFeatures::AMD_GCN_SHADEREXTENSION;
                    }

                    // TODO: rayracing
                    // // KHRONOS VULKAN RAY TRACING
                    // uint32_t khrRaytracingSupported = 1;
                    //
                    // if (strcmp(wantedDeviceExtensions[k], VK_KHR_SHADER_FLOAT_CONTROLS_EXTENSION_NAME) == 0)
                    // pRenderer->mVulkan.mShaderFloatControlsExtension = 1;
                    // khrRaytracingSupported &= pRenderer->mVulkan.mShaderFloatControlsExtension;
                    //
                    // if (strcmp(wantedDeviceExtensions[k], VK_KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME) == 0)
                    // pRenderer->mVulkan.mBufferDeviceAddressExtension = 1;
                    // khrRaytracingSupported &= pRenderer->mVulkan.mBufferDeviceAddressExtension;
                    //
                    // if (strcmp(wantedDeviceExtensions[k], VK_KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME) == 0)
                    // pRenderer->mVulkan.mDeferredHostOperationsExtension = 1;
                    // khrRaytracingSupported &= pRenderer->mVulkan.mDeferredHostOperationsExtension;
                    //
                    // if (strcmp(wantedDeviceExtensions[k], VK_KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME) == 0)
                    // pRenderer->mVulkan.mKHRAccelerationStructureExtension = 1;
                    // khrRaytracingSupported &= pRenderer->mVulkan.mKHRAccelerationStructureExtension;
                    //
                    // if (strcmp(wantedDeviceExtensions[k], VK_KHR_SPIRV_1_4_EXTENSION_NAME) == 0)
                    // pRenderer->mVulkan.mKHRSpirv14Extension = 1;
                    // khrRaytracingSupported &= pRenderer->mVulkan.mKHRSpirv14Extension;
                    //
                    // if (strcmp(wantedDeviceExtensions[k], VK_KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME) == 0)
                    // pRenderer->mVulkan.mKHRRayTracingPipelineExtension = 1;
                    // khrRaytracingSupported &= pRenderer->mVulkan.mKHRRayTracingPipelineExtension;
                    //
                    // if (khrRaytracingSupported)
                    // pRenderer->mVulkan.mRaytracingSupported = 1;
                    //
                    // if (strcmp(wantedDeviceExtensions[k], VK_KHR_RAY_QUERY_EXTENSION_NAME) == 0)
                    // pRenderer->mVulkan.mKHRRayQueryExtension = 1;

                    if wanted_extension.cmp(&ash::vk::KhrSamplerYcbcrConversionFn::name()) == Ordering::Equal
                    {
                        features |= VulkanSupportedFeatures::YCBCR_EXTENSION;
                    }
                    if wanted_extension.cmp(&ash::vk::ExtFragmentShaderInterlockFn::name()) == Ordering::Equal
                    {
                        fragment_shader_interlock_extension = true;
                    }
                    break;
                }
            }
        }
        let mut physical_features = ash::vk::PhysicalDeviceFeatures2KHR::builder();

        let mut fragment_shader_interlock_features =
            ash::vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT::builder();
        if fragment_shader_interlock_extension {
            physical_features = physical_features.push_next(&mut fragment_shader_interlock_features.deref_mut());
        }

        let mut descriptor_indexing_features = ash::vk::PhysicalDeviceDescriptorIndexingFeaturesEXT::builder();
        if renderer
            .features
            .contains(VulkanSupportedFeatures::DESCRIPTOR_INDEXING_EXTENSION) {
            physical_features = physical_features.push_next(&mut descriptor_indexing_features.deref_mut());
        }

        let mut ycbcr_features = ash::vk::PhysicalDeviceSamplerYcbcrConversionFeatures::builder();
        if renderer
            .features
            .contains(VulkanSupportedFeatures::YCBCR_EXTENSION) {
            physical_features = physical_features.push_next(&mut ycbcr_features.deref_mut());
        }

        let mut enabled_buffer_device_address_features = ash::vk::PhysicalDeviceBufferDeviceAddressFeatures::builder();
        if renderer
            .features
            .contains(VulkanSupportedFeatures::BUFFER_DEVICE_ADDRESS_EXTENSION) {
            physical_features = physical_features.push_next(&mut enabled_buffer_device_address_features.deref_mut());
        }

        let mut enabled_acceleration_structure_features = ash::vk::PhysicalDeviceAccelerationStructureFeaturesKHR::builder();
        if  renderer
            .features
            .contains(VulkanSupportedFeatures::KHR_ACCELERATION_STRUCTURE_EXTENSION) {
            physical_features = physical_features.push_next(&mut enabled_acceleration_structure_features.deref_mut());
        }

        instance.get_physical_device_features2(active_gpu, &mut physical_features);

        let linked_node_count: u16 = 1;
        let queue_family_properties = instance.get_physical_device_queue_family_properties(active_gpu);
        let mut queue_create_infos: Vec<ash::vk::DeviceQueueCreateInfo> = Vec::with_capacity(queue_family_properties.len() as usize);

        let mut available_queue_count: Vec<[u32; MAX_QUEUE_FLAGS as usize]> = vec![[0; MAX_QUEUE_FLAGS as usize]; linked_node_count as usize];
        let mut used_queue_count: Vec<[u32; MAX_QUEUE_FLAGS as usize]> = vec![[0; MAX_QUEUE_FLAGS as usize]; linked_node_count as usize];

        for (queue_index, &property) in queue_family_properties.iter().enumerate() {
            let mut queue_count = property.queue_count;
            if queue_count > 0 {
                // Request only one queue of each type if mRequestAllAvailableQueues is not set to true
                if queue_count > 1
                    && !(match &desc.imp {
                    RenderDescImp::Vulkan(imp) => imp.request_all_available_queues,
                    _ => false,
                })
                {
                    queue_count = 1;
                }
                assert!(queue_count <= MAX_QUEUE_COUNT);
                queue_count = min(queue_count, MAX_QUEUE_COUNT);

                let mut create_info = ash::vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(queue_index as u32)
                    .queue_priorities(&queue_family_priorities[queue_index][0 .. queue_count]);

                queue_create_infos.push(create_info.build());

                for i in 0..linked_node_count {
                    available_queue_count[i as usize][property.queueFlags as usize] =
                        queue_count;
                }
            }
        }

        let extension_result: Vec<*const c_char> = device_extension_cache
            .into_iter()
            .map(|st| st.as_ptr())
            .collect();

        let mut create_info = ash::vk::DeviceCreateInfo::builder()
            .queue_create_infos(queue_create_infos.as_slice())
            .enabled_extension_names(extension_result.as_slice())
            .push_next(gpu_features2);

        /************************************************************************/
        // Memory allocator
        /************************************************************************/

        let allocator = Allocator::new(&AllocatorCreateDesc {
            instance: instance.clone(),
            device: device.clone(),
            physical_device: active_gpu,
            debug_settings: Default::default(),
            buffer_device_address: true
        }).unwrap();

        let swapchain_loader = ash::extensions::khr::Swapchain::new(&instance, active_gpu);
        let surface_loader = ash::extensions::khr::Surface::new(&entry, active_gpu);

        let device = instance.create_device(active_gpu, &create_info, None).unwrap();
        let mut renderer = Arc::new_cyclic(|me| VulkanRenderer {
            entry: ash::Entry::load()?,
            instance,
            allocator,
            linked_node_count,
            device,
            swapchain_loader,
            surface_loader,
            active_gpu,
            active_gpu_properties,
            active_gpu_common_info: {
                let common = gpu.to_common();
                        info!(
                    "Name of selected gpu: {}",
                    common.vendor_info.gpu_name.to_string_lossy()
                );
                        info!(
                    "Vendor id of selected gpu: {}",
                    common.vendor_info.vendor_id.to_string_lossy()
                );
                        info!(
                    "Model id of selected gpu: {}",
                    common.vendor_info.model_id.to_string_lossy()
                );
                Box::new(common)
            },
            available_queue_count,
            used_queue_count,
            features,
            graphics_queue_family_index: 0,
            transfer_queue_family_index: 0,
            compute_queue_family_index: 0,
            me: me.clone()
        });
        let mut mut_render = Arc::get_mut(&mut renderer).unwrap();

        // initialize device
        // init_device(&mut mut_render, desc)?;
        let family_queue =
            util_find_queue_family_index(&mut_render, 0, QueueType::QueueTypeGraphics);
        let compute_queue =
            util_find_queue_family_index(&mut_render, 0, QueueType::QueueTypeCompute);
        let transfer_queue =
            util_find_queue_family_index(&mut_render, 0, QueueType::QueueTypeTransfer);

        mut_render.graphics_queue_family_index = family_queue.family_index;
        mut_render.graphics_queue_family_index = compute_queue.family_index;
        mut_render.graphics_queue_family_index = transfer_queue.family_index;

        //
        // let mut vma_functions = ffi::vk::VmaVulkanFunctions {
        //     vkAllocateMemory: Some(ffi::vk::vkAllocateMemory),
        //     vkBindBufferMemory: Some(ffi::vk::vkBindBufferMemory),
        //     vkBindImageMemory: Some(ffi::vk::vkBindImageMemory),
        //     vkCreateBuffer: Some(ffi::vk::vkCreateBuffer),
        //     vkCreateImage: Some(ffi::vk::vkCreateImage),
        //     vkDestroyBuffer: Some(ffi::vk::vkDestroyBuffer),
        //     vkDestroyImage: Some(ffi::vk::vkDestroyImage),
        //     vkFreeMemory: Some(ffi::vk::vkFreeMemory),
        //     vkGetBufferMemoryRequirements: Some(ffi::vk::vkGetBufferMemoryRequirements),
        //     vkGetBufferMemoryRequirements2KHR: Some(ffi::vk::vkGetBufferMemoryRequirements2KHR),
        //     vkGetImageMemoryRequirements: Some(ffi::vk::vkGetImageMemoryRequirements),
        //     vkGetImageMemoryRequirements2KHR: Some(ffi::vk::vkGetImageMemoryRequirements2KHR),
        //     vkGetPhysicalDeviceMemoryProperties: Some(ffi::vk::vkGetPhysicalDeviceMemoryProperties),
        //     vkGetPhysicalDeviceProperties: Some(ffi::vk::vkGetPhysicalDeviceProperties),
        //     vkMapMemory: Some(ffi::vk::vkMapMemory),
        //     vkUnmapMemory: Some(ffi::vk::vkUnmapMemory),
        //     vkFlushMappedMemoryRanges: Some(ffi::vk::vkFlushMappedMemoryRanges),
        //     vkInvalidateMappedMemoryRanges: Some(ffi::vk::vkInvalidateMappedMemoryRanges),
        //     vkCmdCopyBuffer: Some(ffi::vk::vkCmdCopyBuffer),
        //     vkBindBufferMemory2KHR: None,
        //     vkBindImageMemory2KHR: None,
        //     vkGetDeviceProcAddr: None,
        //     vkGetInstanceProcAddr: None,
        //     vkGetPhysicalDeviceMemoryProperties2KHR: None,
        // };
        // let mut create_info = ffi::vk::VmaAllocatorCreateInfo {
        //     flags: {
        //         let mut result: ffi::vk::VmaAllocatorCreateFlags = 0;
        //         if mut_render
        //             .features
        //             .contains(VulkanSupportedFeatures::DEDICATED_ALLOCATION_EXTENSION)
        //         {
        //             result |= ffi::vk::VmaAllocatorCreateFlagBits_VMA_ALLOCATOR_CREATE_KHR_DEDICATED_ALLOCATION_BIT;
        //         }
        //         if mut_render
        //             .features
        //             .contains(VulkanSupportedFeatures::BUFFER_DEVICE_ADDRESS_EXTENSION)
        //         {
        //             result |= ffi::vk::VmaAllocatorCreateFlagBits_VMA_ALLOCATOR_CREATE_BUFFER_DEVICE_ADDRESS_BIT;
        //         }
        //         result
        //     },
        //     physicalDevice: mut_render.active_gpu,
        //     device: mut_render.device,
        //     preferredLargeHeapBlockSize: 0,
        //     pAllocationCallbacks: ptr::null_mut(),
        //     pDeviceMemoryCallbacks: ptr::null_mut(),
        //     pHeapSizeLimit: ptr::null_mut(),
        //     pVulkanFunctions: &mut vma_functions,
        //     instance: mut_render.instance,
        //     vulkanApiVersion: 0,
        //     pTypeExternalMemoryHandleTypes: ptr::null_mut(),
        // };
        // ffi::vk::vmaCreateAllocator(&mut create_info, &mut mut_render.vma_allocator);
        Ok(renderer)
    }

    unsafe fn add_pipeline(&self) -> VulkanPipeline {
        todo!()
    }

    unsafe fn drop_pipeline(&self, pipeline: &mut super::VulkanPipeline) {
        assert!(self.device != ptr::null_mut());
        assert!(pipeline.pipeline != ptr::null_mut());
        ffi::vk::vkDestroyPipeline(self.device, pipeline.pipeline, ptr::null_mut());
        pipeline.pipeline = ptr::null_mut();
    }

    unsafe fn add_fence(&self) -> RendererResult<super::VulkanFence> {
        assert!(self.device != ptr::null_mut());

        let fence_info = ash::vk::FenceCreateInfo::builder();
        let vulkan_fence = self.device.create_fence(&fence_info, None).unwrap();
        let mut fence = super::VulkanFence {
            render: self.me.clone().upgrade().unwrap(),
            fence: vulkan_fence,
            submitted: false,
        };
        Ok(fence)
    }

    unsafe fn add_semaphore(&self) -> RendererResult<super::VulkanSemaphore> {
        assert!(self.device != ptr::null_mut());

        let add_info = ash::vk::SemaphoreCreateInfo::builder();
        let vk_semaphore = self.device.create_semaphore(&add_info, None).unwrap();
        let mut semaphore = VulkanSemaphore {
            render: self.me.clone().upgrade().unwrap(),
            semaphore: vk_semaphore,
            current_node: 0,
            signaled: false,
        };
        Ok(semaphore)
    }

    unsafe fn add_cmd_pool<'a>(
        &self,
        desc: &CmdPoolDesc<'a, VulkanAPI>,
    ) -> RendererResult<VulkanCommandPool<'a>> {
        assert!(self.device != ptr::null_mut());

        let mut add_info = ash::vk::CommandPoolCreateInfo::builder()
            .flags(if desc.transient {
                ash::vk::CommandPoolCreateFlags::TRANSIENT
            } else {
                ash::vk::CommandPoolCreateFlags::empty()
            }).queue_family_index(desc.queue.family_index);

        let mut vk_command_pool = self.device.create_command_pool(&add_info, None).unwrap();

        let mut cmd_pool = VulkanCommandPool {
            renderer: self.me.clone().upgrade().unwrap(),
            cmd_pool: vk_command_pool,
            queue: desc.queue,
        };
        Ok(cmd_pool)
    }

    unsafe fn add_cmd<'a>(
        &self,
        desc: &mut CmdDesc<'a, VulkanAPI>,
    ) -> RendererResult<VulkanCommand<'a>> {
        // let mut cmd = VulkanCommand {
        //     renderer: self.me.clone().upgrade().unwrap(),
        //     cmd_buf: ptr::null_mut(),
        //     active_render_pass: ptr::null_mut(),
        //     bound_pipeline_layout: ptr::null_mut(),
        //     pool: desc.cmd_pool,
        // };
        //
        // let mut alloc_info = ffi::vk::VkCommandBufferAllocateInfo {
        //     sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
        //     pNext: ptr::null_mut(),
        //     commandPool: desc.cmd_pool.cmd_pool,
        //     level: if desc.secondary {
        //         ffi::vk::VkCommandBufferLevel_VK_COMMAND_BUFFER_LEVEL_SECONDARY
        //     } else {
        //         ffi::vk::VkCommandBufferLevel_VK_COMMAND_BUFFER_LEVEL_PRIMARY
        //     },
        //     commandBufferCount: 1,
        // };
        // let result =
        //     ffi::vk::vkAllocateCommandBuffers(self.device, ptr::null_mut(), &mut cmd.cmd_buf);
        // if result != ffi::vk::VkResult_VK_SUCCESS {
        //     return Err(VulkanError(result));
        // }

        // Ok(cmd)
        Err(VulkanError(0))
    }

    unsafe fn add_queue(&mut self, desc: &QueueDesc) -> RendererResult<super::VulkanQueue> {
        let node_index = 0;
        let queue_result = util_find_queue_family_index(self, node_index, desc.queue_type);
        self.used_queue_count[node_index as usize][queue_result.properties.queueFlags as usize] +=
            1;

        let mut vk_queue = self.device.get_device_queue(
            queue_result.family_index,
            queue_result.queue_index
        );

        let mut queue = VulkanQueue {
            render: self.me.clone().upgrade().unwrap(),
            queue: vk_queue,

            submission_mutex: Mutex::new(()),
            family_index: queue_result.family_index,
            queue_index: queue_result.queue_index,
            queue_flag: queue_result.properties.queueFlags,

            queue_type: desc.queue_type,
            node_index: desc.node_index,
        };

        Ok(queue)
    }

    unsafe fn add_swap_chain<'a>(
        &self,
        desc: &'a SwapChainDesc<'a, VulkanAPI>,
        window_handle: &impl HasRawWindowHandle,
    ) -> RendererResult<VulkanSwapChain> {
        assert!(self.instance != ptr::null_mut());
        assert!(desc.presentation_queues.len() > 0);
        let surface_loader = ash::extensions::khr::Surface::new(&self.entry, &self.instance);

        unsafe {
            // : ffi::vk::VkSurfaceKHR = ptr::null_mut();
            let mut surface_khr: ash::vk::SurfaceKHR = ash::vk::SurfaceKHR::null();
             match window_handle.raw_window_handle() {
                RawWindowHandle::UiKit(_) => {
                    panic!("uiKit is not configured")
                }
                RawWindowHandle::AppKit(_) => {
                    panic!("app kit is not configured")
                }
                RawWindowHandle::Orbital(_) => {
                    panic!("orbital is not configured")
                }
                RawWindowHandle::Xlib(handle) => {
                    // cfg_if::cfg_if! {
                        // if  #[cfg(feature = "xlib")] {
                            let xlib_surface = ash::vk::XlibSurfaceCreateInfoKHR::builder()
                                .dpy(handle.display as _)
                                .window(handle.window);

                            let surface = ash::extensions::khr::XlibSurface::new(&self.entry, &self.instance);
                            surface_khr = surface.create_xlib_surface(&xlib_surface, None).unwrap();


                            // ::create_xlib_surface(&xlib_surface,
                            //     None).unwrap()

                            // ash::extensions::khr::XlibSurface::create_xlib_surface()
                             // let mut surface_info = ffi::vk::VkXlibSurfaceCreateInfoKHR {
                             //        sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR,
                             //        pNext: ptr::null_mut(),
                             //        flags: 0,
                             //        dpy: handle.display as *mut ffi::vk::Display,
                             //        window: handle.window
                             //    };
                             //    let result = ffi::vk::vkCreateXlibSurfaceKHR(self.instance, &surface_info, ptr::null_mut(), &mut vk_surface);
                             //    if result != ffi::vk::VkResult_VK_SUCCESS {
                             //        return Err(VulkanError(result))
                             //    }
                        // } else {
                        //     panic!("xlib is not configured")
                        // }
                    // }
                }
                RawWindowHandle::Xcb(handle) => {
                    let xcb_surface_info = ash::vk::XcbSurfaceCreateInfoKHR::builder()
                        .connection(handle.connection as _)
                        .window(handle.window);
                    let surface = ash::extensions::khr::XcbSurface::new(&self.entry, &self.instance);
                    surface_khr = surface.create_xcb_surface(&create_info, None).unwrap();

                    // cfg_if::cfg_if! {
                    //     if #[cfg(feature = "xcb")] {
                    //         let mut surface_info = ffi::vk::VkXcbSurfaceCreateInfoKHR {
                    //             sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR,
                    //             pNext: ptr::null_mut(),
                    //             flags: 0,
                    //             connection: handle.connection as *mut ffi::vk::xcb_connection_t,
                    //             window: handle.window
                    //         };
                    //         let result = ffi::vk::vkCreateXcbSurfaceKHR(self.instance, &surface_info, ptr::null_mut(), &mut vk_surface);
                    //         if result != ffi::vk::VkResult_VK_SUCCESS {
                    //             return Err(VulkanError(result))
                    //         }
                    //     } else {
                    //         panic!("xcb is not configured")
                    //     }
                    // }
                }
                RawWindowHandle::Wayland(_) => {
                    panic!("TODO: wayland is not configured")
                }
                RawWindowHandle::Win32(_) => {
                    panic!("TODO: win32 is not configured")
                }
                RawWindowHandle::WinRt(_) => {
                    panic!("TODO: winRT is not configured")
                }
                RawWindowHandle::Web(_) => {
                    panic!("TODO: web is not configured")
                }
                RawWindowHandle::AndroidNdk(_) => {
                    panic!("TODO: android ndk is not configured")
                }
                _ => {
                    panic!("unhandled surface")
                }
            }
            assert!(self.active_gpu != ptr::null_mut());


            // image count
            let mut image_count = if desc.image_count == 0 {
                2
            } else {
                desc.image_count
            };
            let caps =
                surface_loader.get_physical_device_surface_capabilities(self.active_gpu, surface_khr).unwrap();

            if caps.max_image_count > 0 && desc.image_count > caps.max_image_count {
                warn!(
                    "Changed requested SwapChain image {} to maximum allowed SwaphChain image {}",
                    desc.image_count, caps.max_image_count
                );
                image_count = caps.max_image_count;
            }

            if desc.image_count < caps.min_image_count {
                warn!(
                    "Changed requested SwapChain image {} to maximum allowed SwaphChain image {}",
                    desc.image_count, caps.min_image_count
                );
                image_count = caps.min_image_count;
            }

            // Surface format
            // select a surface format, depending on whether HRD is available.
            // let mut surface_format = ffi::vk::VkSurfaceFormatKHR {
            //     format: ffi::vk::VkFormat_VK_FORMAT_UNDEFINED,
            //     colorSpace: ffi::vk::VkColorSpaceKHR_VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
            // };

            let mut surface_format =  ash::vk::SurfaceFormat2KHR::builder()
                .surface_format(ash::vk::SurfaceFormatKHR::builder()
                    .color_space(ash::vk::ColorSpaceKHR::SRGB_NONLINEAR).build());
            let mut formats =
                surface_loader.get_physical_device_surface_formats(self.active_gpu, surface_khr).unwrap();

            if formats.len() == 1 && formats[0].color_space == 0
            {
                surface_format = surface_format.surface_format(
                    ash::vk::SurfaceFormatKHR::builder()
                        .format(ash::vk::Format::B8G8R8A8_UNORM)
                        .color_space(ash::vk::ColorSpaceKHR::SRGB_NONLINEAR).build());
            } else {
                let hrd_surface_format = ash::vk::SurfaceFormatKHR::builder()
                    .format(ash::vk::Format::A2B10G10R10_UNORM_PACK32)
                    .color_space(ash::vk::ColorSpaceKHR::HDR10_ST2084_EXT).build();

                let requested_format: ash::vk::Format = desc.color_format.to_vk_format();
                let requested_color_space = if requested_format == hrd_surface_format.format {
                    hrd_surface_format.color_space
                } else {
                    ash::vk::ColorSpaceKHR::SRGB_NONLINEAR
                };

                for format in formats {
                    if format.format == requested_format
                        && requested_color_space == format.colorSpace
                    {
                        surface_format = surface_format.surface_format(
                            ash::vk::SurfaceFormatKHR::builder()
                                .format(requested_format)
                                .color_space(requested_color_space).build()
                        );
                        break;
                    }
                }
            }


            let mut present_mode: ash::vk::PresentModeKHR = ash::vk::PresentModeKHR::FIFO;
            let mut modes = surface_loader.get_physical_device_surface_present_modes(self.active_gpu, surface_khr).unwrap();

            let preferred_modes = if desc.enable_vsync {
                [
                    ash::vk::PresentModeKHR::FIFO_RELAXED,
                    ash::vk::PresentModeKHR::FIFO
                ]
                .as_slice()
            } else {
                [
                    ash::vk::PresentModeKHR::IMMEDIATE,
                    ash::vk::PresentModeKHR::MAILBOX,
                    ash::vk::PresentModeKHR::FIFO_RELAXED,
                    ash::vk::PresentModeKHR::FIFO,
                ]
                .as_slice()
            };

            for preferred_mode in preferred_modes {
                for mode in &modes {
                    if mode == preferred_mode {
                        present_mode = *mode;
                        break;
                    }
                }
            }

            // swap chain
            let mut extent = ash::vk::Extent2D {
                width: max(
                    min(caps.min_image_extent.width, desc.width),
                    caps.max_image_extent.width,
                ),
                height: max(
                    min(caps.min_image_extent.height, desc.width),
                    caps.max_image_extent.height,
                ),
            };


            let sharing_mode: ash::vk::SharingMode = ash::vk::SharingMode::EXCLUSIVE;
            let mut present_queue_family_index: Option<u32> = None;
            let mut family_properties = self.instance.get_physical_device_queue_family_properties(self.active_gpu);
            // Check if hardware provides dedicated present queue
            if family_properties.len() > 0 {
                for (index, property) in family_properties.iter().enumerate() {
                    let supported = surface_loader.get_physical_device_surface_support(self.active_gpu,
                                                                       index as u32,
                                                                       surface_khr).unwrap();

                    if supported && desc.presentation_queues[0].family_index != index as u32
                    {
                        present_queue_family_index = Some(index as u32);
                        break;
                    }
                }
                if present_queue_family_index == None {
                    for (index, property) in family_properties.iter().enumerate() {
                        let supported = surface_loader.get_physical_device_surface_support(self.active_gpu,
                                                                                           index as u32,
                                                                                           surface_khr).unwrap();
                        if supported {
                            present_queue_family_index = Some(index as u32);
                            break;
                        }
                    }
                }
            }

            // Find if gpu has a dedicated present queue
            // let mut present_queue: ffi::vk::VkQueue = ptr::null_mut();
            let mut queue_family_index_count = 0;
            let mut queue_family_indices = [desc.presentation_queues[0].family_index, 0];
            let mut final_present_queue_family_index = 0;
            let mut present_queue = if present_queue_family_index != None
                && Some(queue_family_indices[0]) != present_queue_family_index
            {
                queue_family_index_count = 1;
                final_present_queue_family_index = present_queue_family_index.unwrap();
                queue_family_indices[0] = final_present_queue_family_index;
                self.device.get_device_queue(queue_family_indices[0], 0)
            } else {
                final_present_queue_family_index = queue_family_indices[0];
                // present_queue = ptr::null_mut();
                ash::vk::Queue::null()
            };

            let mut pre_transform: ash::vk::SurfaceTransformFlagsKHR = ash::vk::SurfaceTransformFlagsKHR::empty();
            if caps.supported_transforms.contains(ash::vk::SurfaceTransformFlagsKHR::IDENTITY) {
                pre_transform = ash::vk::SurfaceTransformFlagsKHR::IDENTITY;
            } else {
                pre_transform = caps.current_transform;
            }
            let mut composite_alpha: ash::vk::CompositeAlphaFlagsKHR = (ash::vk::CompositeAlphaFlagsKHR::OPAQUE |
            ash::vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED |
                ash::vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED |
                ash::vk::CompositeAlphaFlagsKHR::INHERIT);


            for flag in [
                ash::vk::CompositeAlphaFlagsKHR::INHERIT,
                ash::vk::CompositeAlphaFlagsKHR::OPAQUE,
                ash::vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED,
                ash::vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED,
            ] {
                if caps.supported_composite_alpha.contains(flag) {
                    composite_alpha = flag;
                    break;
                }
            }

            assert!(composite_alpha.contains(ash::vk::CompositeAlphaFlagsKHR::OPAQUE |
                ash::vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED |
                ash::vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED |
                ash::vk::CompositeAlphaFlagsKHR::INHERIT));


            let mut vk_swap_chain: ash::vk::SwapchainKHR = ash::vk::SwapchainKHR::null();
            let mut swap_chain_info = ash::vk::SwapchainCreateInfoKHR::builder()
                .surface(surface_khr)
                .min_image_count(desc.image_count)
                .image_format(surface_format.surface_format.format)
                .image_color_space(surface_format.surface_format.color_space)
                .image_extent(extent)
                .image_array_layers(1)
                .image_usage(
                    ash::vk::ImageUsageFlags::COLOR_ATTACHMENT |
                        ash::vk::ImageUsageFlags::TRANSFER_SRC
                )
                .image_sharing_mode(sharing_mode)
                .queue_family_indices(&queue_family_indices[0..queue_family_index_count])
                .pre_transform(pre_transform)
                .composite_alpha(composite_alpha)
                .present_mode(present_mode)
                .clipped(true);
            //
            // let mut swap_chain_info = ffi::vk::VkSwapchainCreateInfoKHR {
            //     sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
            //     pNext: ptr::null_mut(),
            //     flags: 0,
            //     surface: vk_surface,
            //     minImageCount: desc.image_count,
            //     imageFormat: surface_format.format,
            //     imageColorSpace: surface_format.colorSpace,
            //     imageExtent: extent,
            //     imageArrayLayers: 1,
            //     imageUsage: ffi::vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT
            //         | ffi::vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
            //     imageSharingMode: sharing_mode,
            //     queueFamilyIndexCount: queue_family_index_count,
            //     pQueueFamilyIndices: queue_family_indices.as_mut_ptr(),
            //     preTransform: pre_transform,
            //     compositeAlpha: composite_alpha,
            //     presentMode: present_mode,
            //     clipped: ffi::vk::VK_TRUE,
            //     oldSwapchain: ptr::null_mut(),
            // };
            // {
            //     let result = ffi::vk::vkCreateSwapchainKHR(
            //         self.device,
            //         &mut swap_chain_info,
            //         ptr::null_mut(),
            //         &mut vk_swap_chain,
            //     );
            //     if result != ffi::vk::VkResult_VK_SUCCESS {
            //         return Err(VulkanError(result));
            //     }
            // }
            let vk_swapChain = self.swapchain_loader.create_swapchain(&swap_chain_info, None).unwrap();


            return Err(VulkanError(0));
        }
    }
    unsafe fn add_sampler(&self, desc: &SamplerDesc) -> RendererResult<super::VulkanSampler> {
        assert!(self.device != ptr::null_mut());
        let mut add_info =  ash::vk::SamplerCreateInfo::builder()
            .mag_filter(desc.mag_filter.to_vk_filter())
            .min_filter(desc.min_filter.to_vk_filter())
            .mipmap_mode(desc.mode.to_vk_map_map_mode())
            .address_mode_u(desc.address_u.to_vk_address_mode())
            .address_mode_v(desc.address_v.to_vk_address_mode())
            .address_mode_w(desc.address_w.to_vk_address_mode())
            .mip_lod_bias(desc.mip_load_bias)
            .anisotropy_enable(desc.max_anisotropy > 0.0)
            .max_anisotropy(desc.max_anisotropy)
            .compare_enable(match &desc.compare_func {
                CompareMode::NEVER => true,
                _ => false
            })
            .compare_op(desc.compare_func.to_comparison_vk())
            .max_lod(match &desc.mode {
                MipMapMode::Nearest => 0.0,
                MipMapMode::Linear => f32::MAX,
            })
            .border_color(ash::vk::BorderColor::FLOAT_TRANSPARENT_BLACK)
            .unnormalized_coordinates(false);

        let sampler_vk = self.device.create_sampler(&add_info, None).unwrap();
        let mut sampler = VulkanSampler {
            renderer: self.me.clone().upgrade().unwrap(),
            sampler: sampler_vk,
        };
        Ok(sampler)
    }

    unsafe fn add_render_target(
        &self,
        desc: &RenderTargetDesc,
    ) -> RendererResult<VulkanRenderTarget> {
        let is_depth = desc.format.is_depth_only() || desc.format.is_depth_and_stencil_only();
        assert!(
            !(is_depth
                && desc
                    .descriptors
                    .contains(DescriptorType::DESCRIPTOR_TYPE_RW_BUFFER)),
            "Cannot use depth stencil as UAV"
        );

        let mip_levels = max(desc.mip_levels, 1);
        let array_size = desc.array_size;
        let depth_or_array_size = array_size * desc.depth;
        let mut num_rtvs = desc.mip_levels;

        if desc
            .descriptors
            .contains(DescriptorType::DESCRIPTOR_TYPE_RENDER_TARGET_ARRAY_SLICES)
            || desc
                .descriptors
                .contains(DescriptorType::DESCRIPTOR_TYPE_RENDER_TARGET_DEPTH_SLICES)
        {
            num_rtvs *= depth_or_array_size;
        }

        todo!()
    }

    unsafe fn add_texture(&mut self, desc: &TextureDesc) -> RendererResult<VulkanTexture> {
        assert!(desc.width > 0 && desc.height > 0 && (desc.depth > 0 || desc.array_size > 0));
        let sample_count: u32 = desc.sample_count as u32;
        if (desc.sample_count as u32) > (SampleCount::SampleCount1 as u32) && desc.mip_levels > 1 {
            error!("Multi-Sampled textures cannot have mip maps");
            assert!(false);
            return Err(RendererError::Unhandled);
        }

        let mut texture = VulkanTexture {
            vk_srv_descriptor: ash::vk::ImageView::null(),
            vk_uav_descriptors: vec![],
            vk_srv_stencil_descriptor: ash::vk::ImageView::null(),
            vk_image: ash::vk::Image::null(),
            allocation: Default::default(),
            vk_device_memory: Default::default(),
            width: 0,
            height: 0,
            depth: 0,
            mip_levels: 0,
            array_size_minus_one: 0,
            format: UNDEFINED,
            aspect_mask: 0,
            node_index: 0,
            uav: false,
            own_image: false,
        };

        if desc.native_handle != ptr::null_mut()
            && !desc
                .flags
                .contains(TextureCreationFlags::TEXTURE_CREATION_FLAG_IMPORT_BIT)
        {
            texture.own_image = false;
            texture.vk_image = desc.native_handle as ash::vk::Image;
        } else {
            texture.own_image = true;
        }


        let mut additional_flag: ash::vk::ImageUsageFlags = ash::vk::ImageUsageFlags::empty();
        if desc.start_state.contains(ResourceState::RENDER_TARGET) {
            additional_flag |= ash::vk::ImageUsageFlags::COLOR_ATTACHMENT;
        } else if desc.start_state.contains(ResourceState::DEPTH_WRITE) {
            additional_flag |= ash::vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT;
        }

        let array_size: u32 = desc.array_size;
        let mut image_type: ash::vk::ImageType = if desc
            .flags
            .contains(TextureCreationFlags::TEXTURE_CREATION_FLAG_FORCE_2D)
        {
            assert_eq!(desc.depth, 1);
            ash::vk::ImageType::TYPE_2D
        } else if desc
            .flags
            .contains(TextureCreationFlags::TEXTURE_CREATION_FLAG_FORCE_3D)
        {
            ash::vk::ImageType::TYPE_3D
        } else {
            if desc.depth > 1 {
                ash::vk::ImageType::TYPE_3D
            } else if desc.height > 1 {
                ash::vk::ImageType::TYPE_2D
            } else {
                ash::vk::ImageType::TYPE_1D
            }
        };

        let mut descriptors = desc.descriptors;
        let mut cubemap_required: bool =
            descriptors.contains(DescriptorType::DESCRIPTOR_TYPE_TEXTURE_CUBE);
        let mut array_required: bool = false;

        let is_planer: bool = desc.format.is_planer();
        let num_planes: u32 = desc.format.num_planes();
        let is_single_plane: bool = desc.format.is_single_plane();
        assert!((num_planes == 1 && is_single_plane) && (!is_single_plane && num_planes > 1 && num_planes <= 3),
            "Number of planes for multi-planar formats must be 2 or 3 and for single-planar formats it must be 1.");

        if image_type == ash::vk::ImageType::TYPE_3D {
            array_required = true;
        }

        if texture.vk_image == ptr::null_mut() {
            let target_format = desc.format.to_vk_format();
            let format_props = self.instance.get_physical_device_format_properties(self.active_gpu, target_format);
            let mut add_info = ash::vk::ImageCreateInfo::builder()
                .flags({
                    let mut result: ash::vk::ImageCreateFlags = ash::vk::ImageCreateFlags::empty();
                    if cubemap_required {
                        result |= ash::vk::ImageCreateFlags::CUBE_COMPATIBLE;

                    }
                    if array_required {
                        result |= ash::vk::ImageCreateFlags::TYPE_2D_ARRAY_COMPATIBLE;
                    }

                    // multi-planar formats must have each plane separately bound to memory, rather than having a single memory binding for the whole image
                    if is_planer {
                        assert!(format_props.optimal_tiling_features.contains(ash::vk::FormatFeatureFlags::DISJOINT));
                        add_info.flags |= ash::vk::ImageCreateFlags::DISJOINT;
                    }
                    result
                })
                .image_type(image_type)
                .format(target_format)
                .extent(ash::vk::Extent3D {
                    width: desc.width,
                    height: desc.height,
                    depth: desc.depth,
                })
                .mip_levels(desc.mip_levels)
                .array_layers(array_size)
                .samples(desc.sample_count.to_vk_sample_count())
                .tiling(ash::vk::ImageTiling::OPTIMAL)
                .usage({
                    let mut usage = descriptors.to_vk_usage() | additional_flag;
                    if usage.contains(ash::vk::ImageUsageFlags::SAMPLED) ||
                        usage.contains(ash::vk::ImageUsageFlags::STORAGE) {
                        usage |= ash::vk::ImageUsageFlags::TRANSFER_DST;
                    }
                    let format_features = util_vk_image_usage_to_format_features(usage);
                    assert!(
                        format_features.intersects(format_props.optimal_tiling_features),
                        "Format is not supported for GPU local images (i.e. not host visible images)"
                    );
                    usage
                })
                .sharing_mode(SharingMode::EXCLUSIVE);


            let mut mem_reqs = ffi::vk::VmaAllocationCreateInfo {
                flags: 0,
                usage: ffi::vk::VmaMemoryUsage_VMA_MEMORY_USAGE_GPU_ONLY,
                requiredFlags: 0,
                preferredFlags: 0,
                memoryTypeBits: 0,
                pool: ptr::null_mut(),
                pUserData: ptr::null_mut(),
                priority: 0.0,
            };

            let mut external_info = ash::vk::ExternalMemoryImageCreateInfoKHR::builder();
            let mut export_memory_info = ash::vk::ExportMemoryAllocateInfoKHR::builder();

            if self
                .features
                .contains(VulkanSupportedFeatures::EXTERNAL_MEMORY_EXTENSION)
                && desc
                    .flags
                    .contains(TextureCreationFlags::TEXTURE_CREATION_FLAG_IMPORT_BIT)
            {
                add_info = add_info.push_next(&mut external_info);
            } else if self.features.contains(VulkanSupportedFeatures::EXTERNAL_MEMORY_EXTENSION)
                && desc.flags.contains(TextureCreationFlags::TEXTURE_CREATION_FLAG_EXPORT_BIT)
            {
                mem_reqs.pUserData = ((&export_memory_info)
                    as *const ffi::vk::VkExportMemoryAllocateInfoKHR)
                    as *mut c_void;
                mem_reqs.flags |=
                    ffi::vk::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT;
            }

            if is_single_plane {
                let mut vk_image = self.device.create_image(&add_info, None).unwrap();
                let mut requirements = self.device.get_image_memory_requirements(vk_image);

                let allocation = self.allocator.allocate(&AllocationCreateDesc{
                    name: desc.name.to_str().unwrap(),
                    requirements,
                    location: MemoryLocation::CpuToGpu,
                    linear: true
                }).unwrap();
                self.device.bind_image_memory(vk_image, allocation.memory(), allocation.offset());
                texture.allocation = allocation;
                texture.vk_image = vk_image;
            } else {
                // Create info requires the mutable format flag set for multi planar images
                // Also pass the format list for mutable formats as per recommendation from the spec
                // Might help to keep DCC enabled if we ever use this as a output format
                // DCC gets disabled when we pass mutable format bit to the create info. Passing the format list helps the driver to enable it

                let mut planer_format = desc.format.to_vk_format();
                let mut format_list = ash::vk::ImageFormatListCreateInfoKHR::builder()
                    .view_formats(&[planer_format]);

                (*add_info).flags |= ash::vk::ImageCreateFlags::MUTABLE_FORMAT;
                add_info = add_info.push_next(&mut format_list);
                texture.vk_image = self.device.create_image(&mut add_info, None).unwrap();
                let memory_requirement = util_get_planar_vk_image_memory_requirement(
                    &self.device,
                    texture.vk_image,
                    num_planes,
                );

                let mem_props = self.instance.get_physical_device_memory_properties(self.active_gpu);
                let mut mem_alloc_info = ash::vk::MemoryAllocateInfo::builder()
                    .allocation_size(memory_requirement.requirement.size)
                    .memory_type_index(match util_get_memory_type(
                        mem_reqs.memoryTypeBits,
                        &mem_props,
                        ffi::vk::VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
                    ) {
                        Some(i) => i,
                        None => {
                            error!("could not find a matching memory type");
                            assert!(false);
                            0
                        }
                    });

                texture.vk_device_memory = self.device.allocate_memory(&mem_alloc_info, None).unwrap();

                let mut bind_images_memory_info: Box<[ash::vk::BindImageMemoryInfo]> = vec![ash::vk::BindImageMemoryInfo::default(); num_planes as usize].into_boxed_slice();
                let mut bind_image_planes_memory_info: Box<[ash::vk::BindImagePlaneMemoryInfo]> = vec![ash::vk::BindImagePlaneMemoryInfo::default(); num_planes as usize].into_boxed_slice();

                for i in 0..num_planes {
                    let mut bind_image_plane_memory_info =
                        ash::vk::BindImagePlaneMemoryInfo::builder()
                            .plane_aspect(ash::vk::ImageAspectFlags::PLANE_0 << i).build();
                    bind_image_planes_memory_info[i] = bind_image_plane_memory_info;

                    let mut memory_info = ash::vk::BindImageMemoryInfo::builder()
                        .image(texture.vk_image)
                        .memory(texture.vk_device_memory)
                        .memory_offset(memory_requirement.planar_offset[i as usize] as DeviceSize)
                        .push_next(&mut bind_image_planes_memory_info[i]).build();
                    bind_images_memory_info[i] = memory_info;

                }
                self.device.bind_image_memory2(bind_images_memory_info.as_slice());
            }
        }

        let view_type: ash::vk::ImageViewType = match image_type {
            ash::vk::ImageType::TYPE_1D => {
                if array_size > 1 {
                    ash::vk::ImageViewType::TYPE_1D_ARRAY
                } else {
                    ash::vk::ImageViewType::TYPE_1D
                }
            }
            ash::vk::ImageType::TYPE_2D => {
                if cubemap_required {
                    if array_size > 6 {
                        ash::vk::ImageViewType::CUBE_ARRAY
                    } else {
                        ash::vk::ImageViewType::CUBE
                    }
                } else {
                    if array_size > 1 {
                        ash::vk::ImageViewType::TYPE_2D_ARRAY
                    } else {
                        ash::vk::ImageViewType::TYPE_2D
                    }
                }
            }
            ash::vk::ImageType::TYPE_3D => {
                if array_size > 1 {
                    error!("Cannot support 3D Texture Array in Vulkan");
                    assert!(false);
                }
                ash::vk::ImageViewType::TYPE_3D
            }
            _ => {
                panic!("Image Format not supported")
            }
        };

        let mut srv_desc = ash::vk::ImageViewCreateInfo::builder()
            .image(texture.vk_image)
            .view_type(view_type)
            .format(desc.format.to_vk_format())
            .components(ash::vk::ComponentMapping{
                r: ash::vk::ComponentSwizzle::R,
                g: ash::vk::ComponentSwizzle::G,
                b: ash::vk::ComponentSwizzle::B,
                a: ash::vk::ComponentSwizzle::A
            })
            .subresource_range(ash::vk::ImageSubresourceRange {
                aspect_mask: desc.format.to_vk_aspect_mask(true),
                base_mip_level: 0,
                level_count: desc.mip_levels,
                base_array_layer: 0,
                layer_count: array_size
            });

        if descriptors.contains(DescriptorType::DESCRIPTOR_TYPE_TEXTURE) {
            texture.vk_srv_descriptor = self.device.create_image_view(
                &srv_desc,
                None
            ).unwrap();
        }
        if desc.format.is_stencil() && descriptors.contains(DescriptorType::DESCRIPTOR_TYPE_TEXTURE)
        {
            srv_desc.subresource_range.aspect_mask = ash::vk::ImageAspectFlags::STENCIL;
            texture.vk_srv_stencil_descriptor = self.device.create_image_view(
                &srv_desc,
                None
            ).unwrap();
        }

        if descriptors.contains(DescriptorType::DESCRIPTOR_TYPE_RW_TEXTURE) {
            // let mut uav_desc: ffi::vk::VkImageViewCreateInfo = ImageViewCreateInfo::builder();
            // uav_desc.inner = srv_desc;
            let mut uav_desc = srv_desc;
            if srv_desc.view_type == ash::vk::ImageViewType::CUBE_ARRAY
                || srv_desc.view_type == ash::vk::ImageViewType::CUBE_ARRAY {
                uav_desc = uav_desc.view_type(ash::vk::ImageViewType::TYPE_2D_ARRAY);
            }

            // #NOTE : We dont support imageCube, imageCubeArray for consistency with other APIs
            // All cubemaps will be used as image2DArray for Image Load / Store ops
            if uav_desc.viewType == ffi::vk::VkImageViewType_VK_IMAGE_VIEW_TYPE_CUBE_ARRAY
                || uav_desc.viewType == ffi::vk::VkImageViewType_VK_IMAGE_VIEW_TYPE_CUBE
            {
                uav_desc.viewType = ffi::vk::VkImageViewType_VK_IMAGE_VIEW_TYPE_2D_ARRAY;
            }
            texture.vk_uav_descriptors = vec![ash::vk::ImageView::null(); desc.mip_levels as usize];
            for i in 0..desc.mip_levels {
                uav_desc.subresourceRange.baseMipLevel = i;
                texture.vk_uav_descriptors[i as usize]  = self.device.create_image_view(&uav_desc, None).unwrap();
            }
        }

        texture.node_index = desc.node_index;
        texture.width = desc.width;
        texture.height = desc.height;
        texture.depth = desc.depth;
        texture.mip_levels = desc.mip_levels;
        texture.uav = desc
            .descriptors
            .contains(DescriptorType::DESCRIPTOR_TYPE_RW_TEXTURE);
        texture.array_size_minus_one = array_size - 1;
        texture.format = desc.format;

        Ok(texture)
    }

    unsafe fn add_root_signature(
        &self,
        desc: &RootSignatureDesc<VulkanAPI>,
    ) -> RendererResult<super::VulkanRootSignature> {
        let mut root_signature = VulkanRootSignature {

        };

        for shader in &desc.shader {

        }

        // let mut add_info = ffi::vk::VkPipelineLayoutCreateInfo {
        //
        // };

        todo!()

    }

    unsafe fn add_shader_binary(&self, desc: &BinaryShaderDesc) -> RendererResult<VulkanShader> {
        assert!(self.device != ptr::null_mut());

        let mut shader = VulkanShader {
            render: self.me.clone().upgrade().unwrap(),
            stages: desc.stages,
            shader_module: vec![]
        };
        // let mut init_shader = |desc: &BinaryShaderStageDesc| {
        //     let mut create_info = ffi::vk::VkShaderModuleCreateInfo {
        //         sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
        //         pNext: ptr::null_mut(),
        //         flags: 0,
        //         codeSize: desc.byte_code_size as usize,
        //         pCode: desc.byte_code
        //     };
        //     // let mut module: ffi::vk::VkShaderModule = ptr::null_mut();
        //     // let mut ast = spirv::Ast::<glsl::Target>::parse(&module)?;
        //     // let resources = ast.get_shader_resources();
        //
        //     check_vk_result!(ffi::vk::vkCreateShaderModule(self.device, &mut create_info, ptr::null_mut(), &mut module));
        //     module
        // };
        // for shader_desc in &desc.shaders {
        //     let mut module = spirv::Module::from_words(std::slice::from_raw_parts(shader_desc.byte_code, shader_desc.byte_code_size as usize));
        //     let mut ast = spirv::Ast::<glsl::Target>::parse(&module)?;
        //     let resource = ast.get_shader_resources()?;
        //     // resource.separate_samplers[0].id
        //
        // }

        for stage in [
            ShaderStage::Vertex,
            ShaderStage::TesselationControl,
            ShaderStage::TesselationEvaluation,
            ShaderStage::Geometry,
            ShaderStage::Fragment,
            ShaderStage::Compute,
            ShaderStage::Raytracing,
        ] {
            let stage_bit = ShaderStageFlags::from_bits(1 << (stage as u32)).unwrap();
            if desc.stages.contains(stage_bit) {



                // let mut desc_stage = match stage {
                //     ShaderStage::Vertex => &desc.vert,
                //     ShaderStage::TesselationControl => &desc.hull,
                //     ShaderStage::TesselationEvaluation => &desc.domain,
                //     ShaderStage::Geometry => &desc.geom,
                //     ShaderStage::Fragment => &desc.frag,
                //     ShaderStage::Raytracing | ShaderStage::Compute => &desc.comp,
                //     _ => panic!("logic error"),
                // };
                // match desc_stage {
                //     None => {}
                //     Some(c) => {
                //
                //         let mut module = spirv::Module::from_words(std::slice::from_raw_parts(c.byte_code, c.byte_code_size as usize));
                //         let mut ast = spirv::Ast::<glsl::Target>::parse(&module)?;
                //
                //         shader.shader_module[stage as usize] = Some((c.entry_point_name.clone(), init_shader(&c)));
                //     }
                // }
            }
        }
        Ok(shader)
    }

    unsafe fn drop_swap_chain(&self) {
        todo!()
    }

    unsafe fn remove_render_target(&self, _target: &mut VulkanRenderTarget) {
        todo!()
    }

    unsafe fn remove_root_signature(&self, signature: &mut super::VulkanRootSignature) {
        todo!()
    }

    unsafe fn get_common_info(&self) -> &GPUCommonInfo {
        return self
            .active_gpu_common_info
            .as_ref()
            .expect("render not initialized")
            .borrow();
    }

    unsafe fn add_buffer(&mut self, desc: &BufferDesc) -> RendererResult<super::VulkanBuffer> {
        assert!(desc.size > 0);
        assert!(self.device != ptr::null_mut());
        assert!(!self.active_gpu_common_info.is_none());

        let common_info = self.active_gpu_common_info.as_ref().unwrap();

        let mut allocated_size = desc.size;

        if desc
            .descriptors
            .contains(DescriptorType::DESCRIPTOR_TYPE_UNIFORM_BUFFER)
        {
            let min_alignment = common_info.uniform_buffer_alignment;
            allocated_size = forge_math::round_up(allocated_size, min_alignment as u64);
        }
        let mut add_info = ash::vk::BufferCreateInfo::builder()
            .size(allocated_size)
            .usage({
                let mut usage = desc
                    .descriptors
                    .to_vk_buffer_usage(desc.format != ImageFormat::UNDEFINED);
                if desc.memory_usage == ResourceMemoryUsage::GpuOnly
                    || desc.memory_usage == ResourceMemoryUsage::Unknown
                {
                    usage |= BufferUsageFlags::TRANSFER_DST;
                }
                usage
            })
            .sharing_mode(SharingMode::EXCLUSIVE);

        let mut vk_buffer = self.device.create_buffer(&add_info, None).unwrap();
        let mut requirements = self.device.get_buffer_memory_requirements(vk_buffer);

        let allocation = self.allocator.allocate(&AllocationCreateDesc {
            name: desc.debug_name.to_str().unwrap(),
            requirements,
            location: MemoryLocation::CpuToGpu,
            linear: true
        }).unwrap();

        self.device.bind_buffer_memory(vk_buffer, allocation.memory(), allocation.offset()).unwrap();

        let mut buffer: VulkanBuffer = VulkanBuffer {
            renderer: self.me.clone().upgrade().unwrap(),
            vk_buffer,
            vk_storage_texel_view: ptr::null_mut(),
            vk_uniform_texel_view: ptr::null_mut(),
            allocation,
            offset: 0,
            size: 0,
            descriptors: DescriptorType::DESCRIPTOR_TYPE_UNDEFINED,
            memory_usage: ResourceMemoryUsage::Unknown,
            node_index: 0,
        };


        //
        // let mut vma_mem_reqs = ffi::vk::VmaAllocationCreateInfo {
        //     flags: {
        //         let mut result = 0;
        //         if desc
        //             .flags
        //             .contains(BufferCreationFlag::BUFFER_CREATION_FLAG_OWN_MEMORY_BIT)
        //         {
        //             result |= ffi::vk::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT;
        //         }
        //         if desc
        //             .flags
        //             .contains(BufferCreationFlag::BUFFER_CREATION_FLAG_PERSISTENT_MAP_BIT)
        //         {
        //             result |= ffi::vk::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_MAPPED_BIT;
        //         }
        //         result
        //     },
        //     usage: desc.memory_usage.to_vma_usage(),
        //     requiredFlags: {
        //         let mut result = 0;
        //         if desc
        //             .flags
        //             .contains(BufferCreationFlag::BUFFER_CREATION_FLAG_HOST_VISIBLE)
        //         {
        //             result |= ffi::vk::VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT;
        //         }
        //         if desc
        //             .flags
        //             .contains(BufferCreationFlag::BUFFER_CREATION_FLAG_HOST_COHERENT)
        //         {
        //             result |=
        //                 ffi::vk::VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_HOST_COHERENT_BIT;
        //         }
        //         result
        //     },
        //     preferredFlags: 0,
        //     memoryTypeBits: 0,
        //     pool: ptr::null_mut(),
        //     pUserData: ptr::null_mut(),
        //     priority: 0.0,
        // };

        //
        // let mut alloc_info: ffi::vk::VmaAllocationInfo = MaybeUninit::zeroed().assume_init();
        // {
        //     let result = ffi::vk::vmaCreateBuffer(
        //         self.vma_allocator,
        //         &mut add_info.build(),
        //         &mut vma_mem_reqs,
        //         &mut buffer.vk_buffer,
        //         &mut buffer.vma_allocation,
        //         &mut alloc_info,
        //     );
        //     if result != ffi::vk::VkResult_VK_SUCCESS {
        //         return Err(VulkanError(result));
        //     }
        // }

        // buffer.mapping_address = alloc_info.pMappedData;

        // if desc
        //     .descriptors
        //     .contains(DescriptorType::DESCRIPTOR_TYPE_UNIFORM_BUFFER)
        //     || desc
        //         .descriptors
        //         .contains(DescriptorType::DESCRIPTOR_TYPE_BUFFER)
        //     || desc
        //         .descriptors
        //         .contains(DescriptorType::DESCRIPTOR_TYPE_RW_BUFFER)
        // {
        //     buffer.offset = desc.structure_stride * desc.first_element;
        // }
        //
        // if add_info.usage & ffi::vk::VkBufferUsageFlagBits_VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT
        //     > 0
        // {
        //     let mut view_info = ffi::vk::VkBufferViewCreateInfo {
        //         sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO,
        //         pNext: ptr::null_mut(),
        //         flags: 0,
        //         buffer: ptr::null_mut(),
        //         format: desc.format.to_vk_format(),
        //         offset: desc.first_element * desc.structure_stride,
        //         range: desc.element_count * desc.structure_stride,
        //     };
        //     let mut format_properties: ffi::vk::VkFormatProperties =
        //         MaybeUninit::zeroed().assume_init();
        //     ffi::vk::vkGetPhysicalDeviceFormatProperties(
        //         self.active_gpu,
        //         view_info.format,
        //         &mut format_properties,
        //     );
        //     if !(format_properties.bufferFeatures
        //         & ffi::vk::VkFormatFeatureFlagBits_VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
        //         > 0)
        //     {
        //         warn!(
        //             "Failed to create uniform texel buffer view for format {}",
        //             desc.format as u32
        //         );
        //     } else {
        //         check_vk_result!(ffi::vk::vkCreateBufferView(
        //             self.device,
        //             &mut view_info,
        //             ptr::null_mut(),
        //             &mut buffer.vk_uniform_texel_view
        //         ));
        //     }
        // }
        //
        // if add_info.usage & ffi::vk::VkBufferUsageFlagBits_VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT
        //     > 0
        // {
        //     let mut view_info = ffi::vk::VkBufferViewCreateInfo {
        //         sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO,
        //         pNext: ptr::null_mut(),
        //         flags: 0,
        //         buffer: ptr::null_mut(),
        //         format: desc.format.to_vk_format(),
        //         offset: desc.first_element * desc.structure_stride,
        //         range: desc.element_count * desc.structure_stride,
        //     };
        //     let mut format_properties: ffi::vk::VkFormatProperties =
        //         MaybeUninit::zeroed().assume_init();
        //     ffi::vk::vkGetPhysicalDeviceFormatProperties(
        //         self.active_gpu,
        //         view_info.format,
        //         &mut format_properties,
        //     );
        //     if !(format_properties.bufferFeatures
        //         & ffi::vk::VkBufferUsageFlagBits_VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT
        //         > 0)
        //     {
        //         warn!(
        //             "Failed to create uniform texel buffer view for format {}",
        //             desc.format as u32
        //         );
        //     } else {
        //         check_vk_result!(ffi::vk::vkCreateBufferView(
        //             self.device,
        //             &mut view_info,
        //             ptr::null_mut(),
        //             &mut buffer.vk_storage_texel_view
        //         ));
        //     }
        // }
        //
        // buffer.size = desc.size;
        // buffer.memory_usage = desc.memory_usage;
        // buffer.node_index = desc.node_index;
        // buffer.descriptors = desc.descriptors;
        Ok(buffer)
    }

}
