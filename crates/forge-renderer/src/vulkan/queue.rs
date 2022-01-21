use crate::{ffi, vulkan::VulkanQueue, Queue, QueuePresentDesc, QueueSubmitDesc, VulkanAPI};
use std::mem::MaybeUninit;
use std::ptr;
use crate::vulkan::VulkanSemaphore;

impl Queue<VulkanAPI> for VulkanQueue {
    unsafe fn submit(&self, desc: &mut QueueSubmitDesc<VulkanAPI>) {
        let mut cmds_submit: Vec<ffi::vk::VkCommandBuffer> = (&desc.cmds).into_iter().map(|it| {
            it.cmd_buf
        }).collect();

        // (&desc.wait_semaphores).into_iter().filter(|it|{
        //     it.signaled
        // }).map(|it| {
        //     it.semaphore
        // });
        //
        let mut wait_semaphore: Vec<ffi::vk::VkSemaphore> = Vec::with_capacity(desc.wait_semaphores.len());
        let mut wait_mask: Vec<ffi::vk::VkPipelineStageFlags> = Vec::with_capacity(desc.wait_semaphores.len());
        for sem in &mut desc.wait_semaphores {
            if sem.signaled {
                wait_semaphore.push(sem.semaphore);
                wait_mask.push(ffi::vk::VkPipelineStageFlagBits_VK_PIPELINE_STAGE_ALL_COMMANDS_BIT);
                sem.signaled = false;
            }
        }

        let mut signaled_semaphore: Vec<ffi::vk::VkSemaphore> = Vec::with_capacity(desc.signal_semaphores.len());
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
            waitSemaphoreCount: 0,
            pWaitSemaphores: ptr::null_mut(),
            pWaitDstStageMask: ptr::null_mut(),
            commandBufferCount: cmds_submit.len() as u32,
            pCommandBuffers: cmds_submit.as_ptr(),
            signalSemaphoreCount: 0,
            pSignalSemaphores: ptr::null_mut()
        };
        self.submission_mutex.lock();


        // let res: *mut *ffi::vk::VkCommandBuffer = cmds_submit.as_mut_ptr();
        todo!()
    }

    unsafe fn present(&self, desc: &mut QueuePresentDesc<VulkanAPI>) {

        todo!()
    }

    unsafe fn wait_idle(&self) {
        todo!()
    }

    unsafe fn fence_status(&self) {
        todo!()
    }

    unsafe fn wait_fence(&self) {
        todo!()
    }

    unsafe fn toggle_v_sync(&self) {
        todo!()
    }
}
