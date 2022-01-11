pub enum RendererError {
    Unhandled,

    VulkanError(u32)
}

pub type RendererResult<T> = Result<T, RendererError>;


