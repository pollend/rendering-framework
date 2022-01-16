use bitflags::bitflags;

#[derive(PartialEq)]
pub enum DescriptorUpdateFrequency {
    DescriptorUpdateFreqNone = 0,
    DescriptorUpdateFreqPerFrame,
    DescriptorUpdateFreqPerBatch,
    DescriptorUpdateFreqPerDraw,
    DescriptorUpdateFreqCount,
}

#[derive(PartialEq)]
pub enum QueueType {
    QueueTypeGraphics = 0,
    QueueTypeTransfer,
    QueueTypeCompute,
    MaxQueueType,
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

pub enum GPUPresetLevel {
    GPUPresetNone = 0,
    GPUPresetOffice,    //This means unsupported
    GPUPresetLow,
    GPUPresetMedium,
    GPUPresetHigh,
    GPUPresetUltra,
    GPUPresetCount
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


#[derive(PartialEq)]
pub enum QueuePriority {
    QueuePriorityNormal,
    QueuePriorityHigh,
    QueuePriorityGlobalRealtime,
    MaxQueuePriority,
}
