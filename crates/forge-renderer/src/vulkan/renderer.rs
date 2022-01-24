use crate::{check_vk_result, desc::{CmdDesc, RenderDescImp}, error::RendererError::VulkanError, ffi, types::{CompareMode, DescriptorType, FilterType, MipMapMode, QueueType, ResourceMemoryUsage}, vulkan::{
    device::VulkanGPUInfo,
    types::{
        VulkanSupportedFeatures, GLOBAL_INSTANCE_EXTENSIONS, GLOBAL_WANTED_DEVICE_EXTENSIONS,
        MAX_QUEUE_COUNT, MAX_QUEUE_FAMILIES, MAX_QUEUE_FLAGS,
    },
    VulkanBuffer, VulkanCommand, VulkanCommandPool, VulkanPipeline, VulkanQueue,
    VulkanRenderTarget, VulkanRenderer, VulkanSampler, VulkanSemaphore, VulkanSwapChain,
}, BufferDesc, CmdPoolDesc, GPUCommonInfo, QueueDesc, RenderDesc, Renderer, RendererResult, RootSignatureDesc, SamplerDesc, SwapChainDesc, VulkanAPI, RenderTargetDesc, TextureDesc};
use forge_image_format::ImageFormat;
use log::{error, info, log, warn};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::{
    borrow::{Borrow, BorrowMut},
    cmp::{min, Ordering},
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
use std::cmp::max;
use crate::error::RendererError;
use crate::ffi::vk;
use crate::types::{ResourceState, SampleCount, TextureCreationFlags};
use crate::vulkan::VulkanTexture;

struct QueueFamilyResult {
    properties: ffi::vk::VkQueueFamilyProperties,
    family_index: u32,
    queue_index: u32,
}

fn util_vk_image_usage_to_format_features(usage: ffi::vk::VkImageUsageFlags) -> ffi::vk::VkFormatFeatureFlags {
    let mut result: ffi::vk::VkFormatFeatureFlags = 0;
    if ffi::vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_SAMPLED_BIT == (usage & ffi::vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_SAMPLED_BIT) {
        result |= ffi::vk::VkFormatFeatureFlagBits_VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT;
    }
    if ffi::vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_STORAGE_BIT == (usage & ffi::vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_STORAGE_BIT)
    {
        result |= ffi::vk::VkFormatFeatureFlagBits_VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT;
    }
    if ffi::vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT == (usage & ffi::vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT)
    {
        result |= ffi::vk::VkFormatFeatureFlagBits_VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT;
    }
    if ffi::vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT == (usage & ffi::vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT)
    {
        result |= ffi::vk::VkFormatFeatureFlagBits_VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT;
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

    let mut family_property_count: u32 = 0;
    ffi::vk::vkGetPhysicalDeviceQueueFamilyProperties(
        renderer.active_gpu,
        &mut family_property_count,
        ptr::null_mut(),
    );
    let mut family_properties: Vec<ffi::vk::VkQueueFamilyProperties> =
        vec![MaybeUninit::zeroed().assume_init(); family_property_count as usize];
    ffi::vk::vkGetPhysicalDeviceQueueFamilyProperties(
        renderer.active_gpu,
        &mut family_property_count,
        family_properties.as_mut_ptr(),
    );

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

unsafe fn init_instance(renderer: &mut VulkanRenderer, desc: &RenderDesc) -> RendererResult<()> {
    // layers: Vec<const *c_char> = Vec::new();
    let application_name = CString::new("3DEngine").expect("CString::new failed");
    let engine_name = CString::new("3DEngine").expect("CString::new failed");

    let _loaded_extension: Vec<CString> = vec![];

    let mut layer_count: u32 = 0;
    let mut ext_count: u32 = 0;
    ffi::vk::vkEnumerateInstanceLayerProperties(&mut layer_count, ptr::null_mut());
    ffi::vk::vkEnumerateInstanceExtensionProperties(
        ptr::null_mut(),
        &mut ext_count,
        ptr::null_mut(),
    );

    let mut layer_properties: Vec<ffi::vk::VkLayerProperties> =
        vec![MaybeUninit::zeroed().assume_init(); layer_count as usize];
    let mut ext_properties: Vec<ffi::vk::VkExtensionProperties> =
        vec![MaybeUninit::zeroed().assume_init(); ext_count as usize];
    ffi::vk::vkEnumerateInstanceLayerProperties(&mut layer_count, layer_properties.as_mut_ptr());
    ffi::vk::vkEnumerateInstanceExtensionProperties(
        ptr::null_mut(),
        &mut ext_count,
        ext_properties.as_mut_ptr(),
    );

    for layer_property in &layer_properties {
        info!(
            "vkinstance-layer: {}",
            CStr::from_ptr(layer_property.layerName.as_ptr()).to_string_lossy()
        );
    }

    for ext_property in &ext_properties {
        info!(
            "vkinstance-ext: {}",
            CStr::from_ptr(ext_property.extensionName.as_ptr()).to_string_lossy()
        );
    }

    let create_info = ffi::vk::VkApplicationInfo {
        sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
        pNext: ptr::null_mut(),
        pApplicationName: application_name.as_ptr(),
        engineVersion: ffi::vk::vkMakeVersion(1, 0, 0),
        applicationVersion: ffi::vk::vkMakeVersion(1, 0, 0),
        pEngineName: engine_name.as_ptr(),
        apiVersion: ffi::vk::vkMakeVersion(1, 0, 0),
    };

    let mut wanted_instance_extensions: HashSet<CString> = HashSet::new();
    let mut wanted_instance_layers: HashSet<CString> = HashSet::new();
    for ext in GLOBAL_INSTANCE_EXTENSIONS {
        wanted_instance_extensions.insert(CString::from(CStr::from_bytes_with_nul_unchecked(ext)));
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
        ffi::vk::vkEnumerateInstanceExtensionProperties(
            target_layer.as_ptr(),
            &mut layer_target_ext_count,
            ptr::null_mut(),
        );

        let mut layer_target_ext: Vec<ffi::vk::VkExtensionProperties> =
            vec![MaybeUninit::zeroed().assume_init(); ext_count as usize];
        ffi::vk::vkEnumerateInstanceExtensionProperties(
            target_layer.as_ptr(),
            &mut layer_target_ext_count,
            layer_target_ext.as_mut_ptr(),
        );

        wanted_instance_extensions.retain(move |layer| {
            for ext_property in &layer_target_ext {
                let extension_name = CStr::from_ptr(ext_property.extensionName.as_ptr());
                if extension_name.eq(layer) {
                    return true;
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
    let create_info = ffi::vk::VkInstanceCreateInfo {
        sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pNext: ptr::null(),
        flags: 0,
        pApplicationInfo: &create_info,
        enabledLayerCount: enabled_layers.len() as u32,
        ppEnabledLayerNames: enabled_layers.as_ptr(),
        enabledExtensionCount: enabled_extensions.len() as u32,
        ppEnabledExtensionNames: enabled_extensions.as_ptr(),
    };

    check_vk_result!(ffi::vk::vkCreateInstance(
        &create_info,
        ptr::null(),
        &mut renderer.instance
    ));

    Ok(())
}

unsafe fn init_device(renderer: &mut VulkanRenderer, desc: &RenderDesc) -> RendererResult<()> {
    assert!(renderer.instance != ptr::null_mut());

    let devices = VulkanGPUInfo::all(renderer.instance);
    let gpu = VulkanGPUInfo::select_best_gpu(renderer.instance, &devices)?;
    assert!(gpu.get_device() != ptr::null_mut());

    renderer.linked_node_count = 1;
    renderer.active_gpu_properties = Some(gpu.get_device_properties().clone());
    renderer.active_gpu_common_info = {
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
        Some(Box::new(common))
    };
    renderer.active_gpu = gpu.get_device();

    let mut layer_count: u32 = 0;
    let mut ext_count: u32 = 0;
    ffi::vk::vkEnumerateDeviceLayerProperties(
        renderer.active_gpu,
        &mut layer_count,
        ptr::null_mut(),
    );
    ffi::vk::vkEnumerateDeviceExtensionProperties(
        renderer.active_gpu,
        ptr::null_mut(),
        &mut ext_count,
        ptr::null_mut(),
    );

    let mut layers: Vec<ffi::vk::VkLayerProperties> =
        vec![MaybeUninit::zeroed().assume_init(); layer_count as usize];
    ffi::vk::vkEnumerateDeviceLayerProperties(
        renderer.active_gpu,
        &mut layer_count,
        layers.as_mut_ptr(),
    );

    let mut ext: Vec<ffi::vk::VkExtensionProperties> =
        vec![MaybeUninit::zeroed().assume_init(); ext_count as usize];
    ffi::vk::vkEnumerateDeviceExtensionProperties(
        renderer.active_gpu,
        ptr::null_mut(),
        &mut ext_count,
        ext.as_mut_ptr(),
    );

    for layer in &layers {
        let layer_name = CStr::from_ptr(layer.layerName.as_ptr());
        info!("vkdevice-layer: {}", layer_name.to_string_lossy());
        // if layer_name.cmp(CString::new("VK_LAYER_RENDERDOC_Capture").unwrap().as_c_str()) == Ordering::Equal {
        // }
    }

    for ext in &ext {
        let ext_name = CStr::from_ptr(ext.extensionName.as_ptr());
        info!("vkdevice-ext: {}", ext_name.to_string_lossy());
    }

    let mut device_extension_cache: Vec<CString> = Vec::new();
    let mut extension_count: u32 = 0;
    let mut dedicated_allocation_extension: bool = false;
    let mut memory_req2extension: bool = false;
    let mut external_memory_extension: bool = false;
    let mut fragment_shader_interlock_extension: bool = false;
    ffi::vk::vkEnumerateDeviceExtensionProperties(
        renderer.active_gpu,
        ptr::null_mut(),
        &mut extension_count,
        ptr::null_mut(),
    );
    if extension_count > 0 {
        let mut properties: Vec<ffi::vk::VkExtensionProperties> =
            vec![MaybeUninit::zeroed().assume_init(); extension_count as usize];
        let empty_ext = Vec::new();
        let user_extensions: &Vec<CString> = match &desc.imp {
            RenderDescImp::Vulkan(des) => &des.device_extensions,
            _ => &empty_ext,
        };

        let mut wanted_device_extensions: Vec<&CStr> =
            Vec::with_capacity(GLOBAL_WANTED_DEVICE_EXTENSIONS.len() + user_extensions.len());
        for extension in GLOBAL_WANTED_DEVICE_EXTENSIONS {
            wanted_device_extensions.push(CStr::from_bytes_with_nul_unchecked(extension));
        }
        for extension in user_extensions {
            wanted_device_extensions.push(extension);
        }
        ffi::vk::vkEnumerateDeviceExtensionProperties(
            renderer.active_gpu,
            ptr::null_mut(),
            &mut extension_count,
            properties.as_mut_ptr(),
        );
        for property in &properties {
            let current_property_name = CStr::from_ptr(property.extensionName.as_ptr());
            for wanted_extension in &wanted_device_extensions {
                if current_property_name.cmp(wanted_extension) == Ordering::Equal {
                    device_extension_cache.push(CString::from(*wanted_extension));

                    if wanted_extension.cmp(&CStr::from_bytes_with_nul_unchecked(
                        ffi::vk::VK_KHR_DEDICATED_ALLOCATION_EXTENSION_NAME,
                    )) == Ordering::Equal
                    {
                        // dedicated_allocation_extension = true;
                    }
                    if wanted_extension.cmp(&CStr::from_bytes_with_nul_unchecked(
                        ffi::vk::VK_KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME,
                    )) == Ordering::Equal
                    {
                        // memory_req2extension = true;
                    }

                    if wanted_extension.cmp(&CStr::from_bytes_with_nul_unchecked(
                        ffi::vk::VK_KHR_EXTERNAL_MEMORY_EXTENSION_NAME,
                    )) == Ordering::Equal
                    {
                        // external_memory_extension = true;
                    }
                    // #if defined(VK_USE_PLATFORM_WIN32_KHR)
                    // if (strcmp(wantedDeviceExtensions[k], VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME) == 0)
                    // externalMemoryWin32Extension = true;
                    // #endif
                    if wanted_extension.cmp(&CStr::from_bytes_with_nul_unchecked(
                        ffi::vk::VK_KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME,
                    )) == Ordering::Equal
                    {
                        renderer.features |= VulkanSupportedFeatures::DRAW_INDIRECT_COUNT_EXTENSION;
                    }

                    if wanted_extension.cmp(&CStr::from_bytes_with_nul_unchecked(
                        ffi::vk::VK_AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME,
                    )) == Ordering::Equal
                    {
                        renderer.features |=
                            VulkanSupportedFeatures::AMD_DRAW_INDIRECT_COUNT_EXTENSION;
                    }

                    if wanted_extension.cmp(&CStr::from_bytes_with_nul_unchecked(
                        ffi::vk::VK_AMD_GCN_SHADER_EXTENSION_NAME,
                    )) == Ordering::Equal
                    {
                        renderer.features |= VulkanSupportedFeatures::AMD_GCN_SHADEREXTENSION;
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

                    if wanted_extension.cmp(&CStr::from_bytes_with_nul_unchecked(
                        ffi::vk::VK_KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME,
                    )) == Ordering::Equal
                    {
                        renderer.features |= VulkanSupportedFeatures::YCBCR_EXTENSION;
                    }
                    if wanted_extension.cmp(&CStr::from_bytes_with_nul_unchecked(
                        ffi::vk::VK_EXT_FRAGMENT_SHADER_INTERLOCK_EXTENSION_NAME,
                    )) == Ordering::Equal
                    {
                        fragment_shader_interlock_extension = true;
                    }
                    break;
                }
            }
        }
    }

    let mut gpu_features2: ffi::vk::VkPhysicalDeviceFeatures2KHR =
        MaybeUninit::zeroed().assume_init();
    gpu_features2.sType = ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR;
    let mut base: *mut ffi::vk::VkBaseOutStructure = (&mut gpu_features2 as *mut _) as *mut _;

    unsafe fn next_to_chain<T>(
        base: *mut *mut ffi::vk::VkBaseOutStructure,
        next: &mut T,
        condition: bool,
    ) {
        if condition {
            (**base).pNext = (next as *mut T) as *mut ffi::vk::VkBaseOutStructure;
            *base = (**base).pNext;
        }
    }

    // add extensions
    let mut fragment_shader_interlock_features: ffi::vk::VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT = MaybeUninit::zeroed().assume_init();
    fragment_shader_interlock_features.sType =  ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT;
    next_to_chain(
        &mut base,
        &mut fragment_shader_interlock_features,
        fragment_shader_interlock_extension,
    );

    let mut descriptor_indexing_features: ffi::vk::VkPhysicalDeviceDescriptorIndexingFeaturesEXT =
        MaybeUninit::zeroed().assume_init();
    descriptor_indexing_features.sType =
        ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT;
    next_to_chain(
        &mut base,
        &mut descriptor_indexing_features,
        renderer
            .features
            .contains(VulkanSupportedFeatures::DESCRIPTOR_INDEXING_EXTENSION),
    );

    let mut ycbcr_features: ffi::vk::VkPhysicalDeviceSamplerYcbcrConversionFeatures =
        MaybeUninit::zeroed().assume_init();
    ycbcr_features.sType = ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES;
    next_to_chain(
        &mut base,
        &mut ycbcr_features,
        renderer
            .features
            .contains(VulkanSupportedFeatures::YCBCR_EXTENSION),
    );

    let mut enabled_buffer_device_address_features: ffi::vk::VkPhysicalDeviceBufferDeviceAddressFeatures = MaybeUninit::zeroed().assume_init();
    enabled_buffer_device_address_features.sType =
        ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES;
    next_to_chain(
        &mut base,
        &mut enabled_buffer_device_address_features,
        renderer
            .features
            .contains(VulkanSupportedFeatures::BUFFER_DEVICE_ADDRESS_EXTENSION),
    );

    // let mut enabled_ray_tracing_pipeline_features: ffi::vk::VkPhysicalDeviceRayTracingPipelineFeaturesKHR = MaybeUninit::zeroed().assume_init();
    // enabled_ray_tracing_pipeline_features.sType = ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR;
    // next_to_chain(&mut base, &mut enabled_ray_tracing_pipeline_features, true);

    let mut enabled_acceleration_structure_features: ffi::vk::VkPhysicalDeviceAccelerationStructureFeaturesKHR = MaybeUninit::zeroed().assume_init();
    enabled_acceleration_structure_features.sType = ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR;
    next_to_chain(
        &mut base,
        &mut enabled_acceleration_structure_features,
        renderer
            .features
            .contains(VulkanSupportedFeatures::KHR_ACCELERATION_STRUCTURE_EXTENSION),
    );

    // let mut enabled_ray_query_features: ffi::vk::VkPhysicalDeviceRayQueryFeaturesKHR = MaybeUninit::zeroed().assume_init();
    // enabled_ray_query_features.sType = ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR;
    // next_to_chain(&mut base, &mut enabled_ray_query_features, true);

    ffi::vk::vkGetPhysicalDeviceFeatures2(renderer.active_gpu, &mut gpu_features2);

    // Get queue family properties
    let mut queue_family_count = 0;
    ffi::vk::vkGetPhysicalDeviceQueueFamilyProperties(
        renderer.active_gpu,
        &mut queue_family_count,
        ptr::null_mut(),
    );
    let mut queue_family_properties: Vec<ffi::vk::VkQueueFamilyProperties> =
        vec![MaybeUninit::zeroed().assume_init(); queue_family_count as usize];
    ffi::vk::vkGetPhysicalDeviceQueueFamilyProperties(
        renderer.active_gpu,
        &mut queue_family_count,
        queue_family_properties.as_mut_ptr(),
    );

    // need a queue_priority for each queue in the queue family we create
    let mut queue_family_priorities: [[f32; MAX_QUEUE_COUNT as usize];
        MAX_QUEUE_FAMILIES as usize] =
        [[0.0; MAX_QUEUE_COUNT as usize]; MAX_QUEUE_FAMILIES as usize];
    let mut queue_create_infos: Vec<ffi::vk::VkDeviceQueueCreateInfo> =
        Vec::with_capacity(queue_family_count as usize);
    renderer.available_queue_count.resize(
        renderer.linked_node_count as usize,
        [0; MAX_QUEUE_FLAGS as usize],
    );
    renderer.used_queue_count.resize(
        renderer.linked_node_count as usize,
        [0; MAX_QUEUE_FLAGS as usize],
    );

    for (queue_index, &property) in queue_family_properties.iter().enumerate() {
        let mut queue_count = property.queueCount;
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

            let create_info = ffi::vk::VkDeviceQueueCreateInfo {
                sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
                pNext: ptr::null_mut(),
                flags: 0,
                queueFamilyIndex: queue_index as u32,
                queueCount: queue_count,
                pQueuePriorities: queue_family_priorities[queue_index].as_mut_ptr(),
            };
            queue_create_infos.push(create_info);

            for i in 0..renderer.linked_node_count {
                renderer.available_queue_count[i as usize][property.queueFlags as usize] =
                    queue_count;
            }
        }
    }

    let extension_result: Vec<*const c_char> = device_extension_cache
        .into_iter()
        .map(|st| st.as_ptr())
        .collect();
    let mut create_info = ffi::vk::VkDeviceCreateInfo {
        sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
        pNext: (&mut gpu_features2 as *mut _) as *mut c_void,
        flags: 0,
        queueCreateInfoCount: queue_create_infos.len() as u32,
        pQueueCreateInfos: queue_create_infos.as_mut_ptr(),
        enabledLayerCount: 0,
        ppEnabledLayerNames: ptr::null_mut(),
        enabledExtensionCount: extension_result.len() as u32,
        ppEnabledExtensionNames: extension_result.as_ptr(),
        pEnabledFeatures: ptr::null_mut(),
    };

    let result = ffi::vk::vkCreateDevice(
        renderer.active_gpu,
        &mut create_info,
        ptr::null_mut(),
        &mut renderer.device,
    );
    if result != ffi::vk::VkResult_VK_SUCCESS {
        return Err(VulkanError(result));
    }
    Ok(())
}

impl Renderer<VulkanAPI> for VulkanRenderer {
    unsafe fn add_buffer(&self, desc: &BufferDesc) -> RendererResult<super::VulkanBuffer> {
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
        let mut add_info = ffi::vk::VkBufferCreateInfo {
            sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            pNext: ptr::null_mut(),
            flags: 0,
            size: allocated_size,
            usage: desc
                .descriptors
                .to_vk_buffer_usage(desc.format != ImageFormat::UNDEFINED),
            sharingMode: ffi::vk::VkSharingMode_VK_SHARING_MODE_EXCLUSIVE,
            queueFamilyIndexCount: 0,
            pQueueFamilyIndices: ptr::null_mut(),
        };

        if desc.memory_usage == ResourceMemoryUsage::GpuOnly
            || desc.memory_usage == ResourceMemoryUsage::Unknown
        {
            add_info.usage |= ffi::vk::VkBufferUsageFlagBits_VK_BUFFER_USAGE_TRANSFER_DST_BIT;
        }

        todo!()
    }

    unsafe fn drop_buffer(&self, buffer: &mut super::VulkanBuffer) {
        todo!()
    }

    unsafe fn init(_name: &CStr, desc: &RenderDesc) -> RendererResult<Arc<VulkanRenderer>> {
        let mut renderer = Arc::new_cyclic(|me| VulkanRenderer {
            instance: ptr::null_mut(),
            active_gpu: ptr::null_mut(),
            active_gpu_properties: None,
            active_gpu_common_info: None,
            available_queue_count: vec![],
            used_queue_count: vec![],
            linked_node_count: 0,
            device: ptr::null_mut(),
            features: VulkanSupportedFeatures::NONE,
            graphics_queue_family_index: 0,
            transfer_queue_family_index: 0,
            compute_queue_family_index: 0,
            me: me.clone(),
        });
        let mut mut_render = Arc::get_mut(&mut renderer).unwrap();
        // initialize instance
        init_instance(&mut mut_render, desc)?;

        // initialize device
        init_device(&mut mut_render, desc)?;
        let family_queue =
            util_find_queue_family_index(&mut_render, 0, QueueType::QueueTypeGraphics);
        let compute_queue =
            util_find_queue_family_index(&mut_render, 0, QueueType::QueueTypeCompute);
        let transfer_queue =
            util_find_queue_family_index(&mut_render, 0, QueueType::QueueTypeTransfer);

        mut_render.graphics_queue_family_index = family_queue.family_index;
        mut_render.graphics_queue_family_index = compute_queue.family_index;
        mut_render.graphics_queue_family_index = transfer_queue.family_index;

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

        let fence_info: ffi::vk::VkFenceCreateInfo = ffi::vk::VkFenceCreateInfo {
            sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
            pNext: ptr::null_mut(),
            flags: 0,
        };
        let mut fence = super::VulkanFence {
            render: self.me.clone().upgrade().unwrap(),
            fence: ptr::null_mut(),
            submitted: false,
        };

        let result =
            ffi::vk::vkCreateFence(self.device, &fence_info, ptr::null_mut(), &mut fence.fence);
        if result != ffi::vk::VkResult_VK_SUCCESS {
            return Err(VulkanError(result));
        }
        Ok(fence)
    }

    unsafe fn add_semaphore(&self) -> RendererResult<super::VulkanSemaphore> {
        assert!(self.device != ptr::null_mut());
        let add_info = ffi::vk::VkSemaphoreCreateInfo {
            sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
            pNext: ptr::null_mut(),
            flags: 0,
        };
        let mut semaphore = VulkanSemaphore {
            render: self.me.clone().upgrade().unwrap(),
            semaphore: ptr::null_mut(),
            current_node: 0,
            signaled: false,
        };
        let result = ffi::vk::vkCreateSemaphore(
            self.device,
            &add_info,
            ptr::null_mut(),
            &mut semaphore.semaphore,
        );
        if result != ffi::vk::VkResult_VK_SUCCESS {
            return Err(VulkanError(result));
        }
        Ok(semaphore)
    }

    unsafe fn add_queue(&mut self, desc: &QueueDesc) -> RendererResult<super::VulkanQueue> {
        let node_index = 0;
        let queue_result = util_find_queue_family_index(self, node_index, desc.queue_type);
        self.used_queue_count[node_index as usize][queue_result.properties.queueFlags as usize] +=
            1;

        let mut result = VulkanQueue {
            render: self.me.clone().upgrade().unwrap(),
            queue: ptr::null_mut(),

            submission_mutex: Mutex::new(()),
            family_index: queue_result.family_index,
            queue_index: queue_result.queue_index,
            queue_flag: queue_result.properties.queueFlags,

            queue_type: desc.queue_type,
            node_index: desc.node_index,
        };

        ffi::vk::vkGetDeviceQueue(
            self.device,
            result.family_index,
            result.queue_index,
            &mut result.queue,
        );
        assert!(result.queue != ptr::null_mut());
        Ok(result)
    }

    unsafe fn drop_swap_chain(&self) {
        todo!()
    }
    unsafe fn add_cmd_pool<'a>(
        &self,
        desc: &CmdPoolDesc<'a, VulkanAPI>,
    ) -> RendererResult<VulkanCommandPool<'a>> {
        assert!(self.device != ptr::null_mut());

        let mut cmd_pool = VulkanCommandPool {
            renderer: self.me.clone().upgrade().unwrap(),
            cmd_pool: ptr::null_mut(),
            queue: desc.queue,
        };
        let mut add_info = ffi::vk::VkCommandPoolCreateInfo {
            sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
            pNext: ptr::null_mut(),
            flags: (if desc.transient {
                ffi::vk::VkCommandPoolCreateFlagBits_VK_COMMAND_POOL_CREATE_TRANSIENT_BIT
            } else {
                0
            }),
            queueFamilyIndex: desc.queue.family_index,
        };
        let result = ffi::vk::vkCreateCommandPool(
            self.device,
            &mut add_info,
            ptr::null_mut(),
            &mut cmd_pool.cmd_pool,
        );
        if result != ffi::vk::VkResult_VK_SUCCESS {
            return Err(VulkanError(result));
        }
        Ok(cmd_pool)
    }

    unsafe fn add_cmd<'a>(
        &self,
        desc: &mut CmdDesc<'a, VulkanAPI>,
    ) -> RendererResult<VulkanCommand<'a>> {
        let mut cmd = VulkanCommand {
            renderer: self.me.clone().upgrade().unwrap(),
            cmd_buf: ptr::null_mut(),
            active_render_pass: ptr::null_mut(),
            bound_pipeline_layout: ptr::null_mut(),
            pool: desc.cmd_pool,
        };

        let mut alloc_info = ffi::vk::VkCommandBufferAllocateInfo {
            sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            pNext: ptr::null_mut(),
            commandPool: desc.cmd_pool.cmd_pool,
            level: if desc.secondary {
                ffi::vk::VkCommandBufferLevel_VK_COMMAND_BUFFER_LEVEL_SECONDARY
            } else {
                ffi::vk::VkCommandBufferLevel_VK_COMMAND_BUFFER_LEVEL_PRIMARY
            },
            commandBufferCount: 1,
        };
        let result =
            ffi::vk::vkAllocateCommandBuffers(self.device, ptr::null_mut(), &mut cmd.cmd_buf);
        if result != ffi::vk::VkResult_VK_SUCCESS {
            return Err(VulkanError(result));
        }

        Ok(cmd)
    }

    unsafe fn add_render_target(&self, desc: &RenderTargetDesc) -> RendererResult<VulkanRenderTarget> {

        let is_depth = desc.format.is_depth_only() || desc.format.is_depth_and_stencil_only();
        assert!(!(is_depth && desc.descriptors.contains( DescriptorType::DESCRIPTOR_TYPE_RW_BUFFER)), "Cannot use depth stencil as UAV");

        let mip_levels = max(desc.mip_levels, 1);
        let array_size = desc.array_size;
        let depth_or_array_size = array_size * desc.depth;
        let mut num_rtvs = desc.mip_levels;

        if desc.descriptors.contains(DescriptorType::DESCRIPTOR_TYPE_RENDER_TARGET_ARRAY_SLICES) ||
            desc.descriptors.contains(DescriptorType::DESCRIPTOR_TYPE_RENDER_TARGET_DEPTH_SLICES) {
            num_rtvs *= depth_or_array_size;
        }



        todo!()
    }

    unsafe fn remove_render_target(&self, _target: &mut VulkanRenderTarget) {
        todo!()
    }

    unsafe fn add_sampler(&self, desc: &SamplerDesc) -> RendererResult<super::VulkanSampler> {
        assert!(self.device != ptr::null_mut());

        let mut add_info = ffi::vk::VkSamplerCreateInfo {
            sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
            pNext: ptr::null_mut(),
            flags: 0,
            magFilter: desc.mag_filter.to_vk_filter(),
            minFilter: desc.min_filter.to_vk_filter(),
            mipmapMode: desc.mode.to_vk_map_map_mode(),
            addressModeU: desc.address_u.to_vk_address_mode(),
            addressModeV: desc.address_v.to_vk_address_mode(),
            addressModeW: desc.address_w.to_vk_address_mode(),
            mipLodBias: desc.mip_load_bias,
            anisotropyEnable: if desc.max_anisotropy > 0.0 {
                ffi::vk::VK_TRUE
            } else {
                ffi::vk::VK_FALSE
            },
            maxAnisotropy: desc.max_anisotropy,
            compareEnable: match desc.compare_func {
                CompareMode::Never => ffi::vk::VK_TRUE,
                _ => ffi::vk::VK_FALSE,
            },
            compareOp: desc.compare_func.to_comparison_vk(),
            minLod: 0.0,
            maxLod: match &desc.mode {
                MipMapMode::Nearest => 0.0,
                MipMapMode::Linear => f32::MAX,
            },
            borderColor: ffi::vk::VkBorderColor_VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK,
            unnormalizedCoordinates: ffi::vk::VK_FALSE,
        };
        let mut sampler = VulkanSampler {
            renderer: self.me.clone().upgrade().unwrap(),
            sampler: ptr::null_mut(),
        };

        let result = ffi::vk::vkCreateSampler(
            self.device,
            &mut add_info,
            ptr::null_mut(),
            &mut sampler.sampler,
        );
        if result != ffi::vk::VkResult_VK_SUCCESS {
            return Err(VulkanError(result));
        }
        Ok(sampler)
    }

    unsafe fn add_root_signature(
        &self,
        signature: &RootSignatureDesc<VulkanAPI>,
    ) -> RendererResult<super::VulkanRootSignature> {
        todo!()
    }

    unsafe fn remove_root_signature(&self, signature: &mut super::VulkanRootSignature) {
        todo!()
    }

    unsafe fn reset_cmd_pool(&self) {
        todo!()
    }

    unsafe fn get_common_info(&self) -> &GPUCommonInfo {
        return self
            .active_gpu_common_info
            .as_ref()
            .expect("render not initialized")
            .borrow();
    }

    unsafe fn add_swap_chain<'a>(
        &self,
        desc: &'a SwapChainDesc<'a, VulkanAPI>,
        window_handle: &impl HasRawWindowHandle,
    ) -> RendererResult<VulkanSwapChain> {
        assert!(self.instance != ptr::null_mut());
        assert!(desc.presentation_queues.len() > 0);

        unsafe {
            let mut vk_surface: ffi::vk::VkSurfaceKHR = ptr::null_mut();
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
                    cfg_if::cfg_if! {
                        if  #[cfg(feature = "xlib")] {
                             let mut surface_info = ffi::vk::VkXlibSurfaceCreateInfoKHR {
                                    sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR,
                                    pNext: ptr::null_mut(),
                                    flags: 0,
                                    dpy: handle.display as *mut ffi::vk::Display,
                                    window: handle.window
                                };
                                let result = ffi::vk::vkCreateXlibSurfaceKHR(self.instance, &surface_info, ptr::null_mut(), &mut vk_surface);
                                if result != ffi::vk::VkResult_VK_SUCCESS {
                                    return Err(VulkanError(result))
                                }
                        } else {
                            panic!("xlib is not configured")
                        }
                    }
                }
                RawWindowHandle::Xcb(handle) => {
                    cfg_if::cfg_if! {
                        if #[cfg(feature = "xcb")] {
                            let mut surface_info = ffi::vk::VkXcbSurfaceCreateInfoKHR {
                                sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR,
                                pNext: ptr::null_mut(),
                                flags: 0,
                                connection: handle.connection as *mut ffi::vk::xcb_connection_t,
                                window: handle.window
                            };
                            let result = ffi::vk::vkCreateXcbSurfaceKHR(self.instance, &surface_info, ptr::null_mut(), &mut vk_surface);
                            if result != ffi::vk::VkResult_VK_SUCCESS {
                                return Err(VulkanError(result))
                            }
                        } else {
                            panic!("xcb is not configured")
                        }
                    }
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
            let mut caps: ffi::vk::VkSurfaceCapabilitiesKHR = MaybeUninit::zeroed().assume_init();
            {
                let result = ffi::vk::vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
                    self.active_gpu,
                    vk_surface,
                    &mut caps,
                );
                if result != ffi::vk::VkResult_VK_SUCCESS {
                    return Err(VulkanError(result));
                }
            }

            if caps.maxImageCount > 0 && desc.image_count > caps.maxImageCount {
                warn!(
                    "Changed requested SwapChain image {} to maximum allowed SwaphChain image {}",
                    desc.image_count, caps.maxImageCount
                );
                image_count = caps.maxImageCount;
            }

            if desc.image_count < caps.minImageCount {
                warn!(
                    "Changed requested SwapChain image {} to maximum allowed SwaphChain image {}",
                    desc.image_count, caps.minImageCount
                );
                image_count = caps.minImageCount;
            }

            // Surface format
            // select a surface format, depending on whether HRD is available.
            let mut surface_format = ffi::vk::VkSurfaceFormatKHR {
                format: ffi::vk::VkFormat_VK_FORMAT_UNDEFINED,
                colorSpace: ffi::vk::VkColorSpaceKHR_VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
            };
            let mut surface_format_count: u32 = 0;
            ffi::vk::vkGetPhysicalDeviceSurfaceFormatsKHR(
                self.active_gpu,
                vk_surface,
                &mut surface_format_count,
                ptr::null_mut(),
            );
            let mut formats: Vec<ffi::vk::VkSurfaceFormatKHR> =
                vec![MaybeUninit::zeroed().assume_init(); surface_format_count as usize];
            ffi::vk::vkGetPhysicalDeviceSurfaceFormatsKHR(
                self.active_gpu,
                vk_surface,
                &mut surface_format_count,
                formats.as_mut_ptr(),
            );

            if formats.len() == 1 && formats[0].colorSpace == ffi::vk::VkFormat_VK_FORMAT_UNDEFINED
            {
                surface_format.format = ffi::vk::VkFormat_VK_FORMAT_B8G8R8A8_UNORM;
                surface_format.colorSpace =
                    ffi::vk::VkColorSpaceKHR_VK_COLOR_SPACE_SRGB_NONLINEAR_KHR;
            } else {
                let hrd_surface_format = ffi::vk::VkSurfaceFormatKHR {
                    format: ffi::vk::VkFormat_VK_FORMAT_A2B10G10R10_UNORM_PACK32,
                    colorSpace: ffi::vk::VkColorSpaceKHR_VK_COLOR_SPACE_HDR10_ST2084_EXT,
                };
                let requested_format: ffi::vk::VkFormat = desc.color_format.to_vk_format();
                let requested_color_space = if requested_format == hrd_surface_format.format {
                    hrd_surface_format.colorSpace
                } else {
                    ffi::vk::VkColorSpaceKHR_VK_COLOR_SPACE_SRGB_NONLINEAR_KHR
                };

                for format in formats {
                    if format.format == requested_format
                        && requested_color_space == format.colorSpace
                    {
                        surface_format.format = requested_format;
                        surface_format.colorSpace = requested_color_space;
                        break;
                    }
                }
            }

            let mut present_mode: ffi::vk::VkPresentModeKHR =
                ffi::vk::VkPresentModeKHR_VK_PRESENT_MODE_FIFO_KHR;
            let mut swap_chain_image_count: u32 = 0;
            {
                let result = ffi::vk::vkGetPhysicalDeviceSurfacePresentModesKHR(
                    self.active_gpu,
                    vk_surface,
                    &mut swap_chain_image_count,
                    ptr::null_mut(),
                );
                if result != ffi::vk::VkResult_VK_SUCCESS {
                    return Err(VulkanError(result));
                }
            }
            let mut modes: Vec<ffi::vk::VkPresentModeKHR> =
                vec![MaybeUninit::zeroed().assume_init(); swap_chain_image_count as usize];
            {
                let result = ffi::vk::vkGetPhysicalDeviceSurfacePresentModesKHR(
                    self.active_gpu,
                    vk_surface,
                    &mut swap_chain_image_count,
                    modes.as_mut_ptr(),
                );
                if result != ffi::vk::VkResult_VK_SUCCESS {
                    return Err(VulkanError(result));
                }
            }

            let preferred_modes = if desc.enable_vsync {
                [
                    ffi::vk::VkPresentModeKHR_VK_PRESENT_MODE_FIFO_RELAXED_KHR,
                    ffi::vk::VkPresentModeKHR_VK_PRESENT_MODE_FIFO_KHR,
                ].as_slice()
            } else {
                [
                    ffi::vk::VkPresentModeKHR_VK_PRESENT_MODE_IMMEDIATE_KHR,
                    ffi::vk::VkPresentModeKHR_VK_PRESENT_MODE_MAILBOX_KHR,
                    ffi::vk::VkPresentModeKHR_VK_PRESENT_MODE_FIFO_RELAXED_KHR,
                    ffi::vk::VkPresentModeKHR_VK_PRESENT_MODE_FIFO_KHR,
                ].as_slice()
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
            let mut extent = ffi::vk::VkExtent2D {
                width: max(min(caps.minImageExtent.width, desc.width), caps.maxImageExtent.width),
                height: max(min(caps.minImageExtent.height, desc.width), caps.maxImageExtent.height),
            };
            let sharing_mode: ffi::vk::VkSharingMode = ffi::vk::VkSharingMode_VK_SHARING_MODE_EXCLUSIVE;
            let mut present_queue_family_index: Option<u32> = None;

            let mut family_property_count: u32 = 0;
            ffi::vk::vkGetPhysicalDeviceQueueFamilyProperties(self.active_gpu, &mut family_property_count, ptr::null_mut());
            let mut family_properties: Vec<ffi::vk::VkQueueFamilyProperties> = vec![MaybeUninit::zeroed().assume_init(); family_property_count as usize];
            ffi::vk::vkGetPhysicalDeviceQueueFamilyProperties(self.active_gpu, &mut family_property_count, family_properties.as_mut_ptr());

            // Check if hardware provides dedicated present queue
            if family_property_count > 0 {
                for (index, property) in family_properties.iter().enumerate() {
                    let mut is_supported: ffi::vk::VkBool32 = 0;
                    let result = ffi::vk::vkGetPhysicalDeviceSurfaceSupportKHR(self.active_gpu, index as u32, vk_surface, &mut is_supported);
                    if result == ffi::vk::VkResult_VK_SUCCESS && ffi::vk::VK_TRUE == is_supported && desc.presentation_queues[0].family_index != index as u32 {
                        present_queue_family_index = Some(index as u32);
                        break;
                    }
                }
                if present_queue_family_index == None {
                    for (index, property) in family_properties.iter().enumerate() {
                        let mut is_supported: ffi::vk::VkBool32 = 0;
                        let result = ffi::vk::vkGetPhysicalDeviceSurfaceSupportKHR(self.active_gpu, index as u32, vk_surface, &mut is_supported);
                        if result == ffi::vk::VkResult_VK_SUCCESS && ffi::vk::VK_TRUE == is_supported {
                            present_queue_family_index = Some(index as u32);
                            break;
                        }
                    }
                }
            }

            // Find if gpu has a dedicated present queue
            let mut present_queue: ffi::vk::VkQueue = ptr::null_mut();
            let mut queue_family_index_count = 0;
            let mut queue_family_indices = [desc.presentation_queues[0].family_index, 0];
            let mut final_present_queue_family_index = 0;
            if present_queue_family_index != None && Some(queue_family_indices[0]) != present_queue_family_index {
                queue_family_index_count = 1;
                final_present_queue_family_index = present_queue_family_index.unwrap();
                queue_family_indices[0] = final_present_queue_family_index;
                ffi::vk::vkGetDeviceQueue(self.device, queue_family_indices[0], 0, &mut present_queue);
            } else {
                final_present_queue_family_index = queue_family_indices[0];
                present_queue = ptr::null_mut();
            }

            let mut pre_transform: ffi::vk::VkSurfaceTransformFlagBitsKHR = 0;
            if caps.supportedTransforms & ffi::vk::VkSurfaceTransformFlagBitsKHR_VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR > 0 {
                pre_transform = ffi::vk::VkSurfaceTransformFlagBitsKHR_VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR;
            } else {
                pre_transform = caps.currentTransform;
            }
            let mut composite_alpha: ffi::vk::VkCompositeAlphaFlagBitsKHR = ffi::vk::VkCompositeAlphaFlagBitsKHR_VK_COMPOSITE_ALPHA_FLAG_BITS_MAX_ENUM_KHR;

            for flag in [
                ffi::vk::VkCompositeAlphaFlagBitsKHR_VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR,
                ffi::vk::VkCompositeAlphaFlagBitsKHR_VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
                ffi::vk::VkCompositeAlphaFlagBitsKHR_VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR,
                ffi::vk::VkCompositeAlphaFlagBitsKHR_VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR
            ] {
                if caps.supportedCompositeAlpha & flag > 0 {
                    composite_alpha = flag;
                    break
                }
            }
            assert!(composite_alpha != ffi::vk::VkCompositeAlphaFlagBitsKHR_VK_COMPOSITE_ALPHA_FLAG_BITS_MAX_ENUM_KHR);

            let mut vk_swap_chain: ffi::vk::VkSwapchainKHR = ptr::null_mut();
            let mut swap_chain_info = ffi::vk::VkSwapchainCreateInfoKHR {
                sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
                pNext: ptr::null_mut(),
                flags: 0,
                surface: vk_surface,
                minImageCount: desc.image_count,
                imageFormat: surface_format.format,
                imageColorSpace: surface_format.colorSpace,
                imageExtent: extent,
                imageArrayLayers: 1,
                imageUsage: ffi::vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | ffi::vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
                imageSharingMode: sharing_mode,
                queueFamilyIndexCount: queue_family_index_count,
                pQueueFamilyIndices: queue_family_indices.as_mut_ptr(),
                preTransform: pre_transform,
                compositeAlpha: composite_alpha,
                presentMode: present_mode,
                clipped: ffi::vk::VK_TRUE,
                oldSwapchain: ptr::null_mut()
            };
            {
                let result = ffi::vk::vkCreateSwapchainKHR(self.device, &mut swap_chain_info, ptr::null_mut(), &mut vk_swap_chain);
                if result != ffi::vk::VkResult_VK_SUCCESS {
                    return Err(VulkanError(result));
                }
            }

            // desc.color_format = ImageFormat::from_vk_format(surface_format.format);


            Ok(VulkanSwapChain {})
        }
    }

    unsafe fn add_texture(&self, desc: &TextureDesc) -> RendererResult<VulkanTexture> {
        assert!(desc.width > 0 && desc.height > 0 && (desc.depth > 0 || desc.array_size > 0));
        let sample_count: u32 = desc.sample_count as u32;
        if (desc.sample_count as u32) > (SampleCount::SampleCount1 as u32) && desc.mip_levels > 1 {
            error!("Multi-Sampled textures cannot have mip maps");
            assert!(false);
            return Err(RendererError::Unhandled);
        }

        let mut texture = VulkanTexture {
            vk_srv_descriptor: ptr::null_mut(),
            vk_uav_descriptors: vec![],
            vk_srv_stencil_descriptor: ptr::null_mut(),
            vk_image: ptr::null_mut(),
            own_image: false
        };

        if desc.native_handle != ptr::null_mut() && !desc.flags.contains(TextureCreationFlags::TEXTURE_CREATION_FLAG_IMPORT_BIT) {
            texture.own_image = false;
            texture.vk_image = desc.native_handle as ffi::vk::VkImageView;
        } else  {
            texture.own_image = true;
        }

        let mut additional_flag: ffi::vk::VkImageUsageFlags = 0;
        if desc.start_state.contains(ResourceState::RENDER_TARGET) {
            additional_flag |= ffi::vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;
        } else if desc.start_state.contains(ResourceState::DEPTH_WRITE) {
            additional_flag |= ffi::vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT;
        }

        let array_size: u32 = desc.array_size;
        let mut image_type: ffi::vk::VkImageType = ffi::vk::VkImageType_VK_IMAGE_TYPE_MAX_ENUM;
        if desc.flags.contains(TextureCreationFlags::TEXTURE_CREATION_FLAG_FORCE_2D) {
            assert_eq!(desc.depth, 1);
            image_type = ffi::vk::VkImageType_VK_IMAGE_TYPE_2D;
        } else if desc.flags.contains(TextureCreationFlags::TEXTURE_CREATION_FLAG_FORCE_3D) {
            image_type = ffi::vk::VkImageType_VK_IMAGE_TYPE_3D;
        } else {
            if desc.depth > 1 {
                image_type =  ffi::vk::VkImageType_VK_IMAGE_TYPE_3D;
            } else if desc.height > 1 {
                image_type = ffi::vk::VkImageType_VK_IMAGE_TYPE_2D;
            } else {
                image_type = ffi::vk::VkImageType_VK_IMAGE_TYPE_1D;
            }
        }

        let mut descriptors = desc.descriptors;
        let mut cubemap_required: bool = descriptors.contains(DescriptorType::DESCRIPTOR_TYPE_TEXTURE_CUBE);
        let mut array_required: bool = false;

        let is_planer: bool = desc.format.is_planer();
        let num_planes: u32 = desc.format.num_planes();
        let is_single_plane: bool = desc.format.is_single_plane();
        assert!((num_planes == 1 && is_single_plane) && (!is_single_plane && num_planes > 1 && num_planes <= 3),
            "Number of planes for multi-planar formats must be 2 or 3 and for single-planar formats it must be 1.");

        if image_type == ffi::vk::VkImageType_VK_IMAGE_TYPE_3D {
            array_required = true;
        }

        if texture.vk_image == ptr::null_mut() {


            let mut add_info = ffi::vk::VkImageCreateInfo {
                sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
                pNext: ptr::null_mut(),
                flags: {
                    let mut result: ffi::vk::VkImageCreateFlags = 0;
                    if cubemap_required {
                        result |= ffi::vk::VkImageCreateFlagBits_VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT;
                    }
                    if array_required {
                        result |= ffi::vk::VkImageCreateFlagBits_VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR;
                    }
                    result
                },
                imageType: image_type,
                format: desc.format.to_vk_format(),
                extent: ffi::vk::VkExtent3D {
                    width: desc.width,
                    height: desc.height,
                    depth: desc.depth
                },
                mipLevels: desc.mip_levels,
                arrayLayers: array_size,
                samples: desc.sample_count.to_vk_sample_count(),
                tiling: ffi::vk::VkImageTiling_VK_IMAGE_TILING_OPTIMAL,
                usage: descriptors.to_vk_usage() | additional_flag,
                sharingMode: ffi::vk::VkSharingMode_VK_SHARING_MODE_EXCLUSIVE,
                queueFamilyIndexCount: 0,
                pQueueFamilyIndices: ptr::null_mut(),
                initialLayout: ffi::vk::VkImageLayout_VK_IMAGE_LAYOUT_UNDEFINED
            };

            let mut format_props: ffi::vk::VkFormatProperties = MaybeUninit::zeroed().assume_init();
            ffi::vk::vkGetPhysicalDeviceFormatProperties(self.active_gpu, add_info.format, &mut format_props);

            // multi-planar formats must have each plane separately bound to memory, rather than having a single memory binding for the whole image
            if is_planer {
                assert!((format_props.optimalTilingFeatures & ffi::vk::VkFormatFeatureFlagBits_VK_FORMAT_FEATURE_DISJOINT_BIT) > 0);
                add_info.flags |= ffi::vk::VkImageCreateFlagBits_VK_IMAGE_CREATE_DISJOINT_BIT;
            }

            if (add_info.usage & ffi::vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_SAMPLED_BIT) > 0 || (add_info.usage & ffi::vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_STORAGE_BIT) > 0 {
                // Make it easy to copy to and from textures
                add_info.usage |= ffi::vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_TRANSFER_SRC_BIT | ffi::vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_TRANSFER_DST_BIT;
            }

            // ASSERT(pRenderer->pCapBits->canShaderReadFrom[pDesc->mFormat] && "GPU shader can't' read from this format");
            // Verify that GPU supports this format
            let format_features: ffi::vk::VkFormatFeatureFlags = util_vk_image_usage_to_format_features(add_info.usage);
            let flags: ffi::vk::VkFormatFeatureFlags = format_props.optimalTilingFeatures & format_features;
            assert!((0 != flags), "Format is not supported for GPU local images (i.e. not host visible images)");

            let mut mem_reqs = ffi::vk::VmaAllocationCreateInfo {
                flags: 0,
                usage: ffi::vk::VmaMemoryUsage_VMA_MEMORY_USAGE_GPU_ONLY,
                requiredFlags: 0,
                preferredFlags: 0,
                memoryTypeBits: 0,
                pool: ptr::null_mut(),
                pUserData: ptr::null_mut(),
                priority: 0.0
            };


        }
        todo!()
    }
}
