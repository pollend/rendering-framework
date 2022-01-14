use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::os::raw::c_char;
use std::ptr;
use log::{info, warn};
use crate::vulkan::{renderer, VulkanFence, VulkanPipeline, VulkanRenderer, VulkanRenderTarget, VulkanSemaphore};
use crate::{check_vk_result, CmdPoolDesc, ffi, QueueDesc, RenderDesc, Renderer, RendererResult, VulkanAPI};
use crate::desc::RenderDescImp;
use crate::error::RendererError::VulkanError;


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


fn init_device(
    renderer: &mut VulkanRenderer,
    desc: &RenderDesc
) -> RendererResult<()>{
    assert!(renderer.instance != ptr::null_mut());

    Ok(())
}


fn init_instance(
    renderer: &mut VulkanRenderer,
    desc: &RenderDesc,
) -> RendererResult<()> {
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

impl Renderer<VulkanAPI> for VulkanRenderer {
    fn init(name: &CStr, desc: &RenderDesc) -> RendererResult<VulkanRenderer> {
        let mut renderer = VulkanRenderer {
            instance: ptr::null_mut(),
            active_gpu: ptr::null_mut(),
            device: ptr::null_mut(),
        };

        match init_instance(&mut renderer, desc) {
            Ok(()) => info!("instance created"),
            Err(e) => return Err(e),
        }
        Ok(renderer)
    }

    fn add_pipeline(&self) -> VulkanPipeline {
        todo!()
    }

    fn drop_pipeline(&self, pipeline: &mut super::VulkanPipeline ) {
        assert!(self.device != ptr::null_mut());
        assert!(pipeline.pipeline != ptr::null_mut());

        unsafe {
            ffi::vk::vkDestroyPipeline(self.device, pipeline.pipeline,
                                       ptr::null_mut());
        }
        pipeline.pipeline = ptr::null_mut();
    }

    unsafe fn add_fence(&self) -> RendererResult<super::VulkanFence> {
        assert!(self.device != ptr::null_mut());

        let fence_info: ffi::vk::VkFenceCreateInfo = ffi::vk::VkFenceCreateInfo{
            sType : ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
            pNext: ptr::null_mut(),
            flags: 0,
        };
        let mut fence = super::VulkanFence {
            fence: ptr::null_mut(),
            submitted: false
        };
        if let result = ffi::vk::vkCreateFence(
                self.device,
                &fence_info,
                ptr::null_mut(),
                &mut fence.fence
            ) != ffi::vk::VkResult_VK_SUCCESS {
            Err(VulkanError(result));
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
            flags: 0
        };
        let mut semaphore = VulkanSemaphore {
            semaphore: ptr::null_mut(),
            signaled: false
        };
        if let result = ffi::vk::vkCreateSemaphore(self.device, &add_info, ptr::null_mut(), &mut semaphore.semaphore) != ffi::vk::VkResult_VK_SUCCESS {
            Err(VulkanError(result))
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
        todo!()
    }

    unsafe fn remove_queue(&self, queue: &mut super::VulkanQueue) {
        todo!()
    }

    fn add_swap_chain(&self) {
        todo!()
    }

    fn drop_swap_chain(&self) {
        todo!()
    }

    fn add_cmd_pool(&self, desc: &CmdPoolDesc<VulkanAPI>) {
        todo!()
    }

    fn drop_cmd_pool(&self) {
        todo!()
    }

    fn add_cmd(&self) {
        todo!()
    }

    fn drop_cmd(&self) {
        todo!()
    }

    fn add_render_target(&self) -> RendererResult<VulkanRenderTarget> {
        todo!()
    }

    fn remove_render_target(&self, target: &mut VulkanRenderTarget) {
        todo!()
    }

    fn add_root_signature(&self) {
        todo!()
    }

    fn remove_root_signature() {
        todo!()
    }

    fn reset_cmd_pool(&self) {
        todo!()
    }

}
