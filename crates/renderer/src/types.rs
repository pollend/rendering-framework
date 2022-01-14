
pub enum DescriptorUpdateFrequency {
    DescriptorUpdateFreqNone = 0,
    DescriptorUpdateFreqPerFrame,
    DescriptorUpdateFreqPerBatch,
    DescriptorUpdateFreqPerDraw,
    DescriptorUpdateFreqCount,
}


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

enum QueuePriority {
    QueuePriorityNormal,
    QueuePriorityHigh,
    QueuePriorityGlobalRealtime,
    MaxQueuePriority
}
