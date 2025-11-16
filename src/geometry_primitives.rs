use bitflags::bitflags;
use bon::Builder;
use static_assertions::const_assert_eq;

use crate::{Boxed, data_blob::DataBlob};

bitflags! {
    #[derive(Clone, Copy)]
    pub struct GeometryPrimitiveVertexFlags: diligent_sys::GEOMETRY_PRIMITIVE_VERTEX_FLAGS {
        const None     = diligent_sys::GEOMETRY_PRIMITIVE_VERTEX_FLAG_NONE as _;
        const Position = diligent_sys::GEOMETRY_PRIMITIVE_VERTEX_FLAG_POSITION as _;
        const Normal   = diligent_sys::GEOMETRY_PRIMITIVE_VERTEX_FLAG_NORMAL as _;
        const TexCoord = diligent_sys::GEOMETRY_PRIMITIVE_VERTEX_FLAG_TEXCOORD as _;

        const All = diligent_sys::GEOMETRY_PRIMITIVE_VERTEX_FLAG_ALL as _;

        const PosNorm = diligent_sys::GEOMETRY_PRIMITIVE_VERTEX_FLAG_POS_NORM as _;
        const PosTex  = diligent_sys::GEOMETRY_PRIMITIVE_VERTEX_FLAG_POS_TEX as _;
    }
}
const_assert_eq!(diligent_sys::GEOMETRY_PRIMITIVE_VERTEX_FLAG_LAST, 4);

pub enum GeometryPrimitive {
    Cube { size: f32 },
    Sphere { radius: f32 },
}
const_assert_eq!(diligent_sys::GEOMETRY_PRIMITIVE_TYPE_COUNT, 3);

#[derive(Builder)]
pub struct GeometryPrimitiveAttributes {
    pub geometry_type: GeometryPrimitive,

    #[builder(default = GeometryPrimitiveVertexFlags::All)]
    pub vertex_flags: GeometryPrimitiveVertexFlags,

    #[builder(default = 1)]
    num_subdivisions: usize,
}

pub struct GeometryPrimitiveInfo {
    pub num_vertices: usize,
    pub num_indices: usize,
    pub vertex_size: usize,
}

pub fn create_geometry_primitive(
    attribs: &GeometryPrimitiveAttributes,
) -> Result<(Boxed<DataBlob>, Boxed<DataBlob>, GeometryPrimitiveInfo), ()> {
    enum GeometryPrimitiveType {
        Cube(diligent_sys::CubeGeometryPrimitiveAttributes),
        Sphere(diligent_sys::SphereGeometryPrimitiveAttributes),
    }

    let attribs = match attribs.geometry_type {
        GeometryPrimitive::Cube { size } => {
            GeometryPrimitiveType::Cube(diligent_sys::CubeGeometryPrimitiveAttributes {
                Size: size,
                _GeometryPrimitiveAttributes: diligent_sys::GeometryPrimitiveAttributes {
                    Type: diligent_sys::GEOMETRY_PRIMITIVE_TYPE_CUBE as _,
                    VertexFlags: attribs.vertex_flags.bits(),
                    NumSubdivisions: attribs.num_subdivisions as u32,
                },
            })
        }
        GeometryPrimitive::Sphere { radius } => {
            GeometryPrimitiveType::Sphere(diligent_sys::SphereGeometryPrimitiveAttributes {
                Radius: radius,
                _GeometryPrimitiveAttributes: diligent_sys::GeometryPrimitiveAttributes {
                    Type: diligent_sys::GEOMETRY_PRIMITIVE_TYPE_SPHERE as _,
                    VertexFlags: attribs.vertex_flags.bits(),
                    NumSubdivisions: attribs.num_subdivisions as u32,
                },
            })
        }
    };

    let mut vertices = std::ptr::null_mut();
    let mut indices = std::ptr::null_mut();

    let mut info = std::mem::MaybeUninit::<diligent_sys::GeometryPrimitiveInfo>::uninit();

    unsafe {
        diligent_sys::Diligent_CreateGeometryPrimitive(
            match attribs {
                GeometryPrimitiveType::Cube(attribs) => {
                    std::ptr::from_ref(&attribs._GeometryPrimitiveAttributes)
                }
                GeometryPrimitiveType::Sphere(attribs) => {
                    std::ptr::from_ref(&attribs._GeometryPrimitiveAttributes)
                }
            },
            std::ptr::addr_of_mut!(vertices),
            std::ptr::addr_of_mut!(indices),
            info.as_mut_ptr(),
        );
    }

    let info = unsafe { info.assume_init() };

    if vertices.is_null() || indices.is_null() {
        Err(())
    } else {
        Ok((
            Boxed::<DataBlob>::new(vertices as _),
            Boxed::<DataBlob>::new(indices as _),
            GeometryPrimitiveInfo {
                num_indices: info.NumIndices as usize,
                num_vertices: info.NumVertices as usize,
                vertex_size: info.VertexSize as usize,
            },
        ))
    }
}

pub fn get_geometry_primitive_vertex_size(vertex_flags: GeometryPrimitiveVertexFlags) -> u32 {
    unsafe { diligent_sys::Diligent_GetGeometryPrimitiveVertexSize(vertex_flags.bits()) }
}
