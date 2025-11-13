use std::ffi::CStr;

use static_assertions::const_assert_eq;

use crate::graphics_types::ValueType;

#[derive(Clone, Copy)]
pub enum InputElementFrequency {
    PerVertex,
    PerInstance,
}
const_assert_eq!(diligent_sys::INPUT_ELEMENT_FREQUENCY_NUM_FREQUENCIES, 3);

impl From<InputElementFrequency> for diligent_sys::INPUT_ELEMENT_FREQUENCY {
    fn from(value: InputElementFrequency) -> Self {
        (match value {
            InputElementFrequency::PerVertex => diligent_sys::INPUT_ELEMENT_FREQUENCY_PER_VERTEX,
            InputElementFrequency::PerInstance => {
                diligent_sys::INPUT_ELEMENT_FREQUENCY_PER_INSTANCE
            }
        }) as _
    }
}

#[repr(transparent)]
pub struct LayoutElement(pub(crate) diligent_sys::LayoutElement);

#[bon::bon]
impl LayoutElement {
    #[builder]
    pub fn new(
        input_index: u32,
        slot: u32,

        num_components: u32,

        value_type: ValueType,

        #[builder(default = true)] is_normalized: bool,

        #[builder(default = c"ATTRIB")] hlsl_semantic: &CStr,

        #[builder(default = diligent_sys::LAYOUT_ELEMENT_AUTO_OFFSET)] relative_offset: u32,

        #[builder(default = diligent_sys::LAYOUT_ELEMENT_AUTO_STRIDE)] stride: u32,

        #[builder(default = InputElementFrequency::PerVertex)] frequency: InputElementFrequency,

        #[builder(default = 1)] instance_data_step_rate: u32,
    ) -> Self {
        LayoutElement(diligent_sys::LayoutElement {
            InputIndex: input_index,
            HLSLSemantic: hlsl_semantic.as_ptr(),
            BufferSlot: slot,
            NumComponents: num_components,
            ValueType: value_type.into(),
            IsNormalized: is_normalized,
            RelativeOffset: relative_offset,
            Stride: stride,
            Frequency: frequency.into(),
            InstanceDataStepRate: instance_data_step_rate,
        })
    }
}

use layout_element_builder::{IsUnset, SetIsNormalized, SetNumComponents, SetValueType, State};
macro_rules! impl_layout_element_builder {
    ($func_name:ident, $value_type:expr, $num_components:expr, $normalized:expr) => {
        impl<'a, S: State> LayoutElementBuilder<'a, S> {
            pub fn $func_name(
                self,
            ) -> LayoutElementBuilder<'a, SetIsNormalized<SetNumComponents<SetValueType<S>>>>
            where
                S::IsNormalized: IsUnset,
                S::NumComponents: IsUnset,
                S::ValueType: IsUnset,
            {
                self.value_type($value_type)
                    .num_components($num_components)
                    .is_normalized($normalized)
            }
        }
    };
}

impl_layout_element_builder!(i8, ValueType::Int8, 1, true);
impl_layout_element_builder!(i8_2, ValueType::Int8, 2, true);
impl_layout_element_builder!(i8_3, ValueType::Int8, 3, true);
impl_layout_element_builder!(i8_4, ValueType::Int8, 4, true);

impl_layout_element_builder!(i32, ValueType::Int32, 1, true);
impl_layout_element_builder!(i32_2, ValueType::Int32, 2, true);
impl_layout_element_builder!(i32_3, ValueType::Int32, 3, true);
impl_layout_element_builder!(i32_4, ValueType::Int32, 4, true);

impl_layout_element_builder!(u8, ValueType::Uint8, 1, true);
impl_layout_element_builder!(u8_2, ValueType::Uint8, 2, true);
impl_layout_element_builder!(u8_3, ValueType::Uint8, 3, true);
impl_layout_element_builder!(u8_4, ValueType::Uint8, 4, true);

impl_layout_element_builder!(u32, ValueType::Uint32, 1, true);
impl_layout_element_builder!(u32_2, ValueType::Uint32, 2, true);
impl_layout_element_builder!(u32_3, ValueType::Uint32, 3, true);
impl_layout_element_builder!(u32_4, ValueType::Uint32, 4, true);

impl_layout_element_builder!(f32, ValueType::Float32, 1, false);
impl_layout_element_builder!(f32_2, ValueType::Float32, 2, false);
impl_layout_element_builder!(f32_3, ValueType::Float32, 3, false);
impl_layout_element_builder!(f32_4, ValueType::Float32, 4, false);

#[repr(transparent)]
pub struct InputLayoutDesc(diligent_sys::InputLayoutDesc);

#[bon::bon]
impl InputLayoutDesc {
    #[builder]
    pub fn new(elements: &[LayoutElement]) -> Self {
        InputLayoutDesc(diligent_sys::InputLayoutDesc {
            LayoutElements: if elements.is_empty() {
                std::ptr::null()
            } else {
                elements.as_ptr() as _
            },
            NumElements: elements.len() as u32,
        })
    }
}
