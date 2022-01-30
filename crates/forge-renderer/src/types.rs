use bitflags::bitflags;

#[derive(PartialEq, Copy, Clone)]
pub enum DescriptorUpdateFrequency {
    DescriptorUpdateFreqNone = 0,
    DescriptorUpdateFreqPerFrame,
    DescriptorUpdateFreqPerBatch,
    DescriptorUpdateFreqPerDraw,
    DescriptorUpdateFreqCount,
}

#[derive(PartialEq, Copy, Clone)]
pub enum FenceStatus {
    Complete,
    Incomplete,
    NotSubmitted,
}

#[derive(PartialEq, Copy, Clone)]
pub enum QueueType {
    QueueTypeGraphics = 0,
    QueueTypeTransfer,
    QueueTypeCompute,
    MaxQueueType,
}

#[derive(PartialEq, Copy, Clone)]
pub enum FilterType {
    Nearest,
    Linear,
}

#[derive(PartialEq, Copy, Clone)]
pub enum AddressMode {
    AddressModeMirror,
    AddressModeRepeat,
    AddressModeClampToEdge,
    AddressModeClampToBorder,
}

#[derive(PartialEq, Copy, Clone)]
pub enum CompareMode {
    Never,
    Less,
    Equal,
    LeEqual,
    Greater,
    NotEqual,
    GeEqual,
    Always,
}

#[derive(PartialEq, Copy, Clone)]
pub enum MipMapMode {
    Nearest = 0,
    Linear,
}

pub enum IndirectArgumentType {
    Draw,
    DrawIndex,
    Dispatch,
    VertexBuffer,
    IndexBuffer,
    Constant,
    DescriptorTable,    // only for vulkan
    Pipeline,           // only for vulkan now, probably will add to dx when it comes to xbox
    ConstantBufferView, // only for dx
    ShaderResourceView, // only for dx
    UnorderedAccessView, // only for dx
                        // #if defined(METAL)
                        // INDIRECT_COMMAND_BUFFER,            // metal ICB
                        // INDIRECT_COMMAND_BUFFER_RESET,      // metal ICB reset
                        // INDIRECT_COMMAND_BUFFER_OPTIMIZE    // metal ICB optimization
                        // #endif
                        // } IndirectArgumentType;
}

#[derive(PartialEq, Copy, Clone)]
pub enum ShaderStage {
    Vertex,
    TesselationControl,
    TesselationEvaluation,
    Geometry,
    Fragment,
    Compute,
    Raytracing,
    Count
}

bitflags! {
    // tessellation
    pub struct ShaderStageFlags: u16 {
        const STAGE_NONE = 0x0000;
        const STAGE_VERTEX = 1 << (ShaderStage::Vertex as u32);
        const STAGE_TESSELLATION_CONTROL =   1 << (ShaderStage::TesselationControl as u32);
        const STAGE_TESSELLATION_EVALUATION =  1 << (ShaderStage::TesselationEvaluation as u32);
        const STAGE_GEOMETRY = 1 << (ShaderStage::Geometry as u32);
        const STAGE_FRAGMENT =  1 << (ShaderStage::Fragment as u32);
        const STAGE_COMPUTE =  1 << (ShaderStage::Compute as u32);
        const STAGE_RAYTRACING =  1 << (ShaderStage::Raytracing as u32);

        const STAGE_ALL_GRAPHICS = (
            Self::STAGE_VERTEX.bits |
            Self::STAGE_TESSELLATION_CONTROL.bits |
            Self::STAGE_TESSELLATION_EVALUATION.bits |
            Self::STAGE_GEOMETRY.bits |
            Self::STAGE_FRAGMENT.bits
        );
    }
}

bitflags! {
    pub struct RootSignatureFlags: u8 {
        const NONE_FLAG = 0x0;
        const LOCAL_FLAG = 0x1;
    }
}


