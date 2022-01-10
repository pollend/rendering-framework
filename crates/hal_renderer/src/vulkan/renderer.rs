use crate::{
    check_vk_result,
    configuration::*,
    error::{HalError, HalResult},
    ffi,
    renderer::Renderer,
};
use log::{error, info, trace, warn};
use std::{
    collections::HashSet,
    ffi::{CStr, CString},
    marker::PhantomData,
    mem::MaybeUninit,
    os::raw::c_char,
    ptr,
    rc::{Rc, Weak},
    vec::Vec,
};

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

static mut GLOBAL_ALLOCATION_CALLBACK: ffi::vk::VkAllocationCallbacks =
    ffi::vk::VkAllocationCallbacks {
        pUserData: ptr::null_mut(),
        pfnAllocation: Option::None,
        pfnReallocation: Option::None,
        pfnFree: Option::None,
        pfnInternalAllocation: Option::None,
        pfnInternalFree: Option::None,
    };

pub struct VulkanRenderer {
    pub(in crate::vulkan) instance: ffi::vk::VkInstance,
    pub(in crate::vulkan) active_gpu: ffi::vk::VkPhysicalDevice,
    pub(in crate::vulkan) device: ffi::vk::VkDevice,
}

pub fn select_best_gpu(renderer: &mut VulkanRenderer) -> HalResult<()> {
    // assert!(renderer.instance != ptr.null());
    //
    // let mut device_count: u32 = 0;
    // let vk_result = ffi::vk::vkEnumeratePhysicalDevices(renderer.instance, &device_count, ptr::null_mut());
    // assert!(vk_result == ffi::vk::VkResult_VK_SUCCESS);
    //
    // if device_count < 1 {
    //     error!("failed to find physical Vulkan Device");
    //     Err(HalError::UnsupportedDevice)
    // }
    // let mut physical_devices : Vec<ffi::vk::VkPhysicalDevice> = Vec::with_capacity(device_count as usize);
    // let mut device_properties: Vec<ffi::vk::VkPhysicalDeviceProperties2> = Vec::with_capacity(device_count as usize);
    // let mut device_memory_properties: Vec<ffi::vk::VkPhysicalDeviceMemoryProperties> = Vec::with_capacity(device_count as usize);
    // let mut device_features: Vec<ffi::vk::VkPhysicalDeviceFeatures2KHR> = Vec::with_capacity(device_count as usize);
    // let mut device_family_properties: Vec<ffi::vk::VkQueueFamilyProperties> = Vec::with_capacity(device_count as usize);

    Ok(())
}

pub fn create_device(
    renderer: &mut VulkanRenderer,
    configuration: &RendererConfig,
) -> HalResult<()> {
    assert!(renderer.instance != ptr::null_mut());
    let mut device_extensions: Vec<CString> = Vec::new();
    Ok(())
}

pub fn create_instance(
    renderer: &mut VulkanRenderer,
    configuration: &RendererConfig,
) -> HalResult<()> {
    // layers: Vec<const *c_char> = Vec::new();
    let application_name = CString::new("3DEngine").expect("CString::new failed");
    let engine_name = CString::new("3DEngine").expect("CString::new failed");

    let mut loaded_extension: Vec<CString> = vec![];

    let mut layer_count: u32 = 0;
    let mut ext_count: u32 = 0;
    unsafe {
        ffi::vk::vkEnumerateInstanceLayerProperties(&mut layer_count, ptr::null_mut());
        ffi::vk::vkEnumerateInstanceExtensionProperties(
            ptr::null_mut(),
            &mut ext_count,
            ptr::null_mut(),
        );
    }

    let mut layer_properties: Vec<ffi::vk::VkLayerProperties> =
        vec![unsafe { MaybeUninit::zeroed().assume_init() }; layer_count as usize];
    let mut ext_properties: Vec<ffi::vk::VkExtensionProperties> =
        vec![unsafe { MaybeUninit::zeroed().assume_init() }; ext_count as usize];
    unsafe {
        ffi::vk::vkEnumerateInstanceLayerProperties(
            &mut layer_count,
            layer_properties.as_mut_ptr(),
        );
        ffi::vk::vkEnumerateInstanceExtensionProperties(
            ptr::null_mut(),
            &mut ext_count,
            ext_properties.as_mut_ptr(),
        );
    }

    for layer_property in &layer_properties {
        info!(
            "vkinstance-layer: {}",
            unsafe { CStr::from_ptr(layer_property.layerName.as_ptr()) }.to_string_lossy()
        );
    }

    for ext_property in &ext_properties {
        info!(
            "vkinstance-ext: {}",
            unsafe { CStr::from_ptr(ext_property.extensionName.as_ptr()) }.to_string_lossy()
        );
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
        unsafe {
            wanted_instance_extensions
                .insert(CString::from(CStr::from_bytes_with_nul_unchecked(ext)));
        }
    }

    match &configuration.render_type {
        RendererConfigType::Vulkan(config) => {
            for extension in &config.instance_extensions {
                wanted_instance_extensions.insert(extension.clone());
            }

            for layer in &config.instance_layers {
                wanted_instance_layers.insert(layer.clone());
            }
        }
        _ => panic!("invalid configuration"),
    }

    wanted_instance_layers.retain(move |layer| {
        for layer_property in &layer_properties {
            let layer_name = unsafe { CStr::from_ptr(layer_property.layerName.as_ptr()) };
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
        unsafe {
            ffi::vk::vkEnumerateInstanceExtensionProperties(
                target_layer.as_ptr(),
                &mut layer_target_ext_count,
                ptr::null_mut(),
            );
        }
        let mut layer_target_ext: Vec<ffi::vk::VkExtensionProperties> =
            vec![unsafe { MaybeUninit::zeroed().assume_init() }; ext_count as usize];
        unsafe {
            ffi::vk::vkEnumerateInstanceExtensionProperties(
                target_layer.as_ptr(),
                &mut layer_target_ext_count,
                layer_target_ext.as_mut_ptr(),
            );
        }

        wanted_instance_extensions.retain(move |layer| {
            for ext_property in &layer_target_ext {
                let extension_name = unsafe { CStr::from_ptr(ext_property.extensionName.as_ptr()) };
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
            let extension_name = unsafe { CStr::from_ptr(ext_property.extensionName.as_ptr()) };
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
    unsafe {
        check_vk_result!(ffi::vk::vkCreateInstance(
            &create_info,
            ptr::null(),
            &mut renderer.instance
        ));
    }

    Ok(())
}

impl Drop for VulkanRenderer {
    fn drop(&mut self) {
        if self.instance != ptr::null_mut() {
            unsafe {
                ffi::vk::vkDestroyInstance(self.instance, ptr::null());
            }
        }
    }
}

impl VulkanRenderer {
    pub fn new(config: &RendererConfig) -> HalResult<VulkanRenderer> {
        let mut renderer = VulkanRenderer {
            instance: ptr::null_mut(),
            active_gpu: ptr::null_mut(),
            device: ptr::null_mut(),
        };
        create_instance(&mut renderer, &config)?;

        create_device(&mut renderer, &config)?;

        Ok(renderer)
    }
}
