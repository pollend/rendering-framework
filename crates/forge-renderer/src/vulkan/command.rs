use crate::{
    error::RendererError::VulkanError,
    ffi,
    vulkan::{VulkanBuffer, VulkanCommand, VulkanRenderTarget},
    Command, RenderTarget, RendererResult, VulkanAPI,
};
use std::ptr;

impl<'a> Command<VulkanAPI> for VulkanCommand<'a> {
    unsafe fn begin_cmd(&mut self) -> RendererResult<()> {
        assert!(self.cmd_buf != ptr::null_mut());

        let mut begin_info = ffi::vk::VkCommandBufferBeginInfo {
            sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
            pNext: ptr::null_mut(),
            flags:
                ffi::vk::VkCommandBufferUsageFlagBits_VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
            pInheritanceInfo: ptr::null_mut(),
        };

        let result = ffi::vk::vkBeginCommandBuffer(self.cmd_buf, &mut begin_info);
        if result != ffi::vk::VkResult_VK_SUCCESS {
            return Err(VulkanError(result));
        }

        self.bound_pipeline_layout = ptr::null_mut();
        Ok(())
    }

    unsafe fn end_cmd(&mut self) -> RendererResult<()> {
        assert!(self.cmd_buf != ptr::null_mut());
        if self.active_render_pass != ptr::null_mut() {
            ffi::vk::vkCmdEndRenderPass(self.cmd_buf);
        }
        self.active_render_pass = ptr::null_mut();
        let result = ffi::vk::vkEndCommandBuffer(self.cmd_buf);
        if result != ffi::vk::VkResult_VK_SUCCESS {
            return Err(VulkanError(result));
        }
        Ok(())
    }

    unsafe fn cmd_bind_render_target(
        &mut self,
        targets: &[&VulkanRenderTarget],
        depth_stencil: Option<&VulkanRenderTarget>,
    ) {
        assert!(self.cmd_buf != ptr::null_mut());

        if self.cmd_buf != ptr::null_mut() {
            ffi::vk::vkCmdEndRenderPass(self.cmd_buf);
            self.active_render_pass = ptr::null_mut();
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
        &self,
        vertex_count: u32,
        first_vertex: u32,
        instance_count: u32,
        first_instance: u32,
    ) {
        assert!(self.cmd_buf != ptr::null_mut());
        ffi::vk::vkCmdDraw(
            self.cmd_buf,
            vertex_count,
            first_vertex,
            instance_count,
            first_instance,
        );
    }

    unsafe fn cmd_draw_indexed(&self, index_count: u32, first_index: u32, first_vertex: i32) {
        assert!(self.cmd_buf != ptr::null_mut());
        ffi::vk::vkCmdDrawIndexed(self.cmd_buf, index_count, 1, first_index, first_vertex, 0);
    }

    unsafe fn cmd_draw_indexed_instanced(
        &self,
        index_count: u32,
        first_index: u32,
        instance_count: u32,
        first_instance: u32,
        first_vertex: i32,
    ) {
        assert!(self.cmd_buf != ptr::null_mut());
        ffi::vk::vkCmdDrawIndexed(
            self.cmd_buf,
            index_count,
            instance_count,
            first_index,
            first_vertex,
            first_instance,
        );
    }

    unsafe fn cmd_dispatch(&self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        assert!(self.cmd_buf != ptr::null_mut());
        ffi::vk::vkCmdDispatch(self.cmd_buf, group_count_x, group_count_y, group_count_z);
    }

    unsafe fn cmd_update_buffer(
        &mut self,
        buffer: &VulkanBuffer,
        dst_offset: u64,
        src_buffer: &VulkanBuffer,
        size: u64,
    ) {
        todo!()
    }

    unsafe fn cmd_resource_barrier(&self) {
        todo!()
    }

    unsafe fn cmd_update_virtual_texture(&self) {
        todo!()
    }
}
