use std::{ffi::CString, ops::Deref};

use bitflags::bitflags;
use bon::Builder;
use static_assertions::const_assert;

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

pub struct BottomLevelAS {
    pub(crate) sys_ptr: *mut diligent_sys::IBottomLevelAS,
    virtual_functions: *mut diligent_sys::IBottomLevelASVtbl,

    device_object: DeviceObject,
}

impl AsRef<DeviceObject> for BottomLevelAS {
    fn as_ref(&self) -> &DeviceObject {
        &self.device_object
    }
}

pub struct ScratchBufferSizes {
    pub build: u64,
    pub update: u64,
}

impl BottomLevelAS {
    pub(crate) fn new(sys_ptr: *mut diligent_sys::IBottomLevelAS) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IDeviceObject>()
                == std::mem::size_of::<diligent_sys::IBottomLevelAS>()
        );

        Self {
            sys_ptr,
            virtual_functions: unsafe { (*sys_ptr).pVtbl },
            device_object: DeviceObject::new(sys_ptr as *mut diligent_sys::IDeviceObject),
        }
    }

    pub fn get_geometry_desc_index(&self, name: impl AsRef<str>) -> u32 {
        let name = CString::new(name.as_ref()).unwrap();
        unsafe {
            (*self.virtual_functions)
                .BottomLevelAS
                .GetGeometryDescIndex
                .unwrap_unchecked()(self.sys_ptr, name.as_ptr())
        }
    }

    pub fn get_geometry_index(&self, name: impl AsRef<str>) -> u32 {
        let name = CString::new(name.as_ref()).unwrap();
        unsafe {
            (*self.virtual_functions)
                .BottomLevelAS
                .GetGeometryIndex
                .unwrap_unchecked()(self.sys_ptr, name.as_ptr())
        }
    }

    pub fn get_actual_geometry_count(&self) -> u32 {
        unsafe {
            (*self.virtual_functions)
                .BottomLevelAS
                .GetActualGeometryCount
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn get_scratch_buffer_sizes(&self) -> ScratchBufferSizes {
        let sbs = unsafe {
            (*self.virtual_functions)
                .BottomLevelAS
                .GetScratchBufferSizes
                .unwrap_unchecked()(self.sys_ptr)
        };
        ScratchBufferSizes {
            build: sbs.Build,
            update: sbs.Update,
        }
    }

    pub fn get_native_handle(&self) -> u64 {
        unsafe {
            (*self.virtual_functions)
                .BottomLevelAS
                .GetNativeHandle
                .unwrap_unchecked()(self.sys_ptr)
        }
    }
    pub fn set_state(&self, state: ResourceState) {
        unsafe {
            (*self.virtual_functions)
                .BottomLevelAS
                .SetState
                .unwrap_unchecked()(self.sys_ptr, state.bits())
        }
    }

    pub fn get_state(&self) -> ResourceState {
        ResourceState::from_bits_retain(unsafe {
            (*self.virtual_functions)
                .BottomLevelAS
                .GetState
                .unwrap_unchecked()(self.sys_ptr)
        })
    }
}
