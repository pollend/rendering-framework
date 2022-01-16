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
        const MultiDrawIndirect = 0x01;
	    const ROVsSupported = 0x02;
	    const TessellationSupported = 0x04;
	    const GeometryShaderSupported = 0x08;
	    const GpuBreadcrumbs = 0x10;
	    const HDRSupported = 0x20;
    }
}

bitflags! {
     pub struct ShadingRates: u8 {
        const ShadingRateNotSupported = 0x00;
        const ShadingRateFull = 0x01;
        const ShadingRateHalf = 0x02;
        const ShadingRateQuarter = 0x04;
        const ShadingRateEighth = 0x08;
        const ShadingRate1x2 = 0x10;
        const ShadingRate2x1 = 0x20;
        const ShadingRate2x4 = 0x40;
        const ShadingRate4x2 = 0x80;
     }
}


bitflags! {
    pub struct QueueFlag: u32 {
         const QueueFlagNone = 0x0;
         const QueueFlagDisableGpuTimeout = 0x1;
         const QueueFlagInitMicroprofile = 0x2;
         const MaxQueueFlag = 0xFFFFFFFF;
    }
}


#[derive(PartialEq)]
pub enum QueuePriority {
    QueuePriorityNormal,
    QueuePriorityHigh,
    QueuePriorityGlobalRealtime,
    MaxQueuePriority,
}
