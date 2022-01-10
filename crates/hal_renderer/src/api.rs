use crate::{
    configuration::{PipelineConfig, RootSignatureConfig, SamplerConfig},
    renderer::{Pipeline, Renderer, RootSignature, Sampler},
};

// type AddPipeline = fn(render: &mut Renderer, config: &PipelineConfig) -> Pipeline;
// type RemovePipeline = fn(render: &mut Renderer, pipeline: &mut Pipeline);
//
// type AddRootSignature = fn(render: &mut Renderer, config: &RootSignatureConfig) -> RootSignature;
// type DropRootSignature = fn(render: &mut Renderer, signature: &mut RootSignature);
//
// type AddSampler = fn(render: &mut Renderer, config: &SamplerConfig);
// type RemoveSampler = fn(render: &mut Renderer, sampler: &Sampler);
//
// pub(crate) static mut ADD_PIPELINE: Option<AddPipeline> = None;
// pub(crate) static mut DROP_PIPELINE: Option<RemovePipeline> = None;
//
// pub(crate) static mut ADD_ROOT_SIGNATURE: Option<AddRootSignature> = None;
// pub(crate) static mut DROP_ROOT_SIGNATURE: Option<DropRootSignature> = None;
//
// pub(crate) static mut ADD_SAMPLER: Option<AddSampler> = None;
