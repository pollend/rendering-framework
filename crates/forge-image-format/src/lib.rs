

#[allow(non_camel_case_types)]
#[derive(PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum ImageFormat {
    UNDEFINED,
    R1_UNORM,
    R2_UNORM,
    R4_UNORM,
    R4G4_UNORM,
    G4R4_UNORM,
    A8_UNORM,
    R8_UNORM,
    R8_SNORM,
    R8_UINT,
    R8_SINT,
    R8_SRGB,
    B2G3R3_UNORM,
    R4G4B4A4_UNORM,
    R4G4B4X4_UNORM,
    B4G4R4A4_UNORM,
    B4G4R4X4_UNORM,
    A4R4G4B4_UNORM,
    X4R4G4B4_UNORM,
    A4B4G4R4_UNORM,
    X4B4G4R4_UNORM,
    R5G6B5_UNORM,
    B5G6R5_UNORM,
    R5G5B5A1_UNORM,
    B5G5R5A1_UNORM,
    A1B5G5R5_UNORM,
    A1R5G5B5_UNORM,
    R5G5B5X1_UNORM,
    B5G5R5X1_UNORM,
    X1R5G5B5_UNORM,
    X1B5G5R5_UNORM,
    B2G3R3A8_UNORM,
    R8G8_UNORM,
    R8G8_SNORM,
    G8R8_UNORM,
    G8R8_SNORM,
    R8G8_UINT,
    R8G8_SINT,
    R8G8_SRGB,
    R16_UNORM,
    R16_SNORM,
    R16_UINT,
    R16_SINT,
    R16_SFLOAT,
    R16_SBFLOAT,
    R8G8B8_UNORM,
    R8G8B8_SNORM,
    R8G8B8_UINT,
    R8G8B8_SINT,
    R8G8B8_SRGB,
    B8G8R8_UNORM,
    B8G8R8_SNORM,
    B8G8R8_UINT,
    B8G8R8_SINT,
    B8G8R8_SRGB,
    R8G8B8A8_UNORM,
    R8G8B8A8_SNORM,
    R8G8B8A8_UINT,
    R8G8B8A8_SINT,
    R8G8B8A8_SRGB,
    B8G8R8A8_UNORM,
    B8G8R8A8_SNORM,
    B8G8R8A8_UINT,
    B8G8R8A8_SINT,
    B8G8R8A8_SRGB,
    R8G8B8X8_UNORM,
    B8G8R8X8_UNORM,
    R16G16_UNORM,
    G16R16_UNORM,
    R16G16_SNORM,
    G16R16_SNORM,
    R16G16_UINT,
    R16G16_SINT,
    R16G16_SFLOAT,
    R16G16_SBFLOAT,
    R32_UINT,
    R32_SINT,
    R32_SFLOAT,
    A2R10G10B10_UNORM,
    A2R10G10B10_UINT,
    A2R10G10B10_SNORM,
    A2R10G10B10_SINT,
    A2B10G10R10_UNORM,
    A2B10G10R10_UINT,
    A2B10G10R10_SNORM,
    A2B10G10R10_SINT,
    R10G10B10A2_UNORM,
    R10G10B10A2_UINT,
    R10G10B10A2_SNORM,
    R10G10B10A2_SINT,
    B10G10R10A2_UNORM,
    B10G10R10A2_UINT,
    B10G10R10A2_SNORM,
    B10G10R10A2_SINT,
    B10G11R11_UFLOAT,
    E5B9G9R9_UFLOAT,
    R16G16B16_UNORM,
    R16G16B16_SNORM,
    R16G16B16_UINT,
    R16G16B16_SINT,
    R16G16B16_SFLOAT,
    R16G16B16_SBFLOAT,
    R16G16B16A16_UNORM,
    R16G16B16A16_SNORM,
    R16G16B16A16_UINT,
    R16G16B16A16_SINT,
    R16G16B16A16_SFLOAT,
    R16G16B16A16_SBFLOAT,
    R32G32_UINT,
    R32G32_SINT,
    R32G32_SFLOAT,
    R32G32B32_UINT,
    R32G32B32_SINT,
    R32G32B32_SFLOAT,
    R32G32B32A32_UINT,
    R32G32B32A32_SINT,
    R32G32B32A32_SFLOAT,
    R64_UINT,
    R64_SINT,
    R64_SFLOAT,
    R64G64_UINT,
    R64G64_SINT,
    R64G64_SFLOAT,
    R64G64B64_UINT,
    R64G64B64_SINT,
    R64G64B64_SFLOAT,
    R64G64B64A64_UINT,
    R64G64B64A64_SINT,
    R64G64B64A64_SFLOAT,
    D16_UNORM,
    X8_D24_UNORM,
    D32_SFLOAT,
    S8_UINT,
    D16_UNORM_S8_UINT,
    D24_UNORM_S8_UINT,
    D32_SFLOAT_S8_UINT,
    DXBC1_RGB_UNORM,
    DXBC1_RGB_SRGB,
    DXBC1_RGBA_UNORM,
    DXBC1_RGBA_SRGB,
    DXBC2_UNORM,
    DXBC2_SRGB,
    DXBC3_UNORM,
    DXBC3_SRGB,
    DXBC4_UNORM,
    DXBC4_SNORM,
    DXBC5_UNORM,
    DXBC5_SNORM,
    DXBC6H_UFLOAT,
    DXBC6H_SFLOAT,
    DXBC7_UNORM,
    DXBC7_SRGB,
    PVRTC1_2BPP_UNORM,
    PVRTC1_4BPP_UNORM,
    PVRTC2_2BPP_UNORM,
    PVRTC2_4BPP_UNORM,
    PVRTC1_2BPP_SRGB,
    PVRTC1_4BPP_SRGB,
    PVRTC2_2BPP_SRGB,
    PVRTC2_4BPP_SRGB,
    ETC2_R8G8B8_UNORM,
    ETC2_R8G8B8_SRGB,
    ETC2_R8G8B8A1_UNORM,
    ETC2_R8G8B8A1_SRGB,
    ETC2_R8G8B8A8_UNORM,
    ETC2_R8G8B8A8_SRGB,
    ETC2_EAC_R11_UNORM,
    ETC2_EAC_R11_SNORM,
    ETC2_EAC_R11G11_UNORM,
    ETC2_EAC_R11G11_SNORM,
    ASTC_4x4_UNORM,
    ASTC_4x4_SRGB,
    ASTC_5x4_UNORM,
    ASTC_5x4_SRGB,
    ASTC_5x5_UNORM,
    ASTC_5x5_SRGB,
    ASTC_6x5_UNORM,
    ASTC_6x5_SRGB,
    ASTC_6x6_UNORM,
    ASTC_6x6_SRGB,
    ASTC_8x5_UNORM,
    ASTC_8x5_SRGB,
    ASTC_8x6_UNORM,
    ASTC_8x6_SRGB,
    ASTC_8x8_UNORM,
    ASTC_8x8_SRGB,
    ASTC_10x5_UNORM,
    ASTC_10x5_SRGB,
    ASTC_10x6_UNORM,
    ASTC_10x6_SRGB,
    ASTC_10x8_UNORM,
    ASTC_10x8_SRGB,
    ASTC_10x10_UNORM,
    ASTC_10x10_SRGB,
    ASTC_12x10_UNORM,
    ASTC_12x10_SRGB,
    ASTC_12x12_UNORM,
    ASTC_12x12_SRGB,
    CLUT_P4,
    CLUT_P4A4,
    CLUT_P8,
    CLUT_P8A8,
    R4G4B4A4_UNORM_PACK16,
    B4G4R4A4_UNORM_PACK16,
    R5G6B5_UNORM_PACK16,
    B5G6R5_UNORM_PACK16,
    R5G5B5A1_UNORM_PACK16,
    B5G5R5A1_UNORM_PACK16,
    A1R5G5B5_UNORM_PACK16,
    G16B16G16R16_422_UNORM,
    B16G16R16G16_422_UNORM,
    R12X4G12X4B12X4A12X4_UNORM_4PACK16,
    G12X4B12X4G12X4R12X4_422_UNORM_4PACK16,
    B12X4G12X4R12X4G12X4_422_UNORM_4PACK16,
    R10X6G10X6B10X6A10X6_UNORM_4PACK16,
    G10X6B10X6G10X6R10X6_422_UNORM_4PACK16,
    B10X6G10X6R10X6G10X6_422_UNORM_4PACK16,
    G8B8G8R8_422_UNORM,
    B8G8R8G8_422_UNORM,
    G8_B8_R8_3PLANE_420_UNORM,
    G8_B8R8_2PLANE_420_UNORM,
    G8_B8_R8_3PLANE_422_UNORM,
    G8_B8R8_2PLANE_422_UNORM,
    G8_B8_R8_3PLANE_444_UNORM,
    G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16,
    G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16,
    G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16,
    G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16,
    G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16,
    G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16,
    G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16,
    G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16,
    G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16,
    G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16,
    G16_B16_R16_3PLANE_420_UNORM,
    G16_B16_R16_3PLANE_422_UNORM,
    G16_B16_R16_3PLANE_444_UNORM,
    G16_B16R16_2PLANE_420_UNORM,
    G16_B16R16_2PLANE_422_UNORM,
}

