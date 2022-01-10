use crate::ffi;

pub struct VulkanShader {
    shader_module: ffi::vk::VkShaderModule,
    name: Box<str>,
    specialization_info: ffi::vk::VkSpecializationInfo,
}
