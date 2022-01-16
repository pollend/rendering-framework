#![feature(prelude_import)]
#![feature(associated_type_defaults)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use crate::{
    desc::{CmdPoolDesc, QueueDesc, RenderDesc},
    error::RendererResult,
    vulkan::VulkanAPI,
};
use std::{ffi::CStr, rc::Rc, sync::Arc};
use bitfield::bitfield;
mod desc {
    use crate::{
        types::{QueueFlag, QueuePriority, QueueType},
        Api,
    };
    use std::ffi::{CStr, CString};
    pub struct VulkanRenderDesc {
        pub(crate) instance_layers: Vec<CString>,
        pub(crate) instance_extensions: Vec<CString>,
        device_extensions: Vec<CString>,
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
    pub struct QueueDesc {
        pub queue_type: QueueType,
        pub flag: QueueFlag,
        pub priority: QueuePriority,
        pub node_index: u32,
    }
}
mod error {
    use crate::ffi;
    pub enum RendererError {
        Unhandled,
        VulkanError(ffi::vk::VkResult),
    }
    pub type RendererResult<T> = Result<T, RendererError>;
}
mod types {
    pub enum DescriptorUpdateFrequency {
        DescriptorUpdateFreqNone = 0,
        DescriptorUpdateFreqPerFrame,
        DescriptorUpdateFreqPerBatch,
        DescriptorUpdateFreqPerDraw,
        DescriptorUpdateFreqCount,
    }
    impl ::core::marker::StructuralPartialEq for DescriptorUpdateFrequency {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for DescriptorUpdateFrequency {
        #[inline]
        fn eq(&self, other: &DescriptorUpdateFrequency) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    pub enum QueueType {
        QueueTypeGraphics = 0,
        QueueTypeTransfer,
        QueueTypeCompute,
        MaxQueueType,
    }
    impl ::core::marker::StructuralPartialEq for QueueType {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for QueueType {
        #[inline]
        fn eq(&self, other: &QueueType) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    enum ShadingRate {
        ShadingRateNotSupported = 0x00,
        ShadingRateFull = 0x01,
        ShadingRateHalf = 0x02,
        ShadingRateQuarter = 0x04,
        ShadingRateEighth = 0x08,
        ShadingRate1x2 = 0x10,
        ShadingRate2x1 = 0x20,
        ShadingRate2x4 = 0x40,
        ShadingRate4x2 = 0x80,
    }
    pub enum QueueFlag {
        QueueFlagNone = 0x0,
        QueueFlagDisableGpuTimeout = 0x1,
        QueueFlagInitMicroprofile = 0x2,
        MaxQueueFlag = 0xFFFFFFFF,
    }
    impl ::core::marker::StructuralPartialEq for QueueFlag {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for QueueFlag {
        #[inline]
        fn eq(&self, other: &QueueFlag) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    pub enum QueuePriority {
        QueuePriorityNormal,
        QueuePriorityHigh,
        QueuePriorityGlobalRealtime,
        MaxQueuePriority,
    }
    impl ::core::marker::StructuralPartialEq for QueuePriority {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for QueuePriority {
        #[inline]
        fn eq(&self, other: &QueuePriority) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
}
mod vulkan {
    mod desc {}
    mod device {
        use crate::{ffi, vulkan::VulkanRenderer, GPUCommonInfo, RendererResult};
        use std::{mem, mem::MaybeUninit, ptr};
        pub(in crate::vulkan) struct VulkanGPUInfo {
            physical_device_features: ffi::vk::VkPhysicalDeviceFeatures2,
            physical_memory_properties: ffi::vk::VkPhysicalDeviceMemoryProperties,
            physical_device_properties: ffi::vk::VkPhysicalDeviceProperties2,
            queue_family_properties: Vec<ffi::vk::VkQueueFamilyProperties>,
            pub device: ffi::vk::VkPhysicalDevice,
        }
        impl VulkanGPUInfo {
            pub fn get_physical_features(&self) -> &ffi::vk::VkPhysicalDeviceFeatures {
                return &self.physical_device_features.features;
            }
            pub fn get_device_properties(&self) -> &ffi::vk::VkPhysicalDeviceProperties {
                return &self.physical_device_properties.properties;
            }
            pub fn get_memory_properties(&self) -> &ffi::vk::VkPhysicalDeviceMemoryProperties {
                return &self.physical_memory_properties;
            }
            pub fn get_queue_family_properties(&self) -> &Vec<ffi::vk::VkQueueFamilyProperties> {
                return &self.queue_family_properties;
            }
            pub fn get_device(&self) -> ffi::vk::VkPhysicalDevice {
                return self.device;
            }
            pub fn to_common(&self) -> GPUCommonInfo {
                return GPUCommonInfo {};
            }
            pub unsafe fn all(instance: ffi::vk::VkInstance) -> Vec<VulkanGPUInfo> {
                if !(instance != ptr::null_mut()) {
                    ::core::panicking::panic("assertion failed: instance != ptr::null_mut()")
                };
                let mut device_count: u32 = 0;
                let mut vk_result = ffi::vk::vkEnumeratePhysicalDevices(
                    instance,
                    &mut device_count,
                    ptr::null_mut(),
                );
                if !(vk_result == ffi::vk::VkResult_VK_SUCCESS) {
                    ::core::panicking::panic(
                        "assertion failed: vk_result == ffi::vk::VkResult_VK_SUCCESS",
                    )
                };
                let mut details: Vec<VulkanGPUInfo> = Vec::with_capacity(device_count as usize);
                let mut physical_devices: Vec<ffi::vk::VkPhysicalDevice> =
                    Vec::with_capacity(device_count as usize);
                vk_result = ffi::vk::vkEnumeratePhysicalDevices(
                    instance,
                    &mut device_count,
                    physical_devices.as_mut_ptr(),
                );
                if !(vk_result == ffi::vk::VkResult_VK_SUCCESS) {
                    ::core::panicking::panic(
                        "assertion failed: vk_result == ffi::vk::VkResult_VK_SUCCESS",
                    )
                };
                for device in physical_devices {
                    match VulkanGPUInfo::gpu(device) {
                        Ok(detail) => details.push(detail),
                        _ => {}
                    }
                }
                details
            }
            pub unsafe fn gpu(device: ffi::vk::VkPhysicalDevice) -> RendererResult<VulkanGPUInfo> {
                let mut detail : VulkanGPUInfo = VulkanGPUInfo { physical_device_features : ffi :: vk :: VkPhysicalDeviceFeatures2 { sType : ffi :: vk :: VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR , pNext : ptr :: null_mut () , features : unsafe { MaybeUninit :: zeroed () . assume_init () } , } , physical_memory_properties : unsafe { MaybeUninit :: zeroed () . assume_init () } , physical_device_properties : ffi :: vk :: VkPhysicalDeviceProperties2 { sType : ffi :: vk :: VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR , pNext : ptr :: null_mut () , properties : unsafe { MaybeUninit :: zeroed () . assume_init () } , } , queue_family_properties : :: alloc :: vec :: Vec :: new () , device , } ;
                let mut subgroup_properties : ffi :: vk :: VkPhysicalDeviceSubgroupProperties = ffi :: vk :: VkPhysicalDeviceSubgroupProperties { sType : ffi :: vk :: VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES , pNext : ptr :: null_mut () , subgroupSize : 0 , supportedStages : 0 , supportedOperations : 0 , quadOperationsInAllStages : 0 , } ;
                subgroup_properties.pNext = detail.physical_device_properties.pNext;
                detail.physical_device_properties.pNext = mem::transmute(&mut subgroup_properties);
                ffi::vk::vkGetPhysicalDeviceMemoryProperties(
                    device,
                    &mut detail.physical_memory_properties,
                );
                ffi::vk::vkGetPhysicalDeviceFeatures2(device, &mut detail.physical_device_features);
                ffi::vk::vkGetPhysicalDeviceProperties2KHR(
                    device,
                    &mut detail.physical_device_properties,
                );
                let mut queue_family_property_count: u32 = 0;
                ffi::vk::vkGetPhysicalDeviceQueueFamilyProperties(
                    device,
                    &mut queue_family_property_count,
                    ptr::null_mut(),
                );
                detail.queue_family_properties = ::alloc::vec::from_elem(
                    MaybeUninit::zeroed().assume_init(),
                    queue_family_property_count as usize,
                );
                ffi::vk::vkGetPhysicalDeviceQueueFamilyProperties(
                    device,
                    &mut queue_family_property_count,
                    detail.queue_family_properties.as_mut_ptr(),
                );
                Ok(detail)
            }
        }
    }
    mod renderer {
        use crate::{
            check_vk_result,
            desc::RenderDescImp,
            error::RendererError::{Unhandled, VulkanError},
            ffi,
            types::QueueType,
            vulkan::{
                device::VulkanGPUInfo, renderer, VulkanFence, VulkanPipeline, VulkanRenderTarget,
                VulkanRenderer, VulkanSemaphore,
            },
            Api, CmdPoolDesc, QueueDesc, RenderDesc, Renderer, RendererResult, VulkanAPI,
        };
        use log::{info, warn};
        use std::{
            collections::HashSet,
            ffi::{CStr, CString},
            mem::MaybeUninit,
            option::{IntoIter, Iter},
            os::raw::c_char,
            ptr,
        };
        unsafe fn select_best_gpu(
            instance: ffi::vk::VkInstance,
            vk_gpus: &Vec<VulkanGPUInfo>,
        ) -> RendererResult<&VulkanGPUInfo> {
            let is_device_better = |current: &VulkanGPUInfo, to_test: &VulkanGPUInfo| -> bool {
                let current_device_properties = current.get_device_properties();
                let test_device_properties = to_test.get_device_properties();
                if test_device_properties.deviceType
                    == ffi::vk::VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU
                    && current_device_properties.deviceType
                        != ffi::vk::VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU
                {
                    return true;
                }
                if test_device_properties.deviceType
                    != ffi::vk::VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU
                    && current_device_properties.deviceType
                        == ffi::vk::VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU
                {
                    return false;
                }
                if current_device_properties.vendorID == test_device_properties.vendorID
                    && current_device_properties.deviceID == test_device_properties.deviceID
                {
                    let current_memory_properties = current.get_memory_properties();
                    let test_memory_properties = to_test.get_memory_properties();
                    let mut total_test_vram: ffi::vk::VkDeviceSize = 0;
                    let mut total_current_vram: ffi::vk::VkDeviceSize = 0;
                    for i in 0..current_memory_properties.memoryHeapCount as usize {
                        let heap = &current_memory_properties.memoryHeaps[i];
                        if heap.flags
                            & ffi::vk::VkMemoryHeapFlagBits_VK_MEMORY_HEAP_DEVICE_LOCAL_BIT
                            > 0
                        {
                            total_current_vram += heap.size
                        }
                    }
                    for i in 0..test_memory_properties.memoryHeapCount as usize {
                        let heap = &test_memory_properties.memoryHeaps[i];
                        if heap.flags
                            & ffi::vk::VkMemoryHeapFlagBits_VK_MEMORY_HEAP_DEVICE_LOCAL_BIT
                            > 0
                        {
                            total_test_vram += heap.size
                        }
                    }
                    return total_test_vram > total_current_vram;
                }
                return false;
            };
            let mut best_option: Option<usize> = None;
            for (j, el) in vk_gpus.iter().enumerate() {
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            ::core::fmt::Arguments::new_v1(
                                &["GPU[", "]"],
                                &match (&el.get_device_properties().vendorID,) {
                                    _args => [::core::fmt::ArgumentV1::new(
                                        _args.0,
                                        ::core::fmt::Display::fmt,
                                    )],
                                },
                            ),
                            lvl,
                            &(
                                "renderer::vulkan::renderer",
                                "renderer::vulkan::renderer",
                                "src/vulkan/renderer.rs",
                                77u32,
                            ),
                        );
                    }
                };
                if best_option == None || is_device_better(el, &vk_gpus[best_option.unwrap()]) {
                    for property in el.get_queue_family_properties() {
                        if property.queueFlags & ffi::vk::VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT > 0
                        {
                            best_option = Some(j);
                            break;
                        }
                    }
                }
            }
            match best_option {
                Some(idx) => Ok(&vk_gpus[idx]),
                _ => Err(Unhandled),
            }
        }
        struct QueueFamilyResult {
            properties: ffi::vk::VkQueueFamilyProperties,
            family_index: u8,
            queue_index: u8,
        }
        unsafe fn util_find_queue_family_index(
            renderer: &VulkanRenderer,
            node_index: u32,
            queue_type: QueueType,
        ) -> RendererResult<QueueFamilyResult> {
            let mut queue_family_index: u32 = u32::MAX;
            let mut queue_index: u32 = u32::MAX;
            let mut required_flags = queue_type.to_vk_queue();
            let mut found = false;
            let mut family_property_count: u32 = 0;
            ffi::vk::vkGetPhysicalDeviceQueueFamilyProperties(
                renderer.active_gpu,
                &mut family_property_count,
                ptr::null_mut(),
            );
            let mut family_properties: Vec<ffi::vk::VkQueueFamilyProperties> =
                ::alloc::vec::from_elem(
                    MaybeUninit::zeroed().assume_init(),
                    family_property_count as usize,
                );
            ffi::vk::vkGetPhysicalDeviceQueueFamilyProperties(
                renderer.active_gpu,
                &mut family_property_count,
                family_properties.as_mut_ptr(),
            );
            let mut min_queue_flag: u32 = u32::MAX;
            for (i, value) in family_properties.iter().enumerate() {
                let queue_flags = family_properties[i].queueFlags;
                let is_graphics_queue =
                    (queue_flags & ffi::vk::VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT) > 0;
                let filter_flags = queue_flags & required_flags;
                if queue_type == QueueType::QueueTypeGraphics && is_graphics_queue {
                    found = true;
                    queue_family_index = i as u32;
                    queue_index = 0;
                    break;
                }
                if (queue_flags & required_flags) > 0 && (queue_flags & !required_flags) == 0 {}
            }
            let mut result = QueueFamilyResult {
                properties: MaybeUninit::zeroed().assume_init(),
                family_index: 0,
                queue_index: 0,
            };
            return Ok(result);
        }
        pub const GLOBAL_INSTANCE_EXTENSIONS: &[&[u8]] = &[
            ffi::vk::VK_KHR_SURFACE_EXTENSION_NAME,
            ffi::vk::VK_NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME,
            ffi::vk::VK_EXT_SWAPCHAIN_COLOR_SPACE_EXTENSION_NAME,
            ffi::vk::VK_KHR_DISPLAY_EXTENSION_NAME,
            ffi::vk::VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME,
        ];
        unsafe fn init_instance(
            renderer: &mut VulkanRenderer,
            desc: &RenderDesc,
        ) -> RendererResult<()> {
            let application_name = CString::new("3DEngine").expect("CString::new failed");
            let engine_name = CString::new("3DEngine").expect("CString::new failed");
            let mut loaded_extension: Vec<CString> = ::alloc::vec::Vec::new();
            let mut layer_count: u32 = 0;
            let mut ext_count: u32 = 0;
            ffi::vk::vkEnumerateInstanceLayerProperties(&mut layer_count, ptr::null_mut());
            ffi::vk::vkEnumerateInstanceExtensionProperties(
                ptr::null_mut(),
                &mut ext_count,
                ptr::null_mut(),
            );
            let mut layer_properties: Vec<ffi::vk::VkLayerProperties> =
                ::alloc::vec::from_elem(MaybeUninit::zeroed().assume_init(), layer_count as usize);
            let mut ext_properties: Vec<ffi::vk::VkExtensionProperties> =
                ::alloc::vec::from_elem(MaybeUninit::zeroed().assume_init(), ext_count as usize);
            ffi::vk::vkEnumerateInstanceLayerProperties(
                &mut layer_count,
                layer_properties.as_mut_ptr(),
            );
            ffi::vk::vkEnumerateInstanceExtensionProperties(
                ptr::null_mut(),
                &mut ext_count,
                ext_properties.as_mut_ptr(),
            );
            for layer_property in &layer_properties {
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            ::core::fmt::Arguments::new_v1(
                                &["vkinstance-layer: "],
                                &match (&CStr::from_ptr(layer_property.layerName.as_ptr())
                                    .to_string_lossy(),)
                                {
                                    _args => [::core::fmt::ArgumentV1::new(
                                        _args.0,
                                        ::core::fmt::Display::fmt,
                                    )],
                                },
                            ),
                            lvl,
                            &(
                                "renderer::vulkan::renderer",
                                "renderer::vulkan::renderer",
                                "src/vulkan/renderer.rs",
                                208u32,
                            ),
                        );
                    }
                };
            }
            for ext_property in &ext_properties {
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            ::core::fmt::Arguments::new_v1(
                                &["vkinstance-ext: "],
                                &match (&CStr::from_ptr(ext_property.extensionName.as_ptr())
                                    .to_string_lossy(),)
                                {
                                    _args => [::core::fmt::ArgumentV1::new(
                                        _args.0,
                                        ::core::fmt::Display::fmt,
                                    )],
                                },
                            ),
                            lvl,
                            &(
                                "renderer::vulkan::renderer",
                                "renderer::vulkan::renderer",
                                "src/vulkan/renderer.rs",
                                215u32,
                            ),
                        );
                    }
                };
            }
            let mut create_info = ffi::vk::VkApplicationInfo {
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
                wanted_instance_extensions
                    .insert(CString::from(CStr::from_bytes_with_nul_unchecked(ext)));
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
                _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["invalid configuration"],
                    &match () {
                        _args => [],
                    },
                )),
            }
            wanted_instance_layers.retain(move |layer| {
                for layer_property in &layer_properties {
                    let layer_name = unsafe { CStr::from_ptr(layer_property.layerName.as_ptr()) };
                    if layer_name.eq(layer.as_c_str()) {
                        return true;
                    }
                }
                {
                    let lvl = ::log::Level::Warn;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            ::core::fmt::Arguments::new_v1(
                                &["vkinstance-layer-missing: "],
                                &match (&layer.to_string_lossy(),) {
                                    _args => [::core::fmt::ArgumentV1::new(
                                        _args.0,
                                        ::core::fmt::Display::fmt,
                                    )],
                                },
                            ),
                            lvl,
                            &(
                                "renderer::vulkan::renderer",
                                "renderer::vulkan::renderer",
                                "src/vulkan/renderer.rs",
                                257u32,
                            ),
                        );
                    }
                };
                return false;
            });
            for target_layer in &wanted_instance_layers {
                let mut layer_target_ext_count: u32 = 0;
                ffi::vk::vkEnumerateInstanceExtensionProperties(
                    target_layer.as_ptr(),
                    &mut layer_target_ext_count,
                    ptr::null_mut(),
                );
                let mut layer_target_ext: Vec<ffi::vk::VkExtensionProperties> =
                    ::alloc::vec::from_elem(
                        MaybeUninit::zeroed().assume_init(),
                        ext_count as usize,
                    );
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
            wanted_instance_extensions.retain(move |layer| {
                for ext_property in &ext_properties {
                    let extension_name = CStr::from_ptr(ext_property.extensionName.as_ptr());
                    if extension_name.eq(layer) {
                        return true;
                    }
                }
                return false;
            });
            let mut enabled_layers: Vec<*const c_char> =
                Vec::with_capacity(wanted_instance_layers.len());
            let mut enabled_extensions: Vec<*const c_char> =
                Vec::with_capacity(wanted_instance_extensions.len());
            for layer in &wanted_instance_layers {
                enabled_layers.push(layer.as_ptr());
            }
            for ext in &wanted_instance_extensions {
                enabled_extensions.push(ext.as_ptr());
            }
            let mut create_info = ffi::vk::VkInstanceCreateInfo {
                sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                pNext: ptr::null(),
                flags: 0,
                pApplicationInfo: &create_info,
                enabledLayerCount: enabled_layers.len() as u32,
                ppEnabledLayerNames: enabled_layers.as_ptr(),
                enabledExtensionCount: enabled_extensions.len() as u32,
                ppEnabledExtensionNames: enabled_extensions.as_ptr(),
            };
            {
                let result =
                    ffi::vk::vkCreateInstance(&create_info, ptr::null(), &mut renderer.instance);
                if result != ffi::vk::VkResult_VK_SUCCESS {
                    return Err(crate::error::RendererError::VulkanError(result));
                }
            };
            Ok(())
        }
        unsafe fn init_device(
            renderer: &mut VulkanRenderer,
            desc: &RenderDesc,
        ) -> RendererResult<()> {
            if !(renderer.instance != ptr::null_mut()) {
                ::core::panicking::panic("assertion failed: renderer.instance != ptr::null_mut()")
            };
            let devices = VulkanGPUInfo::all(renderer.instance);
            let gpu = select_best_gpu(renderer.instance, &devices)?;
            renderer.active_gpu_properties = Some(gpu.get_device_properties().clone());
            renderer.active_gpu = gpu.get_device();
            Ok(())
        }
        impl Renderer<VulkanAPI> for VulkanRenderer {
            unsafe fn init(name: &CStr, desc: &RenderDesc) -> RendererResult<VulkanRenderer> {
                let mut renderer = VulkanRenderer {
                    instance: ptr::null_mut(),
                    active_gpu: ptr::null_mut(),
                    active_gpu_properties: None,
                    active_gpu_common_info: None,
                    device: ptr::null_mut(),
                };
                match init_instance(&mut renderer, desc) {
                    Err(e) => return Err(e),
                    _ => {}
                }
                Ok(renderer)
            }
            fn add_pipeline(&self) -> VulkanPipeline {
                ::core::panicking::panic("not yet implemented")
            }
            fn drop_pipeline(&self, pipeline: &mut super::VulkanPipeline) {
                if !(self.device != ptr::null_mut()) {
                    ::core::panicking::panic("assertion failed: self.device != ptr::null_mut()")
                };
                if !(pipeline.pipeline != ptr::null_mut()) {
                    ::core::panicking::panic(
                        "assertion failed: pipeline.pipeline != ptr::null_mut()",
                    )
                };
                unsafe {
                    ffi::vk::vkDestroyPipeline(self.device, pipeline.pipeline, ptr::null_mut());
                }
                pipeline.pipeline = ptr::null_mut();
            }
            unsafe fn add_fence(&self) -> RendererResult<super::VulkanFence> {
                if !(self.device != ptr::null_mut()) {
                    ::core::panicking::panic("assertion failed: self.device != ptr::null_mut()")
                };
                let fence_info: ffi::vk::VkFenceCreateInfo = ffi::vk::VkFenceCreateInfo {
                    sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
                    pNext: ptr::null_mut(),
                    flags: 0,
                };
                let mut fence = super::VulkanFence {
                    fence: ptr::null_mut(),
                    submitted: false,
                };
                let result = ffi::vk::vkCreateFence(
                    self.device,
                    &fence_info,
                    ptr::null_mut(),
                    &mut fence.fence,
                );
                if result != ffi::vk::VkResult_VK_SUCCESS {
                    return Err(VulkanError(result));
                }
                Ok(fence)
            }
            unsafe fn drop_fence(&self, fence: &mut super::VulkanFence) {
                if !(fence.fence != ptr::null_mut()) {
                    ::core::panicking::panic("assertion failed: fence.fence != ptr::null_mut()")
                };
                if !(self.device != ptr::null_mut()) {
                    ::core::panicking::panic("assertion failed: self.device != ptr::null_mut()")
                };
                ffi::vk::vkDestroyFence(self.device, fence.fence, ptr::null_mut());
                fence.fence = ptr::null_mut();
            }
            unsafe fn add_semaphore(&self) -> RendererResult<super::VulkanSemaphore> {
                if !(self.device != ptr::null_mut()) {
                    ::core::panicking::panic("assertion failed: self.device != ptr::null_mut()")
                };
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
            unsafe fn drop_semaphore(
                &self,
                semaphore: &mut super::VulkanSemaphore,
            ) -> RendererResult<()> {
                if !(semaphore.semaphore != ptr::null_mut()) {
                    ::core::panicking::panic(
                        "assertion failed: semaphore.semaphore != ptr::null_mut()",
                    )
                };
                if !(self.device != ptr::null_mut()) {
                    ::core::panicking::panic("assertion failed: self.device != ptr::null_mut()")
                };
                ffi::vk::vkDestroySemaphore(self.device, semaphore.semaphore, ptr::null_mut());
                semaphore.semaphore = ptr::null_mut();
                Ok(())
            }
            unsafe fn add_queue(&self, desc: &QueueDesc) -> RendererResult<super::VulkanQueue> {
                let node_index = desc.node_index;
                ::core::panicking::panic("not yet implemented")
            }
            unsafe fn remove_queue(&self, queue: &mut super::VulkanQueue) {
                ::core::panicking::panic("not yet implemented")
            }
            fn add_swap_chain(&self) {
                ::core::panicking::panic("not yet implemented")
            }
            fn drop_swap_chain(&self) {
                ::core::panicking::panic("not yet implemented")
            }
            fn add_cmd_pool(&self, desc: &CmdPoolDesc<VulkanAPI>) {
                ::core::panicking::panic("not yet implemented")
            }
            fn drop_cmd_pool(&self) {
                ::core::panicking::panic("not yet implemented")
            }
            fn add_cmd(&self) {
                ::core::panicking::panic("not yet implemented")
            }
            fn drop_cmd(&self) {
                ::core::panicking::panic("not yet implemented")
            }
            fn add_render_target(&self) -> RendererResult<VulkanRenderTarget> {
                ::core::panicking::panic("not yet implemented")
            }
            fn remove_render_target(&self, target: &mut VulkanRenderTarget) {
                ::core::panicking::panic("not yet implemented")
            }
            fn add_root_signature(&self) {
                ::core::panicking::panic("not yet implemented")
            }
            fn remove_root_signature() {
                ::core::panicking::panic("not yet implemented")
            }
            fn reset_cmd_pool(&self) {
                ::core::panicking::panic("not yet implemented")
            }
        }
    }
    mod types {
        use crate::{ffi, types::QueueType};
        impl QueueType {
            pub fn to_vk_queue(&self) -> ffi::vk::VkQueueFlagBits {
                match self {
                    QueueType::QueueTypeGraphics => ffi::vk::VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT,
                    QueueType::QueueTypeTransfer => ffi::vk::VkQueueFlagBits_VK_QUEUE_TRANSFER_BIT,
                    QueueType::QueueTypeCompute => ffi::vk::VkQueueFlagBits_VK_QUEUE_COMPUTE_BIT,
                    _ => {
                        if !false {
                            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                                &["invalid Queue Type"],
                                &match () {
                                    _args => [],
                                },
                            ))
                        };
                        ffi::vk::VkQueueFlagBits_VK_QUEUE_FLAG_BITS_MAX_ENUM
                    }
                }
            }
        }
    }
    use crate::{
        ffi, APIType, Api, Command, DescriptorIndexMap, Fence, GPUCommonInfo, Queue, RenderContext,
        RenderTarget, Renderer, RendererResult, Sampler, Semaphore, Shader, Texture,
    };
    pub struct VulkanAPI;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for VulkanAPI {
        #[inline]
        fn clone(&self) -> VulkanAPI {
            match *self {
                VulkanAPI => VulkanAPI,
            }
        }
    }
    impl crate::Api for VulkanAPI {
        type RenderContext = VulkanRenderContext;
        type Renderer = VulkanRenderer;
        type Pipeline = VulkanPipeline;
        type Fence = VulkanFence;
        type Semaphore = VulkanSemaphore;
        type Queue = VulkanQueue;
        type Texture = VulkanTexture;
        type Shader = VulkanShader;
        type RenderTarget = VulkanRenderTarget;
        type DescriptorIndexMap = VulkanDescriptorIndexMap;
        type Sampler = VulkanSampler;
        type Command = VulkanCommand;
        const CURRENT_API: APIType = APIType::Vulkan;
    }
    pub struct VulkanCommand {}
    impl Command for VulkanCommand {
        fn begin_cmd(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn end_cmd(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn cmd_bind_render_target(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn cmd_set_shading_rate(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn cmd_set_viewport(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn cmd_set_scissor(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn cmd_set_stencil_reference_value(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn cmd_bind_pipeline(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn cmd_bind_descriptor_set(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn cmd_bind_index_buffer(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn cmd_raw(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn cmd_draw_instanced(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn cmd_draw_indexed(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn cmd_draw_indexed_instanced(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn cmd_dispatch(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn cmd_resource_barrier(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn cmd_update_virtual_texture(&self) {
            ::core::panicking::panic("not yet implemented")
        }
    }
    pub struct VulkanRenderContext {
        gpu: ffi::vk::VkPhysicalDevice,
        gpu_properties: ffi::vk::VkPhysicalDeviceProperties2,
        common: GPUCommonInfo,
    }
    impl RenderContext for VulkanRenderContext {}
    pub struct VulkanRenderTarget {}
    impl RenderTarget for VulkanRenderTarget {}
    pub struct VulkanSampler {}
    impl Sampler for VulkanSampler {}
    pub struct VulkanDescriptorIndexMap {}
    impl DescriptorIndexMap for VulkanDescriptorIndexMap {}
    pub struct VulkanShader {}
    impl Shader for VulkanShader {}
    pub struct VulkanTexture {}
    impl Texture for VulkanTexture {}
    pub struct VulkanSemaphore {
        semaphore: ffi::vk::VkSemaphore,
        signaled: bool,
    }
    impl Semaphore for VulkanSemaphore {}
    pub struct VulkanQueue {}
    impl Queue<VulkanAPI> for VulkanQueue {
        fn submit(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn present(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn wait_idle(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn fence_status(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn wait_fence(&self) {
            ::core::panicking::panic("not yet implemented")
        }
        fn toggle_v_sync(&self) {
            ::core::panicking::panic("not yet implemented")
        }
    }
    pub struct VulkanFence {
        pub(in crate::vulkan) fence: ffi::vk::VkFence,
        pub(in crate::vulkan) submitted: bool,
    }
    impl Fence for VulkanFence {}
    pub struct VulkanPipeline {
        pipeline: ffi::vk::VkPipeline,
    }
    impl crate::Pipeline for VulkanPipeline {}
    pub struct VulkanRenderer {
        pub(in crate::vulkan) instance: ffi::vk::VkInstance,
        pub(in crate::vulkan) active_gpu: ffi::vk::VkPhysicalDevice,
        pub(in crate::vulkan) active_gpu_properties: Option<ffi::vk::VkPhysicalDeviceProperties>,
        pub(in crate::vulkan) active_gpu_common_info: Option<GPUCommonInfo>,
        pub(in crate::vulkan) device: ffi::vk::VkDevice,
    }
}
pub mod ffi {
    pub use vulkan_sys as vk;
}
#[bitfield]
pub struct GPUSupported {
    RaytracingSupported: bool,
    YCbCrExtension: bool,
    KHRSpirv14Extension: bool,
    KHRAccelerationStructureExtension: bool,
    KHRRayTracingPipelineExtension: bool,
    KHRRayQueryExtension: bool,
    AMDGCNShaderExtension: bool,
    AMDDrawIndirectCountExtension: bool,
    DescriptorIndexingExtension: bool,
    ShaderFloatControlsExtension: bool,
    BufferDeviceAddressExtension: bool,
    DeferredHostOperationsExtension: bool,
    DrawIndirectCountExtension: bool,
    DedicatedAllocationExtension: bool,
    ExternalMemoryExtension: bool,
    DebugMarkerSupport: bool,
}
pub struct GPUCommonInfo {}
pub enum APIType {
    None,
    Vulkan,
}
pub trait Api: Clone + Sized {
    type RenderContext: RenderContext;
    type Renderer: Renderer<Self>;
    type Pipeline: Pipeline;
    type Fence: Fence;
    type Semaphore: Semaphore;
    type Queue: Queue<Self>;
    type Texture: Texture;
    type Shader: Shader;
    type RenderTarget: RenderTarget;
    type DescriptorIndexMap: DescriptorIndexMap;
    type Sampler: Sampler;
    type Command: Command;
    const CURRENT_API: APIType;
}
pub trait Renderer<A: Api>: Sized {
    unsafe fn init(name: &CStr, desc: &RenderDesc) -> RendererResult<A::Renderer>;
    fn add_pipeline(&self) -> A::Pipeline;
    fn drop_pipeline(&self, pipeline: &mut A::Pipeline);
    unsafe fn add_fence(&self) -> RendererResult<A::Fence>;
    unsafe fn drop_fence(&self, fence: &mut A::Fence);
    unsafe fn add_semaphore(&self) -> RendererResult<A::Semaphore>;
    unsafe fn drop_semaphore(&self, semaphore: &mut A::Semaphore) -> RendererResult<()>;
    unsafe fn add_queue(&self, desc: &QueueDesc) -> RendererResult<A::Queue>;
    unsafe fn remove_queue(&self, queue: &mut A::Queue);
    fn add_swap_chain(&self);
    fn drop_swap_chain(&self);
    fn add_cmd_pool(&self, desc: &CmdPoolDesc<A>);
    fn drop_cmd_pool(&self);
    fn add_cmd(&self);
    fn drop_cmd(&self);
    fn add_render_target(&self) -> RendererResult<A::RenderTarget>;
    fn remove_render_target(&self, target: &mut A::RenderTarget);
    fn add_root_signature(&self);
    fn remove_root_signature();
    fn reset_cmd_pool(&self);
}
pub trait Command {
    fn begin_cmd(&self);
    fn end_cmd(&self);
    fn cmd_bind_render_target(&self);
    fn cmd_set_shading_rate(&self);
    fn cmd_set_viewport(&self);
    fn cmd_set_scissor(&self);
    fn cmd_set_stencil_reference_value(&self);
    fn cmd_bind_pipeline(&self);
    fn cmd_bind_descriptor_set(&self);
    fn cmd_bind_index_buffer(&self);
    fn cmd_raw(&self);
    fn cmd_draw_instanced(&self);
    fn cmd_draw_indexed(&self);
    fn cmd_draw_indexed_instanced(&self);
    fn cmd_dispatch(&self);
    fn cmd_resource_barrier(&self);
    fn cmd_update_virtual_texture(&self);
}
pub trait RenderContext {}
pub trait Texture {}
pub trait Shader {}
pub trait Queue<A: Api> {
    fn submit(&self);
    fn present(&self);
    fn wait_idle(&self);
    fn fence_status(&self);
    fn wait_fence(&self);
    fn toggle_v_sync(&self);
}
pub trait Sampler {}
pub trait DescriptorIndexMap {}
pub trait RenderTarget {}
pub trait Semaphore {}
pub trait Fence {}
pub trait Pipeline {}
