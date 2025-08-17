use std::{ffi::CString, ops::Deref};

use bitflags::bitflags;
use bon::Builder;
use static_assertions::{const_assert, const_assert_eq};

use crate::{
    device_object::DeviceObject,
    graphics_types::{ResourceState, ValueType},
};

bitflags! {
    #[derive(Clone,Copy)]
    pub struct RayTracingBuildAsFlags: diligent_sys::RAYTRACING_BUILD_AS_FLAGS {
        const None            = diligent_sys::RAYTRACING_BUILD_AS_NONE as _;
        const AllowUpdate     = diligent_sys::RAYTRACING_BUILD_AS_ALLOW_UPDATE as _;
        const AllowCompaction = diligent_sys::RAYTRACING_BUILD_AS_ALLOW_COMPACTION as _;
        const PreferFastTrace = diligent_sys::RAYTRACING_BUILD_AS_PREFER_FAST_TRACE as _;
        const PreferFastBuild = diligent_sys::RAYTRACING_BUILD_AS_PREFER_FAST_BUILD as _;
        const LowMemory       = diligent_sys::RAYTRACING_BUILD_AS_LOW_MEMORY as _;
        const FlagLast        = diligent_sys::RAYTRACING_BUILD_AS_FLAG_LAST as _;
    }
}
const_assert!(diligent_sys::RAYTRACING_BUILD_AS_FLAG_LAST == 16);

impl Default for RayTracingBuildAsFlags {
    fn default() -> Self {
        RayTracingBuildAsFlags::None
    }
}

#[derive(Builder)]
pub struct BLASTriangleDesc {
    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    geometry_name: CString,

    pub max_vertex_count: usize,

    pub vertex_value_type: ValueType,

    pub vertex_component_count: u8,

    pub max_primitive_count: usize,

    pub index_type: ValueType,

    #[cfg(feature = "vulkan")]
    #[builder(default = false)]
    pub allows_transforms: bool,
}

#[derive(Builder)]
pub struct BLASBoundingBoxDesc {
    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    geometry_name: CString,

    max_box_count: usize,
}

#[derive(Builder)]
pub struct BottomLevelASDesc<'a> {
    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    name: CString,

    #[builder(default = Vec::new())]
    #[builder(into)]
    triangles: Vec<&'a BLASTriangleDesc>,

    #[builder(default = Vec::new())]
    #[builder(into)]
    boxes: Vec<BLASBoundingBoxDesc>,

    #[builder(default)]
    flags: RayTracingBuildAsFlags,

    #[builder(default = 0)]
    compacted_size: u64,

    #[builder(default = 1)]
    immediate_context_mask: u64,
}

pub(crate) struct BottomLevelASDescWrapper {
    _triangles: Vec<diligent_sys::BLASTriangleDesc>,
    _boxes: Vec<diligent_sys::BLASBoundingBoxDesc>,
    desc: diligent_sys::BottomLevelASDesc,
}

impl Deref for BottomLevelASDescWrapper {
    type Target = diligent_sys::BottomLevelASDesc;
    fn deref(&self) -> &Self::Target {
        &self.desc
    }
}

impl From<&BottomLevelASDesc<'_>> for BottomLevelASDescWrapper {
    fn from(value: &BottomLevelASDesc) -> Self {
        let triangles = value
            .triangles
            .iter()
            .map(|triangle| diligent_sys::BLASTriangleDesc {
                #[cfg(feature = "vulkan")]
                AllowsTransforms: triangle.allows_transforms,
                #[cfg(not(feature = "vulkan"))]
                AllowsTransforms: false,
                GeometryName: triangle.geometry_name.as_ptr(),
                IndexType: triangle.index_type.into(),
                MaxPrimitiveCount: triangle.max_primitive_count as u32,
                MaxVertexCount: triangle.max_vertex_count as u32,
                VertexComponentCount: triangle.vertex_component_count,
                VertexValueType: triangle.vertex_value_type.into(),
            })
            .collect::<Vec<_>>();

        let boxes = value
            .boxes
            .iter()
            .map(|bx| diligent_sys::BLASBoundingBoxDesc {
                GeometryName: bx.geometry_name.as_ptr(),
                MaxBoxCount: bx.max_box_count as u32,
            })
            .collect::<Vec<_>>();

        let desc = diligent_sys::BottomLevelASDesc {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: value.name.as_ptr(),
            },
            pTriangles: if triangles.is_empty() {
                std::ptr::null()
            } else {
                triangles.as_ptr()
            },
            TriangleCount: triangles.len() as u32,
            pBoxes: if boxes.is_empty() {
                std::ptr::null()
            } else {
                boxes.as_ptr()
            },
            BoxCount: boxes.len() as u32,
            Flags: value.flags.bits(),
            CompactedSize: value.compacted_size,
            ImmediateContextMask: value.immediate_context_mask,
        };
        Self {
            _triangles: triangles,
            _boxes: boxes,
            desc,
        }
    }
}

#[repr(transparent)]
pub struct BottomLevelAS(DeviceObject);

impl Deref for BottomLevelAS {
    type Target = DeviceObject;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ScratchBufferSizes {
    pub build: u64,
    pub update: u64,
}

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IBottomLevelASMethods>(),
    7 * std::mem::size_of::<*const ()>()
);

impl BottomLevelAS {
    pub(crate) fn new(sys_ptr: *mut diligent_sys::IBottomLevelAS) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IDeviceObject>()
                == std::mem::size_of::<diligent_sys::IBottomLevelAS>()
        );

        Self(DeviceObject::new(
            sys_ptr as *mut diligent_sys::IDeviceObject,
        ))
    }

    pub fn get_geometry_desc_index(&self, name: impl AsRef<str>) -> u32 {
        let name = CString::new(name.as_ref()).unwrap();
        unsafe_member_call!(self, BottomLevelAS, GetGeometryDescIndex, name.as_ptr())
    }

    pub fn get_geometry_index(&self, name: impl AsRef<str>) -> u32 {
        let name = CString::new(name.as_ref()).unwrap();
        unsafe_member_call!(self, BottomLevelAS, GetGeometryIndex, name.as_ptr())
    }

    pub fn get_actual_geometry_count(&self) -> u32 {
        unsafe_member_call!(self, BottomLevelAS, GetActualGeometryCount)
    }

    pub fn get_scratch_buffer_sizes(&self) -> ScratchBufferSizes {
        let sbs = unsafe_member_call!(self, BottomLevelAS, GetScratchBufferSizes);

        ScratchBufferSizes {
            build: sbs.Build,
            update: sbs.Update,
        }
    }

    pub fn get_native_handle(&self) -> u64 {
        unsafe_member_call!(self, BottomLevelAS, GetNativeHandle)
    }

    pub fn set_state(&self, state: ResourceState) {
        unsafe_member_call!(self, BottomLevelAS, SetState, state.bits())
    }

    pub fn get_state(&self) -> ResourceState {
        ResourceState::from_bits_retain(unsafe_member_call!(self, BottomLevelAS, GetState))
    }
}
