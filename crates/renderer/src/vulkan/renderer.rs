use crate::{
    check_vk_result,
    desc::RenderDescImp,
    error::RendererError::{VulkanError},
    ffi,
    types::QueueType,
    vulkan::{
        types::{GLOBAL_INSTANCE_EXTENSIONS, VulkanSupportedFeatures},
        device::VulkanGPUInfo, VulkanPipeline, VulkanRenderTarget,
        VulkanRenderer, VulkanSemaphore,
    }, CmdPoolDesc, QueueDesc, RenderDesc, Renderer, RendererResult, VulkanAPI,
};
use log::{info, warn};
use std::{
    collections::HashSet,
    ffi::{CStr, CString},
    mem::MaybeUninit,
    os::raw::c_char,
    ptr
};


struct QueueFamilyResult {
    properties: ffi::vk::VkQueueFamilyProperties,
    family_index: u8,
    queue_index: u8,
}

unsafe fn util_find_queue_family_index(
    renderer: &VulkanRenderer,
    _node_index: u32,
    queue_type: QueueType,
) -> RendererResult<QueueFamilyResult> {
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

    let _min_queue_flag: u32 = u32::MAX;

    for (i, _value) in family_properties.iter().enumerate() {
        let queue_flags = family_properties[i].queueFlags;
        let is_graphics_queue = (queue_flags & ffi::vk::VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT) > 0;
        let _filter_flags = queue_flags & required_flags;
        if queue_type == QueueType::QueueTypeGraphics && is_graphics_queue {
            found = true;
            queue_family_index = i as u32;
            queue_index = 0;
            break;
        }
        if (queue_flags & required_flags) > 0 && (queue_flags & !required_flags) == 0 {}
    }

    let result = QueueFamilyResult {
        properties: MaybeUninit::zeroed().assume_init(),
        family_index: 0,
        queue_index: 0,
    };

    return Ok(result);
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

unsafe fn init_device(renderer: &mut VulkanRenderer, _desc: &RenderDesc) -> RendererResult<()> {
    assert!(renderer.instance != ptr::null_mut());

    let devices = VulkanGPUInfo::all(renderer.instance);
    let gpu = VulkanGPUInfo::select_best_gpu(renderer.instance, &devices)?;

    renderer.active_gpu_properties = Some(gpu.get_device_properties().clone());
    // renderer.active_gpu_common_info = Some(gpu
    renderer.active_gpu = gpu.get_device();
    Ok(())
}

impl Renderer<VulkanAPI> for VulkanRenderer {
    unsafe fn init(_name: &CStr, desc: &RenderDesc) -> RendererResult<VulkanRenderer> {
        let mut renderer = VulkanRenderer {
            instance: ptr::null_mut(),
            active_gpu: ptr::null_mut(),
            active_gpu_properties: None,
            active_gpu_common_info: None,
            device: ptr::null_mut(),
            features: VulkanSupportedFeatures::NONE
        };

        match init_instance(&mut renderer, desc) {
            // Ok() => ,
            Err(e) => return Err(e),
            _ => {}
        }
        Ok(renderer)
    }

    fn add_pipeline(&self) -> VulkanPipeline {
        todo!()
    }

    fn drop_pipeline(&self, pipeline: &mut super::VulkanPipeline) {
        assert!(self.device != ptr::null_mut());
        assert!(pipeline.pipeline != ptr::null_mut());

        unsafe {
            ffi::vk::vkDestroyPipeline(self.device, pipeline.pipeline, ptr::null_mut());
        }
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

    fn add_swap_chain(&self) {
        todo!()
    }

    fn drop_swap_chain(&self) {
        todo!()
    }

    fn add_cmd_pool(&self, _desc: &CmdPoolDesc<VulkanAPI>) {
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

    fn remove_render_target(&self, _target: &mut VulkanRenderTarget) {
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
