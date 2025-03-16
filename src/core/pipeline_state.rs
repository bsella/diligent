use std::ffi::CString;
use std::str::FromStr;

use bitflags::bitflags;
use static_assertions::const_assert;

use super::device_object::{AsDeviceObject, DeviceObject};
use super::graphics_types::{PrimitiveTopology, ShaderType, ShaderTypes};
use super::input_layout::LayoutElement;
use super::object::AsObject;
use super::pipeline_resource_signature::{ImmutableSamplerDesc, PipelineResourceSignature};
use super::resource_mapping::ResourceMapping;
use super::shader::Shader;
use super::shader_resource_binding::ShaderResourceBinding;
use super::shader_resource_variable::{
    ShaderResourceVariable, ShaderResourceVariableDesc, ShaderResourceVariableType,
};

pub enum BlendFactor {
    Zero,
    One,
    SrcColor,
    InvSrcColor,
    SrcAlpha,
    InvSrcAlpha,
    DestAlpha,
    InvDestAlpha,
    DestColor,
    InvDestColor,
    SrcAlphaSat,
    BlendFactor,
    InvBlendFactor,
    Src1Color,
    InvSrc1Color,
    Src1Alpha,
    InvSrc1Alpha,
}
const_assert!(diligent_sys::BLEND_OPERATION_NUM_OPERATIONS == 6);

impl From<&BlendFactor> for diligent_sys::BLEND_FACTOR {
    fn from(value: &BlendFactor) -> Self {
        (match value {
            BlendFactor::Zero => diligent_sys::BLEND_FACTOR_ZERO,
            BlendFactor::One => diligent_sys::BLEND_FACTOR_ONE,
            BlendFactor::SrcColor => diligent_sys::BLEND_FACTOR_SRC_COLOR,
            BlendFactor::InvSrcColor => diligent_sys::BLEND_FACTOR_INV_SRC_COLOR,
            BlendFactor::SrcAlpha => diligent_sys::BLEND_FACTOR_SRC_ALPHA,
            BlendFactor::InvSrcAlpha => diligent_sys::BLEND_FACTOR_INV_SRC_ALPHA,
            BlendFactor::DestAlpha => diligent_sys::BLEND_FACTOR_DEST_ALPHA,
            BlendFactor::InvDestAlpha => diligent_sys::BLEND_FACTOR_INV_DEST_ALPHA,
            BlendFactor::DestColor => diligent_sys::BLEND_FACTOR_DEST_COLOR,
            BlendFactor::InvDestColor => diligent_sys::BLEND_FACTOR_INV_DEST_COLOR,
            BlendFactor::SrcAlphaSat => diligent_sys::BLEND_FACTOR_SRC_ALPHA_SAT,
            BlendFactor::BlendFactor => diligent_sys::BLEND_FACTOR_BLEND_FACTOR,
            BlendFactor::InvBlendFactor => diligent_sys::BLEND_FACTOR_INV_BLEND_FACTOR,
            BlendFactor::Src1Color => diligent_sys::BLEND_FACTOR_SRC1_COLOR,
            BlendFactor::InvSrc1Color => diligent_sys::BLEND_FACTOR_INV_SRC1_COLOR,
            BlendFactor::Src1Alpha => diligent_sys::BLEND_FACTOR_SRC1_ALPHA,
            BlendFactor::InvSrc1Alpha => diligent_sys::BLEND_FACTOR_INV_SRC1_ALPHA,
        }) as diligent_sys::BLEND_FACTOR
    }
}

pub enum BlendOperation {
    Add,
    Subtract,
    RevSubtract,
    Min,
    Max,
}
const_assert!(diligent_sys::BLEND_OPERATION_NUM_OPERATIONS == 6);

impl From<&BlendOperation> for diligent_sys::BLEND_OPERATION {
    fn from(value: &BlendOperation) -> Self {
        (match value {
            BlendOperation::Add => diligent_sys::BLEND_OPERATION_ADD,
            BlendOperation::Subtract => diligent_sys::BLEND_OPERATION_SUBTRACT,
            BlendOperation::RevSubtract => diligent_sys::BLEND_OPERATION_REV_SUBTRACT,
            BlendOperation::Min => diligent_sys::BLEND_OPERATION_MIN,
            BlendOperation::Max => diligent_sys::BLEND_OPERATION_MAX,
        }) as diligent_sys::BLEND_OPERATION
    }
}

pub enum LogicOperation {
    Clear,
    Set,
    Copy,
    CopyInverted,
    NoOp,
    Invert,
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Equiv,
    AndReverse,
    AndInverted,
    OrReverse,
    OrInverted,
}
const_assert!(diligent_sys::LOGIC_OP_NUM_OPERATIONS == 16);

impl From<&LogicOperation> for diligent_sys::LOGIC_OPERATION {
    fn from(value: &LogicOperation) -> Self {
        (match value {
            LogicOperation::Clear => diligent_sys::LOGIC_OP_CLEAR,
            LogicOperation::Set => diligent_sys::LOGIC_OP_SET,
            LogicOperation::Copy => diligent_sys::LOGIC_OP_COPY,
            LogicOperation::CopyInverted => diligent_sys::LOGIC_OP_COPY_INVERTED,
            LogicOperation::NoOp => diligent_sys::LOGIC_OP_NOOP,
            LogicOperation::Invert => diligent_sys::LOGIC_OP_INVERT,
            LogicOperation::And => diligent_sys::LOGIC_OP_AND,
            LogicOperation::Nand => diligent_sys::LOGIC_OP_NAND,
            LogicOperation::Or => diligent_sys::LOGIC_OP_OR,
            LogicOperation::Nor => diligent_sys::LOGIC_OP_NOR,
            LogicOperation::Xor => diligent_sys::LOGIC_OP_XOR,
            LogicOperation::Equiv => diligent_sys::LOGIC_OP_EQUIV,
            LogicOperation::AndReverse => diligent_sys::LOGIC_OP_AND_REVERSE,
            LogicOperation::AndInverted => diligent_sys::LOGIC_OP_AND_INVERTED,
            LogicOperation::OrReverse => diligent_sys::LOGIC_OP_OR_REVERSE,
            LogicOperation::OrInverted => diligent_sys::LOGIC_OP_OR_INVERTED,
        }) as diligent_sys::LOGIC_OPERATION
    }
}

pub enum FillMode {
    Wireframe,
    Solid,
}

