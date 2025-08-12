use std::{ffi::CString, ops::Deref};

use bitflags::bitflags;
use bon::Builder;
use static_assertions::{const_assert, const_assert_eq};

use crate::{
    blas::{BottomLevelAS, RayTracingBuildAsFlags, ScratchBufferSizes},
    device_object::DeviceObject,
    graphics_types::ResourceState,
};

pub const TLAS_INSTANCE_DATA_SIZE: u32 = diligent_sys::DILIGENT_TLAS_INSTANCE_DATA_SIZE;

#[derive(Clone, Copy)]
pub enum HitGroupBindingMode {
    PerGeometry,
    PerInstance,
    PerTLAS,
    UserDefined,
}
const_assert!(diligent_sys::HIT_GROUP_BINDING_MODE_LAST == 3);

impl From<HitGroupBindingMode> for diligent_sys::HIT_GROUP_BINDING_MODE {
    fn from(value: HitGroupBindingMode) -> Self {
        (match value {
            HitGroupBindingMode::PerGeometry => diligent_sys::HIT_GROUP_BINDING_MODE_PER_GEOMETRY,
            HitGroupBindingMode::PerInstance => diligent_sys::HIT_GROUP_BINDING_MODE_PER_INSTANCE,
            HitGroupBindingMode::PerTLAS => diligent_sys::HIT_GROUP_BINDING_MODE_PER_TLAS,
            HitGroupBindingMode::UserDefined => diligent_sys::HIT_GROUP_BINDING_MODE_USER_DEFINED,
        }) as _
    }
}

bitflags! {
    #[derive(Clone,Copy)]
    pub struct RayTracingInstanceFlags: diligent_sys::RAYTRACING_INSTANCE_FLAGS {
        const None                          = diligent_sys::RAYTRACING_INSTANCE_NONE as _;
        const TriangleFacingCullDisable     = diligent_sys::RAYTRACING_INSTANCE_TRIANGLE_FACING_CULL_DISABLE as _;
        const TriangleFrontCounterclockwise = diligent_sys::RAYTRACING_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE as _;
        const ForceOpaque                   = diligent_sys::RAYTRACING_INSTANCE_FORCE_OPAQUE as _;
        const ForceNoOpaque                 = diligent_sys::RAYTRACING_INSTANCE_FORCE_NO_OPAQUE as _;
    }
}
const_assert!(diligent_sys::RAYTRACING_INSTANCE_FLAG_LAST == 8);

impl Default for RayTracingInstanceFlags {
    fn default() -> Self {
        RayTracingInstanceFlags::None
    }
}

#[derive(Builder)]
pub struct TLASBuildInstanceData<'a> {
    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    instance_name: CString,

    blas: &'a BottomLevelAS,

    transform: [f32; 4 * 3],

    #[builder(default = 0)]
    custom_id: u32,

    #[builder(default)]
    flags: RayTracingInstanceFlags,

    #[builder(default = 0xff)]
    mask: u8,

    #[builder(default = diligent_sys::TLAS_INSTANCE_OFFSET_AUTO)]
    contribution_to_hit_group_index: u32,
}

impl<'a> From<&TLASBuildInstanceData<'a>> for diligent_sys::TLASBuildInstanceData {
    fn from(value: &TLASBuildInstanceData<'a>) -> Self {
        Self {
            InstanceName: value.instance_name.as_ptr(),
            pBLAS: value.blas.sys_ptr as _,
            Transform: diligent_sys::InstanceMatrix {
                data: [
                    [
                        value.transform[0],
                        value.transform[1],
                        value.transform[2],
                        value.transform[3],
                    ],
                    [
                        value.transform[4],
                        value.transform[5],
                        value.transform[6],
                        value.transform[7],
                    ],
                    [
                        value.transform[8],
                        value.transform[9],
                        value.transform[10],
                        value.transform[11],
                    ],
                ],
            },
            CustomId: value.custom_id,
            Flags: value.flags.bits(),
            Mask: value.mask,
            ContributionToHitGroupIndex: value.contribution_to_hit_group_index,
        }
    }
}

#[derive(Builder)]
pub struct TopLevelASDesc {
    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    name: CString,

    #[builder(default = 0)]
    max_instance_count: u32,

    #[builder(default)]
    flags: RayTracingBuildAsFlags,

    #[builder(default = 0)]
    compacted_size: u64,

    #[builder(default = 1)]
    immediate_context_mask: u64,
}

impl From<&TopLevelASDesc> for diligent_sys::TopLevelASDesc {
    fn from(value: &TopLevelASDesc) -> Self {
        Self {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: value.name.as_ptr(),
            },
            MaxInstanceCount: value.max_instance_count,
            Flags: value.flags.bits(),
            CompactedSize: value.compacted_size,
            ImmediateContextMask: value.immediate_context_mask,
        }
    }
}

const_assert_eq!(
    std::mem::size_of::<diligent_sys::ITopLevelASMethods>(),
    6 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct TopLevelAS {
    device_object: DeviceObject,
}

impl Deref for TopLevelAS {
    type Target = DeviceObject;
    fn deref(&self) -> &Self::Target {
        &self.device_object
    }
}

impl TopLevelAS {
    pub(crate) fn new(sys_ptr: *mut diligent_sys::ITopLevelAS) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IDeviceObject>()
                == std::mem::size_of::<diligent_sys::ITopLevelAS>()
        );

        Self {
            device_object: DeviceObject::new(sys_ptr as *mut diligent_sys::IDeviceObject),
        }
    }

    // TODO pub fn get_instance_desc(&self, name: impl AsRef<str>) -> TLASInstanceDesc {}
    // TODO pub fn get_build_info(&self) -> TLASBuildInfo {}

    pub fn get_scratch_buffer_sizes(&self) -> ScratchBufferSizes {
        let sbs = unsafe_member_call!(self, TopLevelAS, GetScratchBufferSizes,);

        ScratchBufferSizes {
            build: sbs.Build,
            update: sbs.Update,
        }
    }

    pub fn get_native_handle(&self) -> u64 {
        unsafe_member_call!(self, TopLevelAS, GetNativeHandle,)
    }

    pub fn set_state(&self, state: ResourceState) {
        unsafe_member_call!(self, TopLevelAS, SetState, state.bits())
    }

    pub fn get_state(&self) -> ResourceState {
        ResourceState::from_bits_retain(unsafe_member_call!(self, TopLevelAS, GetState,))
    }
}
