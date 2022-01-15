
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
    MaxQueueType
}

pub enum QueueFlag
{
    QueueFlagNone = 0x0,
    QueueFlagDisableGpuTimeout = 0x1,
    QueueFlagInitMicroprofile = 0x2,
    MaxQueueFlag = 0xFFFFFFFF
}

pub enum QueuePriority {
    QueuePriorityNormal,
    QueuePriorityHigh,
    QueuePriorityGlobalRealtime,
    MaxQueuePriority
}