impl From<&FillMode> for diligent_sys::FILL_MODE {
    fn from(value: &FillMode) -> Self {
        (match value {
            FillMode::Wireframe => diligent_sys::FILL_MODE_WIREFRAME,
            FillMode::Solid => diligent_sys::FILL_MODE_SOLID,
        }) as diligent_sys::FILL_MODE
    }
}

pub enum CullMode {
    None,
    Front,
    Back,
}

impl From<&CullMode> for diligent_sys::CULL_MODE {
    fn from(value: &CullMode) -> Self {
        (match value {
            CullMode::None => diligent_sys::CULL_MODE_NONE,
            CullMode::Front => diligent_sys::CULL_MODE_FRONT,
            CullMode::Back => diligent_sys::CULL_MODE_BACK,
        }) as diligent_sys::CULL_MODE
    }
}

pub enum StencilOperation {
    Keep,
    Zero,
    Replace,
    IncrSat,
    DecrSat,
    Invert,
    IncrWrap,
    DecrWrap,
}
const_assert!(diligent_sys::STENCIL_OP_NUM_OPS == 9);

impl From<&StencilOperation> for diligent_sys::STENCIL_OP {
    fn from(value: &StencilOperation) -> Self {
        (match value {
            StencilOperation::Keep => diligent_sys::STENCIL_OP_KEEP,
            StencilOperation::Zero => diligent_sys::STENCIL_OP_ZERO,
            StencilOperation::Replace => diligent_sys::STENCIL_OP_REPLACE,
            StencilOperation::IncrSat => diligent_sys::STENCIL_OP_INCR_SAT,
            StencilOperation::DecrSat => diligent_sys::STENCIL_OP_DECR_SAT,
            StencilOperation::Invert => diligent_sys::STENCIL_OP_INVERT,
            StencilOperation::IncrWrap => diligent_sys::STENCIL_OP_INCR_WRAP,
            StencilOperation::DecrWrap => diligent_sys::STENCIL_OP_DECR_WRAP,
        }) as diligent_sys::STENCIL_OP
    }
}

pub enum ComparisonFunction {
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
}
const_assert!(diligent_sys::COMPARISON_FUNC_NUM_FUNCTIONS == 9);

impl From<&ComparisonFunction> for diligent_sys::COMPARISON_FUNCTION {
    fn from(value: &ComparisonFunction) -> Self {
        (match value {
            ComparisonFunction::Never => diligent_sys::COMPARISON_FUNC_NEVER,
            ComparisonFunction::Less => diligent_sys::COMPARISON_FUNC_LESS,
            ComparisonFunction::Equal => diligent_sys::COMPARISON_FUNC_EQUAL,
            ComparisonFunction::LessEqual => diligent_sys::COMPARISON_FUNC_LESS_EQUAL,
            ComparisonFunction::Greater => diligent_sys::COMPARISON_FUNC_GREATER,
            ComparisonFunction::NotEqual => diligent_sys::COMPARISON_FUNC_NOT_EQUAL,
            ComparisonFunction::GreaterEqual => diligent_sys::COMPARISON_FUNC_GREATER_EQUAL,
            ComparisonFunction::Always => diligent_sys::COMPARISON_FUNC_ALWAYS,
        }) as diligent_sys::COMPARISON_FUNCTION
    }
}

bitflags! {
    pub struct ShaderVariableFlags: diligent_sys::SHADER_VARIABLE_FLAGS {
        const None                           = diligent_sys::SHADER_VARIABLE_FLAG_NONE as diligent_sys::SHADER_VARIABLE_FLAGS;
        const NoDynamicBuffers               = diligent_sys::SHADER_VARIABLE_FLAG_NO_DYNAMIC_BUFFERS as diligent_sys::SHADER_VARIABLE_FLAGS;
        const GeneralInputAttachmentVk       = diligent_sys::SHADER_VARIABLE_FLAG_GENERAL_INPUT_ATTACHMENT_VK as diligent_sys::SHADER_VARIABLE_FLAGS;
        const UnfilterableFloatTextureWebgpu = diligent_sys::SHADER_VARIABLE_FLAG_UNFILTERABLE_FLOAT_TEXTURE_WEBGPU as diligent_sys::SHADER_VARIABLE_FLAGS;
        const NonFilteringSamplerWebgpu      = diligent_sys::SHADER_VARIABLE_FLAG_NON_FILTERING_SAMPLER_WEBGPU as diligent_sys::SHADER_VARIABLE_FLAGS;
    }
}
const_assert!(diligent_sys::SHADER_VARIABLE_FLAG_LAST == 8);

bitflags! {
    pub struct ColorMask: diligent_sys::COLOR_MASK {
        const NONE  = diligent_sys::COLOR_MASK_NONE as diligent_sys::COLOR_MASK;
        const RED   = diligent_sys::COLOR_MASK_RED as diligent_sys::COLOR_MASK;
        const GREEN = diligent_sys::COLOR_MASK_GREEN as diligent_sys::COLOR_MASK;
        const BLUE  = diligent_sys::COLOR_MASK_BLUE as diligent_sys::COLOR_MASK;
        const ALPHA = diligent_sys::COLOR_MASK_ALPHA as diligent_sys::COLOR_MASK;
        const RGB   = diligent_sys::COLOR_MASK_RGB as diligent_sys::COLOR_MASK;
        const RGBA  = diligent_sys::COLOR_MASK_ALL as diligent_sys::COLOR_MASK;
    }
}

bitflags! {
    pub struct PipelineStateObjectCreateFlags: diligent_sys::PSO_CREATE_FLAGS {
        const None                           = diligent_sys::PSO_CREATE_FLAG_NONE as diligent_sys::PSO_CREATE_FLAGS;
        const IgnoreMissingVariables         = diligent_sys::PSO_CREATE_FLAG_IGNORE_MISSING_VARIABLES as diligent_sys::PSO_CREATE_FLAGS;
        const IgnoreMissingImmutableSamplers = diligent_sys::PSO_CREATE_FLAG_IGNORE_MISSING_IMMUTABLE_SAMPLERS as diligent_sys::PSO_CREATE_FLAGS;
        const DontRemapShaderResources       = diligent_sys::PSO_CREATE_FLAG_DONT_REMAP_SHADER_RESOURCES as diligent_sys::PSO_CREATE_FLAGS;
        const Asynchronous                   = diligent_sys::PSO_CREATE_FLAG_ASYNCHRONOUS as diligent_sys::PSO_CREATE_FLAGS;
    }
}
const_assert!(diligent_sys::PSO_CREATE_FLAG_LAST == 8);

