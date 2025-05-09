use std::{ffi::CString, ops::Deref};

use bon::Builder;
use static_assertions::const_assert;

use crate::graphics_types::ValueType;

#[derive(Clone, Copy)]
pub enum InputElementFrequency {
    PerVertex,
    PerInstance,
}
const_assert!(diligent_sys::INPUT_ELEMENT_FREQUENCY_NUM_FREQUENCIES == 3);

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

#[derive(Builder)]
pub struct LayoutElement {
    slot: u32,

    num_components: u32,

    value_type: ValueType,

    #[builder(default = true)]
    is_normalized: bool,

    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    #[builder(default = c"ATTRIB".to_owned())]
    hlsl_semantic: CString,

    #[builder(default = diligent_sys::LAYOUT_ELEMENT_AUTO_OFFSET)]
    relative_offset: u32,

    #[builder(default = diligent_sys::LAYOUT_ELEMENT_AUTO_STRIDE)]
    stride: u32,

    #[builder(default = InputElementFrequency::PerVertex)]
    frequency: InputElementFrequency,

    #[builder(default = 1)]
    instance_data_step_rate: u32,
}

use layout_element_builder::{IsUnset, SetIsNormalized, SetNumComponents, SetValueType, State};
macro_rules! impl_layout_element_builder {
    ($func_name:ident, $value_type:expr, $num_components:expr, $normalized:expr) => {
        impl<S: State> LayoutElementBuilder<S> {
            pub fn $func_name(
                self,
            ) -> LayoutElementBuilder<SetIsNormalized<SetNumComponents<SetValueType<S>>>>
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

pub(crate) struct InputLayoutDescWrapper {
    elements: Vec<diligent_sys::LayoutElement>,
}

impl Deref for InputLayoutDescWrapper {
    type Target = Vec<diligent_sys::LayoutElement>;
    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

impl From<&Vec<LayoutElement>> for InputLayoutDescWrapper {
    fn from(value: &Vec<LayoutElement>) -> Self {
        InputLayoutDescWrapper {
            elements: value
                .iter()
                .enumerate()
                .map(|(index, element)| diligent_sys::LayoutElement {
                    InputIndex: index as u32,
                    HLSLSemantic: element.hlsl_semantic.as_ptr(),
                    BufferSlot: element.slot,
                    NumComponents: element.num_components,
                    ValueType: element.value_type.into(),
                    IsNormalized: element.is_normalized,
                    RelativeOffset: element.relative_offset,
                    Stride: element.stride,
                    Frequency: element.frequency.into(),
                    InstanceDataStepRate: element.instance_data_step_rate,
                })
                .collect(),
        }
    }
}
