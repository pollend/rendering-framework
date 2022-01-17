#![feature(associated_type_defaults)]

use crate::{
    desc::{CmdPoolDesc, QueueDesc, RenderDesc},
    error::RendererResult,
    types::{GPUPresetLevel, GPUSupportedFeatures, ShadingRates},
    vulkan::VulkanAPI,
};
use std::ffi::{CStr, CString};
use crate::desc::BufferDesc;

mod desc;
mod error;
mod types;
mod vulkan;

pub mod ffi {
    pub use vulkan_sys as vk;
}

pub struct GPUVendorInfo {
    vendor_id: CString,
    model_id: CString,
    revision_id: CString,
    preset_level: GPUPresetLevel,
    gpu_name: CString,
    gpu_driver_version: CString,
    gpu_driver_date: CString,
}

pub struct GPUCommonInfo {
    uniform_buffer_alignment: u32,
    upload_buffer_texture_alignment: u32,
    upload_buffer_texture_row_alignment: u32,
    max_vertex_input_bindings: u32,
    max_root_signature_dwords: u32,
    wave_lane_count: u32,
    features: GPUSupportedFeatures,
    shading_rates: ShadingRates,
    vendor_info: GPUVendorInfo,
}

pub enum APIType {
    None,
    Vulkan,
}

pub trait Api: Clone + Sized {
    type RenderContext: RenderContext;
    type Renderer: Renderer<Self>;
    type Pipeline: Pipeline;
    type Fence: Fence;
    type Semaphore: Semaphore;
    type Queue: Queue<Self>;
    type Texture: Texture;
    type Shader: Shader;
    type RenderTarget: RenderTarget;
    type DescriptorIndexMap: DescriptorIndexMap;
    type Sampler: Sampler;
    type Command: Command;
    type Buffer: Buffer;

    const CURRENT_API: APIType;
}

pub trait Buffer: Sized  {}

pub trait Renderer<A: Api>: Sized {
    // internal utilities
    unsafe fn add_buffer(&self, desc: &BufferDesc) -> RendererResult<A::Buffer>;
    unsafe fn drop_buffer(&self, buffer: &mut A::Buffer );

    unsafe fn init(name: &CStr, desc: &RenderDesc) -> RendererResult<A::Renderer>;

    unsafe fn add_pipeline(&self) -> A::Pipeline;
    unsafe fn drop_pipeline(&self, pipeline: &mut A::Pipeline);

    unsafe fn add_fence(&self) -> RendererResult<A::Fence>;
    unsafe fn drop_fence(&self, fence: &mut A::Fence);

    // semaphore
    unsafe fn add_semaphore(&self) -> RendererResult<A::Semaphore>;
    unsafe fn drop_semaphore(&self, semaphore: &mut A::Semaphore) -> RendererResult<()>;

    unsafe fn add_queue(&self, desc: &QueueDesc) -> RendererResult<A::Queue>;
    unsafe fn remove_queue(&self, queue: &mut A::Queue);

    unsafe fn add_swap_chain(&self);
    unsafe fn drop_swap_chain(&self);

    // command pool functions
    unsafe fn add_cmd_pool(&self, desc: &CmdPoolDesc<A>);
    unsafe fn drop_cmd_pool(&self);
    unsafe fn add_cmd(&self);
    unsafe fn drop_cmd(&self);

    unsafe fn add_render_target(&self) -> RendererResult<A::RenderTarget>;
    unsafe fn remove_render_target(&self, target: &mut A::RenderTarget);

    unsafe fn add_root_signature(&self);
    unsafe fn remove_root_signature();

    // command buffer functions
    unsafe fn reset_cmd_pool(&self);

    unsafe fn get_common_info(&self) -> &GPUCommonInfo;
}

pub trait Command {
    // commands
    fn begin_cmd(&self);
    fn end_cmd(&self);
    fn cmd_bind_render_target(&self);
    fn cmd_set_shading_rate(&self);
    fn cmd_set_viewport(&self);
    fn cmd_set_scissor(&self);
    fn cmd_set_stencil_reference_value(&self);
    fn cmd_bind_pipeline(&self);
    fn cmd_bind_descriptor_set(&self);
    fn cmd_bind_index_buffer(&self);
    fn cmd_raw(&self);
    fn cmd_draw_instanced(&self);
    fn cmd_draw_indexed(&self);
    fn cmd_draw_indexed_instanced(&self);
    fn cmd_dispatch(&self);

    // transition commands
    fn cmd_resource_barrier(&self);

    // virtual textures
    fn cmd_update_virtual_texture(&self);
}

pub trait RenderContext {
    // fn info(&self) -> &GPUCommonInfo;
}

pub trait Texture {}

pub trait Shader {}

pub trait Queue<A: Api> {
    fn submit(&self);
    fn present(&self);
    fn wait_idle(&self);
    fn fence_status(&self);
    fn wait_fence(&self);
    fn toggle_v_sync(&self);
}

pub trait Sampler {}

pub trait DescriptorIndexMap {}

pub trait RenderTarget {}

pub trait Semaphore {}

pub trait Fence {}

pub trait Pipeline {}

// struct Example<A: Api> {
//     render: A::Renderer
// }

// impl<A: Api> Example<A> {
//     fn init() {
//         match A::api {
//             APIType::Vulkan => {

//             }
//             _ => {}
//         }
//     }
// }

// pub fn example() {
//     let a: Example<VulkanAPI>;
// }
