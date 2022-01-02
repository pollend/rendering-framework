#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn vkMakeVersion(major: u32, minor: u32, patch: u32) -> u32{
    return (major << 22) | (minor << 12) | (patch);
}
