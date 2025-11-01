use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    ops::Deref,
};

use bitflags::bitflags;
use bon::{Builder, builder};
use static_assertions::const_assert_eq;

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
const_assert_eq!(diligent_sys::RAYTRACING_BUILD_AS_FLAG_LAST, 16);

impl Default for RayTracingBuildAsFlags {
    fn default() -> Self {
        RayTracingBuildAsFlags::None
    }
}

#[repr(transparent)]
pub struct BLASTriangleDesc<'a>(diligent_sys::BLASTriangleDesc, PhantomData<&'a ()>);
#[bon::bon]
impl BLASTriangleDesc<'_> {
    #[builder]
    pub fn new(
        geometry_name: &CStr,

        max_vertex_count: usize,

        vertex_value_type: ValueType,

        vertex_component_count: u8,

        max_primitive_count: usize,

        index_type: ValueType,

        #[cfg(feature = "vulkan")]
        #[builder(default = false)]
        allows_transforms: bool,
    ) -> Self {
        Self(
            diligent_sys::BLASTriangleDesc {
                GeometryName: geometry_name.as_ptr(),
                MaxVertexCount: max_vertex_count as u32,
                VertexValueType: vertex_value_type.into(),
                VertexComponentCount: vertex_component_count,
                MaxPrimitiveCount: max_primitive_count as u32,
                IndexType: index_type.into(),
                #[cfg(feature = "vulkan")]
                AllowsTransforms: allows_transforms,
                #[cfg(not(feature = "vulkan"))]
                AllowsTransforms: false,
            },
            PhantomData,
        )
    }
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
    name: Option<CString>,

    #[builder(default = Vec::new())]
    #[builder(into)]
    triangles: Vec<BLASTriangleDesc<'a>>,

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
        let triangles = &value.triangles;

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
                Name: value
                    .name
                    .as_ref()
                    .map_or(std::ptr::null(), |name| name.as_ptr()),
            },
            pTriangles: if triangles.is_empty() {
                std::ptr::null()
            } else {
                triangles.as_ptr() as _
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
            _boxes: boxes,
            desc,
        }
    }
}

#[repr(transparent)]
pub struct BottomLevelAS(diligent_sys::IBottomLevelAS);

impl Deref for BottomLevelAS {
    type Target = DeviceObject;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::addr_of!(self.0) as *const diligent_sys::IDeviceObject
                as *const DeviceObject)
        }
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
    pub(crate) fn sys_ptr(&self) -> *mut diligent_sys::IBottomLevelAS {
        std::ptr::addr_of!(self.0) as _
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
