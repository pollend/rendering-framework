use crate::ffi;
use crate::types::QueueType;

impl QueueType {
    pub fn to_vk_queue(&self) -> ffi::vk::VkQueueFlagBits {
        match self {
            QueueType::QueueTypeGraphics => ffi::vk::VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT,
            QueueType::QueueTypeTransfer => ffi::vk::VkQueueFlagBits_VK_QUEUE_TRANSFER_BIT,
            QueueType::QueueTypeCompute => ffi::vk::VkQueueFlagBits_VK_QUEUE_COMPUTE_BIT,
            _ => {
                assert!(false, "invalid Queue Type");
                ffi::vk::VkQueueFlagBits_VK_QUEUE_FLAG_BITS_MAX_ENUM
            }
        }
    }
}
