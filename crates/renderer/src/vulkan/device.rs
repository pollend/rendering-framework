use crate::{ffi, GPUCommonInfo, RendererResult, GPUSupportedFeatures, ShadingRates};
use std::{mem, mem::MaybeUninit, ptr};
use log::info;
use crate::error::RendererError;

pub(in crate::vulkan) struct VulkanGPUInfo {
    physical_device_features: ffi::vk::VkPhysicalDeviceFeatures2,
    physical_memory_properties: ffi::vk::VkPhysicalDeviceMemoryProperties,
    physical_device_properties: ffi::vk::VkPhysicalDeviceProperties2,
    queue_family_properties: Vec<ffi::vk::VkQueueFamilyProperties>,
    subgroup_properties: ffi::vk::VkPhysicalDeviceSubgroupProperties,
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
        let limits = &self.get_device_properties().limits;
        GPUCommonInfo {
            uniform_buffer_alignment: limits.minUniformBufferOffsetAlignment as u32,
            upload_buffer_texture_alignment: limits.optimalBufferCopyOffsetAlignment as u32,
            upload_buffer_texture_row_alignment: limits.optimalBufferCopyRowPitchAlignment as u32,
            max_vertex_input_bindings: limits.maxVertexInputBindings as u32,
            max_root_signature_dwords: 0,
            wave_lane_count: 0,
            features: GPUSupportedFeatures::ROVsSupported,
            shading_rates: ShadingRates::ShadingRateFull
        }

    }

    pub unsafe fn select_best_gpu(
        _instance: ffi::vk::VkInstance,
        vk_gpus: &Vec<VulkanGPUInfo>,
    ) -> RendererResult<&VulkanGPUInfo> {
        let is_device_better = |current: &VulkanGPUInfo, to_test: &VulkanGPUInfo| -> bool {
            let current_device_properties = current.get_device_properties();
            let test_device_properties = to_test.get_device_properties();

            // if the current gpu is discrete and the gpu to test against isn't take preference over discrete
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
                    if heap.flags & ffi::vk::VkMemoryHeapFlagBits_VK_MEMORY_HEAP_DEVICE_LOCAL_BIT > 0 {
                        total_current_vram += heap.size
                    }
                }

                for i in 0..test_memory_properties.memoryHeapCount as usize {
                    let heap = &test_memory_properties.memoryHeaps[i];
                    if heap.flags & ffi::vk::VkMemoryHeapFlagBits_VK_MEMORY_HEAP_DEVICE_LOCAL_BIT > 0 {
                        total_test_vram += heap.size
                    }
                }
                return total_test_vram > total_current_vram;
            }
            return false;
        };

        let mut best_option: Option<usize> = None;
        for (j, el) in vk_gpus.iter().enumerate() {
            info!("GPU[{}]", el.get_device_properties().vendorID);

            if best_option == None || is_device_better(el, &vk_gpus[best_option.unwrap()]) {
                for property in el.get_queue_family_properties() {
                    // get graphics queue family
                    if property.queueFlags & ffi::vk::VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT > 0 {
                        best_option = Some(j);
                        break;
                    }
                }
            }
        }
        match best_option {
            Some(idx) => Ok(&vk_gpus[idx]),
            _ => Err(RendererError::Unhandled),
        }
    }

    pub unsafe fn all(instance: ffi::vk::VkInstance) -> Vec<VulkanGPUInfo> {
        assert!(instance != ptr::null_mut());

        let mut device_count: u32 = 0;
        let mut vk_result =
            ffi::vk::vkEnumeratePhysicalDevices(instance, &mut device_count, ptr::null_mut());

        assert!(vk_result == ffi::vk::VkResult_VK_SUCCESS);
        let mut details: Vec<VulkanGPUInfo> = Vec::with_capacity(device_count as usize);
        let mut physical_devices: Vec<ffi::vk::VkPhysicalDevice> =
            Vec::with_capacity(device_count as usize);
        vk_result = ffi::vk::vkEnumeratePhysicalDevices(
            instance,
            &mut device_count,
            physical_devices.as_mut_ptr(),
        );

        assert!(vk_result == ffi::vk::VkResult_VK_SUCCESS);
        for device in physical_devices {
            match VulkanGPUInfo::gpu(device) {
                Ok(detail) => details.push(detail),
                _ => {}
            }
        }
        details
    }

    pub unsafe fn gpu(device: ffi::vk::VkPhysicalDevice) -> RendererResult<VulkanGPUInfo> {
        let mut detail: VulkanGPUInfo = VulkanGPUInfo {
            physical_device_features: ffi::vk::VkPhysicalDeviceFeatures2 {
                sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR,
                pNext: ptr::null_mut(),
                features: unsafe { MaybeUninit::zeroed().assume_init() },
            },
            physical_memory_properties: unsafe { MaybeUninit::zeroed().assume_init() },
            physical_device_properties: ffi::vk::VkPhysicalDeviceProperties2 {
                sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR,
                pNext: ptr::null_mut(),
                properties: unsafe { MaybeUninit::zeroed().assume_init() },
            },
            queue_family_properties: vec![],
            subgroup_properties: ffi::vk::VkPhysicalDeviceSubgroupProperties {
                sType:
                    ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES,
                pNext: ptr::null_mut(),
                subgroupSize: 0,
                supportedStages: 0,
                supportedOperations: 0,
                quadOperationsInAllStages: 0,
            },
            device,
        };
        detail.subgroup_properties.pNext = detail.physical_device_properties.pNext;
        detail.physical_device_properties.pNext = mem::transmute(&mut detail.subgroup_properties);

        ffi::vk::vkGetPhysicalDeviceMemoryProperties(
            device,
            &mut detail.physical_memory_properties,
        );
        ffi::vk::vkGetPhysicalDeviceFeatures2(device, &mut detail.physical_device_features);
        ffi::vk::vkGetPhysicalDeviceProperties2KHR(device, &mut detail.physical_device_properties);

        let mut queue_family_property_count: u32 = 0;
        ffi::vk::vkGetPhysicalDeviceQueueFamilyProperties(
            device,
            &mut queue_family_property_count,
            ptr::null_mut(),
        );
        detail.queue_family_properties =
            vec![MaybeUninit::zeroed().assume_init(); queue_family_property_count as usize];
        ffi::vk::vkGetPhysicalDeviceQueueFamilyProperties(
            device,
            &mut queue_family_property_count,
            detail.queue_family_properties.as_mut_ptr(),
        );
        Ok(detail)
    }
}
