#![feature(arc_new_cyclic)]
#![feature(drain_filter)]
#![feature(crate_visibility_modifier)]
#![feature(fn_traits)]
#![feature(associated_type_defaults)]

pub extern crate vulkan_sys;
mod api;
pub mod configuration;
mod device;
pub mod error;
pub mod renderer;
mod stub;
pub mod vulkan;

pub mod ffi {
    pub use vulkan_sys as vk;
}