bitflags! {
    pub struct PipelineShadingRateFlags: diligent_sys::PIPELINE_SHADING_RATE_FLAGS {
        const None         = diligent_sys::PIPELINE_SHADING_RATE_FLAG_NONE as diligent_sys::PIPELINE_SHADING_RATE_FLAGS;
        const PerPrimitive = diligent_sys::PIPELINE_SHADING_RATE_FLAG_PER_PRIMITIVE as diligent_sys::PIPELINE_SHADING_RATE_FLAGS;
        const TextureBased = diligent_sys::PIPELINE_SHADING_RATE_FLAG_TEXTURE_BASED as diligent_sys::PIPELINE_SHADING_RATE_FLAGS;
    }
}
const_assert!(diligent_sys::PIPELINE_SHADING_RATE_FLAG_LAST == 2);

pub struct PipelineResourceLayoutDesc<'a> {
    default_variable_type: ShaderResourceVariableType,
    default_variable_merge_stages: ShaderTypes,
    variables: Vec<ShaderResourceVariableDesc>,
    immutable_samplers: Vec<ImmutableSamplerDesc<'a>>,
}

impl<'a> PipelineResourceLayoutDesc<'a> {
    fn new<const PIPELINE_TYPE: diligent_sys::PIPELINE_TYPE>() -> Self {
        PipelineResourceLayoutDesc {
            default_variable_type: ShaderResourceVariableType::Static,
            default_variable_merge_stages: match PIPELINE_TYPE as diligent_sys::_PIPELINE_TYPE {
                diligent_sys::PIPELINE_TYPE_GRAPHICS => ShaderTypes::AllGraphics,
                diligent_sys::PIPELINE_TYPE_COMPUTE => ShaderTypes::Compute,
                diligent_sys::PIPELINE_TYPE_MESH => ShaderTypes::AllMesh,
                diligent_sys::PIPELINE_TYPE_RAY_TRACING => ShaderTypes::AllRayTracing,
                diligent_sys::PIPELINE_TYPE_TILE => ShaderTypes::Tile,
                _ => panic!("Unknown pipeline type"),
            },
            variables: Vec::new(),
            immutable_samplers: Vec::new(),
        }
    }
}

pub(crate) struct PipelineResourceLayoutDescWrapper {
    _variables: Vec<diligent_sys::ShaderResourceVariableDesc>,
    _immutable_samplers: Vec<diligent_sys::ImmutableSamplerDesc>,
    prld: diligent_sys::PipelineResourceLayoutDesc,
}

impl PipelineResourceLayoutDescWrapper {
    pub(crate) fn get(&self) -> diligent_sys::PipelineResourceLayoutDesc {
        self.prld
    }
}

impl From<&PipelineResourceLayoutDesc<'_>> for PipelineResourceLayoutDescWrapper {
    fn from(value: &PipelineResourceLayoutDesc<'_>) -> Self {
        let variables: Vec<_> = value
            .variables
            .iter()
            .map(|var| diligent_sys::ShaderResourceVariableDesc::from(var))
            .collect();

        let immutable_samplers: Vec<_> = value
            .immutable_samplers
            .iter()
            .map(|var| diligent_sys::ImmutableSamplerDesc::from(var))
            .collect();

        let prld = diligent_sys::PipelineResourceLayoutDesc {
            DefaultVariableType: diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE::from(
                &value.default_variable_type,
            ),
            DefaultVariableMergeStages: value.default_variable_merge_stages.bits(),
            NumVariables: variables.len() as u32,
            Variables: if variables.is_empty() {
                std::ptr::null()
            } else {
                variables.as_ptr()
            },
            NumImmutableSamplers: immutable_samplers.len() as u32,
            ImmutableSamplers: if immutable_samplers.is_empty() {
                std::ptr::null()
            } else {
                immutable_samplers.as_ptr()
            },
        };

        PipelineResourceLayoutDescWrapper {
            prld,
            _variables: variables,
            _immutable_samplers: immutable_samplers,
        }
    }
}

struct PipelineStateDesc<'a, const PIPELINE_TYPE: diligent_sys::PIPELINE_TYPE> {
    name: CString,
    srb_allocation_granularity: u32,
    immediate_context_mask: u64,
    resource_layout: PipelineResourceLayoutDesc<'a>,
}

impl<'a, const PIPELINE_TYPE: diligent_sys::PIPELINE_TYPE> PipelineStateDesc<'a, PIPELINE_TYPE> {
    fn new(name: &'a str) -> Self {
        PipelineStateDesc {
            name: CString::from_str(name).unwrap(),
            srb_allocation_granularity: 1,
            immediate_context_mask: 1,
            resource_layout: PipelineResourceLayoutDesc::new::<PIPELINE_TYPE>(),
        }
    }
}

pub(crate) struct PipelineStateDescWrapper {
    _prld: PipelineResourceLayoutDescWrapper,
    psd: diligent_sys::PipelineStateDesc,
}

impl PipelineStateDescWrapper {
    pub(crate) fn get(&self) -> diligent_sys::PipelineStateDesc {
        self.psd
    }
}

impl<const PIPELINE_TYPE: diligent_sys::PIPELINE_TYPE> From<&PipelineStateDesc<'_, PIPELINE_TYPE>>
    for PipelineStateDescWrapper
{
    fn from(value: &PipelineStateDesc<'_, PIPELINE_TYPE>) -> Self {
        let prld = PipelineResourceLayoutDescWrapper::from(&value.resource_layout);

        let psd = diligent_sys::PipelineStateDesc {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: value.name.as_ptr(),
            },
            PipelineType: PIPELINE_TYPE,
            SRBAllocationGranularity: value.srb_allocation_granularity,
            ImmediateContextMask: value.immediate_context_mask,
            ResourceLayout: prld.get(),
        };

        PipelineStateDescWrapper { _prld: prld, psd }
    }
}

struct PipelineStateCreateInfo<'a, const PIPELINE_TYPE: diligent_sys::PIPELINE_TYPE> {
    pso_desc: PipelineStateDesc<'a, PIPELINE_TYPE>,
    flags: PipelineStateObjectCreateFlags,
    resource_signatures: Vec<&'a PipelineResourceSignature>,
    //TODO
    //pub pPSOCache: *mut IPipelineStateCache,
}

pub(crate) struct PipelineStateCreateInfoWrapper {
    _psd: PipelineStateDescWrapper,
    _resource_signatures: Vec<*mut diligent_sys::IPipelineResourceSignature>,
    ci: diligent_sys::PipelineStateCreateInfo,
}

