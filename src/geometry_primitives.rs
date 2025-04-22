use bitflags::bitflags;
use static_assertions::const_assert;

use crate::data_blob::DataBlob;

bitflags! {
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
const_assert!(diligent_sys::GEOMETRY_PRIMITIVE_VERTEX_FLAG_LAST == 4);

pub enum GeometryPrimitive {
    Cube { size: f32 },
    Sphere { radius: f32 },
}
const_assert!(diligent_sys::GEOMETRY_PRIMITIVE_TYPE_COUNT == 3);

pub struct GeometryPrimitiveAttributes {
    pub(crate) geometry_type: GeometryPrimitive,
    pub(crate) vertex_flags: GeometryPrimitiveVertexFlags,
    pub(crate) num_subdivisions: u32,
}

impl GeometryPrimitiveAttributes {
    pub fn new(geometry_type: GeometryPrimitive) -> Self {
        GeometryPrimitiveAttributes {
            geometry_type,
            vertex_flags: GeometryPrimitiveVertexFlags::All,
            num_subdivisions: 1,
        }
    }
    pub fn vertex_flags(mut self, vertex_flags: GeometryPrimitiveVertexFlags) -> Self {
        self.vertex_flags = vertex_flags;
        self
    }

    pub fn num_subdivisions(mut self, num_subdivisions: u32) -> Self {
        self.num_subdivisions = num_subdivisions;
        self
    }
}

pub struct GeometryPrimitiveInfo {
    pub num_vertices: u32,
    pub num_indices: u32,
    pub vertex_size: u32,
}

pub fn create_geometry_primitive(
    attribs: &GeometryPrimitiveAttributes,
) -> Result<(DataBlob, DataBlob, GeometryPrimitiveInfo), ()> {
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
                    NumSubdivisions: attribs.num_subdivisions,
                },
            })
        }
        GeometryPrimitive::Sphere { radius } => {
            GeometryPrimitiveType::Sphere(diligent_sys::SphereGeometryPrimitiveAttributes {
                Radius: radius,
                _GeometryPrimitiveAttributes: diligent_sys::GeometryPrimitiveAttributes {
                    Type: diligent_sys::GEOMETRY_PRIMITIVE_TYPE_SPHERE as _,
                    VertexFlags: attribs.vertex_flags.bits(),
                    NumSubdivisions: attribs.num_subdivisions,
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
            DataBlob::new(vertices),
            DataBlob::new(indices),
            GeometryPrimitiveInfo {
                num_indices: info.NumIndices,
                num_vertices: info.NumVertices,
                vertex_size: info.VertexSize,
            },
        ))
    }
}

pub fn get_geometry_primitive_vertex_size(vertex_flags: &GeometryPrimitiveVertexFlags) -> u32 {
    unsafe { diligent_sys::Diligent_GetGeometryPrimitiveVertexSize(vertex_flags.bits()) }
}
