pub enum HalError {
    Unhandled,

    VkError(u32)
}

#[macro_export]
macro_rules! check_vk_result {
    ($x:expr) => {
        {
            let result = $x;
            if result != ffi::vk::VkResult_VK_SUCCESS {
                panic!("{} : Failed with VkResult: {}", stringify!($x),result)
            }
        }
    };
}

pub type HalResult<T> = Result<T, HalError>;