impl PipelineStateCreateInfoWrapper {
    pub(crate) fn get(&self) -> diligent_sys::PipelineStateCreateInfo {
        self.ci
    }
}

impl<const PIPELINE_TYPE: diligent_sys::PIPELINE_TYPE>
    From<&PipelineStateCreateInfo<'_, PIPELINE_TYPE>> for PipelineStateCreateInfoWrapper
{
    fn from(value: &PipelineStateCreateInfo<'_, PIPELINE_TYPE>) -> Self {
        let psd = PipelineStateDescWrapper::from(&value.pso_desc);

        let mut resource_signatures = value
            .resource_signatures
            .iter()
            .map(|&rs| rs.pipeline_resource_signature)
            .collect::<Vec<*mut diligent_sys::IPipelineResourceSignature>>();

        let ci = diligent_sys::PipelineStateCreateInfo {
            PSODesc: psd.get(),
            Flags: value.flags.bits(),
            ResourceSignaturesCount: value.resource_signatures.len() as u32,
            ppResourceSignatures: if value.resource_signatures.is_empty() {
                std::ptr::null_mut()
            } else {
                resource_signatures.as_mut_ptr()
            },
            pPSOCache: std::ptr::null_mut(), // TODO
            pInternalData: std::ptr::null_mut(),
        };

        PipelineStateCreateInfoWrapper {
            _psd: psd,
            _resource_signatures: resource_signatures,
            ci,
        }
    }
}

impl<'a, const PIPELINE_TYPE: diligent_sys::PIPELINE_TYPE>
    PipelineStateCreateInfo<'a, PIPELINE_TYPE>
{
    fn new(name: &'a str) -> Self {
        PipelineStateCreateInfo {
            pso_desc: PipelineStateDesc::new(name),
            flags: PipelineStateObjectCreateFlags::None,
            resource_signatures: Vec::new(),
        }
    }
}

pub struct RenderTargetBlendDesc {
    blend_enable: bool,
    logic_operation_enable: bool,
    src_blend: BlendFactor,
    dest_blend: BlendFactor,
    blend_op: BlendOperation,
    src_blend_alpha: BlendFactor,
    dest_blend_alpha: BlendFactor,
    blend_op_alpha: BlendOperation,
    logic_op: LogicOperation,
    render_target_write_mask: ColorMask,
}

impl RenderTargetBlendDesc {
    pub fn blend_enable(mut self, blend_enable: bool) -> Self {
        self.blend_enable = blend_enable;
        self
    }
    pub fn logic_operation_enable(mut self, logic_operation_enable: bool) -> Self {
        self.logic_operation_enable = logic_operation_enable;
        self
    }
    pub fn src_blend(mut self, src_blend: BlendFactor) -> Self {
        self.src_blend = src_blend;
        self
    }
    pub fn dest_blend(mut self, dest_blend: BlendFactor) -> Self {
        self.dest_blend = dest_blend;
        self
    }
    pub fn blend_op(mut self, blend_op: BlendOperation) -> Self {
        self.blend_op = blend_op;
        self
    }
    pub fn src_blend_alpha(mut self, src_blend_alpha: BlendFactor) -> Self {
        self.src_blend_alpha = src_blend_alpha;
        self
    }
    pub fn dest_blend_alpha(mut self, dest_blend_alpha: BlendFactor) -> Self {
        self.dest_blend_alpha = dest_blend_alpha;
        self
    }
    pub fn blend_op_alpha(mut self, blend_op_alpha: BlendOperation) -> Self {
        self.blend_op_alpha = blend_op_alpha;
        self
    }
    pub fn logic_op(mut self, logic_op: LogicOperation) -> Self {
        self.logic_op = logic_op;
        self
    }
    pub fn render_target_write_mask(mut self, render_target_write_mask: ColorMask) -> Self {
        self.render_target_write_mask = render_target_write_mask;
        self
    }
}

impl From<&RenderTargetBlendDesc> for diligent_sys::RenderTargetBlendDesc {
    fn from(value: &RenderTargetBlendDesc) -> Self {
        diligent_sys::RenderTargetBlendDesc {
            BlendEnable: value.blend_enable,
            LogicOperationEnable: value.logic_operation_enable,
            SrcBlend: diligent_sys::BLEND_FACTOR::from(&value.src_blend),
            DestBlend: diligent_sys::BLEND_FACTOR::from(&value.dest_blend),
            BlendOp: diligent_sys::BLEND_OPERATION::from(&value.blend_op),
            SrcBlendAlpha: diligent_sys::BLEND_FACTOR::from(&value.src_blend_alpha),
            DestBlendAlpha: diligent_sys::BLEND_FACTOR::from(&value.dest_blend_alpha),
            BlendOpAlpha: diligent_sys::BLEND_OPERATION::from(&value.blend_op_alpha),
            LogicOp: diligent_sys::LOGIC_OPERATION::from(&value.logic_op),
            RenderTargetWriteMask: value.render_target_write_mask.bits()
                as diligent_sys::COLOR_MASK,
        }
    }
}

impl Default for RenderTargetBlendDesc {
    fn default() -> Self {
        RenderTargetBlendDesc {
            blend_enable: false,
            logic_operation_enable: false,
            src_blend: BlendFactor::One,
            dest_blend: BlendFactor::Zero,
            blend_op: BlendOperation::Add,
            src_blend_alpha: BlendFactor::One,
            dest_blend_alpha: BlendFactor::Zero,
            blend_op_alpha: BlendOperation::Add,
            logic_op: LogicOperation::NoOp,
            render_target_write_mask: ColorMask::RGBA,
        }
    }
}

pub struct BlendStateDesc {
    alpha_to_coverage_enable: bool,
    independent_blend_enable: bool,
    render_targets: [RenderTargetBlendDesc; diligent_sys::DILIGENT_MAX_RENDER_TARGETS as usize],
}

impl BlendStateDesc {
    pub fn alpha_to_coverage_enable(mut self, alpha_to_coverage_enable: bool) -> Self {
        self.alpha_to_coverage_enable = alpha_to_coverage_enable;
        self
    }
    pub fn independent_blend_enable(mut self, independent_blend_enable: bool) -> Self {
        self.independent_blend_enable = independent_blend_enable;
        self
    }
    pub fn render_target_blend_desc<const INDEX: usize>(
        mut self,
        render_target_blend_desc: RenderTargetBlendDesc,
    ) -> Self {
        self.render_targets[INDEX] = render_target_blend_desc;
        self
    }
}