impl ImageFormat {
    pub fn is_depth_only(&self) -> bool {
        match self {
            ImageFormat::D16_UNORM | ImageFormat::X8_D24_UNORM | ImageFormat::D32_SFLOAT => true,
            _ => false,
        }
    }

    pub fn is_depth(&self) -> bool {
        return self.is_depth_only() || self.is_depth_and_stencil_only();
    }

    pub fn is_stencil(&self) -> bool {
        return self.is_stencil_only() || self.is_depth_and_stencil_only();
    }

    pub fn is_depth_and_stencil_only(&self) -> bool {
        match self {
            ImageFormat::D16_UNORM_S8_UINT
            | ImageFormat::D24_UNORM_S8_UINT
            | ImageFormat::D32_SFLOAT_S8_UINT => true,
            _ => false,
        }
    }

    pub fn is_stencil_only(&self) -> bool {
        match self {
            ImageFormat::S8_UINT => true,
            _ => false
        }
    }

    pub fn is_single_plane(&self) -> bool {
        return !self.is_planer() || self.num_planes() < 2;
    }

    pub fn num_planes(&self) -> u32 {
        match self {
            ImageFormat::G8_B8_R8_3PLANE_420_UNORM
            | ImageFormat::G8_B8_R8_3PLANE_422_UNORM
            | ImageFormat::G8_B8_R8_3PLANE_444_UNORM
            | ImageFormat::G16_B16_R16_3PLANE_420_UNORM
            | ImageFormat::G16_B16_R16_3PLANE_422_UNORM
            | ImageFormat::G16_B16_R16_3PLANE_444_UNORM
            | ImageFormat::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16
            | ImageFormat::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16
            | ImageFormat::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16
            | ImageFormat::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16
            | ImageFormat::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16
            | ImageFormat::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => 3,
            ImageFormat::G8_B8R8_2PLANE_420_UNORM
            | ImageFormat::G8_B8R8_2PLANE_422_UNORM
            | ImageFormat::G16_B16R16_2PLANE_420_UNORM
            | ImageFormat::G16_B16R16_2PLANE_422_UNORM
            | ImageFormat::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16
            | ImageFormat::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16
            | ImageFormat::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16
            | ImageFormat::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => 2,
            _ => 1,
        }
    }

    pub fn is_planer(&self) -> bool {
        match self {
            ImageFormat::G8_B8R8_2PLANE_420_UNORM
            | ImageFormat::G8_B8R8_2PLANE_422_UNORM
            | ImageFormat::G8_B8_R8_3PLANE_420_UNORM
            | ImageFormat::G8_B8_R8_3PLANE_422_UNORM
            | ImageFormat::G8_B8_R8_3PLANE_444_UNORM
            | ImageFormat::G16_B16R16_2PLANE_420_UNORM
            | ImageFormat::G16_B16R16_2PLANE_422_UNORM
            | ImageFormat::G16_B16_R16_3PLANE_420_UNORM
            | ImageFormat::G16_B16_R16_3PLANE_422_UNORM
            | ImageFormat::G16_B16_R16_3PLANE_444_UNORM
            | ImageFormat::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16
            | ImageFormat::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16
            | ImageFormat::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16
            | ImageFormat::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16
            | ImageFormat::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16
            | ImageFormat::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16
            | ImageFormat::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16
            | ImageFormat::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16
            | ImageFormat::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16
            | ImageFormat::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => true,
            _ => false,
        }
    }

