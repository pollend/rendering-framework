use crate::{
    configuration::*,
    error::{HalError, HalResult},
    ffi,
    renderer::*,
    vulkan::*,
};
use std::{
    mem::ManuallyDrop,
    ptr,
    rc::{Rc, Weak},
};
use std::borrow::Borrow;