impl From<&BlendStateDesc> for diligent_sys::BlendStateDesc {
    fn from(value: &BlendStateDesc) -> Self {
        diligent_sys::BlendStateDesc {
            AlphaToCoverageEnable: value.alpha_to_coverage_enable,
            IndependentBlendEnable: value.independent_blend_enable,
            RenderTargets: value
                .render_targets
                .each_ref()
                .map(|rt| diligent_sys::RenderTargetBlendDesc::from(rt)),
        }
    }
}

impl Default for BlendStateDesc {
    fn default() -> Self {
        BlendStateDesc {
            alpha_to_coverage_enable: false,
            independent_blend_enable: false,
            render_targets: [
                RenderTargetBlendDesc::default(),
                RenderTargetBlendDesc::default(),
                RenderTargetBlendDesc::default(),
                RenderTargetBlendDesc::default(),
                RenderTargetBlendDesc::default(),
                RenderTargetBlendDesc::default(),
                RenderTargetBlendDesc::default(),
                RenderTargetBlendDesc::default(),
            ],
        }
    }
}

pub struct RasterizerStateDesc {
    fill_mode: FillMode,
    cull_mode: CullMode,
    front_counter_clockwise: bool,
    depth_clip_enable: bool,
    scissor_enable: bool,
    antialiased_line_enable: bool,
    depth_bias: i32,
    depth_bias_clamp: f32,
    slope_scaled_depth_bias: f32,
}

impl RasterizerStateDesc {
    pub fn fill_mode(mut self, fill_mode: FillMode) -> Self {
        self.fill_mode = fill_mode;
        self
    }
    pub fn cull_mode(mut self, cull_mode: CullMode) -> Self {
        self.cull_mode = cull_mode;
        self
    }
    pub fn front_counter_clockwise(mut self, front_counter_clockwise: bool) -> Self {
        self.front_counter_clockwise = front_counter_clockwise;
        self
    }
    pub fn depth_clip_enable(mut self, depth_clip_enable: bool) -> Self {
        self.depth_clip_enable = depth_clip_enable;
        self
    }
    pub fn scissor_enable(mut self, scissor_enable: bool) -> Self {
        self.scissor_enable = scissor_enable;
        self
    }
    pub fn antialiased_line_enable(mut self, antialiased_line_enable: bool) -> Self {
        self.antialiased_line_enable = antialiased_line_enable;
        self
    }
    pub fn depth_bias(mut self, depth_bias: i32) -> Self {
        self.depth_bias = depth_bias;
        self
    }
    pub fn depth_bias_clamp(mut self, depth_bias_clamp: f32) -> Self {
        self.depth_bias_clamp = depth_bias_clamp;
        self
    }
    pub fn slope_scaled_depth_bias(mut self, slope_scaled_depth_bias: f32) -> Self {
        self.slope_scaled_depth_bias = slope_scaled_depth_bias;
        self
    }
}

impl From<&RasterizerStateDesc> for diligent_sys::RasterizerStateDesc {
    fn from(value: &RasterizerStateDesc) -> Self {
        diligent_sys::RasterizerStateDesc {
            FillMode: diligent_sys::FILL_MODE::from(&value.fill_mode),
            CullMode: diligent_sys::CULL_MODE::from(&value.cull_mode),
            FrontCounterClockwise: value.front_counter_clockwise,
            DepthClipEnable: value.depth_clip_enable,
            ScissorEnable: value.scissor_enable,
            AntialiasedLineEnable: value.antialiased_line_enable,
            DepthBias: value.depth_bias,
            DepthBiasClamp: value.depth_bias_clamp,
            SlopeScaledDepthBias: value.slope_scaled_depth_bias,
        }
    }
}

impl Default for RasterizerStateDesc {
    fn default() -> Self {
        RasterizerStateDesc {
            fill_mode: FillMode::Solid,
            cull_mode: CullMode::Back,
            front_counter_clockwise: false,
            depth_clip_enable: true,
            scissor_enable: false,
            antialiased_line_enable: false,
            depth_bias: 0,
            depth_bias_clamp: 0.0,
            slope_scaled_depth_bias: 0.0,
        }
    }
}

pub struct StencilOperationsDesc {
    stencil_fail_op: StencilOperation,
    stencil_depth_fail_op: StencilOperation,
    stencil_pass_op: StencilOperation,
    stencil_func: ComparisonFunction,
}

impl StencilOperationsDesc {
    pub fn stencil_fail_op(mut self, stencil_fail_op: StencilOperation) -> Self {
        self.stencil_fail_op = stencil_fail_op;
        self
    }
    pub fn stencil_depth_fail_op(mut self, stencil_depth_fail_op: StencilOperation) -> Self {
        self.stencil_depth_fail_op = stencil_depth_fail_op;
        self
    }
    pub fn stencil_pass_op(mut self, stencil_pass_op: StencilOperation) -> Self {
        self.stencil_pass_op = stencil_pass_op;
        self
    }
    pub fn stencil_func(mut self, stencil_func: ComparisonFunction) -> Self {
        self.stencil_func = stencil_func;
        self
    }
}

impl From<&StencilOperationsDesc> for diligent_sys::StencilOpDesc {
    fn from(value: &StencilOperationsDesc) -> Self {
        diligent_sys::StencilOpDesc {
            StencilFailOp: diligent_sys::STENCIL_OP::from(&value.stencil_fail_op),
            StencilDepthFailOp: diligent_sys::STENCIL_OP::from(&value.stencil_depth_fail_op),
            StencilPassOp: diligent_sys::STENCIL_OP::from(&value.stencil_pass_op),
            StencilFunc: diligent_sys::COMPARISON_FUNCTION::from(&value.stencil_func),
        }
    }
}

impl Default for StencilOperationsDesc {
    fn default() -> Self {
        StencilOperationsDesc {
            stencil_fail_op: StencilOperation::Keep,
            stencil_depth_fail_op: StencilOperation::Keep,
            stencil_pass_op: StencilOperation::Keep,
            stencil_func: ComparisonFunction::Always,
        }
    }
}

pub struct DepthStencilStateDesc {
    depth_enable: bool,
    depth_write_enable: bool,
    depth_func: ComparisonFunction,
    stencil_enable: bool,
    stencil_read_mask: u8,
    stencil_write_mask: u8,
    front_face: StencilOperationsDesc,
    back_face: StencilOperationsDesc,
}

