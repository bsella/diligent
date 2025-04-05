use std::{ffi::CString, ops::Deref, str::FromStr};

use static_assertions::const_assert;

use super::graphics_types::ValueType;

#[derive(Clone, Copy)]
pub enum InputElementFrequency {
    PerVertex,
    PerInstance,
}
const_assert!(diligent_sys::INPUT_ELEMENT_FREQUENCY_NUM_FREQUENCIES == 3);

impl From<&InputElementFrequency> for diligent_sys::INPUT_ELEMENT_FREQUENCY {
    fn from(value: &InputElementFrequency) -> Self {
        (match value {
            InputElementFrequency::PerVertex => diligent_sys::INPUT_ELEMENT_FREQUENCY_PER_VERTEX,
            InputElementFrequency::PerInstance => {
                diligent_sys::INPUT_ELEMENT_FREQUENCY_PER_INSTANCE
            }
        }) as diligent_sys::INPUT_ELEMENT_FREQUENCY
    }
}

#[derive(Clone)]
pub struct LayoutElement {
    buffer_slot: u32,
    num_components: u32,
    value_type: ValueType,

    hlsl_semantic: CString,
    is_normalized: bool,
    relative_offset: u32,
    stride: u32,
    frequency: InputElementFrequency,
    instance_data_step_rate: u32,
}

impl LayoutElement {
    pub fn new(buffer_slot: u32, num_components: u32, value_type: ValueType) -> Self {
        LayoutElement {
            buffer_slot,
            num_components,
            value_type,

            hlsl_semantic: c"ATTRIB".to_owned(),
            is_normalized: true,
            relative_offset: diligent_sys::LAYOUT_ELEMENT_AUTO_OFFSET,
            stride: diligent_sys::LAYOUT_ELEMENT_AUTO_STRIDE,
            frequency: InputElementFrequency::PerVertex,
            instance_data_step_rate: 1,
        }
    }

    pub fn hlsl_semantic(mut self, hlsl_semantic: impl AsRef<str>) -> Self {
        self.hlsl_semantic = CString::from_str(hlsl_semantic.as_ref()).unwrap();
        self
    }
    pub fn is_normalized(mut self, is_normalized: bool) -> Self {
        self.is_normalized = is_normalized;
        self
    }
    pub fn relative_offset(mut self, relative_offset: u32) -> Self {
        self.relative_offset = relative_offset;
        self
    }
    pub fn stride(mut self, stride: u32) -> Self {
        self.stride = stride;
        self
    }
    pub fn frequency(mut self, frequency: InputElementFrequency) -> Self {
        self.frequency = frequency;
        self
    }
    pub fn instance_data_step_rate(mut self, instance_data_step_rate: u32) -> Self {
        self.instance_data_step_rate = instance_data_step_rate;
        self
    }
}

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
                    BufferSlot: element.buffer_slot,
                    NumComponents: element.num_components,
                    ValueType: diligent_sys::VALUE_TYPE::from(&element.value_type),
                    IsNormalized: element.is_normalized,
                    RelativeOffset: element.relative_offset,
                    Stride: element.stride,
                    Frequency: diligent_sys::INPUT_ELEMENT_FREQUENCY::from(&element.frequency),
                    InstanceDataStepRate: element.instance_data_step_rate,
                })
                .collect(),
        }
    }
}
