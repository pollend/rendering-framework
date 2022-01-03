use std::path::PathBuf;
use std::env;


fn main() {

    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let vulkan_dir = PathBuf::new().join("Vulkan-Headers");
    let vulkan_include_dir = vulkan_dir.join("include/vulkan");
    let mut binding = bindgen::Builder::default()
        .size_t_is_usize(true);

    if cfg!(feature = "vulkan-xlib") {
        binding = binding.clang_arg("-DVK_USE_PLATFORM_XLIB_KHR=1");
    }
    if cfg!(feature = "vulkan-xlib_xrandr") {
        binding = binding.clang_arg("-DVK_USE_PLATFORM_XLIB_XRANDR_EXT=1");
    }
    if cfg!(feature = "vulkan-xcb") {
        binding = binding.clang_arg("-DVK_USE_PLATFORM_XCB_KHR=1");
    }
    if cfg!(feature = "vulkan-wayland") {
        binding = binding.clang_arg("-DVK_USE_PLATFORM_WAYLAND_KHR=1");
    }
    if cfg!(feature = "vulkan-directfb") {
        binding = binding.clang_arg("-DVK_USE_PLATFORM_DIRECTFB_EXT=1");
    }
    if cfg!(feature = "vulkan-android") {
        binding = binding.clang_arg("-DVK_USE_PLATFORM_ANDROID_KHR=1");
    }
    if cfg!(feature = "vulkan-win32") {
        binding = binding.clang_arg("-DVK_USE_PLATFORM_WIN32_KHR=1");
    }
    if cfg!(feature = "vulkan-vi") {
        binding = binding.clang_arg("-DVK_USE_PLATFORM_VI_NN=1");
    }
    if cfg!(feature = "vulkan-ios") {
        binding = binding.clang_arg("-DVK_USE_PLATFORM_IOS_MVK=1");
    }
    if cfg!(feature = "vulkan-macos") {
        binding = binding.clang_arg("-DVK_USE_PLATFORM_MACOS_MVK=1");
    }
    if cfg!(feature = "vulkan-metal") {
        binding = binding.clang_arg("-DVK_USE_PLATFORM_METAL_EXT=1");
    }
    if cfg!(feature = "vulkan-fuchsia") {
        binding = binding.clang_arg("-DVK_USE_PLATFORM_FUCHSIA=1");
    }
    if cfg!(feature = "vulkan-ggp") {
        binding = binding.clang_arg("-DVK_USE_PLATFORM_GGP=1");
    }
    if cfg!(feature = "vulkan-beta") {
        binding = binding.clang_arg("-DVK_ENABLE_BETA_EXTENSIONS=1");
    }
    if cfg!(feature = "vulkan-screen-qnx") {
        binding = binding.clang_arg("-DVK_USE_PLATFORM_SCREEN_QNX=1");
    }

    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
    let target_pointer_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap();
    let suffix = match (&*target_family, &*target_pointer_width) {
        ("windows", "32") => "Lib32",
        ("windows", "64") => "Lib",
        _ => "lib",
    };
    let lib = match &*target_family {
        "windows" => "vulkan-1",
        _ => "vulkan",
    };
    println!("cargo:rustc-link-lib={}", lib);

    let bindings = binding
        .clang_arg(format!("-I{}", vulkan_include_dir.to_string_lossy()))
        .header(PathBuf::new().join("wrapper.h").to_string_lossy())
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(dst.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
