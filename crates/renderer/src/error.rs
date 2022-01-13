pub enum RendererError {
    Unhandled,

    VulkanError(i32)
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
