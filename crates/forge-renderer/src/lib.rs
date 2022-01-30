#![feature(associated_type_defaults)]
#![feature(arc_new_cyclic)]
#![feature(generic_associated_types)]

use crate::{
    desc::{
        BufferDesc, CmdDesc, CmdPoolDesc, QueueDesc, QueuePresentDesc, QueueSubmitDesc, RenderDesc,
        RenderTargetDesc, RootSignatureDesc, SamplerDesc, SwapChainDesc, TextureDesc,
    },
    error::RendererResult,
    types::{FenceStatus, GPUPresetLevel, GPUSupportedFeatures, ShadingRates},
    vulkan::VulkanAPI,
};
use std::{
    ffi::{CStr, CString},
    sync::Arc,
};
use crate::desc::BinaryShaderDesc;

mod desc;
mod error;
mod types;
mod vulkan;
mod shader_reflection;

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
    type RootSignature: RootSignature;
    type Pipeline: Pipeline;
    type Fence: Fence<Self>;
    type Semaphore: Semaphore;
    type Queue: Queue<Self>;
    type Texture: Texture;
    type Shader: Shader;
    type RenderTarget: RenderTarget;
    type DescriptorIndexMap: DescriptorIndexMap;
    type Sampler: Sampler;
    type Command<'a>: Command<Self>;
    type CommandPool<'a>: CommandPool;
    type Buffer: Buffer<Self>;
    type SwapChain: SwapChain;

    const CURRENT_API: APIType;
}

pub struct BufferBarrier<'a, A: Api> {
    buffer: &'a A::Buffer,
}

pub trait CommandPool: Sized {
    fn reset(&mut self) -> RendererResult<()>;
}

pub trait SwapChain: Sized {}

pub trait RootSignature: Sized {}

pub trait Buffer<A: Api>: Sized {
    unsafe fn map_buffer(&mut self, offset: u32, size: u32);
    unsafe fn write<T>(&mut self, offset: u32, payload: &T);
    unsafe fn unmap_buffer(&mut self);
}

pub trait Renderer<A: Api>: Sized {
    unsafe fn init(name: &CStr, desc: &RenderDesc) -> RendererResult<Arc<A::Renderer>>;

    // internal utilities
    unsafe fn add_pipeline(&self) -> A::Pipeline;
    unsafe fn drop_pipeline(&self, pipeline: &mut A::Pipeline);

    unsafe fn add_fence(&self) -> RendererResult<A::Fence>;
    unsafe fn add_semaphore(&self) -> RendererResult<A::Semaphore>;
    unsafe fn add_cmd_pool<'a>(
        &self,
        desc: &CmdPoolDesc<'a, A>,
    ) -> RendererResult<A::CommandPool<'a>>;
    unsafe fn add_cmd<'a>(&self, desc: &mut CmdDesc<'a, A>) -> RendererResult<A::Command<'a>>;
    unsafe fn add_queue(&mut self, desc: &QueueDesc) -> RendererResult<A::Queue>;
    unsafe fn add_swap_chain<'a>(
        &self,
        desc: &'a SwapChainDesc<'a, A>,
        window_handle: &impl raw_window_handle::HasRawWindowHandle,
    ) -> RendererResult<A::SwapChain>;
    unsafe fn add_sampler(&self, desc: &SamplerDesc) -> RendererResult<A::Sampler>;
    unsafe fn add_render_target(&self, desc: &RenderTargetDesc) -> RendererResult<A::RenderTarget>;
    unsafe fn add_texture(&self, desc: &TextureDesc) -> RendererResult<A::Texture>;
    unsafe fn add_root_signature(
        &self,
        signature: &RootSignatureDesc<A>,
    ) -> RendererResult<A::RootSignature>;

    unsafe fn add_shader_binary(&self, desc: &BinaryShaderDesc) -> RendererResult<A::Shader>;

    unsafe fn drop_swap_chain(&self);
    unsafe fn remove_render_target(&self, target: &mut A::RenderTarget);

    unsafe fn remove_root_signature(&self, signature: &mut A::RootSignature);

    // command buffer functions
    unsafe fn get_common_info(&self) -> &GPUCommonInfo;

    // resource functions
    unsafe fn add_buffer(&self, desc: &BufferDesc) -> RendererResult<A::Buffer>;
}

pub trait Command<A: Api>: Sized {
    // commands
    unsafe fn begin_cmd(&mut self) -> RendererResult<()>;
    unsafe fn end_cmd(&mut self) -> RendererResult<()>;
    unsafe fn cmd_bind_render_target(
        &mut self,
        targets: &[&A::RenderTarget],
        depth_stencil: Option<&A::RenderTarget>,
    );
    unsafe fn cmd_set_shading_rate(&self);
    unsafe fn cmd_set_viewport(&self);
    unsafe fn cmd_set_scissor(&self);
    unsafe fn cmd_set_stencil_reference_value(&self);
    unsafe fn cmd_bind_pipeline(&self);
    unsafe fn cmd_bind_descriptor_set(&self);
    unsafe fn cmd_bind_index_buffer(&self);
    unsafe fn cmd_raw(&self);
    unsafe fn cmd_draw_instanced(
        &self,
        vertex_count: u32,
        first_vertex: u32,
        instance_count: u32,
        first_instance: u32,
    );
    unsafe fn cmd_draw_indexed(&self, index_count: u32, first_index: u32, first_vertex: i32);
    unsafe fn cmd_draw_indexed_instanced(
        &self,
        index_count: u32,
        first_index: u32,
        instance_count: u32,
        first_instance: u32,
        first_vertex: i32,
    );
    unsafe fn cmd_dispatch(&self, group_count_x: u32, group_count_y: u32, group_count_z: u32);

    // transition commands
    unsafe fn cmd_resource_barrier(&self);

    // virtual textures
    unsafe fn cmd_update_virtual_texture(&self);

    unsafe fn update_buffer(&mut self, src_buffer: &A::Buffer, src_offset: u64, dest_buffer: &A::Buffer, dst_offset: u64, size: u64);

}

pub trait RenderContext {
    // fn info(&self) -> &GPUCommonInfo;
}

pub trait Texture {}

pub trait Shader {}

pub trait Queue<A: Api> {
    unsafe fn submit(&self, desc: &mut QueueSubmitDesc<A>) -> RendererResult<()>;
    unsafe fn present(&self, desc: &mut QueuePresentDesc<A>) -> RendererResult<FenceStatus>;
    unsafe fn wait_idle(&self);
    unsafe fn wait_fence(&self);
    unsafe fn toggle_v_sync(&self);
}

pub trait Sampler {}

pub trait DescriptorIndexMap {}

pub trait RenderTarget {}

pub trait Semaphore {}

pub trait Fence<T: Api> {
    unsafe fn status(&mut self, rend: &T::Renderer) -> FenceStatus;
}

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