impl DepthStencilStateDesc {
    pub fn depth_enable(mut self, depth_enable: bool) -> Self {
        self.depth_enable = depth_enable;
        self
    }
    pub fn depth_write_enable(mut self, depth_write_enable: bool) -> Self {
        self.depth_write_enable = depth_write_enable;
        self
    }
    pub fn depth_func(mut self, depth_func: ComparisonFunction) -> Self {
        self.depth_func = depth_func;
        self
    }
    pub fn stencil_enable(mut self, stencil_enable: bool) -> Self {
        self.stencil_enable = stencil_enable;
        self
    }
    pub fn stencil_read_mask(mut self, stencil_read_mask: u8) -> Self {
        self.stencil_read_mask = stencil_read_mask;
        self
    }
    pub fn stencil_write_mask(mut self, stencil_write_mask: u8) -> Self {
        self.stencil_write_mask = stencil_write_mask;
        self
    }
    pub fn front_face(mut self, front_face: StencilOperationsDesc) -> Self {
        self.front_face = front_face;
        self
    }
    pub fn back_face(mut self, back_face: StencilOperationsDesc) -> Self {
        self.back_face = back_face;
        self
    }
}

impl From<&DepthStencilStateDesc> for diligent_sys::DepthStencilStateDesc {
    fn from(value: &DepthStencilStateDesc) -> Self {
        diligent_sys::DepthStencilStateDesc {
            DepthEnable: value.depth_enable,
            DepthWriteEnable: value.depth_write_enable,
            DepthFunc: diligent_sys::COMPARISON_FUNCTION::from(&value.depth_func),
            StencilEnable: value.stencil_enable,
            StencilReadMask: value.stencil_read_mask,
            StencilWriteMask: value.stencil_write_mask,
            FrontFace: diligent_sys::StencilOpDesc::from(&value.front_face),
            BackFace: diligent_sys::StencilOpDesc::from(&value.back_face),
        }
    }
}

impl Default for DepthStencilStateDesc {
    fn default() -> Self {
        DepthStencilStateDesc {
            depth_enable: true,
            depth_write_enable: true,
            depth_func: ComparisonFunction::Less,
            stencil_enable: false,
            stencil_read_mask: 0xff,
            stencil_write_mask: 0xff,
            front_face: StencilOperationsDesc::default(),
            back_face: StencilOperationsDesc::default(),
        }
    }
}

pub struct GraphicsPipelineDesc<'a> {
    blend_desc: BlendStateDesc,
    sample_mask: u32,
    rasterizer_desc: RasterizerStateDesc,
    depth_stencil_desc: DepthStencilStateDesc,
    input_layouts: Vec<LayoutElement<'a>>,
    primitive_topology: PrimitiveTopology,
    num_viewports: u8,
    num_render_targets: u8,
    subpass_index: u8,
    shading_rate_flags: PipelineShadingRateFlags,
    rtv_formats:
        [diligent_sys::_TEXTURE_FORMAT; diligent_sys::DILIGENT_MAX_RENDER_TARGETS as usize],
    dsv_format: diligent_sys::_TEXTURE_FORMAT,
    read_only_dsv: bool,
    sample_desc: diligent_sys::SampleDesc,
    // TODO
    // pub render_pass: Option<&RenderPass>,
    node_mask: u32,
}

impl<'a> Default for GraphicsPipelineDesc<'a> {
    fn default() -> Self {
        GraphicsPipelineDesc::new(
            BlendStateDesc::default(),
            RasterizerStateDesc::default(),
            DepthStencilStateDesc::default(),
        )
    }
}

impl<'a> GraphicsPipelineDesc<'a> {
    pub fn new(
        blend_desc: BlendStateDesc,
        rasterizer_desc: RasterizerStateDesc,
        depth_stencil_desc: DepthStencilStateDesc,
    ) -> Self {
        GraphicsPipelineDesc {
            blend_desc: blend_desc,
            sample_mask: 0xFFFFFFFF,
            rasterizer_desc,
            depth_stencil_desc: depth_stencil_desc,
            input_layouts: Vec::new(),
            primitive_topology: PrimitiveTopology::TriangleList,
            num_viewports: 1,
            num_render_targets: 0,
            subpass_index: 0,
            shading_rate_flags: PipelineShadingRateFlags::None,
            rtv_formats: [
                diligent_sys::TEX_FORMAT_UNKNOWN,
                diligent_sys::TEX_FORMAT_UNKNOWN,
                diligent_sys::TEX_FORMAT_UNKNOWN,
                diligent_sys::TEX_FORMAT_UNKNOWN,
                diligent_sys::TEX_FORMAT_UNKNOWN,
                diligent_sys::TEX_FORMAT_UNKNOWN,
                diligent_sys::TEX_FORMAT_UNKNOWN,
                diligent_sys::TEX_FORMAT_UNKNOWN,
            ],
            dsv_format: diligent_sys::TEX_FORMAT_UNKNOWN,
            read_only_dsv: false,
            node_mask: 0,
            sample_desc: diligent_sys::SampleDesc {
                Count: 1,
                Quality: 0,
            },
        }
    }

    pub fn sample_mask(mut self, sample_mask: u32) -> Self {
        self.sample_mask = sample_mask;
        self
    }
    pub fn primitive_topology(mut self, primitive_topology: PrimitiveTopology) -> Self {
        self.primitive_topology = primitive_topology;
        self
    }
    pub fn num_viewports(mut self, num_viewports: u8) -> Self {
        self.num_viewports = num_viewports;
        self
    }
    pub fn num_render_targets(mut self, num_render_targets: u8) -> Self {
        self.num_render_targets = num_render_targets;
        self
    }
    pub fn subpass_index(mut self, subpass_index: u8) -> Self {
        self.subpass_index = subpass_index;
        self
    }
    pub fn shading_rate_flags(mut self, shading_rate_flags: PipelineShadingRateFlags) -> Self {
        self.shading_rate_flags = shading_rate_flags;
        self
    }
    pub fn rtv_format<const INDEX: usize>(mut self, value: diligent_sys::_TEXTURE_FORMAT) -> Self {
        self.rtv_formats[INDEX] = value;
        self
    }
    pub fn dsv_format(mut self, dsv_format: diligent_sys::_TEXTURE_FORMAT) -> Self {
        self.dsv_format = dsv_format;
        self
    }
    pub fn read_only_dsv(mut self, read_only_dsv: bool) -> Self {
        self.read_only_dsv = read_only_dsv;
        self
    }
    pub fn add_input_layout(mut self, input_layout: LayoutElement<'a>) -> Self {
        self.input_layouts.push(input_layout);
        self
    }
}

