use static_assertions::const_assert;

use crate::bindings;

use super::graphics_types::ValueType;

pub enum InputElementFrequency {
    PerVertex,
    PerInstance,
}
const_assert!(bindings::INPUT_ELEMENT_FREQUENCY_NUM_FREQUENCIES == 3);

impl From<&InputElementFrequency> for bindings::INPUT_ELEMENT_FREQUENCY {
    fn from(value: &InputElementFrequency) -> Self {
        (match value {
            InputElementFrequency::PerVertex => bindings::INPUT_ELEMENT_FREQUENCY_PER_VERTEX,
            InputElementFrequency::PerInstance => bindings::INPUT_ELEMENT_FREQUENCY_PER_INSTANCE,
        }) as bindings::INPUT_ELEMENT_FREQUENCY
    }
}

pub struct LayoutElement<'a> {
    input_index: u32,
    buffer_slot: u32,
    num_components: u32,
    value_type: ValueType,

    hlsl_semantic: &'a std::ffi::CStr,
    is_normalized: bool,
    relative_offset: u32,
    stride: u32,
    frequency: InputElementFrequency,
    instance_data_step_rate: u32,
}

impl<'a> LayoutElement<'a> {
    pub fn new(
        input_index: u32,
        buffer_slot: u32,
        num_components: u32,
        value_type: ValueType,
    ) -> Self {
        LayoutElement {
            input_index,
            buffer_slot,
            num_components,
            value_type,

            hlsl_semantic: &c"ATTRIB",
            is_normalized: true,
            relative_offset: bindings::LAYOUT_ELEMENT_AUTO_OFFSET,
            stride: bindings::LAYOUT_ELEMENT_AUTO_STRIDE,
            frequency: InputElementFrequency::PerVertex,
            instance_data_step_rate: 1,
        }
    }

    pub fn hlsl_semantic(mut self, hlsl_semantic: &'a std::ffi::CStr) -> Self {
        self.hlsl_semantic = hlsl_semantic;
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

impl From<&LayoutElement<'_>> for bindings::LayoutElement {
    fn from(value: &LayoutElement) -> Self {
        bindings::LayoutElement {
            HLSLSemantic: value.hlsl_semantic.as_ptr(),
            InputIndex: value.input_index,
            BufferSlot: value.buffer_slot,
            NumComponents: value.num_components,
            ValueType: bindings::VALUE_TYPE::from(&value.value_type),
            IsNormalized: value.is_normalized,
            RelativeOffset: value.relative_offset,
            Stride: value.stride,
            Frequency: bindings::INPUT_ELEMENT_FREQUENCY::from(&value.frequency),
            InstanceDataStepRate: value.instance_data_step_rate,
        }
    }
}
