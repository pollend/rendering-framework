#![feature(arc_new_cyclic)]
#![feature(drain_filter)]

pub extern crate vulkan_sys;
pub mod configuration;
pub mod error;
pub mod renderer;
pub mod vulkan;
mod device;

pub mod ffi {
    pub use vulkan_sys as vk;
}
