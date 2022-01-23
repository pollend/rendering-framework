use crate::{
    error::RendererError::VulkanError,
    ffi,
    vulkan::{VulkanFence, VulkanQueue, VulkanSemaphore},
    FenceStatus, Queue, QueuePresentDesc, QueueSubmitDesc, RendererResult, VulkanAPI,
};
use std::{mem::MaybeUninit, ptr, sync::Mutex};

impl Queue<VulkanAPI> for VulkanQueue {
    unsafe fn submit(&self, desc: &mut QueueSubmitDesc<VulkanAPI>) -> RendererResult<()> {
        let mut cmds_submit: Vec<ffi::vk::VkCommandBuffer> =
            (&desc.cmds).into_iter().map(|it| it.cmd_buf).collect();

        let mut wait_semaphore: Vec<ffi::vk::VkSemaphore> =
            Vec::with_capacity(desc.wait_semaphores.len());
        let mut wait_mask: Vec<ffi::vk::VkPipelineStageFlags> =
            Vec::with_capacity(desc.wait_semaphores.len());
        for sem in &mut desc.wait_semaphores {
            if sem.signaled {
                wait_semaphore.push(sem.semaphore);
                wait_mask.push(ffi::vk::VkPipelineStageFlagBits_VK_PIPELINE_STAGE_ALL_COMMANDS_BIT);
                sem.signaled = false;
            }
        }

        let mut signaled_semaphore: Vec<ffi::vk::VkSemaphore> =
            Vec::with_capacity(desc.signal_semaphores.len());
        for sem in &mut desc.signal_semaphores {
            if sem.signaled {
                signaled_semaphore.push(sem.semaphore);
                sem.current_node = self.queue_index;
                sem.signaled = true;
            }
        }

        let submit_info = ffi::vk::VkSubmitInfo {
            sType: ffi::vk::VkStructureType_VK_STRUCTURE_TYPE_SUBMIT_INFO,
            pNext: ptr::null_mut(),
            waitSemaphoreCount: wait_semaphore.len() as u32,
            pWaitSemaphores: wait_semaphore.as_ptr(),
            pWaitDstStageMask: wait_mask.as_ptr(),
            commandBufferCount: cmds_submit.len() as u32,
            pCommandBuffers: cmds_submit.as_ptr(),
            signalSemaphoreCount: signaled_semaphore.len() as u32,
            pSignalSemaphores: signaled_semaphore.as_ptr(),
        };
        let _guard = self.submission_mutex.lock().unwrap();
        let result = ffi::vk::vkQueueSubmit(
            self.queue,
            1,
            &submit_info,
            match &mut desc.signal_fences {
                None => ptr::null_mut(),
                Some(res) => {
                    res.submitted = true;
                    res.fence
                }
            },
        );
        if result != ffi::vk::VkResult_VK_SUCCESS {
            return Err(VulkanError(result));
        }
        Ok(())
    }

    unsafe fn present(
        &self,
        desc: &mut QueuePresentDesc<VulkanAPI>,
    ) -> RendererResult<FenceStatus> {


        let mut status = FenceStatus::Complete;
        todo!()
    }

    unsafe fn wait_idle(&self) {
        ffi::vk::vkQueueWaitIdle(self.queue);
    }

    unsafe fn wait_fence(&self) {
        todo!()
    }

    unsafe fn toggle_v_sync(&self) {
        todo!()
    }
}
