use crate::{
    error::RendererError::VulkanError,
    vulkan::{VulkanBuffer, VulkanCommand, VulkanRenderTarget, VulkanRenderer},
    Command, RenderTarget, RendererResult, VulkanAPI,
};
use ash::vk::CommandBuffer;
use std::{ptr, sync::Arc};

impl<'a> Command<VulkanAPI> for VulkanCommand<'a> {
    unsafe fn begin_cmd(&mut self) -> RendererResult<()> {
        match Arc::get_mut(&mut self.renderer) {
            Some(renderer) => {
                assert_ne!(self.cmd_buf, ash::vk::CommandBuffer::null());
                let mut begin_info = ash::vk::CommandBufferBeginInfo::builder()
                    .flags(ash::vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
                renderer
                    .device
                    .begin_command_buffer(self.cmd_buf, &begin_info)
                    .unwrap();
            }
            None => {
                assert!(false, "failed to correctly dispose of fence");
            }
        };

        // let mut begin_info = ffi::vk::VkCommandBufferBeginInfo {
        //     sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
        //     pNext: ptr::null_mut(),
        //     flags:
        //         ffi::vk::VkCommandBufferUsageFlagBits_VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
        //     pInheritanceInfo: ptr::null_mut(),
        // };

        // let result = ffi::vk::vkBeginCommandBuffer(self.cmd_buf, &mut begin_info);
        // if result != ffi::vk::VkResult_VK_SUCCESS {
        //     return Err(VulkanError(result));
        // }
        //
        // self.bound_pipeline_layout = ptr::null_mut();
        Ok(())
    }

    unsafe fn end_cmd(&mut self) -> RendererResult<()> {
        match Arc::get_mut(&mut self.renderer) {
            Some(renderer) => {
                assert_ne!(self.cmd_buf, ash::vk::CommandBuffer::null());
                if self.active_render_pass != ash::vk::RenderPass::null() {
                    renderer.device.cmd_end_render_pass(self.cmd_buf);
                }
                self.active_render_pass = ash::vk::RenderPass::null();
                renderer.device.end_command_buffer(self.cmd_buf).unwrap();
            }
            None => {
                assert!(false, "failed to correctly dispose of fence");
            }
        }
        Ok(())
    }

    unsafe fn cmd_bind_render_target(
        &mut self,
        targets: &[&VulkanRenderTarget],
        depth_stencil: Option<&VulkanRenderTarget>,
    ) {
        match Arc::get_mut(&mut self.renderer) {
            Some(renderer) => {
                assert_ne!(self.cmd_buf, ash::vk::CommandBuffer::null());
                if self.cmd_buf != ash::vk::CommandBuffer::null() {
                    renderer.device.cmd_end_render_pass(self.cmd_buf);
                    self.active_render_pass = ash::vk::RenderPass::null();
                }

                // if self.cmd_buf != ptr::null_mut() {
                //     ffi::vk::vkCmdEndRenderPass(self.cmd_buf);
                // }
            }
            None => {
                assert!(false, "failed to correctly dispose of fence");
            }
        }

        todo!()
    }

    unsafe fn cmd_set_shading_rate(&self) {
        todo!()
    }

    unsafe fn cmd_set_viewport(&self) {
        todo!()
    }

    unsafe fn cmd_set_scissor(&self) {
        todo!()
    }

    unsafe fn cmd_set_stencil_reference_value(&self) {
        todo!()
    }

    unsafe fn cmd_bind_pipeline(&self) {
        todo!()
    }

    unsafe fn cmd_bind_descriptor_set(&self) {
        todo!()
    }

    unsafe fn cmd_bind_index_buffer(&self) {
        todo!()
    }

    unsafe fn cmd_raw(&self) {
        todo!()
    }

    unsafe fn cmd_draw_instanced(
        &mut self,
        vertex_count: u32,
        first_vertex: u32,
        instance_count: u32,
        first_instance: u32,
    ) {
        match Arc::get_mut(&mut self.renderer) {
            Some(renderer) => {
                assert_ne!(self.cmd_buf, ash::vk::CommandBuffer::null());
                renderer.device.cmd_draw(
                    self.cmd_buf,
                    vertex_count,
                    first_vertex,
                    instance_count,
                    first_instance,
                );
            }
            None => {
                assert!(false, "failed to correctly dispose of fence");
            }
        }
        //
        // ffi::vk::vkCmdDraw(
        //     self.cmd_buf,
        //     vertex_count,
        //     first_vertex,
        //     instance_count,
        //     first_instance,
        // );
    }

    unsafe fn cmd_draw_indexed(&mut self, index_count: u32, first_index: u32, first_vertex: i32) {
        assert_ne!(self.cmd_buf, ash::vk::CommandBuffer::null());
        match Arc::get_mut(&mut self.renderer) {
            Some(renderer) => {
                renderer.device.cmd_draw_indexed(
                    self.cmd_buf,
                    index_count,
                    1,
                    first_index,
                    first_vertex,
                    0,
                );
            }
            None => {
                assert!(false, "failed to correctly dispose of fence");
            }
        }
    }

    unsafe fn cmd_draw_indexed_instanced(
        &mut self,
        index_count: u32,
        first_index: u32,
        instance_count: u32,
        first_instance: u32,
        first_vertex: i32,
    ) {
        match Arc::get_mut(&mut self.renderer) {
            Some(renderer) => {
                assert_ne!(self.cmd_buf, ash::vk::CommandBuffer::null());
                renderer.device.cmd_draw_indexed(
                    self.cmd_buf,
                    index_count,
                    instance_count,
                    first_index,
                    first_vertex,
                    first_instance,
                );
            }
            None => {
                assert!(false, "failed to correctly dispose of fence");
            }
        }
    }

    unsafe fn cmd_dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        assert_ne!(self.cmd_buf, ash::vk::CommandBuffer::null());
        match Arc::get_mut(&mut self.renderer) {
            Some(renderer) => {
                renderer.device.cmd_dispatch(
                    self.cmd_buf,
                    group_count_x,
                    group_count_y,
                    group_count_z,
                );
            }
            None => {
                assert!(false, "failed to correctly dispose of fence");
            }
        }
    }

    unsafe fn cmd_resource_barrier(&self) {
        todo!()
    }

    unsafe fn cmd_update_virtual_texture(&self) {
        todo!()
    }

    unsafe fn update_buffer(
        &mut self,
        src_buffer: &VulkanBuffer,
        src_offset: u64,
        dest_buffer: &VulkanBuffer,
        dst_offset: u64,
        size: u64,
    ) {
        match Arc::get_mut(&mut self.renderer) {
            Some(renderer) => {
                // assert!(self.vk_buffer != ptr::null_mut());
                assert!(src_offset + size <= src_buffer.size);
                assert!(dst_offset + size <= dest_buffer.size);
                let mut region = ash::vk::BufferCopy::builder()
                    .src_offset(src_offset)
                    .dst_offset(dst_offset)
                    .size(size);
                renderer.device.cmd_copy_buffer(
                    self.cmd_buf,
                    src_buffer.vk_buffer,
                    dest_buffer.vk_buffer,
                    &[*region],
                );
            }
            None => {
                assert!(false, "failed to correctly dispose of fence");
            }
        }

        //
        // let mut region = ffi::vk::VkBufferCopy {
        //     srcOffset: src_offset,
        //     dstOffset: dst_offset,
        //     size: size
        // };
        // ffi::vk::vkCmdCopyBuffer(self.cmd_buf, src_buffer.vk_buffer, dest_buffer.vk_buffer, 1, &mut region);
    }
}