pub(crate) struct GraphicsPipelineDescWrapper {
    _aux_input_layouts: Vec<diligent_sys::LayoutElement>,
    desc: diligent_sys::GraphicsPipelineDesc,
}

impl GraphicsPipelineDescWrapper {
    pub(crate) fn get(&self) -> diligent_sys::GraphicsPipelineDesc {
        self.desc
    }
}

impl From<&GraphicsPipelineDesc<'_>> for GraphicsPipelineDescWrapper {
    fn from(value: &GraphicsPipelineDesc<'_>) -> Self {
        let aux_input_layouts: Vec<diligent_sys::LayoutElement> = value
            .input_layouts
            .iter()
            .map(|layout| diligent_sys::LayoutElement::from(layout))
            .collect();

        let desc = diligent_sys::GraphicsPipelineDesc {
            BlendDesc: diligent_sys::BlendStateDesc::from(&value.blend_desc),
            SampleMask: value.sample_mask,
            RasterizerDesc: diligent_sys::RasterizerStateDesc::from(&value.rasterizer_desc),
            DepthStencilDesc: diligent_sys::DepthStencilStateDesc::from(&value.depth_stencil_desc),
            InputLayout: diligent_sys::InputLayoutDesc {
                LayoutElements: if aux_input_layouts.is_empty() {
                    std::ptr::null()
                } else {
                    aux_input_layouts.as_ptr()
                },
                NumElements: aux_input_layouts.len() as u32,
            },
            PrimitiveTopology: diligent_sys::PRIMITIVE_TOPOLOGY::from(&value.primitive_topology),
            NumViewports: value.num_viewports,
            NumRenderTargets: value.num_render_targets,
            SubpassIndex: value.subpass_index,
            ShadingRateFlags: value.shading_rate_flags.bits()
                as diligent_sys::PIPELINE_SHADING_RATE_FLAGS,
            RTVFormats: value
                .rtv_formats
                .map(|format| format as diligent_sys::TEXTURE_FORMAT),
            DSVFormat: value.dsv_format as diligent_sys::TEXTURE_FORMAT,
            ReadOnlyDSV: value.read_only_dsv,
            SmplDesc: value.sample_desc.into(),
            pRenderPass: std::ptr::null_mut(),
            NodeMask: value.node_mask,
        };

        GraphicsPipelineDescWrapper {
            _aux_input_layouts: aux_input_layouts,
            desc,
        }
    }
}

// For now, couldn't find any practical way to provide the `diligent_sys::PIPELINE_TYPE_GRAPHICS` value
// directly to the PipelineStateCreateInfo<> template member. This happens because the compiler can't
// convert a `::std::os::raw::c_uint` into a `u8` implicitly in compile time. If you know of a better
// way of doing this, feel free to make a pull request.
const_assert!(diligent_sys::PIPELINE_TYPE_GRAPHICS == 0);
pub struct GraphicsPipelineStateCreateInfo<'a> {
    pipeline_state_create_info: PipelineStateCreateInfo<'a, 0>,
    graphics_pipeline_desc: GraphicsPipelineDesc<'a>,
    vertex_shader: Option<&'a Shader>,
    pixel_shader: Option<&'a Shader>,
    domain_shader: Option<&'a Shader>,
    hull_shader: Option<&'a Shader>,
    geometry_shader: Option<&'a Shader>,
    amplification_shader: Option<&'a Shader>,
    mesh_shader: Option<&'a Shader>,
}

impl<'a> GraphicsPipelineStateCreateInfo<'a> {
    pub fn new(name: &'a str, graphics_pipeline_desc: GraphicsPipelineDesc<'a>) -> Self {
        GraphicsPipelineStateCreateInfo {
            pipeline_state_create_info: PipelineStateCreateInfo::new(name),
            graphics_pipeline_desc,
            vertex_shader: None,
            pixel_shader: None,
            domain_shader: None,
            hull_shader: None,
            geometry_shader: None,
            amplification_shader: None,
            mesh_shader: None,
        }
    }

    pub fn default_variable_type(
        mut self,
        default_variable_type: ShaderResourceVariableType,
    ) -> Self {
        self.pipeline_state_create_info
            .pso_desc
            .resource_layout
            .default_variable_type = default_variable_type;
        self
    }

    pub fn default_variable_merge_stages(
        mut self,
        default_variable_merge_stages: ShaderTypes,
    ) -> Self {
        self.pipeline_state_create_info
            .pso_desc
            .resource_layout
            .default_variable_merge_stages = default_variable_merge_stages;
        self
    }

    pub fn add_resource_signature(mut self, signature: &'a PipelineResourceSignature) -> Self {
        self.pipeline_state_create_info
            .resource_signatures
            .push(&signature);
        self
    }

    pub fn add_shader_resource_variable(mut self, variable: ShaderResourceVariableDesc) -> Self {
        self.pipeline_state_create_info
            .pso_desc
            .resource_layout
            .variables
            .push(variable);
        self
    }

    pub fn add_immutable_sampler_desc(mut self, sampler: ImmutableSamplerDesc<'a>) -> Self {
        self.pipeline_state_create_info
            .pso_desc
            .resource_layout
            .immutable_samplers
            .push(sampler);
        self
    }

    pub fn vertex_shader(mut self, shader: &'a Shader) -> Self {
        self.vertex_shader = Some(shader);
        self
    }

    pub fn pixel_shader(mut self, shader: &'a Shader) -> Self {
        self.pixel_shader = Some(shader);
        self
    }

    pub fn domain_shader(mut self, shader: &'a Shader) -> Self {
        self.domain_shader = Some(shader);
        self
    }

    pub fn hull_shader(mut self, shader: &'a Shader) -> Self {
        self.hull_shader = Some(shader);
        self
    }

    pub fn geometry_shader(mut self, shader: &'a Shader) -> Self {
        self.geometry_shader = Some(shader);
        self
    }

    pub fn amplification_shader(mut self, shader: &'a Shader) -> Self {
        self.amplification_shader = Some(shader);
        self
    }

    pub fn mesh_shader(mut self, shader: &'a Shader) -> Self {
        self.mesh_shader = Some(shader);
        self
    }
}

pub(crate) struct GraphicsPipelineStateCreateInfoWrapper {
    _pci: PipelineStateCreateInfoWrapper,
    _gpd: GraphicsPipelineDescWrapper,
    ci: diligent_sys::GraphicsPipelineStateCreateInfo,
}

