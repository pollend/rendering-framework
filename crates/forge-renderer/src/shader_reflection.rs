use std::ffi::CString;
use std::marker::PhantomData;
use spirv_cross::{ErrorCode, glsl, spirv};
use spirv_cross::spirv::{Ast, Compile, Dim, Parse, Resource, ShaderResources, Target, Type};
use crate::shader_reflection::ResourceType::{TypeStorageTexelBuffers, TypeUniformTexelBuffers};


#[derive(PartialEq, Copy, Clone)]
pub enum ResourceType  {
    TypeStageInputs = 0,
    TypeStageOutputs,
    TypeUniformBuffers,
    TypeStorageBuffers,
    TypeImages,
    TypeStorageImages,
    TypeSamplers,
    TypePushConstant,
    TypeSubpassInputs,
    TypeUniformTexelBuffers,
    TypeStorageTexelBuffers,
    TypeAccelerationStructures,
    TypeCombinedSamplers,
    TypeCount
}

#[derive(PartialEq, Copy, Clone)]
pub enum ResourceDim
{
    DimUndefined,
    DimBuffer,
    DimTexture1D,
    DimTexture1DArray,
    DimTexture2D,
    DimTexture2DArray,
    DimTexture2DMS,
    DimTexture2DMSArray,
    DimTexture3D,
    DimTextureCube,
    DimTextureCubeArray,
    DimCount,
}



pub struct SprivResource {
    pub name: String,

    pub code: Resource,

    pub resource_type: ResourceType,

    pub resource_dim: ResourceDim,

    pub set: u32,

    pub binding: u32,

    pub array_size: Vec<u32>,

    pub is_used: bool
}

pub struct ShaderReflection {
    pub resources: Vec<SprivResource>

}


pub fn get_shader_resource<TTarget>(ast: &Ast<TTarget>) -> Result<ShaderReflection, ErrorCode> where
    TTarget: Target,
    Ast<TTarget>: Parse<TTarget> + Compile<TTarget>
{
    let mut shader_reflection = ShaderReflection {
        resources: vec![]
    };

    let mut reflect_bound_resource = |reflection: &mut ShaderReflection, ast: &mut Ast<TTarget>, resources: &Vec<Resource>, resource_type: ResourceType|-> Result<(), ErrorCode> {

        for resource in resources {
            let spirv_type = ast.get_type(resource.type_id)?;
            let mut resource_name = ast.get_name(resource.id)?;


            let mut resource = SprivResource {
                name: resource_name,
                code: resource.clone(),
                resource_type: {
                    match &spirv_type {
                        Type::Image { image, ..} | Type::SampledImage { image, ..} => {
                            match image.dim {
                                Dim::DimBuffer => {
                                    match resource_type {
                                        ResourceType::TypeImages => {
                                            TypeUniformTexelBuffers
                                        },
                                        ResourceType::TypeStorageImages => {
                                            TypeStorageTexelBuffers
                                        }
                                        _ => resource_type
                                    }
                                }
                                _ => resource_type
                            }
                        }
                        _ => resource_type
                    }
                },
                resource_dim: {
                    match &spirv_type {
                        Type::Image  { image, ..} | Type::SampledImage { image , ..} => {
                            match image.dim  {
                                Dim::DimBuffer => ResourceDim::DimBuffer,
                                Dim::Dim1D => {
                                    if image.arrayed { ResourceDim::DimTexture1DArray} else { ResourceDim::DimTexture1D }
                                }
                                Dim::Dim2D => {
                                    if image.ms {
                                        if image.arrayed { ResourceDim::DimTexture2DMSArray} else { ResourceDim::DimTexture2DMS }
                                    } else {
                                        if image.arrayed { ResourceDim::DimTexture2DArray } else { ResourceDim::DimTexture1D }
                                    }
                                },
                                Dim::Dim3D => ResourceDim::DimTexture3D,
                                Dim::DimCube => if image.arrayed {ResourceDim::DimTextureCubeArray} else {ResourceDim::DimTextureCube}
                                _ => ResourceDim::DimUndefined
                            }
                        }
                        _ => ResourceDim::DimUndefined
                    }
                },
                set: 0,
                binding: 0,
                array_size: match &spirv_type {
                    Type::Boolean {array, ..} => array.clone(),
                    Type::Char {array, ..}  => array.clone(),
                    Type::Int {array, ..}  => array.clone(),
                    Type::UInt {array, ..}  => array.clone(),
                    Type::Int64 {array, ..}  => array.clone(),
                    Type::UInt64 {array, ..}  => array.clone(),
                    Type::AtomicCounter {array, ..}  => array.clone(),
                    Type::Half {array, ..}  => array.clone(),
                    Type::Float {array, ..}  => array.clone(),
                    Type::Double {array, ..}  => array.clone(),
                    Type::Struct {array, ..}  => array.clone(),
                    Type::Image {array, ..}  => array.clone(),
                    Type::SampledImage {array, ..}  => array.clone(),
                    Type::Sampler {array, ..}  => array.clone(),
                    Type::SByte {array, ..}  => array.clone(),
                    Type::UByte {array, ..}  => array.clone(),
                    Type::Short {array, ..}  => array.clone(),
                    Type::UShort {array, ..}  => array.clone(),
                    _ => vec![]
                },
                is_used: false
            };
        }
        Ok(())
    };



    let mut resource = ast.get_shader_resources()?;
    Ok(shader_reflection)
}



// impl <T> Ast<T> where
//     Self: Parse<TTarget> + Compile<TTarget>,
//     TTarget: Target,
// {
//
// }



// pub struct ReflectionUtil {
// }

// impl <T> ReflectionUtil {
//     pub fn get_shader_resource<T>(ast: &mut Ast<T>) -> Result<ShaderReflection, ErrorCode> {
//         let mut resource = ast.get_shader_resources()?;
//
//         Ok(ShaderReflection{})
//     }
//
// }

// pub fn get_shader_resource<T>(ast: &mut Ast<T>) -> Result<ShaderReflection, ErrorCode> where
//     Self: Parse<TTarget> + Compile<TTarget>,
//     T: Target {
//     let mut resource = ast.get_shader_resources()?;
//
//     Ok(ShaderReflection {})
// }

