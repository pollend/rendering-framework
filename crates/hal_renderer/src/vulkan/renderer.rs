use crate::configuration::*;
use crate::error::{HalError, HalResult};
use crate::{check_vk_result, ffi};
use log::{error, info, trace, warn};
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::{
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


pub struct HalVKRenderConfiguration<'config> {
    instanceLayers: &'config Vec<CString>,
    instanceExtensions: &'config Vec<CString>
}

pub struct HalVKRenderer {
    instance: ffi::vk::VkInstance,
    active_gpu: ffi::vk::VkPhysicalDevice,
    device: ffi::vk::VkDevice,
}

pub fn create_instance (
    renderer: &mut HalVKRenderer,
    user_defined_layers: &Vec<CString>,
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

    let mut loaded_layers = user_defined_layers.clone();
    loaded_layers.retain(move |layer| {
        for layer_property in &layer_properties {
            let layer_name = unsafe { CStr::from_ptr(layer_property.layerName.as_ptr()) };
            if layer_name.eq(layer.as_c_str()) {
                return true;
            }
        }
        warn!("vkinstance-layer-missing: {}", layer.to_string_lossy());
        return false;
    });

    let mut wanted_instance_extensions: Vec<CString> = vec![];
    for ext in GLOBAL_INSTANCE_EXTENSIONS {
        // let a = CStr::from_bytes_with_nul_unchecked(ext);
        unsafe {
            wanted_instance_extensions.push(CString::from(CStr::from_bytes_with_nul_unchecked(ext)));
        }
    }

    match &configuration.render_type {
        RendererConfigType::Vulkan(config) => {
            for extension in config.instanceExtensions {
                wanted_instance_extensions.push(extension.clone());
            }
        }
        _ => panic!("invalid configuration"),
    }

    // Layer extensions
    for target_layer in &loaded_layers {
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

        for extension in wanted_instance_extensions.drain_filter(move |extension| {
            for ext_property in &layer_target_ext {
                let extension_name = unsafe { CStr::from_ptr(ext_property.extensionName.as_ptr()) };
                if extension_name.eq(extension) {
                    return true;
                }
            }
            return false;
        }) {
            loaded_extension.push(extension);
        }
    }

    // Standalone extensions
    for extension in wanted_instance_extensions.drain_filter(move |extension| {
        for ext_property in &ext_properties {
            let extension_name = unsafe { CStr::from_ptr(ext_property.extensionName.as_ptr()) };
            if extension_name.eq(extension) {
                // loaded_extension.push(extension.clone());
                return true;
            }
        }
        return false;
    }) {
        loaded_extension.push(extension);
    }


    let mut enabled_layers: Vec<*const c_char> = Vec::with_capacity(loaded_layers.len());
    let mut enabled_extensions: Vec<*const c_char> = Vec::with_capacity(loaded_extension.len());
    for layer in &loaded_layers {
        enabled_layers.push(layer.as_ptr());
    }

    for ext in &loaded_extension {
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
        ppEnabledExtensionNames: enabled_extensions.as_ptr()
    };
    unsafe {
        check_vk_result!(ffi::vk::vkCreateInstance(&create_info, ptr::null(), &mut renderer.instance));
    }

    Ok(())
}

impl Drop for HalVKRenderer {
    fn drop(&mut self) {
        if self.instance != ptr::null_mut() {
            ffi::vk::vkDestroyInstance(self.instance);
        }
    }
}

impl HalVKRenderer {
    pub fn new(config: &RendererConfig) -> HalResult<HalVKRenderer> {
        let mut renderer = HalVKRenderer {
            instance: ptr::null_mut(),
            active_gpu: ptr::null_mut(),
            device: ptr::null_mut(),
        };
        let features: Vec<CString> = vec![];
        create_instance(&mut renderer, &features, &config)?;

        Ok(renderer)
    }
}