impl GraphicsPipelineStateCreateInfoWrapper {
    pub(crate) fn get(&self) -> diligent_sys::GraphicsPipelineStateCreateInfo {
        self.ci
    }
}

impl From<&GraphicsPipelineStateCreateInfo<'_>> for GraphicsPipelineStateCreateInfoWrapper {
    fn from(value: &GraphicsPipelineStateCreateInfo<'_>) -> Self {
        let pci = PipelineStateCreateInfoWrapper::from(&value.pipeline_state_create_info);
        let gpd = GraphicsPipelineDescWrapper::from(&value.graphics_pipeline_desc);
        let ci = diligent_sys::GraphicsPipelineStateCreateInfo {
            _PipelineStateCreateInfo: pci.get(),
            GraphicsPipeline: gpd.get(),
            pVS: value
                .vertex_shader
                .map_or(std::ptr::null_mut(), |shader| shader.shader),
            pPS: value
                .pixel_shader
                .map_or(std::ptr::null_mut(), |shader| shader.shader),
            pDS: value
                .domain_shader
                .map_or(std::ptr::null_mut(), |shader| shader.shader),
            pHS: value
                .hull_shader
                .map_or(std::ptr::null_mut(), |shader| shader.shader),
            pGS: value
                .geometry_shader
                .map_or(std::ptr::null_mut(), |shader| shader.shader),
            pAS: value
                .amplification_shader
                .map_or(std::ptr::null_mut(), |shader| shader.shader),
            pMS: value
                .mesh_shader
                .map_or(std::ptr::null_mut(), |shader| shader.shader),
        };

        GraphicsPipelineStateCreateInfoWrapper {
            _pci: pci,
            _gpd: gpd,
            ci,
        }
    }
}

pub struct PipelineState {
    pub(crate) pipeline_state: *mut diligent_sys::IPipelineState,
    virtual_functions: *mut diligent_sys::IPipelineStateVtbl,

    device_object: DeviceObject,
}

impl AsDeviceObject for PipelineState {
    fn as_device_object(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl PipelineState {
    pub(crate) fn new(pipeline_state_ptr: *mut diligent_sys::IPipelineState) -> Self {
        PipelineState {
            pipeline_state: pipeline_state_ptr,
            virtual_functions: unsafe { (*pipeline_state_ptr).pVtbl },
            device_object: DeviceObject::new(
                pipeline_state_ptr as *mut diligent_sys::IDeviceObject,
            ),
        }
    }

    pub fn get_desc(&self) -> &diligent_sys::PipelineStateDesc {
        unsafe {
            ((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(
                self.pipeline_state as *mut diligent_sys::IDeviceObject
            ) as *const diligent_sys::PipelineStateDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }

    pub fn get_graphics_pipeline_desc(&self) -> &diligent_sys::GraphicsPipelineDesc {
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .GetGraphicsPipelineDesc
                .unwrap_unchecked()(self.pipeline_state)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    pub fn get_ray_tracing_pipeline_desc(&self) -> &diligent_sys::RayTracingPipelineDesc {
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .GetRayTracingPipelineDesc
                .unwrap_unchecked()(self.pipeline_state)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    pub fn get_tile_pipeline_desc(&self) -> &diligent_sys::TilePipelineDesc {
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .GetTilePipelineDesc
                .unwrap_unchecked()(self.pipeline_state)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    pub fn bind_static_resources(
        &mut self,
        shader_type: ShaderType,
        resource_mapping: &ResourceMapping,
        flags: diligent_sys::BIND_SHADER_RESOURCES_FLAGS,
    ) {
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .BindStaticResources
                .unwrap_unchecked()(
                self.pipeline_state,
                diligent_sys::SHADER_TYPE::from(&shader_type),
                resource_mapping.resource_mapping,
                flags,
            )
        }
    }

    pub fn get_static_variables(
        &self,
        _shader_type: ShaderType,
    ) -> Option<&[ShaderResourceVariable]> {
        todo!()
    }

    pub fn get_static_variable_by_name(
        &self,
        shader_type: ShaderType,
        name: &str,
    ) -> Option<ShaderResourceVariable> {
        let name = CString::from_str(name).unwrap();

        let shader_resource_variable = unsafe {
            (*self.virtual_functions)
                .PipelineState
                .GetStaticVariableByName
                .unwrap_unchecked()(
                self.pipeline_state,
                diligent_sys::SHADER_TYPE::from(&shader_type),
                name.as_ptr(),
            )
        };

        if shader_resource_variable.is_null() {
            None
        } else {
            let srv = ShaderResourceVariable::new(shader_resource_variable);
            srv.as_object().add_ref();
            Some(srv)
        }
    }

    pub fn create_shader_resource_binding(
        &self,
        init_static_resources: bool,
    ) -> Option<ShaderResourceBinding> {
        let mut shader_resource_binding_ptr: *mut diligent_sys::IShaderResourceBinding =
            std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .CreateShaderResourceBinding
                .unwrap_unchecked()(
                self.pipeline_state,
                std::ptr::addr_of_mut!(shader_resource_binding_ptr),
                init_static_resources,
            );
        }
        if shader_resource_binding_ptr.is_null() {
            None
        } else {
            let srb = ShaderResourceBinding::new(shader_resource_binding_ptr);
            srb.as_object().add_ref();
            Some(srb)
        }
    }

    pub fn initialize_static_srb_resources(
        &self,
        shader_resource_binding: &mut ShaderResourceBinding,
    ) {
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .InitializeStaticSRBResources
                .unwrap_unchecked()(
                self.pipeline_state,
                shader_resource_binding.shader_resource_binding,
            )
        }
    }

    pub fn copy_static_resources(&self, pipeline_state: &mut PipelineState) {
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .CopyStaticResources
                .unwrap_unchecked()(self.pipeline_state, pipeline_state.pipeline_state)
        }
    }

    pub fn is_compatible_with(&self, pipeline_state: &PipelineState) -> bool {
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .IsCompatibleWith
                .unwrap_unchecked()(self.pipeline_state, pipeline_state.pipeline_state)
        }
    }

    pub fn get_resource_signatures(&self) -> &[PipelineResourceSignature] {
        todo!()
    }

    pub fn get_status(
        &self,
        wait_for_completion: Option<bool>,
    ) -> diligent_sys::PIPELINE_STATE_STATUS {
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .GetStatus
                .unwrap_unchecked()(
                self.pipeline_state, wait_for_completion.unwrap_or(false)
            )
        }
    }
}
