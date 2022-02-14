use crate::{
    error::RendererError, ffi, GPUCommonInfo, GPUPresetLevel, GPUSupportedFeatures, GPUVendorInfo,
    RendererResult, ShadingRates,
};
use log::info;
use std::{
    ffi::{CStr, CString},
    mem,
    mem::MaybeUninit,
    ptr,
};
use std::ops::DerefMut;

pub(in crate::vulkan) struct VulkanGPUInfo {
    physical_device_features: ash::vk::PhysicalDeviceFeatures2,
    physical_memory_properties: ash::vk::PhysicalDeviceMemoryProperties,
    physical_sub_group_properties: ash::vk::PhysicalDeviceSubgroupProperties,
    physical_device_properties: ash::vk::PhysicalDeviceProperties2,
    queue_family_properties: Vec<ash::vk::QueueFamilyProperties>,
    pub device: ash::vk::PhysicalDevice,
}

impl VulkanGPUInfo {
    pub fn get_physical_features(&self) -> &ash::vk::PhysicalDeviceFeatures {
        return &self.physical_device_features.features;
    }

    pub fn get_device_properties(&self) -> &ash::vk::PhysicalDeviceProperties {
        return &self.physical_device_properties.properties;
    }

    pub fn get_memory_properties(&self) -> &ash::vk::PhysicalDeviceMemoryProperties {
        return &self.physical_memory_properties;
    }

    pub fn get_queue_family_properties(&self) -> &[ash::vk::QueueFamilyProperties] {
        return &self.queue_family_properties.as_slice();
    }

    pub fn get_device(&self) -> ffi::vk::VkPhysicalDevice {
        return self.device;
    }

    pub unsafe fn to_common(&self) -> GPUCommonInfo {
        let properties = &self.get_physical_features();
        let limits = &self.get_device_properties().limits;
        GPUCommonInfo {
            uniform_buffer_alignment: limits.minUniformBufferOffsetAlignment as u32,
            upload_buffer_texture_alignment: limits.optimalBufferCopyOffsetAlignment as u32,
            upload_buffer_texture_row_alignment: limits.optimalBufferCopyRowPitchAlignment as u32,
            max_vertex_input_bindings: limits.maxVertexInputBindings as u32,
            max_root_signature_dwords: 0,
            wave_lane_count: 0,
            features: {
                let mut features = GPUSupportedFeatures::NONE;
                if properties.tessellationShader > 0 {
                    features |= GPUSupportedFeatures::TESSELLATION_SUPPORTED;
                }
                if limits.maxDrawIndirectCount > 0 {
                    features |= GPUSupportedFeatures::MULTI_DRAW_INDIRECT;
                }
                if properties.geometryShader > 0 {
                    features |= GPUSupportedFeatures::GEOMETRY_SHADER_SUPPORTED;
                }
                features
            },
            shading_rates: ShadingRates::SHADING_RATE_NOT_SUPPORTED,
            vendor_info: GPUVendorInfo {
                vendor_id: CString::new(format!("{:#x}", self.get_device_properties().vendorID))
                    .unwrap(),
                model_id: CString::new(format!("{:#x}", self.get_device_properties().deviceID))
                    .unwrap(),
                revision_id: CString::new("0x00").unwrap(),
                preset_level: GPUPresetLevel::PresetNone,
                gpu_name: CString::from(CStr::from_ptr(
                    self.get_device_properties().deviceName.as_ptr(),
                )),
                gpu_driver_version: Default::default(),
                gpu_driver_date: Default::default(),
            },
        }
    }

    pub unsafe fn select_best_gpu(
        instance: &ash::Instance,
        vk_gpus: &Vec<VulkanGPUInfo>,
    ) -> RendererResult<&VulkanGPUInfo> {

        let is_device_better = |current: &VulkanGPUInfo, to_test: &VulkanGPUInfo| -> bool {
            let current_device_properties = current.get_device_properties();
            let test_device_properties = to_test.get_device_properties();

            // if the current gpu is discrete and the gpu to test against isn't take preference over discrete
            if test_device_properties.device_type == ash::vk::PhysicalDeviceType::DISCRETE_GPU
                && current_device_properties.device_type == ash::vk::PhysicalDeviceType::DISCRETE_GPU
            {
                return true;
            }

            if test_device_properties.device_type != ash::vk::PhysicalDeviceType::DISCRETE_GPU
                && current_device_properties.device_type == ash::vk::PhysicalDeviceType::DISCRETE_GPU
            {
                return false;
            }

            if current_device_properties.vendor_id == test_device_properties.vendor_id
                && current_device_properties.vendor_id == test_device_properties.vendor_id
            {
                let current_memory_properties = current.get_memory_properties();
                let test_memory_properties = to_test.get_memory_properties();

                let mut total_test_vram: ffi::vk::VkDeviceSize = 0;
                let mut total_current_vram: ffi::vk::VkDeviceSize = 0;
                for i in 0..current_memory_properties.memory_heap_count as usize {
                    let heap = &current_memory_properties.memory_heaps[i];
                    if heap.flags & ffi::vk::VkMemoryHeapFlagBits_VK_MEMORY_HEAP_DEVICE_LOCAL_BIT
                        > 0
                    {
                        total_current_vram += heap.size
                    }
                }

                for i in 0..test_memory_properties.memory_heap_count as usize {
                    let heap = &test_memory_properties.memory_heaps[i];
                    if heap.flags & ffi::vk::VkMemoryHeapFlagBits_VK_MEMORY_HEAP_DEVICE_LOCAL_BIT
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

    pub unsafe fn all(instance: &ash::Instance) -> Vec<VulkanGPUInfo> {
        let mut physical_devices = instance.enumerate_physical_devices().unwrap();
        assert!(vk_result == ffi::vk::VkResult_VK_SUCCESS);
        for device in physical_devices {
            match VulkanGPUInfo::gpu(&instance, device) {
                Ok(detail) => details.push(detail),
                _ => {}
            }
        }
        details
    }

    pub unsafe fn gpu(instance: &ash::Instance, device: ash::vk::PhysicalDevice) -> RendererResult<VulkanGPUInfo> {
        let mut physical_sub_group_properties = ash::vk::PhysicalDeviceSubgroupProperties::builder();
        let mut physical_device_properties = ash::vk::PhysicalDeviceProperties2::builder()
            .push_next(&mut physical_sub_group_properties);
        let mut physical_device_features =
            ash::vk::PhysicalDeviceFeatures2::builder();
        let mut physical_memory_properties = instance.get_physical_device_memory_properties(device);
        instance.get_physical_device_features2(device, &mut physical_device_features);
        instance.get_physical_device_properties2(device, &mut physical_device_properties);
        let mut queue_family_properties = instance.get_physical_device_queue_family_properties(device);

        let mut detail: VulkanGPUInfo = VulkanGPUInfo {
            physical_sub_group_properties: physical_sub_group_properties.build(),
            physical_device_properties: physical_device_properties.build(),
            physical_device_features: physical_device_features.build(),
            physical_memory_properties,
            queue_family_properties,
            device,
        };

        Ok(detail)
    }
}