    pub fn from_vk_format(format: ash::vk::Format) -> ImageFormat {
        match format {
            ash::vk::Format::R4G4_UNORM_PACK8 => ImageFormat::G4R4_UNORM,
            ash::vk::Format::R4G4B4A4_UNORM_PACK16 => ImageFormat::A4B4G4R4_UNORM,
            ash::vk::Format::B4G4R4A4_UNORM_PACK16 => ImageFormat::A4R4G4B4_UNORM,
            ash::vk::Format::B5G6R5_UNORM_PACK16 => ImageFormat::R5G6B5_UNORM,
            ash::vk::Format::R5G6B5_UNORM_PACK16 => ImageFormat::B5G6R5_UNORM,
            ash::vk::Format::R5G5B5A1_UNORM_PACK16 => ImageFormat::A1B5G5R5_UNORM,
            ash::vk::Format::B5G5R5A1_UNORM_PACK16 => ImageFormat::A1R5G5B5_UNORM,
            ash::vk::Format::A1R5G5B5_UNORM_PACK16 => ImageFormat::B5G5R5A1_UNORM,
            ash::vk::Format::A2B10G10R10_UNORM_PACK32 => ImageFormat::A2B10G10R10_UNORM,
            ash::vk::Format::R8_UNORM => ImageFormat::R8_UNORM,
            ash::vk::Format::R8_SNORM => ImageFormat::R8_SNORM,
            ash::vk::Format::R8_UINT => ImageFormat::R8_UINT,
            ash::vk::Format::R8_SINT => ImageFormat::R8_SINT,
            ash::vk::Format::R8_SRGB => ImageFormat::R8_SRGB,
            ash::vk::Format::R8G8_UNORM => ImageFormat::R8G8_UNORM,
            ash::vk::Format::R8G8_SNORM => ImageFormat::R8G8_SNORM,
            ash::vk::Format::R8G8_UINT => ImageFormat::R8G8_UINT,
            ash::vk::Format::R8G8_SINT => ImageFormat::R8G8_SINT,
            ash::vk::Format::R8G8_SRGB => ImageFormat::R8G8_SRGB,
            ash::vk::Format::R8G8B8_UNORM => ImageFormat::R8G8B8_UNORM,
            ash::vk::Format::R8G8B8_SNORM => ImageFormat::R8G8B8_SNORM,
            ash::vk::Format::R8G8B8_UINT => ImageFormat::R8G8B8_UINT,
            ash::vk::Format::R8G8B8_SINT => ImageFormat::R8G8B8_SINT,
            ash::vk::Format::R8G8B8_SRGB => ImageFormat::R8G8B8_SRGB,
            ash::vk::Format::B8G8R8_UNORM => ImageFormat::B8G8R8_UNORM,
            ash::vk::Format::B8G8R8_SNORM => ImageFormat::B8G8R8_SNORM,
            ash::vk::Format::B8G8R8_UINT => ImageFormat::B8G8R8_UINT,
            ash::vk::Format::B8G8R8_SINT => ImageFormat::B8G8R8_SINT,
            ash::vk::Format::B8G8R8_SRGB => ImageFormat::B8G8R8_SRGB,
            ash::vk::Format::R8G8B8A8_UNORM => ImageFormat::R8G8B8A8_UNORM,
            ash::vk::Format::R8G8B8A8_SNORM => ImageFormat::R8G8B8A8_SNORM,
            ash::vk::Format::R8G8B8A8_UINT => ImageFormat::R8G8B8A8_UINT,
            ash::vk::Format::R8G8B8A8_SINT => ImageFormat::R8G8B8A8_SINT,
            ash::vk::Format::R8G8B8A8_SRGB => ImageFormat::R8G8B8A8_SRGB,
            ash::vk::Format::B8G8R8A8_UNORM => ImageFormat::B8G8R8A8_UNORM,
            ash::vk::Format::B8G8R8A8_SNORM => ImageFormat::B8G8R8A8_SNORM,
            ash::vk::Format::B8G8R8A8_UINT => ImageFormat::B8G8R8A8_UINT,
            ash::vk::Format::B8G8R8A8_SINT => ImageFormat::B8G8R8A8_SINT,
            ash::vk::Format::B8G8R8A8_SRGB => ImageFormat::B8G8R8A8_SRGB,
            ash::vk::Format::R16_UNORM => ImageFormat::R16_UNORM,
            ash::vk::Format::R16_SNORM => ImageFormat::R16_SNORM,
            ash::vk::Format::R16_UINT => ImageFormat::R16_UINT,
            ash::vk::Format::R16_SINT => ImageFormat::R16_SINT,
            ash::vk::Format::R16_SFLOAT => ImageFormat::R16_SFLOAT,
            ash::vk::Format::R16G16_UNORM => ImageFormat::R16G16_UNORM,
            ash::vk::Format::R16G16_SNORM => ImageFormat::R16G16_SNORM,
            ash::vk::Format::R16G16_UINT => ImageFormat::R16G16_UINT,
            ash::vk::Format::R16G16_SINT => ImageFormat::R16G16_SINT,
            ash::vk::Format::R16G16_SFLOAT => ImageFormat::R16G16_SFLOAT,
            ash::vk::Format::R16G16B16_UNORM => ImageFormat::R16G16B16_UNORM,
            ash::vk::Format::R16G16B16_SNORM => ImageFormat::R16G16B16_SNORM,
            ash::vk::Format::R16G16B16_UINT => ImageFormat::R16G16B16_UINT,
            ash::vk::Format::R16G16B16_SINT => ImageFormat::R16G16B16_SINT,
            ash::vk::Format::R16G16B16_SFLOAT => ImageFormat::R16G16B16_SFLOAT,
            ash::vk::Format::R16G16B16A16_UNORM => ImageFormat::R16G16B16A16_UNORM,
            ash::vk::Format::R16G16B16A16_SNORM => ImageFormat::R16G16B16A16_SNORM,
            ash::vk::Format::R16G16B16A16_UINT => ImageFormat::R16G16B16A16_UINT,
            ash::vk::Format::R16G16B16A16_SINT => ImageFormat::R16G16B16A16_SINT,
            ash::vk::Format::R16G16B16A16_SFLOAT => ImageFormat::R16G16B16A16_SFLOAT,
            ash::vk::Format::R32_UINT => ImageFormat::R32_UINT,
            ash::vk::Format::R32_SINT => ImageFormat::R32_SINT,
            ash::vk::Format::R32_SFLOAT => ImageFormat::R32_SFLOAT,
            ash::vk::Format::R32G32_UINT => ImageFormat::R32G32_UINT,
            ash::vk::Format::R32G32_SINT => ImageFormat::R32G32_SINT,
            ash::vk::Format::R32G32_SFLOAT => ImageFormat::R32G32_SFLOAT,
            ash::vk::Format::R32G32B32_UINT => ImageFormat::R32G32B32_UINT,
            ash::vk::Format::R32G32B32_SINT => ImageFormat::R32G32B32_SINT,
            ash::vk::Format::R32G32B32_SFLOAT => ImageFormat::R32G32B32_SFLOAT,
            ash::vk::Format::R32G32B32A32_UINT => ImageFormat::R32G32B32A32_UINT,
            ash::vk::Format::R32G32B32A32_SINT => ImageFormat::R32G32B32A32_SINT,
            ash::vk::Format::R32G32B32A32_SFLOAT => ImageFormat::R32G32B32A32_SFLOAT,
            ash::vk::Format::R64_UINT => ImageFormat::R64_UINT,
            ash::vk::Format::R64_SINT => ImageFormat::R64_SINT,
            ash::vk::Format::R64_SFLOAT => ImageFormat::R64_SFLOAT,
            ash::vk::Format::R64G64_UINT => ImageFormat::R64G64_UINT,
            ash::vk::Format::R64G64_SINT => ImageFormat::R64G64_SINT,
            ash::vk::Format::R64G64_SFLOAT => ImageFormat::R64G64_SFLOAT,
            ash::vk::Format::R64G64B64_UINT => ImageFormat::R64G64B64_UINT,
            ash::vk::Format::R64G64B64_SINT => ImageFormat::R64G64B64_SINT,
            ash::vk::Format::R64G64B64_SFLOAT => ImageFormat::R64G64B64_SFLOAT,
            ash::vk::Format::R64G64B64A64_UINT => ImageFormat::R64G64B64A64_UINT,
            ash::vk::Format::R64G64B64A64_SINT => ImageFormat::R64G64B64A64_SINT,
            ash::vk::Format::R64G64B64A64_SFLOAT => ImageFormat::R64G64B64A64_SFLOAT,
            ash::vk::Format::A2R10G10B10_UNORM_PACK32 => ImageFormat::B10G10R10A2_UNORM,
            ash::vk::Format::A2R10G10B10_UINT_PACK32 => ImageFormat::B10G10R10A2_UINT,
            ash::vk::Format::A2B10G10R10_UNORM_PACK32 => ImageFormat::R10G10B10A2_UNORM,
            ash::vk::Format::A2B10G10R10_UINT_PACK32 => ImageFormat::R10G10B10A2_UINT,
            ash::vk::Format::B10G11R11_UFLOAT_PACK32 => ImageFormat::B10G11R11_UFLOAT,
            ash::vk::Format::E5B9G9R9_UFLOAT_PACK32 => ImageFormat::E5B9G9R9_UFLOAT,
            ash::vk::Format::G16B16G16R16_422_UNORM => {
                ImageFormat::G16B16G16R16_422_UNORM
            }
            ash::vk::Format::B16G16R16G16_422_UNORM => {
                ImageFormat::B16G16R16G16_422_UNORM
            }
            ash::vk::Format::R12X4G12X4B12X4A12X4_UNORM_4PACK16 => {
                ImageFormat::R12X4G12X4B12X4A12X4_UNORM_4PACK16
            }
            ash::vk::Format::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => {
                ImageFormat::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16
            }
            ash::vk::Format::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => {
                ImageFormat::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16
            }
            ash::vk::Format::R10X6G10X6B10X6A10X6_UNORM_4PACK16 => {
                ImageFormat::R10X6G10X6B10X6A10X6_UNORM_4PACK16
            }
            ash::vk::Format::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => {
                ImageFormat::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16
            }
            ash::vk::Format::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => {
                ImageFormat::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16
            }
            ash::vk::Format::G8B8G8R8_422_UNORM => ImageFormat::G8B8G8R8_422_UNORM,
            ash::vk::Format::B8G8R8G8_422_UNORM => ImageFormat::B8G8R8G8_422_UNORM,
            ash::vk::Format::G8_B8_R8_3PLANE_420_UNORM => {
                ImageFormat::G8_B8_R8_3PLANE_420_UNORM
            }
            ash::vk::Format::G8_B8R8_2PLANE_420_UNORM => {
                ImageFormat::G8_B8R8_2PLANE_420_UNORM
            }
            ash::vk::Format::G8_B8_R8_3PLANE_422_UNORM => {
                ImageFormat::G8_B8_R8_3PLANE_422_UNORM
            }
            ash::vk::Format::G8_B8R8_2PLANE_422_UNORM => {
                ImageFormat::G8_B8R8_2PLANE_422_UNORM
            }
            ash::vk::Format::G8_B8_R8_3PLANE_444_UNORM => {
                ImageFormat::G8_B8_R8_3PLANE_444_UNORM
            }
            ash::vk::Format::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => {
                ImageFormat::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16
            }
            ash::vk::Format::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => {
                ImageFormat::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16
            }
            ash::vk::Format::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => {
                ImageFormat::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16
            }
            ash::vk::Format::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => {
                ImageFormat::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16
            }
            ash::vk::Format::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => {
                ImageFormat::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16
            }
            ash::vk::Format::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => {
                ImageFormat::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16
            }
            ash::vk::Format::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => {
                ImageFormat::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16
            }
            ash::vk::Format::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => {
                ImageFormat::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16
            }
            ash::vk::Format::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => {
                ImageFormat::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16
            }
            ash::vk::Format::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => {
                ImageFormat::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16
            }
            ash::vk::Format::G16_B16_R16_3PLANE_420_UNORM => {
                ImageFormat::G16_B16_R16_3PLANE_420_UNORM
            }
            ash::vk::Format::G16_B16_R16_3PLANE_422_UNORM => {
                ImageFormat::G16_B16_R16_3PLANE_422_UNORM
            }
            ash::vk::Format::G16_B16_R16_3PLANE_444_UNORM => {
                ImageFormat::G16_B16_R16_3PLANE_444_UNORM
            }
            ash::vk::Format::G16_B16R16_2PLANE_420_UNORM => {
                ImageFormat::G16_B16R16_2PLANE_420_UNORM
            }
            ash::vk::Format::G16_B16R16_2PLANE_422_UNORM => {
                ImageFormat::G16_B16R16_2PLANE_422_UNORM
            }
            ash::vk::Format::D16_UNORM => ImageFormat::D16_UNORM,
            ash::vk::Format::X8_D24_UNORM_PACK32 => ImageFormat::X8_D24_UNORM,
            ash::vk::Format::D32_SFLOAT => ImageFormat::D32_SFLOAT,
            ash::vk::Format::S8_UINT => ImageFormat::S8_UINT,
            ash::vk::Format::D16_UNORM_S8_UINT => ImageFormat::D16_UNORM_S8_UINT,
            ash::vk::Format::D24_UNORM_S8_UINT => ImageFormat::D24_UNORM_S8_UINT,
            ash::vk::Format::D32_SFLOAT_S8_UINT => ImageFormat::D32_SFLOAT_S8_UINT,
            ash::vk::Format::BC1_RGB_UNORM_BLOCK => ImageFormat::DXBC1_RGB_UNORM,
            ash::vk::Format::BC1_RGB_SRGB_BLOCK => ImageFormat::DXBC1_RGB_SRGB,
            ash::vk::Format::BC1_RGBA_UNORM_BLOCK => ImageFormat::DXBC1_RGBA_UNORM,
            ash::vk::Format::BC1_RGBA_SRGB_BLOCK => ImageFormat::DXBC1_RGBA_SRGB,
            ash::vk::Format::BC2_UNORM_BLOCK => ImageFormat::DXBC2_UNORM,
            ash::vk::Format::BC2_SRGB_BLOCK => ImageFormat::DXBC2_SRGB,
            ash::vk::Format::BC3_UNORM_BLOCK => ImageFormat::DXBC3_UNORM,
            ash::vk::Format::BC3_SRGB_BLOCK => ImageFormat::DXBC3_SRGB,
            ash::vk::Format::BC4_UNORM_BLOCK => ImageFormat::DXBC4_UNORM,
            ash::vk::Format::BC4_SNORM_BLOCK => ImageFormat::DXBC4_SNORM,
            ash::vk::Format::BC5_UNORM_BLOCK => ImageFormat::DXBC5_UNORM,
            ash::vk::Format::BC5_SNORM_BLOCK => ImageFormat::DXBC5_SNORM,
            ash::vk::Format::BC6H_UFLOAT_BLOCK => ImageFormat::DXBC6H_UFLOAT,
            ash::vk::Format::BC6H_SFLOAT_BLOCK => ImageFormat::DXBC6H_SFLOAT,
            ash::vk::Format::BC7_UNORM_BLOCK => ImageFormat::DXBC7_UNORM,
            ash::vk::Format::BC7_SRGB_BLOCK => ImageFormat::DXBC7_SRGB,
            ash::vk::Format::PVRTC1_2BPP_UNORM_BLOCK_IMG => {
                ImageFormat::PVRTC1_2BPP_UNORM
            }
            ash::vk::Format::PVRTC1_4BPP_UNORM_BLOCK_IMG => {
                ImageFormat::PVRTC1_4BPP_UNORM
            }
            ash::vk::Format::PVRTC1_2BPP_SRGB_BLOCK_IMG => ImageFormat::PVRTC1_2BPP_SRGB,
            ash::vk::Format::PVRTC1_4BPP_SRGB_BLOCK_IMG => ImageFormat::PVRTC1_4BPP_SRGB,
            ash::vk::Format::ETC2_R8G8B8_UNORM_BLOCK => ImageFormat::ETC2_R8G8B8_UNORM,
            ash::vk::Format::ETC2_R8G8B8A1_UNORM_BLOCK => {
                ImageFormat::ETC2_R8G8B8A1_UNORM
            }
            ash::vk::Format::ETC2_R8G8B8A8_UNORM_BLOCK => {
                ImageFormat::ETC2_R8G8B8A8_UNORM
            }
            ash::vk::Format::ETC2_R8G8B8_SRGB_BLOCK => ImageFormat::ETC2_R8G8B8_SRGB,
            ash::vk::Format::ETC2_R8G8B8A1_SRGB_BLOCK => ImageFormat::ETC2_R8G8B8A1_SRGB,
            ash::vk::Format::ETC2_R8G8B8A8_SRGB_BLOCK => ImageFormat::ETC2_R8G8B8A8_SRGB,
            ash::vk::Format::EAC_R11_UNORM_BLOCK => ImageFormat::ETC2_EAC_R11_UNORM,
            ash::vk::Format::EAC_R11G11_UNORM_BLOCK => {
                ImageFormat::ETC2_EAC_R11G11_UNORM
            }
            ash::vk::Format::EAC_R11_SNORM_BLOCK => ImageFormat::ETC2_EAC_R11_SNORM,
            ash::vk::Format::EAC_R11G11_SNORM_BLOCK => {
                ImageFormat::ETC2_EAC_R11G11_SNORM
            }
            ash::vk::Format::ASTC_4X4_UNORM_BLOCK => ImageFormat::ASTC_4x4_UNORM,
            ash::vk::Format::ASTC_4X4_SRGB_BLOCK => ImageFormat::ASTC_4x4_SRGB,
            ash::vk::Format::ASTC_5X4_UNORM_BLOCK => ImageFormat::ASTC_5x4_UNORM,
            ash::vk::Format::ASTC_5X4_SRGB_BLOCK => ImageFormat::ASTC_5x4_SRGB,
            ash::vk::Format::ASTC_5X5_UNORM_BLOCK => ImageFormat::ASTC_5x5_UNORM,
            ash::vk::Format::ASTC_5X5_SRGB_BLOCK => ImageFormat::ASTC_5x5_SRGB,
            ash::vk::Format::ASTC_6X5_UNORM_BLOCK => ImageFormat::ASTC_6x5_UNORM,
            ash::vk::Format::ASTC_6X5_SRGB_BLOCK => ImageFormat::ASTC_6x5_SRGB,
            ash::vk::Format::ASTC_6X6_UNORM_BLOCK => ImageFormat::ASTC_6x6_UNORM,
            ash::vk::Format::ASTC_6X6_SRGB_BLOCK => ImageFormat::ASTC_6x6_SRGB,
            ash::vk::Format::ASTC_8X5_UNORM_BLOCK => ImageFormat::ASTC_8x5_UNORM,
            ash::vk::Format::ASTC_8X5_SRGB_BLOCK => ImageFormat::ASTC_8x5_SRGB,
            ash::vk::Format::ASTC_8X6_UNORM_BLOCK => ImageFormat::ASTC_8x6_UNORM,
            ash::vk::Format::ASTC_8X6_SRGB_BLOCK => ImageFormat::ASTC_8x6_SRGB,
            ash::vk::Format::ASTC_8X8_UNORM_BLOCK => ImageFormat::ASTC_8x8_UNORM,
            ash::vk::Format::ASTC_8X8_SRGB_BLOCK => ImageFormat::ASTC_8x8_SRGB,
            ash::vk::Format::ASTC_10X5_UNORM_BLOCK => ImageFormat::ASTC_10x5_UNORM,
            ash::vk::Format::ASTC_10X5_SRGB_BLOCK => ImageFormat::ASTC_10x5_SRGB,
            ash::vk::Format::ASTC_10X6_UNORM_BLOCK => ImageFormat::ASTC_10x6_UNORM,
            ash::vk::Format::ASTC_10X6_SRGB_BLOCK => ImageFormat::ASTC_10x6_SRGB,
            ash::vk::Format::ASTC_10X8_UNORM_BLOCK => ImageFormat::ASTC_10x8_UNORM,
            ash::vk::Format::ASTC_10X8_SRGB_BLOCK => ImageFormat::ASTC_10x8_SRGB,
            ash::vk::Format::ASTC_10X10_UNORM_BLOCK => ImageFormat::ASTC_10x10_UNORM,
            ash::vk::Format::ASTC_10X10_SRGB_BLOCK => ImageFormat::ASTC_10x10_SRGB,
            ash::vk::Format::ASTC_12X10_UNORM_BLOCK => ImageFormat::ASTC_12x10_UNORM,
            ash::vk::Format::ASTC_12X10_SRGB_BLOCK => ImageFormat::ASTC_12x10_SRGB,
            ash::vk::Format::ASTC_12X12_UNORM_BLOCK => ImageFormat::ASTC_12x12_UNORM,
            ash::vk::Format::ASTC_12X12_SRGB_BLOCK => ImageFormat::ASTC_12x12_SRGB,
            ash::vk::Format::PVRTC2_2BPP_UNORM_BLOCK_IMG => {
                ImageFormat::PVRTC2_2BPP_UNORM
            }
            ash::vk::Format::PVRTC2_4BPP_UNORM_BLOCK_IMG => {
                ImageFormat::PVRTC2_4BPP_UNORM
            }
            ash::vk::Format::PVRTC2_2BPP_SRGB_BLOCK_IMG => ImageFormat::PVRTC2_2BPP_SRGB,
            ash::vk::Format::PVRTC2_4BPP_SRGB_BLOCK_IMG => ImageFormat::PVRTC2_4BPP_SRGB,
            _ => ImageFormat::UNDEFINED,
        }
    }