bitflags! {
    pub struct ResourceState: u32 {
        const VERTEX_AND_CONSTANT_BUFFER = 0x1;
        const INDEX_BUFFER = 0x2;
        const RENDER_TARGET = 0x4;
        const UNORDERED_ACCESS = 0x8;
        const DEPTH_WRITE = 0x10;
        const DEPTH_READ = 0x20;
        const NON_PIXEL_SHADER_RESOURCE = 0x40;
        const PIXEL_SHADER_RESOURCE = 0x80;
        const STREAM_OUT = 0x100;
        const INDIRECT_ARGUMENT = 0x200;
        const COPY_DEST = 0x400;
        const COPY_SOURCE = 0x800;
        const PRESENT = 0x1000;
        const COMMON = 0x2000;
        const RAYTRACING_ACCELERATION_STRUCTURE = 0x4000;
        const SHADING_RATE_SOURCE = 0x8000;

        const GENERIC_READ =
            Self::VERTEX_AND_CONSTANT_BUFFER.bits |
            Self::INDEX_BUFFER.bits |
            Self::NON_PIXEL_SHADER_RESOURCE.bits |
            Self::PIXEL_SHADER_RESOURCE.bits |
            Self::INDIRECT_ARGUMENT.bits |
            Self::COPY_SOURCE.bits;

        const SHADER_RESOURCE = Self::NON_PIXEL_SHADER_RESOURCE.bits | Self::PIXEL_SHADER_RESOURCE.bits;
    }
}

