use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::{mem, ptr};
use crate::ffi;
use crate::vulkan::*;
use crate::device::GPUDetail;
use crate::error::HalError::Unhandled;
use crate::error::{HalError, HalResult};

pub struct VulkanGPUDetail {
    physical_device_features: ffi::vk::VkPhysicalDeviceFeatures2,
    physical_memory_properties: ffi::vk::VkPhysicalDeviceMemoryProperties,
    physical_device_properties: ffi::vk::VkPhysicalDeviceProperties2,
    pub common: Box<GPUDetail>,
}

impl VulkanGPUDetail {
    pub fn get_physical_vk_features(&self) -> &ffi::vk::VkPhysicalDeviceFeatures {
        return &self.physical_device_features.features;
    }

    pub fn all(renderer: &VulkanRenderer) -> Vec<VulkanGPUDetail> {

        let mut device_count: u32 = 0;
        let mut vk_result = unsafe { ffi::vk::vkEnumeratePhysicalDevices(renderer.instance, &mut device_count, ptr::null_mut()) };
        assert!(vk_result == ffi::vk::VkResult_VK_SUCCESS);
        let mut details: Vec<VulkanGPUDetail> = Vec::with_capacity(device_count as usize);
        let mut physical_devices : Vec<ffi::vk::VkPhysicalDevice> = Vec::with_capacity(device_count as usize);
        vk_result = unsafe { ffi::vk::vkEnumeratePhysicalDevices(renderer.instance, &mut device_count, physical_devices.as_mut_ptr()) };

        assert!(vk_result == ffi::vk::VkResult_VK_SUCCESS);
        for device in &physical_devices {
            match VulkanGPUDetail::gpu(device) {
                Ok(detail) => details.push(detail),
                _ => {}
            }
        }
        details
    }

    pub fn gpu(device: &ffi::vk::VkPhysicalDevice) -> HalResult<VulkanGPUDetail> {
        let mut detail: VulkanGPUDetail = unsafe { MaybeUninit::zeroed().assume_init() };

        unsafe { ffi::vk::vkGetPhysicalDeviceMemoryProperties(*device, &mut detail.physical_memory_properties); }

         // Get features
        // detail.physical_device_properties.sType = ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR;

        detail.physical_device_features.sType = ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR;

        unsafe { ffi::vk::vkGetPhysicalDeviceFeatures2(*device, &mut detail.physical_device_features); }

        let mut subgroupProperties: ffi::vk::VkPhysicalDeviceSubgroupProperties = ffi::vk::VkPhysicalDeviceSubgroupProperties {
            sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES,
            pNext: ptr::null_mut(),
            subgroupSize: 0,
            supportedStages: 0,
            supportedOperations: 0,
            quadOperationsInAllStages: 0
        };
        detail.physical_device_properties.sType = ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR;
        subgroupProperties.pNext = detail.physical_device_properties.pNext;
        detail.physical_device_properties.pNext = unsafe { mem::transmute(&mut subgroupProperties) };

        unsafe { ffi::vk::vkGetPhysicalDeviceProperties2KHR(*device, &mut detail.physical_device_properties); }

        Ok(detail)
    }
}