    pub fn to_vk_format(&self) -> ash::vk::Format {
        match self {
            ImageFormat::G4R4_UNORM => ash::vk::Format::R4G4_UNORM_PACK8,
            ImageFormat::A4B4G4R4_UNORM => ash::vk::Format::R4G4B4A4_UNORM_PACK16,
            ImageFormat::A4R4G4B4_UNORM => ash::vk::Format::B4G4R4A4_UNORM_PACK16,
            ImageFormat::R5G6B5_UNORM => ash::vk::Format::B5G6R5_UNORM_PACK16,
            ImageFormat::B5G6R5_UNORM => ash::vk::Format::R5G6B5_UNORM_PACK16,
            ImageFormat::A1B5G5R5_UNORM => ash::vk::Format::R5G5B5A1_UNORM_PACK16,
            ImageFormat::A1R5G5B5_UNORM => ash::vk::Format::B5G5R5A1_UNORM_PACK16,
            ImageFormat::B5G5R5A1_UNORM => ash::vk::Format::A1R5G5B5_UNORM_PACK16,
            ImageFormat::A2B10G10R10_UNORM => ash::vk::Format::A2B10G10R10_UNORM_PACK32,
            ImageFormat::R8_UNORM => ash::vk::Format::R8_UNORM,
            ImageFormat::R8_SNORM => ash::vk::Format::R8_SNORM,
            ImageFormat::R8_UINT => ash::vk::Format::R8_UINT,
            ImageFormat::R8_SINT => ash::vk::Format::R8_SINT,
            ImageFormat::R8_SRGB => ash::vk::Format::R8_SRGB,
            ImageFormat::R8G8_UNORM => ash::vk::Format::R8G8_UNORM,
            ImageFormat::R8G8_SNORM => ash::vk::Format::R8G8_SNORM,
            ImageFormat::R8G8_UINT => ash::vk::Format::R8G8_UINT,
            ImageFormat::R8G8_SINT => ash::vk::Format::R8G8_SINT,
            ImageFormat::R8G8_SRGB => ash::vk::Format::R8G8_SRGB,
            ImageFormat::R8G8B8_UNORM => ash::vk::Format::R8G8B8_UNORM,
            ImageFormat::R8G8B8_SNORM => ash::vk::Format::R8G8B8_SNORM,
            ImageFormat::R8G8B8_UINT => ash::vk::Format::R8G8B8_UINT,
            ImageFormat::R8G8B8_SINT => ash::vk::Format::R8G8B8_SINT,
            ImageFormat::R8G8B8_SRGB => ash::vk::Format::R8G8B8_SRGB,
            ImageFormat::B8G8R8_UNORM => ash::vk::Format::B8G8R8_UNORM,
            ImageFormat::B8G8R8_SNORM => ash::vk::Format::B8G8R8_SNORM,
            ImageFormat::B8G8R8_UINT => ash::vk::Format::B8G8R8_UINT,
            ImageFormat::B8G8R8_SINT => ash::vk::Format::B8G8R8_SINT,
            ImageFormat::B8G8R8_SRGB => ash::vk::Format::B8G8R8_SRGB,
            ImageFormat::R8G8B8A8_UNORM => ash::vk::Format::R8G8B8A8_UNORM,
            ImageFormat::R8G8B8A8_SNORM => ash::vk::Format::R8G8B8A8_SNORM,
            ImageFormat::R8G8B8A8_UINT => ash::vk::Format::R8G8B8A8_UINT,
            ImageFormat::R8G8B8A8_SINT => ash::vk::Format::R8G8B8A8_SINT,
            ImageFormat::R8G8B8A8_SRGB => ash::vk::Format::R8G8B8A8_SRGB,
            ImageFormat::B8G8R8A8_UNORM => ash::vk::Format::B8G8R8A8_UNORM,
            ImageFormat::B8G8R8A8_SNORM => ash::vk::Format::B8G8R8A8_SNORM,
            ImageFormat::B8G8R8A8_UINT => ash::vk::Format::B8G8R8A8_UINT,
            ImageFormat::B8G8R8A8_SINT => ash::vk::Format::B8G8R8A8_SINT,
            ImageFormat::B8G8R8A8_SRGB => ash::vk::Format::B8G8R8A8_SRGB,
            ImageFormat::R16_UNORM => ash::vk::Format::R16_UNORM,
            ImageFormat::R16_SNORM => ash::vk::Format::R16_SNORM,
            ImageFormat::R16_UINT => ash::vk::Format::R16_UINT,
            ImageFormat::R16_SINT => ash::vk::Format::R16_SINT,
            ImageFormat::R16_SFLOAT => ash::vk::Format::R16_SFLOAT,
            ImageFormat::R16G16_UNORM => ash::vk::Format::R16G16_UNORM,
            ImageFormat::R16G16_SNORM => ash::vk::Format::R16G16_SNORM,
            ImageFormat::R16G16_UINT => ash::vk::Format::R16G16_UINT,
            ImageFormat::R16G16_SINT => ash::vk::Format::R16G16_SINT,
            ImageFormat::R16G16_SFLOAT => ash::vk::Format::R16G16_SFLOAT,
            ImageFormat::R16G16B16_UNORM => ash::vk::Format::R16G16B16_UNORM,
            ImageFormat::R16G16B16_SNORM => ash::vk::Format::R16G16B16_SNORM,
            ImageFormat::R16G16B16_UINT => ash::vk::Format::R16G16B16_UINT,
            ImageFormat::R16G16B16_SINT => ash::vk::Format::R16G16B16_SINT,
            ImageFormat::R16G16B16_SFLOAT => ash::vk::Format::R16G16B16_SFLOAT,
            ImageFormat::R16G16B16A16_UNORM => ash::vk::Format::R16G16B16A16_UNORM,
            ImageFormat::R16G16B16A16_SNORM => ash::vk::Format::R16G16B16A16_SNORM,
            ImageFormat::R16G16B16A16_UINT => ash::vk::Format::R16G16B16A16_UINT,
            ImageFormat::R16G16B16A16_SINT => ash::vk::Format::R16G16B16A16_SINT,
            ImageFormat::R16G16B16A16_SFLOAT => ash::vk::Format::R16G16B16A16_SFLOAT,
            ImageFormat::R32_UINT => ash::vk::Format::R32_UINT,
            ImageFormat::R32_SINT => ash::vk::Format::R32_SINT,
            ImageFormat::R32_SFLOAT => ash::vk::Format::R32_SFLOAT,
            ImageFormat::R32G32_UINT => ash::vk::Format::R32G32_UINT,
            ImageFormat::R32G32_SINT => ash::vk::Format::R32G32_SINT,
            ImageFormat::R32G32_SFLOAT => ash::vk::Format::R32G32_SFLOAT,
            ImageFormat::R32G32B32_UINT => ash::vk::Format::R32G32B32_UINT,
            ImageFormat::R32G32B32_SINT => ash::vk::Format::R32G32B32_SINT,
            ImageFormat::R32G32B32_SFLOAT => ash::vk::Format::R32G32B32_SFLOAT,
            ImageFormat::R32G32B32A32_UINT => ash::vk::Format::R32G32B32A32_UINT,
            ImageFormat::R32G32B32A32_SINT => ash::vk::Format::R32G32B32A32_SINT,
            ImageFormat::R32G32B32A32_SFLOAT => ash::vk::Format::R32G32B32A32_SFLOAT,
            ImageFormat::R64_UINT => ash::vk::Format::R64_UINT,
            ImageFormat::R64_SINT => ash::vk::Format::R64_SINT,
            ImageFormat::R64_SFLOAT => ash::vk::Format::R64_SFLOAT,
            ImageFormat::R64G64_UINT => ash::vk::Format::R64G64_UINT,
            ImageFormat::R64G64_SINT => ash::vk::Format::R64G64_SINT,
            ImageFormat::R64G64_SFLOAT => ash::vk::Format::R64G64_SFLOAT,
            ImageFormat::R64G64B64_UINT => ash::vk::Format::R64G64B64_UINT,
            ImageFormat::R64G64B64_SINT => ash::vk::Format::R64G64B64_SINT,
            ImageFormat::R64G64B64_SFLOAT => ash::vk::Format::R64G64B64_SFLOAT,
            ImageFormat::R64G64B64A64_UINT => ash::vk::Format::R64G64B64A64_UINT,
            ImageFormat::R64G64B64A64_SINT => ash::vk::Format::R64G64B64A64_SINT,
            ImageFormat::R64G64B64A64_SFLOAT => ash::vk::Format::R64G64B64A64_SFLOAT,

            ImageFormat::B10G10R10A2_UNORM => ash::vk::Format::A2R10G10B10_UNORM_PACK32,
            ImageFormat::B10G10R10A2_UINT => ash::vk::Format::A2R10G10B10_UINT_PACK32,
            ImageFormat::R10G10B10A2_UNORM => ash::vk::Format::A2B10G10R10_UNORM_PACK32,
            ImageFormat::R10G10B10A2_UINT => ash::vk::Format::A2B10G10R10_UINT_PACK32,

            ImageFormat::B10G11R11_UFLOAT => ash::vk::Format::B10G11R11_UFLOAT_PACK32,
            ImageFormat::E5B9G9R9_UFLOAT => ash::vk::Format::E5B9G9R9_UFLOAT_PACK32,

            ImageFormat::G16B16G16R16_422_UNORM => {
                ash::vk::Format::G16B16G16R16_422_UNORM
            }
            ImageFormat::B16G16R16G16_422_UNORM => {
                ash::vk::Format::B16G16R16G16_422_UNORM
            }
            ImageFormat::R12X4G12X4B12X4A12X4_UNORM_4PACK16 => {
                ash::vk::Format::R12X4G12X4B12X4A12X4_UNORM_4PACK16
            }
            ImageFormat::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => {
                ash::vk::Format::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16
            }
            ImageFormat::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => {
                ash::vk::Format::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16
            }
            ImageFormat::R10X6G10X6B10X6A10X6_UNORM_4PACK16 => {
                ash::vk::Format::R10X6G10X6B10X6A10X6_UNORM_4PACK16
            }
            ImageFormat::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => {
                ash::vk::Format::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16
            }
            ImageFormat::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => {
                ash::vk::Format::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16
            }
            ImageFormat::G8B8G8R8_422_UNORM => ash::vk::Format::G8B8G8R8_422_UNORM,
            ImageFormat::B8G8R8G8_422_UNORM => ash::vk::Format::B8G8R8G8_422_UNORM,
            ImageFormat::G8_B8_R8_3PLANE_420_UNORM => {
                ash::vk::Format::G8_B8_R8_3PLANE_420_UNORM
            }
            ImageFormat::G8_B8R8_2PLANE_420_UNORM => {
                ash::vk::Format::G8_B8R8_2PLANE_420_UNORM
            }
            ImageFormat::G8_B8_R8_3PLANE_422_UNORM => {
                ash::vk::Format::G8_B8_R8_3PLANE_422_UNORM
            }
            ImageFormat::G8_B8R8_2PLANE_422_UNORM => {
                ash::vk::Format::G8_B8R8_2PLANE_422_UNORM
            }
            ImageFormat::G8_B8_R8_3PLANE_444_UNORM => {
                ash::vk::Format::G8_B8_R8_3PLANE_444_UNORM
            }
            ImageFormat::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => {
                ash::vk::Format::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16
            }
            ImageFormat::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => {
                ash::vk::Format::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16
            }
            ImageFormat::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => {
                ash::vk::Format::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16
            }
            ImageFormat::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => {
                ash::vk::Format::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16
            }
            ImageFormat::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => {
                ash::vk::Format::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16
            }
            ImageFormat::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => {
                ash::vk::Format::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16
            }
            ImageFormat::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => {
                ash::vk::Format::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16
            }
            ImageFormat::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => {
                ash::vk::Format::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16
            }
            ImageFormat::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => {
                ash::vk::Format::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16
            }
            ImageFormat::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => {
                ash::vk::Format::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16
            }
            ImageFormat::G16_B16_R16_3PLANE_420_UNORM => {
                ash::vk::Format::G16_B16_R16_3PLANE_420_UNORM
            }
            ImageFormat::G16_B16_R16_3PLANE_422_UNORM => {
                ash::vk::Format::G16_B16_R16_3PLANE_422_UNORM
            }
            ImageFormat::G16_B16_R16_3PLANE_444_UNORM => {
                ash::vk::Format::G16_B16_R16_3PLANE_444_UNORM
            }
            ImageFormat::G16_B16R16_2PLANE_420_UNORM => {
                ash::vk::Format::G16_B16R16_2PLANE_420_UNORM
            }
            ImageFormat::G16_B16R16_2PLANE_422_UNORM => {
                ash::vk::Format::G16_B16R16_2PLANE_422_UNORM
            }

            ImageFormat::D16_UNORM => ash::vk::Format::D16_UNORM,
            ImageFormat::X8_D24_UNORM => ash::vk::Format::X8_D24_UNORM_PACK32,
            ImageFormat::D32_SFLOAT => ash::vk::Format::D32_SFLOAT,
            ImageFormat::S8_UINT => ash::vk::Format::S8_UINT,
            ImageFormat::D16_UNORM_S8_UINT => ash::vk::Format::D16_UNORM_S8_UINT,
            ImageFormat::D24_UNORM_S8_UINT => ash::vk::Format::D24_UNORM_S8_UINT,
            ImageFormat::D32_SFLOAT_S8_UINT => ash::vk::Format::D32_SFLOAT_S8_UINT,
            ImageFormat::DXBC1_RGB_UNORM => ash::vk::Format::BC1_RGB_UNORM_BLOCK,
            ImageFormat::DXBC1_RGB_SRGB => ash::vk::Format::BC1_RGB_SRGB_BLOCK,
            ImageFormat::DXBC1_RGBA_UNORM => ash::vk::Format::BC1_RGBA_UNORM_BLOCK,
            ImageFormat::DXBC1_RGBA_SRGB => ash::vk::Format::BC1_RGBA_SRGB_BLOCK,
            ImageFormat::DXBC2_UNORM => ash::vk::Format::BC2_UNORM_BLOCK,
            ImageFormat::DXBC2_SRGB => ash::vk::Format::BC2_SRGB_BLOCK,
            ImageFormat::DXBC3_UNORM => ash::vk::Format::BC3_UNORM_BLOCK,
            ImageFormat::DXBC3_SRGB => ash::vk::Format::BC3_SRGB_BLOCK,
            ImageFormat::DXBC4_UNORM => ash::vk::Format::BC4_UNORM_BLOCK,
            ImageFormat::DXBC4_SNORM => ash::vk::Format::BC4_SNORM_BLOCK,
            ImageFormat::DXBC5_UNORM => ash::vk::Format::BC5_UNORM_BLOCK,
            ImageFormat::DXBC5_SNORM => ash::vk::Format::BC5_SNORM_BLOCK,
            ImageFormat::DXBC6H_UFLOAT => ash::vk::Format::BC6H_UFLOAT_BLOCK,
            ImageFormat::DXBC6H_SFLOAT => ash::vk::Format::BC6H_SFLOAT_BLOCK,
            ImageFormat::DXBC7_UNORM => ash::vk::Format::BC7_UNORM_BLOCK,
            ImageFormat::DXBC7_SRGB => ash::vk::Format::BC7_SRGB_BLOCK,
            ImageFormat::PVRTC1_2BPP_UNORM => {
                ash::vk::Format::PVRTC1_2BPP_UNORM_BLOCK_IMG
            }
            ImageFormat::PVRTC1_4BPP_UNORM => {
                ash::vk::Format::PVRTC1_4BPP_UNORM_BLOCK_IMG
            }
            ImageFormat::PVRTC1_2BPP_SRGB => ash::vk::Format::PVRTC1_2BPP_SRGB_BLOCK_IMG,
            ImageFormat::PVRTC1_4BPP_SRGB => ash::vk::Format::PVRTC1_4BPP_SRGB_BLOCK_IMG,
            ImageFormat::ETC2_R8G8B8_UNORM => ash::vk::Format::ETC2_R8G8B8_UNORM_BLOCK,
            ImageFormat::ETC2_R8G8B8A1_UNORM => {
                ash::vk::Format::ETC2_R8G8B8A1_UNORM_BLOCK
            }
            ImageFormat::ETC2_R8G8B8A8_UNORM => {
                ash::vk::Format::ETC2_R8G8B8A8_UNORM_BLOCK
            }
            ImageFormat::ETC2_R8G8B8_SRGB => ash::vk::Format::ETC2_R8G8B8_SRGB_BLOCK,
            ImageFormat::ETC2_R8G8B8A1_SRGB => ash::vk::Format::ETC2_R8G8B8A1_SRGB_BLOCK,
            ImageFormat::ETC2_R8G8B8A8_SRGB => ash::vk::Format::ETC2_R8G8B8A8_SRGB_BLOCK,
            ImageFormat::ETC2_EAC_R11_UNORM => ash::vk::Format::EAC_R11_UNORM_BLOCK,
            ImageFormat::ETC2_EAC_R11G11_UNORM => {
                ash::vk::Format::EAC_R11G11_UNORM_BLOCK
            }
            ImageFormat::ETC2_EAC_R11_SNORM => ash::vk::Format::EAC_R11_SNORM_BLOCK,
            ImageFormat::ETC2_EAC_R11G11_SNORM => {
                ash::vk::Format::EAC_R11G11_SNORM_BLOCK
            }
            ImageFormat::ASTC_4x4_UNORM => ash::vk::Format::ASTC_4X4_UNORM_BLOCK,
            ImageFormat::ASTC_4x4_SRGB => ash::vk::Format::ASTC_4X4_SRGB_BLOCK,
            ImageFormat::ASTC_5x4_UNORM => ash::vk::Format::ASTC_5X4_UNORM_BLOCK,
            ImageFormat::ASTC_5x4_SRGB => ash::vk::Format::ASTC_5X4_SRGB_BLOCK,
            ImageFormat::ASTC_5x5_UNORM => ash::vk::Format::ASTC_5X5_UNORM_BLOCK,
            ImageFormat::ASTC_5x5_SRGB => ash::vk::Format::ASTC_5X5_SRGB_BLOCK,
            ImageFormat::ASTC_6x5_UNORM => ash::vk::Format::ASTC_6X5_UNORM_BLOCK,
            ImageFormat::ASTC_6x5_SRGB => ash::vk::Format::ASTC_6X5_SRGB_BLOCK,
            ImageFormat::ASTC_6x6_UNORM => ash::vk::Format::ASTC_6X6_UNORM_BLOCK,
            ImageFormat::ASTC_6x6_SRGB => ash::vk::Format::ASTC_6X6_SRGB_BLOCK,
            ImageFormat::ASTC_8x5_UNORM => ash::vk::Format::ASTC_8X5_UNORM_BLOCK,
            ImageFormat::ASTC_8x5_SRGB => ash::vk::Format::ASTC_8X5_SRGB_BLOCK,
            ImageFormat::ASTC_8x6_UNORM => ash::vk::Format::ASTC_8X6_UNORM_BLOCK,
            ImageFormat::ASTC_8x6_SRGB => ash::vk::Format::ASTC_8X6_SRGB_BLOCK,
            ImageFormat::ASTC_8x8_UNORM => ash::vk::Format::ASTC_8X8_UNORM_BLOCK,
            ImageFormat::ASTC_8x8_SRGB => ash::vk::Format::ASTC_8X8_SRGB_BLOCK,
            ImageFormat::ASTC_10x5_UNORM => ash::vk::Format::ASTC_10X5_UNORM_BLOCK,
            ImageFormat::ASTC_10x5_SRGB => ash::vk::Format::ASTC_10X5_SRGB_BLOCK,
            ImageFormat::ASTC_10x6_UNORM => ash::vk::Format::ASTC_10X6_UNORM_BLOCK,
            ImageFormat::ASTC_10x6_SRGB => ash::vk::Format::ASTC_10X6_SRGB_BLOCK,
            ImageFormat::ASTC_10x8_UNORM => ash::vk::Format::ASTC_10X8_UNORM_BLOCK,
            ImageFormat::ASTC_10x8_SRGB => ash::vk::Format::ASTC_10X8_SRGB_BLOCK,
            ImageFormat::ASTC_10x10_UNORM => ash::vk::Format::ASTC_10X10_UNORM_BLOCK,
            ImageFormat::ASTC_10x10_SRGB => ash::vk::Format::ASTC_10X10_SRGB_BLOCK,
            ImageFormat::ASTC_12x10_UNORM => ash::vk::Format::ASTC_12X10_UNORM_BLOCK,
            ImageFormat::ASTC_12x10_SRGB => ash::vk::Format::ASTC_12X10_SRGB_BLOCK,
            ImageFormat::ASTC_12x12_UNORM => ash::vk::Format::ASTC_12X12_UNORM_BLOCK,
            ImageFormat::ASTC_12x12_SRGB => ash::vk::Format::ASTC_12X12_SRGB_BLOCK,
            ImageFormat::PVRTC2_2BPP_UNORM => {
                ash::vk::Format::PVRTC2_2BPP_UNORM_BLOCK_IMG
            }
            ImageFormat::PVRTC2_4BPP_UNORM => {
                ash::vk::Format::PVRTC2_4BPP_UNORM_BLOCK_IMG
            }
            ImageFormat::PVRTC2_2BPP_SRGB => ash::vk::Format::PVRTC2_2BPP_SRGB_BLOCK_IMG,
            ImageFormat::PVRTC2_4BPP_SRGB => ash::vk::Format::PVRTC2_4BPP_SRGB_BLOCK_IMG,
            _ => ash::vk::Format::UNDEFINED,
        }
    }

    pub fn to_vk_aspect_mask(&self, include_stencil_bit: bool) -> ash::vk::ImageAspectFlags {
        match self {
            // Depth
            ImageFormat::D16_UNORM | ImageFormat::X8_D24_UNORM | ImageFormat::D32_SFLOAT => {
                ash::vk::ImageAspectFlags::DEPTH
            },
            // Stencil
            ImageFormat::S8_UINT => ash::vk::ImageAspectFlags::STENCIL,

            ImageFormat::D16_UNORM_S8_UINT
            | ImageFormat::D24_UNORM_S8_UINT
            | ImageFormat::D32_SFLOAT_S8_UINT => {
                ash::vk::ImageAspectFlags::DEPTH
                    | (if include_stencil_bit {
                    ash::vk::ImageAspectFlags::STENCIL
                } else {
                    ash::vk::ImageAspectFlags::empty()
                })
            },
            _ => ash::vk::ImageAspectFlags::COLOR,
        }
    }
}
