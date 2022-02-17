use std::borrow::BorrowMut;
use std::ffi::c_void;
use std::ptr;
use std::sync::Arc;
use gpu_allocator::vulkan::{Allocation, Allocator};
use log::error;
use crate::{Buffer, check_vk_result, ffi, VulkanAPI};
use crate::types::{DescriptorType, ResourceMemoryUsage};
use crate::vulkan::VulkanRenderer;

pub struct VulkanBuffer {
    pub(in crate::vulkan) renderer: Arc<VulkanRenderer>,
    pub(in crate::vulkan) vk_buffer: ash::vk::Buffer,
    /// Buffer view
    pub(in crate::vulkan) vk_storage_texel_view: ash::vk::BufferView,
    pub(in crate::vulkan) vk_uniform_texel_view: ash::vk::BufferView,

    // pub(in crate::vulkan) vma_allocation: ffi::vk::VmaAllocation,
    // pub(in crate::vulkan) mapping_address: *mut c_void,

    pub(in crate::vulkan) allocation: Allocation,

    pub(in crate::vulkan) offset: u64,

    pub(in crate::vulkan) size: u64,
    pub(in crate::vulkan) descriptors: DescriptorType,
    pub(in crate::vulkan) memory_usage: ResourceMemoryUsage,
    pub(in crate::vulkan) node_index: u32,
}

impl Drop for VulkanBuffer {
    fn drop(&mut self) {
        match Arc::get_mut(&mut self.renderer) {
            None => {
                assert!(false, "failed to correctly dispose of command pool");
            }
            Some(renderer) => {

                unsafe {
                    if self.vk_storage_texel_view != ash::vk::BufferView::null() {
                        renderer.device.destroy_buffer_view(self.vk_storage_texel_view, None);
                        self.vk_storage_texel_view = ash::vk::BufferView::null();
                    }

                    if self.vk_uniform_texel_view != ptr::null_mut() {
                        renderer.device.destroy_buffer_view(self.vk_uniform_texel_view, None);
                        self.vk_storage_texel_view = ash::vk::BufferView::null();
                    }
                    // ffi::vk::vmaDestroyBuffer(
                    //     renderer.vma_allocator,
                    //     self.vk_buffer,
                    //     self.vma_allocation,
                    // );
                }
            }
        }
    }
}

impl Buffer<VulkanAPI> for VulkanBuffer {
    unsafe fn map_buffer(&mut self, offset: u32, size: u32) {
        assert!(self.memory_usage == ResourceMemoryUsage::GpuOnly, "Trying to map non-cpu accessible resource");
        match Arc::get_mut(&mut self.renderer) {
            None => {
                assert!(false, "failed to map buffer missing renderer backend");
                error!("failed to map buffer")
            }
            Some(renderer) => {
                check_vk_result!(ffi::vk::vmaMapMemory(renderer.vma_allocator, self.vma_allocation, &mut self.mapping_address));
                let ptr: *const u8 = self.mapping_address as _;
                self.mapping_address = ptr.offset(offset as isize) as _;
            }
        }

    }

    unsafe fn write<T>(&mut self, offset: u32, payload: &T) {
        assert!(self.mapping_address != ptr::null_mut());

        let ptr: *const u8 = self.mapping_address as _;
        let ptr_start = ptr.offset(offset as isize);
        std::ptr::write(ptr_start as _, payload);
    }

    unsafe fn unmap_buffer(&mut self) {
        match Arc::get_mut(&mut self.renderer) {
            None => {
                assert!(false, "failed to unmap buffer missing renderer backend");
                error!("failed to map buffer")
            }
            Some(renderer) => {

                // ffi::vk::vmaUnmapMemory(renderer.vma_allocator, self.vma_allocation);
                self.mapping_address = ptr::null_mut();
            }
        }
    }

}
