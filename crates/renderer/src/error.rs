use crate::ffi;

pub enum RendererError {
    Unhandled,

    VulkanError(ffi::vk::VkResult)
}

pub type RendererResult<T> = Result<T, RendererError>;


#[macro_export]
macro_rules! check_vk_result {
    ($x:expr) => {{
        let result = $x;
        if result != ffi::vk::VkResult_VK_SUCCESS {
            return Err(crate::error::RendererError::VulkanError(result));
        }
    }};
}
