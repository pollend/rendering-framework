use crate::{check_vk_result, desc::RenderDescImp, error::RendererError::VulkanError, ffi, types::QueueType, vulkan::{
    device::VulkanGPUInfo,
    types::{
        VulkanSupportedFeatures, GLOBAL_INSTANCE_EXTENSIONS, GLOBAL_WANTED_DEVICE_EXTENSIONS,
        MAX_QUEUE_COUNT, MAX_QUEUE_FAMILIES, MAX_QUEUE_FLAGS,
    },
    VulkanPipeline, VulkanRenderTarget, VulkanRenderer, VulkanSemaphore,
}, CmdPoolDesc, GPUCommonInfo, QueueDesc, RenderDesc, Renderer, RendererResult, VulkanAPI, BufferDesc, RootSignatureDesc, SamplerDesc};
use log::{error, info, log, warn};
use std::{
    borrow::Borrow,
    cmp::{min, Ordering},
    collections::HashSet,
    ffi::{c_void, CStr, CString},
    mem,
    mem::MaybeUninit,
    os::raw::c_char,
    ptr,
};
use forge_image_format::ImageFormat;
use crate::types::{DescriptorType, ResourceMemoryUsage};
use crate::vulkan::VulkanBuffer;

struct QueueFamilyResult {
    properties: ffi::vk::VkQueueFamilyProperties,
    family_index: u32,
    queue_index: u32,
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
        properties: MaybeUninit::zeroed().assume_init(),
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

        if desc.descriptors.contains(DescriptorType::DESCRIPTOR_TYPE_UNIFORM_BUFFER) {
            let min_alignment = common_info.uniform_buffer_alignment;
            allocated_size = forge_math::round_up(allocated_size, min_alignment as u64);
        }
        let mut add_info = ffi::vk::VkBufferCreateInfo {
            sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            pNext: ptr::null_mut(),
            flags: 0,
            size: allocated_size,
            usage: desc.descriptors.to_vk_buffer_usage(desc.format != ImageFormat::UNDEFINED),
            sharingMode: ffi::vk::VkSharingMode_VK_SHARING_MODE_EXCLUSIVE,
            queueFamilyIndexCount: 0,
            pQueueFamilyIndices: ptr::null_mut()
        };

        if desc.memory_usage == ResourceMemoryUsage::GpuOnly || desc.memory_usage == ResourceMemoryUsage::Unknown {
            add_info.usage |= ffi::vk::VkBufferUsageFlagBits_VK_BUFFER_USAGE_TRANSFER_DST_BIT;
        }


        todo!()
    }

    unsafe fn drop_buffer(&self, buffer: &mut super::VulkanBuffer) {
        todo!()
    }


    unsafe fn init(_name: &CStr, desc: &RenderDesc) -> RendererResult<VulkanRenderer> {
        let mut renderer = VulkanRenderer {
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
        };
        // initialize instance
        init_instance(&mut renderer, desc)?;

        // initialize device
        init_device(&mut renderer, desc)?;

        renderer.graphics_queue_family_index =
            util_find_queue_family_index(&renderer, 0, QueueType::QueueTypeGraphics).family_index;
        renderer.graphics_queue_family_index =
            util_find_queue_family_index(&renderer, 0, QueueType::QueueTypeCompute).family_index;
        renderer.graphics_queue_family_index =
            util_find_queue_family_index(&renderer, 0, QueueType::QueueTypeTransfer).family_index;

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

    unsafe fn drop_fence(&self, fence: &mut super::VulkanFence) {
        assert!(fence.fence != ptr::null_mut());
        assert!(self.device != ptr::null_mut());
        ffi::vk::vkDestroyFence(self.device, fence.fence, ptr::null_mut());
        fence.fence = ptr::null_mut();
    }

    unsafe fn add_semaphore(&self) -> RendererResult<super::VulkanSemaphore> {
        assert!(self.device != ptr::null_mut());
        let add_info = ffi::vk::VkSemaphoreCreateInfo {
            sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
            pNext: ptr::null_mut(),
            flags: 0,
        };
        let mut semaphore = VulkanSemaphore {
            semaphore: ptr::null_mut(),
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

    unsafe fn drop_semaphore(&self, semaphore: &mut super::VulkanSemaphore) -> RendererResult<()> {
        assert!(semaphore.semaphore != ptr::null_mut());
        assert!(self.device != ptr::null_mut());
        ffi::vk::vkDestroySemaphore(self.device, semaphore.semaphore, ptr::null_mut());
        semaphore.semaphore = ptr::null_mut();
        Ok(())
    }

    unsafe fn add_queue(&self, desc: &QueueDesc) -> RendererResult<super::VulkanQueue> {
        let _node_index = desc.node_index;
        // let mut queue_property = ffi::vk::VkQueueFamilyProperties {
        //     queueFlags
        //     queueCount
        //     timestampValidBits
        //     minImageTransferGranularity
        // }
        todo!()
    }

    unsafe fn remove_queue(&self, _queue: &mut super::VulkanQueue) {
        todo!()
    }

    unsafe fn add_swap_chain(&self) {
        todo!()
    }

    unsafe fn drop_swap_chain(&self) {
        todo!()
    }

    unsafe fn add_cmd_pool(&self, _desc: &CmdPoolDesc<VulkanAPI>) {
        todo!()
    }

    unsafe fn drop_cmd_pool(&self) {
        todo!()
    }

    unsafe fn add_cmd(&self) {
        todo!()
    }

    unsafe fn drop_cmd(&self) {
        todo!()
    }

    unsafe fn add_render_target(&self) -> RendererResult<VulkanRenderTarget> {
        todo!()
    }

    unsafe fn remove_render_target(&self, _target: &mut VulkanRenderTarget) {
        todo!()
    }

    unsafe fn add_sampler(&self, desc: &SamplerDesc) -> RendererResult<super::VulkanSampler> {
        assert!(self.device != ptr::null_mut());

        // let sampler = ffi::vk::VkSamplerCreateInfo {
        //
        // };
        todo!()
    }


    unsafe fn add_root_signature(&self, signature: &RootSignatureDesc<VulkanAPI>) -> RendererResult<super::VulkanRootSignature> {


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
}