bitflags! {
    pub struct GPUSupportedFeatures: u32 {
        const NONE = 0x00;
        const MULTI_DRAW_INDIRECT = 0x01;
        const ROV_SUPPORTED = 0x02;
        const TESSELLATION_SUPPORTED = 0x04;
        const GEOMETRY_SHADER_SUPPORTED = 0x08;
        const GPU_BREADCRUMBS = 0x10;
        const HDR_SUPPORTED = 0x20;
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum ResourceMemoryUsage {
    /// No intended memory usage specified.
    Unknown,
    /// Memory will be used on device only, no need to be mapped on host.
    GpuOnly,
    /// Memory will be mapped on host. Could be used for transfer to device.
    CpuOnly,
    /// Memory will be used for frequent (dynamic) updates from host and reads on device.
    CpuToGpu,
    /// Memory will be used for writing on device and readback on host.
    GpuToCpu,
}

pub enum GPUPresetLevel {
    PresetNone = 0,
    PresetOffice, //This means unsupported
    PresetLow,
    PresetMedium,
    PresetHigh,
    PresetUltra,
    PresetCount,
}

#[derive(PartialEq, Copy, Clone)]
pub enum SampleCount {
    SampleCount1 = 1,
    SampleCount2 = 2,
    SampleCount4 = 4,
    SampleCount8 = 8,
    SampleCount16 = 16,
}

bitflags! {
     pub struct ShadingRates: u8 {
        const SHADING_RATE_NOT_SUPPORTED = 0x00;
        const SHADING_RATE_FULL = 0x01;
        const SHADING_RATE_HALF = 0x02;
        const SHADING_RATE_QUARTER = 0x04;
        const SHADING_RATE_EIGHTH = 0x08;
        const SHADING_RATE_1X2 = 0x10;
        const SHADING_RATE_2X1 = 0x20;
        const SHADING_RATE_2X4 = 0x40;
        const SHADING_RATE_4X2 = 0x80;
     }
}

bitflags! {
    pub struct QueueFlag: u32 {
         const QUEUE_FLAG_NONE = 0x0;
         const QUEUE_FLAG_DISABLE_GPU_TIMEOUT = 0x1;
         const QUEUE_FLAG_INIT_MICROPROFILE = 0x2;
         const MAX_QUEUE_FLAG = 0xFFFFFFFF;
    }
}

bitflags! {
    pub struct TextureCreationFlags: u32 {
        /// Default flag (Texture will use default allocation strategy decided by the api specific allocator)
        const TEXTURE_CREATION_FLAG_NONE = 0;
        /// Texture will allocate its own memory (COMMITTED resource)
        const TEXTURE_CREATION_FLAG_OWN_MEMORY_BIT = 0x01;
        /// Texture will be allocated in memory which can be shared among multiple processes
        const TEXTURE_CREATION_FLAG_EXPORT_BIT = 0x02;
        /// Texture will be allocated in memory which can be shared among multiple gpus
        const TEXTURE_CREATION_FLAG_EXPORT_ADAPTER_BIT = 0x04;
        /// Texture will be imported from a handle created in another process
        const TEXTURE_CREATION_FLAG_IMPORT_BIT = 0x08;
        /// Use ESRAM to store this texture
        const TEXTURE_CREATION_FLAG_ESRAM = 0x10;
        /// Use on-tile memory to store this texture
        const TEXTURE_CREATION_FLAG_ON_TILE = 0x20;
        /// Prevent compression meta data from generating (XBox)
        const TEXTURE_CREATION_FLAG_NO_COMPRESSION = 0x40;
        /// Force 2D instead of automatically determining dimension based on width, height, depth
        const TEXTURE_CREATION_FLAG_FORCE_2D = 0x80;
        /// Force 3D instead of automatically determining dimension based on width, height, depth
        const TEXTURE_CREATION_FLAG_FORCE_3D = 0x100;
        /// Display target
        const TEXTURE_CREATION_FLAG_ALLOW_DISPLAY_TARGET = 0x200;
        /// Create an sRGB texture.
        const TEXTURE_CREATION_FLAG_SRGB = 0x400;
        /// Create a normal map texture
        const TEXTURE_CREATION_FLAG_NORMAL_MAP = 0x800;
        /// Fast clear
        const TEXTURE_CREATION_FLAG_FAST_CLEAR = 0x1000;
        /// Fragment mask
        const TEXTURE_CREATION_FLAG_FRAG_MASK = 0x2000;
        /// Doubles the amount of array layers of the texture when rendering VR. Also forces the texture to be a 2D Array texture.
        const TEXTURE_CREATION_FLAG_VR_MULTIVIEW = 0x4000;
        /// Binds the FFR fragment density if this texture is used as a render target.
        const TEXTURE_CREATION_FLAG_VR_FOVEATED_RENDERING = 0x8000;
    }
}

bitflags! {
    pub struct BufferCreationFlag: u32 {
            /// Default flag (Buffer will use aliased memory, buffer will not be cpu accessible until mapBuffer is called)
            const BUFFER_CREATION_FLAG_NONE = 0x01;
            /// Buffer will allocate its own memory (COMMITTED resource)
            const BUFFER_CREATION_FLAG_OWN_MEMORY_BIT = 0x02;
            /// Buffer will be persistently mapped
            const BUFFER_CREATION_FLAG_PERSISTENT_MAP_BIT = 0x04;
            /// Use ESRAM to store this buffer
            const BUFFER_CREATION_FLAG_ESRAM = 0x08;
            /// Flag to specify not to allocate descriptors for the resource
            const BUFFER_CREATION_FLAG_NO_DESCRIPTOR_VIEW_CREATION = 0x10;

            // vulkan flags
            /* Memory Host Flags */
            const BUFFER_CREATION_FLAG_HOST_VISIBLE = 0x100;
            const BUFFER_CREATION_FLAG_HOST_COHERENT = 0x200;
    }
}

bitflags! {
    pub struct DescriptorType: u32 {
        const DESCRIPTOR_TYPE_UNDEFINED = 0;
        const DESCRIPTOR_TYPE_SAMPLER = 0x01;
        // SRV Read only texture
        const DESCRIPTOR_TYPE_TEXTURE = (Self::DESCRIPTOR_TYPE_SAMPLER.bits << 1);
        /// UAV Texture
        const DESCRIPTOR_TYPE_RW_TEXTURE = (Self::DESCRIPTOR_TYPE_TEXTURE.bits << 1);
        // SRV Read only buffer
        const DESCRIPTOR_TYPE_BUFFER = (Self::DESCRIPTOR_TYPE_RW_TEXTURE.bits << 1);
        const DESCRIPTOR_TYPE_BUFFER_RAW = (Self::DESCRIPTOR_TYPE_BUFFER.bits | (Self::DESCRIPTOR_TYPE_BUFFER.bits << 1));
        /// UAV Buffer
        const DESCRIPTOR_TYPE_RW_BUFFER = (Self::DESCRIPTOR_TYPE_BUFFER.bits << 2);
        const DESCRIPTOR_TYPE_RW_BUFFER_RAW = (Self::DESCRIPTOR_TYPE_RW_BUFFER.bits | (Self::DESCRIPTOR_TYPE_RW_BUFFER.bits << 1));
        /// Uniform buffer
        const DESCRIPTOR_TYPE_UNIFORM_BUFFER = (Self::DESCRIPTOR_TYPE_RW_BUFFER.bits << 2);
        /// Push constant / Root constant
        const DESCRIPTOR_TYPE_ROOT_CONSTANT = (Self::DESCRIPTOR_TYPE_UNIFORM_BUFFER.bits << 1);
        /// IA
        const DESCRIPTOR_TYPE_VERTEX_BUFFER = (Self::DESCRIPTOR_TYPE_ROOT_CONSTANT.bits << 1);
        const DESCRIPTOR_TYPE_INDEX_BUFFER = (Self::DESCRIPTOR_TYPE_VERTEX_BUFFER.bits << 1);
        const DESCRIPTOR_TYPE_INDIRECT_BUFFER = (Self::DESCRIPTOR_TYPE_INDEX_BUFFER.bits << 1);
        /// Cubemap SRV
        const DESCRIPTOR_TYPE_TEXTURE_CUBE = (Self::DESCRIPTOR_TYPE_TEXTURE.bits | (Self::DESCRIPTOR_TYPE_INDIRECT_BUFFER.bits << 1));
        /// RTV / DSV per mip slice
        const DESCRIPTOR_TYPE_RENDER_TARGET_MIP_SLICES = (Self::DESCRIPTOR_TYPE_INDIRECT_BUFFER.bits << 2);
        /// RTV / DSV per array slice
        const DESCRIPTOR_TYPE_RENDER_TARGET_ARRAY_SLICES = (Self::DESCRIPTOR_TYPE_RENDER_TARGET_MIP_SLICES.bits << 1);
        /// RTV / DSV per depth slice
        const DESCRIPTOR_TYPE_RENDER_TARGET_DEPTH_SLICES = (Self::DESCRIPTOR_TYPE_RENDER_TARGET_ARRAY_SLICES.bits << 1);
        const DESCRIPTOR_TYPE_RAY_TRACING = (Self::DESCRIPTOR_TYPE_RENDER_TARGET_DEPTH_SLICES.bits << 1);

        // -------------------------------------------------------------------------------------
        // VULKAN API
        // -------------------------------------------------------------------------------------
        /// Subpass input (descriptor type only available in Vulkan)
        const DESCRIPTOR_TYPE_INPUT_ATTACHMENT = (Self::DESCRIPTOR_TYPE_RAY_TRACING.bits << 1);
        const DESCRIPTOR_TYPE_TEXEL_BUFFER = (Self::DESCRIPTOR_TYPE_INPUT_ATTACHMENT.bits << 1);
        const DESCRIPTOR_TYPE_RW_TEXEL_BUFFER = (Self::DESCRIPTOR_TYPE_TEXEL_BUFFER.bits << 1);
        const DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER = (Self::DESCRIPTOR_TYPE_RW_TEXEL_BUFFER.bits << 1);

        /// Khronos extension ray tracing
        const DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE = (Self::DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER.bits << 1);
        const DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_BUILD_INPUT = (Self::DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE.bits << 1);
        const DESCRIPTOR_TYPE_SHADER_DEVICE_ADDRESS = (Self::DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_BUILD_INPUT.bits << 1);
        const DESCRIPTOR_TYPE_SHADER_BINDING_TABLE = (Self::DESCRIPTOR_TYPE_SHADER_DEVICE_ADDRESS.bits << 1);

        // -------------------------------------------------------------------------------------
        // METAL API
        // -------------------------------------------------------------------------------------
        // DESCRIPTOR_TYPE_ARGUMENT_BUFFER = (DESCRIPTOR_TYPE_RAY_TRACING << 1),
        // DESCRIPTOR_TYPE_INDIRECT_COMMAND_BUFFER = (DESCRIPTOR_TYPE_ARGUMENT_BUFFER << 1),
        // DESCRIPTOR_TYPE_RENDER_PIPELINE_STATE = (DESCRIPTOR_TYPE_INDIRECT_COMMAND_BUFFER << 1),
    }
}

#[derive(PartialEq)]
pub enum QueuePriority {
    QueuePriorityNormal,
    QueuePriorityHigh,
    QueuePriorityGlobalRealtime,
    MaxQueuePriority,
}
