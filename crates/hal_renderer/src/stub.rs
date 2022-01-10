use crate::{
    configuration::{PipelineConfig, RootSignatureConfig},
    renderer::{Pipeline, PipelineImpl, PipelineType, Renderer, RootSignature, RootSignatureImpl},
};
use std::mem::ManuallyDrop;

pub struct EmptyPipeline {}
pub struct EmptyRootSignature {}
pub struct EmptyShader {}
//
// pub fn empty_add_pipeline(render: &mut Renderer, configuration: &PipelineConfig) -> Pipeline {
//     // Pipeline {
//     //     imp: PipelineImpl {
//     //         empty: ManuallyDrop::new(EmptyPipeline {}),
//     //     },
//     // }
// }
//
// pub fn empty_drop_pipeline(render: &mut Renderer, pipeline: &mut Pipeline) {
//     unsafe {
//         ManuallyDrop::drop(&mut pipeline.imp.empty);
//     }
// }
//
// pub fn empty_add_root_signature(
//     render: &mut Renderer,
//     config: &RootSignatureConfig,
// ) -> RootSignature {
//     RootSignature {
//         pipeline_type: PipelineType::PipelineNone,
//         imp: RootSignatureImpl {
//             empty: ManuallyDrop::new(EmptyRootSignature {}),
//         },
//     }
// }
//
// pub fn empty_drop_root_signature(render: &mut Renderer, signature: &mut RootSignature) {}
