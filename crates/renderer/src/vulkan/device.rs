use std::mem::MaybeUninit;
use std::{mem, ptr};
use crate::{ffi, GPUCommonInfo, RendererResult};
use crate::vulkan::VulkanRenderer;

pub(in crate::vulkan) struct VulkanGPUInfo {
    physical_device_features: ffi::vk::VkPhysicalDeviceFeatures2,
    physical_memory_properties: ffi::vk::VkPhysicalDeviceMemoryProperties,
    physical_device_properties: ffi::vk::VkPhysicalDeviceProperties2,
    queue_family_properties: Vec<ffi::vk::VkQueueFamilyProperties>,
    common: GPUCommonInfo
}

impl VulkanGPUInfo {
    pub fn get_physical_features(&self) -> &ffi::vk::VkPhysicalDeviceFeatures {
        return &self.physical_device_features.features;
    }

    pub fn get_device_properties(&self) -> &ffi::vk::VkPhysicalDeviceFeatures {
        return &self.physical_device_properties.properties;
    }


    pub unsafe fn all(instance: ffi::vk::VkInstance) -> Vec<VulkanGPUInfo> {
        assert!(instance != ptr::null_mut());

        let mut device_count: u32 = 0;
        let mut vk_result =
            ffi::vk::vkEnumeratePhysicalDevices(
                instance,
                &mut device_count,
                ptr::null_mut(),
            );

        assert!(vk_result == ffi::vk::VkResult_VK_SUCCESS);
        let mut details: Vec<VulkanGPUInfo> = Vec::with_capacity(device_count as usize);
        let mut physical_devices: Vec<ffi::vk::VkPhysicalDevice> =
            Vec::with_capacity(device_count as usize);
        vk_result =
            ffi::vk::vkEnumeratePhysicalDevices(
                instance,
                &mut device_count,
                physical_devices.as_mut_ptr(),
            );

        assert!(vk_result == ffi::vk::VkResult_VK_SUCCESS);
        for device in &physical_devices {
            match VulkanGPUInfo::gpu(device) {
                Ok(detail) => details.push(detail),
                _ => {}
            }
        }
        details
    }

    pub unsafe fn gpu(device: &ffi::vk::VkPhysicalDevice) -> RendererResult<VulkanGPUInfo> {
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
            // common: None,
            common: GPUCommonInfo {

            }
        };
        let mut subgroup_properties: ffi::vk::VkPhysicalDeviceSubgroupProperties =
            ffi::vk::VkPhysicalDeviceSubgroupProperties {
                sType:
                ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES,
                pNext: ptr::null_mut(),
                subgroupSize: 0,
                supportedStages: 0,
                supportedOperations: 0,
                quadOperationsInAllStages: 0,
            };
        subgroup_properties.pNext = detail.physical_device_properties.pNext;
        detail.physical_device_properties.pNext = mem::transmute(&mut subgroup_properties);

        ffi::vk::vkGetPhysicalDeviceMemoryProperties(
            *device,
            &mut detail.physical_memory_properties,
        );
        ffi::vk::vkGetPhysicalDeviceFeatures2(*device, &mut detail.physical_device_features);
        ffi::vk::vkGetPhysicalDeviceProperties2KHR(
            *device,
            &mut detail.physical_device_properties,
        );


        let device_properties = &detail.physical_device_properties.properties;

        Ok(detail)
    }
}
