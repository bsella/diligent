use std::str::FromStr;
use std::{ffi::CString, ops::Deref};

use bitflags::bitflags;
use bon::Builder;
use static_assertions::const_assert;

use crate::pipeline_state_cache::PipelineStateCache;
use crate::{
    device_object::DeviceObject,
    graphics_types::{PrimitiveTopology, ShaderType, ShaderTypes, TextureFormat},
    input_layout::{InputLayoutDescWrapper, LayoutElement},
    pipeline_resource_signature::{ImmutableSamplerDesc, PipelineResourceSignature},
    render_pass::RenderPass,
    resource_mapping::ResourceMapping,
    shader::Shader,
    shader_resource_binding::ShaderResourceBinding,
    shader_resource_variable::{
        BindShaderResourcesFlags, ShaderResourceVariable, ShaderResourceVariableDesc,
        ShaderResourceVariableType,
    },
};

#[derive(Clone, Copy)]
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

impl From<BlendFactor> for diligent_sys::BLEND_FACTOR {
    fn from(value: BlendFactor) -> Self {
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
        }) as _
    }
}

#[derive(Clone, Copy)]
pub enum BlendOperation {
    Add,
    Subtract,
    RevSubtract,
    Min,
    Max,
}
const_assert!(diligent_sys::BLEND_OPERATION_NUM_OPERATIONS == 6);

impl From<BlendOperation> for diligent_sys::BLEND_OPERATION {
    fn from(value: BlendOperation) -> Self {
        (match value {
            BlendOperation::Add => diligent_sys::BLEND_OPERATION_ADD,
            BlendOperation::Subtract => diligent_sys::BLEND_OPERATION_SUBTRACT,
            BlendOperation::RevSubtract => diligent_sys::BLEND_OPERATION_REV_SUBTRACT,
            BlendOperation::Min => diligent_sys::BLEND_OPERATION_MIN,
            BlendOperation::Max => diligent_sys::BLEND_OPERATION_MAX,
        }) as _
    }
}

#[derive(Clone, Copy)]
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

impl From<LogicOperation> for diligent_sys::LOGIC_OPERATION {
    fn from(value: LogicOperation) -> Self {
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
        }) as _
    }
}

#[derive(Clone, Copy)]
pub enum FillMode {
    Wireframe,
    Solid,
}

impl From<FillMode> for diligent_sys::FILL_MODE {
    fn from(value: FillMode) -> Self {
        (match value {
            FillMode::Wireframe => diligent_sys::FILL_MODE_WIREFRAME,
            FillMode::Solid => diligent_sys::FILL_MODE_SOLID,
        }) as _
    }
}

#[derive(Clone, Copy)]
pub enum CullMode {
    None,
    Front,
    Back,
}

impl From<CullMode> for diligent_sys::CULL_MODE {
    fn from(value: CullMode) -> Self {
        (match value {
            CullMode::None => diligent_sys::CULL_MODE_NONE,
            CullMode::Front => diligent_sys::CULL_MODE_FRONT,
            CullMode::Back => diligent_sys::CULL_MODE_BACK,
        }) as _
    }
}

#[derive(Clone, Copy)]
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

impl From<StencilOperation> for diligent_sys::STENCIL_OP {
    fn from(value: StencilOperation) -> Self {
        (match value {
            StencilOperation::Keep => diligent_sys::STENCIL_OP_KEEP,
            StencilOperation::Zero => diligent_sys::STENCIL_OP_ZERO,
            StencilOperation::Replace => diligent_sys::STENCIL_OP_REPLACE,
            StencilOperation::IncrSat => diligent_sys::STENCIL_OP_INCR_SAT,
            StencilOperation::DecrSat => diligent_sys::STENCIL_OP_DECR_SAT,
            StencilOperation::Invert => diligent_sys::STENCIL_OP_INVERT,
            StencilOperation::IncrWrap => diligent_sys::STENCIL_OP_INCR_WRAP,
            StencilOperation::DecrWrap => diligent_sys::STENCIL_OP_DECR_WRAP,
        }) as _
    }
}

#[derive(Clone, Copy)]
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

impl From<ComparisonFunction> for diligent_sys::COMPARISON_FUNCTION {
    fn from(value: ComparisonFunction) -> Self {
        (match value {
            ComparisonFunction::Never => diligent_sys::COMPARISON_FUNC_NEVER,
            ComparisonFunction::Less => diligent_sys::COMPARISON_FUNC_LESS,
            ComparisonFunction::Equal => diligent_sys::COMPARISON_FUNC_EQUAL,
            ComparisonFunction::LessEqual => diligent_sys::COMPARISON_FUNC_LESS_EQUAL,
            ComparisonFunction::Greater => diligent_sys::COMPARISON_FUNC_GREATER,
            ComparisonFunction::NotEqual => diligent_sys::COMPARISON_FUNC_NOT_EQUAL,
            ComparisonFunction::GreaterEqual => diligent_sys::COMPARISON_FUNC_GREATER_EQUAL,
            ComparisonFunction::Always => diligent_sys::COMPARISON_FUNC_ALWAYS,
        }) as _
    }
}

impl From<diligent_sys::COMPARISON_FUNCTION> for ComparisonFunction {
    fn from(value: diligent_sys::COMPARISON_FUNCTION) -> Self {
        match value as _ {
            diligent_sys::COMPARISON_FUNC_NEVER => ComparisonFunction::Never,
            diligent_sys::COMPARISON_FUNC_LESS => ComparisonFunction::Less,
            diligent_sys::COMPARISON_FUNC_EQUAL => ComparisonFunction::Equal,
            diligent_sys::COMPARISON_FUNC_LESS_EQUAL => ComparisonFunction::LessEqual,
            diligent_sys::COMPARISON_FUNC_GREATER => ComparisonFunction::Greater,
            diligent_sys::COMPARISON_FUNC_NOT_EQUAL => ComparisonFunction::NotEqual,
            diligent_sys::COMPARISON_FUNC_GREATER_EQUAL => ComparisonFunction::GreaterEqual,
            diligent_sys::COMPARISON_FUNC_ALWAYS => ComparisonFunction::Always,
            _ => panic!("Unknown comparison function"),
        }
    }
}

bitflags! {
    #[derive(Clone,Copy)]
    pub struct ShaderVariableFlags: diligent_sys::SHADER_VARIABLE_FLAGS {
        const None                           = diligent_sys::SHADER_VARIABLE_FLAG_NONE as diligent_sys::SHADER_VARIABLE_FLAGS;
        const NoDynamicBuffers               = diligent_sys::SHADER_VARIABLE_FLAG_NO_DYNAMIC_BUFFERS as diligent_sys::SHADER_VARIABLE_FLAGS;
        const GeneralInputAttachmentVk       = diligent_sys::SHADER_VARIABLE_FLAG_GENERAL_INPUT_ATTACHMENT_VK as diligent_sys::SHADER_VARIABLE_FLAGS;
        const UnfilterableFloatTextureWebgpu = diligent_sys::SHADER_VARIABLE_FLAG_UNFILTERABLE_FLOAT_TEXTURE_WEBGPU as diligent_sys::SHADER_VARIABLE_FLAGS;
        const NonFilteringSamplerWebgpu      = diligent_sys::SHADER_VARIABLE_FLAG_NON_FILTERING_SAMPLER_WEBGPU as diligent_sys::SHADER_VARIABLE_FLAGS;
    }
}
const_assert!(diligent_sys::SHADER_VARIABLE_FLAG_LAST == 8);

impl Default for ShaderVariableFlags {
    fn default() -> Self {
        ShaderVariableFlags::None
    }
}

bitflags! {
    #[derive(Clone,Copy)]
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
    #[derive(Clone,Copy)]
    pub struct PipelineStateObjectCreateFlags: diligent_sys::PSO_CREATE_FLAGS {
        const None                           = diligent_sys::PSO_CREATE_FLAG_NONE as diligent_sys::PSO_CREATE_FLAGS;
        const IgnoreMissingVariables         = diligent_sys::PSO_CREATE_FLAG_IGNORE_MISSING_VARIABLES as diligent_sys::PSO_CREATE_FLAGS;
        const IgnoreMissingImmutableSamplers = diligent_sys::PSO_CREATE_FLAG_IGNORE_MISSING_IMMUTABLE_SAMPLERS as diligent_sys::PSO_CREATE_FLAGS;
        const DontRemapShaderResources       = diligent_sys::PSO_CREATE_FLAG_DONT_REMAP_SHADER_RESOURCES as diligent_sys::PSO_CREATE_FLAGS;
        const Asynchronous                   = diligent_sys::PSO_CREATE_FLAG_ASYNCHRONOUS as diligent_sys::PSO_CREATE_FLAGS;
    }
}
const_assert!(diligent_sys::PSO_CREATE_FLAG_LAST == 8);

impl Default for PipelineStateObjectCreateFlags {
    fn default() -> Self {
        PipelineStateObjectCreateFlags::None
    }
}

bitflags! {
    #[derive(Clone,Copy)]
    pub struct PipelineShadingRateFlags: diligent_sys::PIPELINE_SHADING_RATE_FLAGS {
        const None         = diligent_sys::PIPELINE_SHADING_RATE_FLAG_NONE as diligent_sys::PIPELINE_SHADING_RATE_FLAGS;
        const PerPrimitive = diligent_sys::PIPELINE_SHADING_RATE_FLAG_PER_PRIMITIVE as diligent_sys::PIPELINE_SHADING_RATE_FLAGS;
        const TextureBased = diligent_sys::PIPELINE_SHADING_RATE_FLAG_TEXTURE_BASED as diligent_sys::PIPELINE_SHADING_RATE_FLAGS;
    }
}
const_assert!(diligent_sys::PIPELINE_SHADING_RATE_FLAG_LAST == 2);

impl Default for PipelineShadingRateFlags {
    fn default() -> Self {
        PipelineShadingRateFlags::None
    }
}

pub(crate) struct PipelineStateDescWrapper {
    _variables: Vec<diligent_sys::ShaderResourceVariableDesc>,
    _immutable_samplers: Vec<diligent_sys::ImmutableSamplerDesc>,
    psd: diligent_sys::PipelineStateDesc,
}

impl Deref for PipelineStateDescWrapper {
    type Target = diligent_sys::PipelineStateDesc;
    fn deref(&self) -> &Self::Target {
        &self.psd
    }
}

#[derive(Builder, Clone)]
#[builder(derive(Clone))]
pub struct PipelineStateCreateInfo<'a> {
    pipeline_type: diligent_sys::PIPELINE_TYPE,

    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    name: CString,

    #[builder(default = 1)]
    srb_allocation_granularity: u32,

    #[builder(default = 1)]
    immediate_context_mask: u64,

    #[builder(default = ShaderResourceVariableType::Static)]
    default_variable_type: ShaderResourceVariableType,

    default_variable_merge_stages: ShaderTypes,

    #[builder(default)]
    #[builder(into)]
    shader_resource_variables: Vec<ShaderResourceVariableDesc>,

    #[builder(default)]
    #[builder(into)]
    immutable_samplers: Vec<ImmutableSamplerDesc<'a>>,

    #[builder(default)]
    flags: PipelineStateObjectCreateFlags,

    #[builder(default)]
    #[builder(into)]
    resource_signatures: Vec<&'a PipelineResourceSignature>,

    pso_cache: Option<&'a PipelineStateCache>,
}

impl<'a, S: pipeline_state_create_info_builder::State> PipelineStateCreateInfoBuilder<'a, S>
where
    S::Name: pipeline_state_create_info_builder::IsUnset,
    S::PipelineType: pipeline_state_create_info_builder::IsUnset,
    S::DefaultVariableMergeStages: pipeline_state_create_info_builder::IsUnset,
{
    pub fn graphics(
        self,
        name: impl AsRef<str>,
    ) -> GraphicsPipelineStateCreateInfoBuilder<
        'a,
        graphics_pipeline_state_create_info_builder::SetPipelineStateCreateInfo,
    > {
        GraphicsPipelineStateCreateInfo::builder().pipeline_state_create_info(
            self.name(name)
                .pipeline_type(diligent_sys::PIPELINE_TYPE_GRAPHICS as _)
                .default_variable_merge_stages(ShaderTypes::AllGraphics)
                .build(),
        )
    }

    pub fn raytracing(
        self,
        name: impl AsRef<str>,
    ) -> RayTracingPipelineStateCreateInfoBuilder<
        'a,
        ray_tracing_pipeline_state_create_info_builder::SetPipelineStateCreateInfo,
    > {
        RayTracingPipelineStateCreateInfo::builder().pipeline_state_create_info(
            self.name(name)
                .pipeline_type(diligent_sys::PIPELINE_TYPE_RAY_TRACING as _)
                .default_variable_merge_stages(ShaderTypes::AllRayTracing)
                .build(),
        )
    }
}

pub(crate) struct PipelineStateCreateInfoWrapper {
    _psd: PipelineStateDescWrapper,
    _resource_signatures: Vec<*mut diligent_sys::IPipelineResourceSignature>,
    ci: diligent_sys::PipelineStateCreateInfo,
}

impl Deref for PipelineStateCreateInfoWrapper {
    type Target = diligent_sys::PipelineStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.ci
    }
}

impl From<&PipelineStateCreateInfo<'_>> for PipelineStateCreateInfoWrapper {
    fn from(value: &PipelineStateCreateInfo<'_>) -> Self {
        let variables: Vec<_> = value
            .shader_resource_variables
            .iter()
            .map(|var| var.into())
            .collect();

        let immutable_samplers: Vec<_> = value
            .immutable_samplers
            .iter()
            .map(|var| var.into())
            .collect();

        let prld = diligent_sys::PipelineResourceLayoutDesc {
            DefaultVariableType: value.default_variable_type.into(),
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

        let psd = PipelineStateDescWrapper {
            _variables: variables,
            _immutable_samplers: immutable_samplers,
            psd: diligent_sys::PipelineStateDesc {
                _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                    Name: value.name.as_ptr(),
                },
                ImmediateContextMask: value.immediate_context_mask,
                PipelineType: value.pipeline_type as _,
                ResourceLayout: prld,
                SRBAllocationGranularity: value.srb_allocation_granularity,
            },
        };

        let mut resource_signatures = value
            .resource_signatures
            .iter()
            .map(|&rs| rs.sys_ptr)
            .collect::<Vec<_>>();

        let ci = diligent_sys::PipelineStateCreateInfo {
            PSODesc: *psd,
            Flags: value.flags.bits(),
            ResourceSignaturesCount: value.resource_signatures.len() as u32,
            ppResourceSignatures: if value.resource_signatures.is_empty() {
                std::ptr::null_mut()
            } else {
                resource_signatures.as_mut_ptr()
            },
            pPSOCache: if let Some(pso_cache) = &value.pso_cache {
                pso_cache.sys_ptr
            } else {
                std::ptr::null_mut()
            },
            pInternalData: std::ptr::null_mut(),
        };

        PipelineStateCreateInfoWrapper {
            _psd: psd,
            _resource_signatures: resource_signatures,
            ci,
        }
    }
}

#[derive(Builder, Clone)]
pub struct RenderTargetBlendDesc {
    #[builder(default = false)]
    blend_enable: bool,

    #[builder(default = false)]
    logic_operation_enable: bool,

    #[builder(default = BlendFactor::One)]
    src_blend: BlendFactor,

    #[builder(default = BlendFactor::Zero)]
    dest_blend: BlendFactor,

    #[builder(default = BlendOperation::Add)]
    blend_op: BlendOperation,

    #[builder(default = BlendFactor::One)]
    src_blend_alpha: BlendFactor,

    #[builder(default = BlendFactor::Zero)]
    dest_blend_alpha: BlendFactor,

    #[builder(default = BlendOperation::Add)]
    blend_op_alpha: BlendOperation,

    #[builder(default = LogicOperation::NoOp)]
    logic_op: LogicOperation,

    #[builder(default = ColorMask::RGBA)]
    render_target_write_mask: ColorMask,
}

impl From<&RenderTargetBlendDesc> for diligent_sys::RenderTargetBlendDesc {
    fn from(value: &RenderTargetBlendDesc) -> Self {
        diligent_sys::RenderTargetBlendDesc {
            BlendEnable: value.blend_enable,
            LogicOperationEnable: value.logic_operation_enable,
            SrcBlend: value.src_blend.into(),
            DestBlend: value.dest_blend.into(),
            BlendOp: value.blend_op.into(),
            SrcBlendAlpha: value.src_blend_alpha.into(),
            DestBlendAlpha: value.dest_blend_alpha.into(),
            BlendOpAlpha: value.blend_op_alpha.into(),
            LogicOp: value.logic_op.into(),
            RenderTargetWriteMask: value.render_target_write_mask.bits(),
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

#[derive(Builder, Clone)]
pub struct BlendStateDesc {
    #[builder(default = false)]
    alpha_to_coverage_enable: bool,

    #[builder(default = false)]
    independent_blend_enable: bool,

    #[builder(default = std::array::from_fn(|_| RenderTargetBlendDesc::default()))]
    render_targets: [RenderTargetBlendDesc; diligent_sys::DILIGENT_MAX_RENDER_TARGETS as usize],
}

impl From<&BlendStateDesc> for diligent_sys::BlendStateDesc {
    fn from(value: &BlendStateDesc) -> Self {
        diligent_sys::BlendStateDesc {
            AlphaToCoverageEnable: value.alpha_to_coverage_enable,
            IndependentBlendEnable: value.independent_blend_enable,
            RenderTargets: value.render_targets.each_ref().map(|rt| rt.into()),
        }
    }
}

impl Default for BlendStateDesc {
    fn default() -> Self {
        BlendStateDesc {
            alpha_to_coverage_enable: false,
            independent_blend_enable: false,
            render_targets: std::array::from_fn(|_| RenderTargetBlendDesc::default()),
        }
    }
}

#[derive(Builder, Clone)]
pub struct RasterizerStateDesc {
    #[builder(default = FillMode::Solid)]
    fill_mode: FillMode,

    #[builder(default = CullMode::Back)]
    cull_mode: CullMode,

    #[builder(default = false)]
    front_counter_clockwise: bool,

    #[builder(default = true)]
    depth_clip_enable: bool,

    #[builder(default = false)]
    scissor_enable: bool,

    #[builder(default = false)]
    antialiased_line_enable: bool,

    #[builder(default = 0)]
    depth_bias: i32,

    #[builder(default = 0.0)]
    depth_bias_clamp: f32,

    #[builder(default = 0.0)]
    slope_scaled_depth_bias: f32,
}

impl From<&RasterizerStateDesc> for diligent_sys::RasterizerStateDesc {
    fn from(value: &RasterizerStateDesc) -> Self {
        diligent_sys::RasterizerStateDesc {
            FillMode: value.fill_mode.into(),
            CullMode: value.cull_mode.into(),
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

#[derive(Builder, Clone)]
pub struct StencilOperationsDesc {
    #[builder(default = StencilOperation::Keep)]
    stencil_fail_op: StencilOperation,

    #[builder(default = StencilOperation::Keep)]
    stencil_depth_fail_op: StencilOperation,

    #[builder(default = StencilOperation::Keep)]
    stencil_pass_op: StencilOperation,

    #[builder(default = ComparisonFunction::Always)]
    stencil_func: ComparisonFunction,
}

impl From<&StencilOperationsDesc> for diligent_sys::StencilOpDesc {
    fn from(value: &StencilOperationsDesc) -> Self {
        diligent_sys::StencilOpDesc {
            StencilFailOp: value.stencil_fail_op.into(),
            StencilDepthFailOp: value.stencil_depth_fail_op.into(),
            StencilPassOp: value.stencil_pass_op.into(),
            StencilFunc: value.stencil_func.into(),
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

#[derive(Builder, Clone)]
pub struct DepthStencilStateDesc {
    #[builder(default = true)]
    depth_enable: bool,

    #[builder(default = true)]
    depth_write_enable: bool,

    #[builder(default = ComparisonFunction::Less)]
    depth_func: ComparisonFunction,

    #[builder(default = false)]
    stencil_enable: bool,

    #[builder(default = 0xff)]
    stencil_read_mask: u8,

    #[builder(default = 0xff)]
    stencil_write_mask: u8,

    #[builder(default)]
    front_face: StencilOperationsDesc,

    #[builder(default)]
    back_face: StencilOperationsDesc,
}

impl From<&DepthStencilStateDesc> for diligent_sys::DepthStencilStateDesc {
    fn from(value: &DepthStencilStateDesc) -> Self {
        diligent_sys::DepthStencilStateDesc {
            DepthEnable: value.depth_enable,
            DepthWriteEnable: value.depth_write_enable,
            DepthFunc: value.depth_func.into(),
            StencilEnable: value.stencil_enable,
            StencilReadMask: value.stencil_read_mask,
            StencilWriteMask: value.stencil_write_mask,
            FrontFace: (&value.front_face).into(),
            BackFace: (&value.back_face).into(),
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

#[derive(Builder, Clone)]
pub struct GraphicsPipelineRenderTargets {
    #[builder(default = 0)]
    num_render_targets: u8,

    #[builder(default = std::array::from_fn(|_| None))]
    rtv_formats: [Option<TextureFormat>; diligent_sys::DILIGENT_MAX_RENDER_TARGETS as usize],

    dsv_format: Option<TextureFormat>,

    #[builder(default = false)]
    read_only_dsv: bool,
}

impl Default for GraphicsPipelineRenderTargets {
    fn default() -> Self {
        GraphicsPipelineRenderTargets {
            num_render_targets: 0,
            rtv_formats: std::array::from_fn(|_| None),
            dsv_format: None,
            read_only_dsv: false,
        }
    }
}

#[derive(Clone)]
pub struct GraphicsPipelineRenderPass<'a> {
    render_pass: &'a RenderPass,
    subpass_index: u8,
}

impl<'a> GraphicsPipelineRenderPass<'a> {
    pub fn new(render_pass: &'a RenderPass) -> Self {
        GraphicsPipelineRenderPass {
            render_pass,
            subpass_index: 0,
        }
    }
}

#[derive(Clone)]
pub enum GraphicsPipelineOutput<'a> {
    RenderTargets(GraphicsPipelineRenderTargets),
    RenderPass(GraphicsPipelineRenderPass<'a>),
}

impl<'a> Into<GraphicsPipelineOutput<'a>> for GraphicsPipelineRenderTargets {
    fn into(self) -> GraphicsPipelineOutput<'a> {
        GraphicsPipelineOutput::RenderTargets(self)
    }
}

impl<'a> Into<GraphicsPipelineOutput<'a>> for GraphicsPipelineRenderPass<'a> {
    fn into(self) -> GraphicsPipelineOutput<'a> {
        GraphicsPipelineOutput::RenderPass(self)
    }
}

impl Default for GraphicsPipelineOutput<'_> {
    fn default() -> Self {
        GraphicsPipelineOutput::RenderTargets(GraphicsPipelineRenderTargets::default())
    }
}

#[derive(Builder, Clone)]
#[builder(derive(Clone))]
pub struct GraphicsPipelineDesc<'a> {
    #[builder(default)]
    blend_desc: BlendStateDesc,

    #[builder(default)]
    rasterizer_desc: RasterizerStateDesc,

    #[builder(default)]
    depth_stencil_desc: DepthStencilStateDesc,

    #[builder(into)]
    #[builder(default)]
    output: GraphicsPipelineOutput<'a>,

    #[builder(default = 0xFFFFFFFF)]
    sample_mask: u32,

    #[builder(into)]
    #[builder(default)]
    input_layouts: Vec<LayoutElement>,

    #[builder(default = PrimitiveTopology::TriangleList)]
    primitive_topology: PrimitiveTopology,

    #[builder(default = 1)]
    num_viewports: u8,

    #[builder(default)]
    shading_rate_flags: PipelineShadingRateFlags,

    #[builder(default = 1)]
    sample_count: u8,

    #[builder(default = 0)]
    sample_quality: u8,

    #[builder(default = 0)]
    node_mask: u32,
}

impl Default for GraphicsPipelineDesc<'_> {
    fn default() -> Self {
        GraphicsPipelineDesc::builder().build()
    }
}

pub(crate) struct GraphicsPipelineDescWrapper {
    _input_layouts: InputLayoutDescWrapper,
    desc: diligent_sys::GraphicsPipelineDesc,
}

impl Deref for GraphicsPipelineDescWrapper {
    type Target = diligent_sys::GraphicsPipelineDesc;
    fn deref(&self) -> &Self::Target {
        &self.desc
    }
}

impl<'a> From<&GraphicsPipelineDesc<'a>> for GraphicsPipelineDescWrapper {
    fn from(value: &GraphicsPipelineDesc) -> Self {
        let input_layouts = InputLayoutDescWrapper::from(&value.input_layouts);

        let desc = diligent_sys::GraphicsPipelineDesc {
            BlendDesc: (&value.blend_desc).into(),
            SampleMask: value.sample_mask,
            RasterizerDesc: (&value.rasterizer_desc).into(),
            DepthStencilDesc: (&value.depth_stencil_desc).into(),
            InputLayout: diligent_sys::InputLayoutDesc {
                LayoutElements: if input_layouts.is_empty() {
                    std::ptr::null()
                } else {
                    input_layouts.as_ptr()
                },
                NumElements: input_layouts.len() as u32,
            },
            PrimitiveTopology: value.primitive_topology.into(),
            NumViewports: value.num_viewports,
            NumRenderTargets: match &value.output {
                GraphicsPipelineOutput::RenderPass(_) => 0,
                GraphicsPipelineOutput::RenderTargets(render_targets) => {
                    render_targets.num_render_targets
                }
            },
            ShadingRateFlags: value.shading_rate_flags.bits(),
            RTVFormats: match &value.output {
                GraphicsPipelineOutput::RenderPass(_) => std::array::from_fn(|_| {
                    diligent_sys::TEX_FORMAT_UNKNOWN as diligent_sys::TEXTURE_FORMAT
                }),
                GraphicsPipelineOutput::RenderTargets(render_targets) => {
                    render_targets.rtv_formats.map(|format| {
                        format.map_or(
                            diligent_sys::TEX_FORMAT_UNKNOWN as diligent_sys::TEXTURE_FORMAT,
                            |format| format.into(),
                        )
                    })
                }
            },
            DSVFormat: match &value.output {
                GraphicsPipelineOutput::RenderPass(_) => {
                    diligent_sys::TEX_FORMAT_UNKNOWN as diligent_sys::TEXTURE_FORMAT
                }
                GraphicsPipelineOutput::RenderTargets(render_targets) => {
                    render_targets.dsv_format.map_or(
                        diligent_sys::TEX_FORMAT_UNKNOWN as diligent_sys::TEXTURE_FORMAT,
                        |format| format.into(),
                    )
                }
            },
            ReadOnlyDSV: match &value.output {
                GraphicsPipelineOutput::RenderPass(_) => false,
                GraphicsPipelineOutput::RenderTargets(render_targets) => {
                    render_targets.read_only_dsv
                }
            },
            SmplDesc: diligent_sys::SampleDesc {
                Count: value.sample_count,
                Quality: value.sample_quality,
            },
            pRenderPass: match &value.output {
                GraphicsPipelineOutput::RenderPass(render_pass) => render_pass.render_pass.sys_ptr,
                GraphicsPipelineOutput::RenderTargets(_) => std::ptr::null_mut(),
            },
            SubpassIndex: match &value.output {
                GraphicsPipelineOutput::RenderPass(render_pass) => render_pass.subpass_index,
                GraphicsPipelineOutput::RenderTargets(_) => 0,
            },
            NodeMask: value.node_mask,
        };

        GraphicsPipelineDescWrapper {
            _input_layouts: input_layouts,
            desc,
        }
    }
}

#[derive(Builder)]
#[builder(derive(Clone))]
pub struct GraphicsPipelineStateCreateInfo<'a> {
    pipeline_state_create_info: PipelineStateCreateInfo<'a>,

    graphics_pipeline_desc: GraphicsPipelineDesc<'a>,

    vertex_shader: Option<&'a Shader>,

    pixel_shader: Option<&'a Shader>,

    domain_shader: Option<&'a Shader>,

    hull_shader: Option<&'a Shader>,

    geometry_shader: Option<&'a Shader>,

    amplification_shader: Option<&'a Shader>,

    mesh_shader: Option<&'a Shader>,
}

#[derive(Builder)]
pub struct RayTracingPipelineStateCreateInfo<'a> {
    #[builder(setters(vis = ""))]
    pipeline_state_create_info: PipelineStateCreateInfo<'a>,

    shader_record_size: u16,

    max_recursion_depth: u8,

    #[builder(with = |shaders : Vec<(impl AsRef<str>, &'a Shader)>| {
        shaders.into_iter().map(|(name, shader)| (CString::new(name.as_ref()).unwrap(), shader)).collect()
    })]
    general_shaders: Vec<(CString, &'a Shader)>,

    #[builder(with = |shaders : Vec<(impl AsRef<str>, &'a Shader, Option<&'a Shader>)>| {
        shaders.into_iter().map(|(name, closest_hit_shader, any_hit_shader)|
            (CString::new(name.as_ref()).unwrap(), closest_hit_shader, any_hit_shader)
        ).collect()
    })]
    triangle_hit_shaders: Option<Vec<(CString, &'a Shader, Option<&'a Shader>)>>,

    #[builder(with = |shaders : Vec<(impl AsRef<str>, &'a Shader, Option<&'a Shader>, Option<&'a Shader>)>| {
        shaders.into_iter().map(|(name, intersection_shader, closest_hit_shader, any_hit_shader)|
            (CString::new(name.as_ref()).unwrap(), intersection_shader, closest_hit_shader, any_hit_shader)
        ).collect()
    })]
    procedural_hit_shaders:
        Option<Vec<(CString, &'a Shader, Option<&'a Shader>, Option<&'a Shader>)>>,

    #[cfg(feature = "d3d12")]
    shader_record_name: CString,

    #[cfg(feature = "d3d12")]
    max_attribute_size: u32,

    #[cfg(feature = "d3d12")]
    max_payload_size: u32,
}

pub(crate) struct RayTracingPipelineStateCreateInfoWrapper {
    _pci: PipelineStateCreateInfoWrapper,
    _general_shaders: Vec<diligent_sys::RayTracingGeneralShaderGroup>,
    _triangle_hit_shaders: Vec<diligent_sys::RayTracingTriangleHitShaderGroup>,
    _procedural_hit_shaders: Vec<diligent_sys::RayTracingProceduralHitShaderGroup>,
    ci: diligent_sys::RayTracingPipelineStateCreateInfo,
}

impl Deref for RayTracingPipelineStateCreateInfoWrapper {
    type Target = diligent_sys::RayTracingPipelineStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.ci
    }
}

impl From<&RayTracingPipelineStateCreateInfo<'_>> for RayTracingPipelineStateCreateInfoWrapper {
    fn from(value: &RayTracingPipelineStateCreateInfo<'_>) -> Self {
        let pci = PipelineStateCreateInfoWrapper::from(&value.pipeline_state_create_info);
        let general_shaders = value
            .general_shaders
            .iter()
            .map(
                |(name, shader)| diligent_sys::RayTracingGeneralShaderGroup {
                    Name: name.as_ptr(),
                    pShader: shader.sys_ptr,
                },
            )
            .collect::<Vec<_>>();

        let triangle_hit_shaders =
            value
                .triangle_hit_shaders
                .as_ref()
                .map_or(Vec::default(), |shaders| {
                    shaders
                        .iter()
                        .map(|(name, closest_hit_shader, any_hit_shader)| {
                            diligent_sys::RayTracingTriangleHitShaderGroup {
                                Name: name.as_ptr(),
                                pClosestHitShader: closest_hit_shader.sys_ptr,
                                pAnyHitShader: any_hit_shader
                                    .map_or(std::ptr::null_mut(), |shader| shader.sys_ptr),
                            }
                        })
                        .collect()
                });

        let procedural_hit_shaders =
            value
                .procedural_hit_shaders
                .as_ref()
                .map_or(Vec::default(), |shaders| {
                    shaders
                        .iter()
                        .map(
                            |(name, intersection_shader, closest_hit_shader, any_hit_shader)| {
                                diligent_sys::RayTracingProceduralHitShaderGroup {
                                    Name: name.as_ptr(),
                                    pIntersectionShader: intersection_shader.sys_ptr,
                                    pClosestHitShader: closest_hit_shader
                                        .map_or(std::ptr::null_mut(), |shader| shader.sys_ptr),
                                    pAnyHitShader: any_hit_shader
                                        .map_or(std::ptr::null_mut(), |shader| shader.sys_ptr),
                                }
                            },
                        )
                        .collect()
                });

        let ci = diligent_sys::RayTracingPipelineStateCreateInfo {
            _PipelineStateCreateInfo: *pci,
            RayTracingPipeline: diligent_sys::RayTracingPipelineDesc {
                ShaderRecordSize: value.shader_record_size,
                MaxRecursionDepth: value.max_recursion_depth,
            },
            pGeneralShaders: general_shaders.as_ptr(),
            GeneralShaderCount: general_shaders.len() as u32,
            pTriangleHitShaders: triangle_hit_shaders.as_ptr(),
            TriangleHitShaderCount: triangle_hit_shaders.len() as u32,
            pProceduralHitShaders: procedural_hit_shaders.as_ptr(),
            ProceduralHitShaderCount: procedural_hit_shaders.len() as u32,
            #[cfg(feature = "d3d12")]
            pShaderRecordName: value.shader_record_name.as_ptr(),
            #[cfg(feature = "d3d12")]
            MaxAttributeSize: value.max_attribute_size,
            #[cfg(feature = "d3d12")]
            MaxPayloadSize: value.max_payload_size,

            #[cfg(not(feature = "d3d12"))]
            pShaderRecordName: std::ptr::null(),
            #[cfg(not(feature = "d3d12"))]
            MaxAttributeSize: 0,
            #[cfg(not(feature = "d3d12"))]
            MaxPayloadSize: 0,
        };

        Self {
            _pci: pci,
            _general_shaders: general_shaders,
            _triangle_hit_shaders: triangle_hit_shaders,
            _procedural_hit_shaders: procedural_hit_shaders,
            ci,
        }
    }
}

pub(crate) struct GraphicsPipelineStateCreateInfoWrapper {
    _pci: PipelineStateCreateInfoWrapper,
    _gpd: GraphicsPipelineDescWrapper,
    ci: diligent_sys::GraphicsPipelineStateCreateInfo,
}

impl Deref for GraphicsPipelineStateCreateInfoWrapper {
    type Target = diligent_sys::GraphicsPipelineStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.ci
    }
}

impl From<&GraphicsPipelineStateCreateInfo<'_>> for GraphicsPipelineStateCreateInfoWrapper {
    fn from(value: &GraphicsPipelineStateCreateInfo<'_>) -> Self {
        let pci = PipelineStateCreateInfoWrapper::from(&value.pipeline_state_create_info);
        let gpd = GraphicsPipelineDescWrapper::from(&value.graphics_pipeline_desc);
        let ci = diligent_sys::GraphicsPipelineStateCreateInfo {
            _PipelineStateCreateInfo: *pci,
            GraphicsPipeline: *gpd,
            pVS: value
                .vertex_shader
                .map_or(std::ptr::null_mut(), |shader| shader.sys_ptr),
            pPS: value
                .pixel_shader
                .map_or(std::ptr::null_mut(), |shader| shader.sys_ptr),
            pDS: value
                .domain_shader
                .map_or(std::ptr::null_mut(), |shader| shader.sys_ptr),
            pHS: value
                .hull_shader
                .map_or(std::ptr::null_mut(), |shader| shader.sys_ptr),
            pGS: value
                .geometry_shader
                .map_or(std::ptr::null_mut(), |shader| shader.sys_ptr),
            pAS: value
                .amplification_shader
                .map_or(std::ptr::null_mut(), |shader| shader.sys_ptr),
            pMS: value
                .mesh_shader
                .map_or(std::ptr::null_mut(), |shader| shader.sys_ptr),
        };

        GraphicsPipelineStateCreateInfoWrapper {
            _pci: pci,
            _gpd: gpd,
            ci,
        }
    }
}

pub struct PipelineState {
    pub(crate) sys_ptr: *mut diligent_sys::IPipelineState,
    virtual_functions: *mut diligent_sys::IPipelineStateVtbl,

    device_object: DeviceObject,
}

impl AsRef<DeviceObject> for PipelineState {
    fn as_ref(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl PipelineState {
    pub(crate) fn new(pipeline_state_ptr: *mut diligent_sys::IPipelineState) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IDeviceObject>()
                == std::mem::size_of::<diligent_sys::IPipelineState>()
        );

        PipelineState {
            sys_ptr: pipeline_state_ptr,
            virtual_functions: unsafe { (*pipeline_state_ptr).pVtbl },
            device_object: DeviceObject::new(
                pipeline_state_ptr as *mut diligent_sys::IDeviceObject,
            ),
        }
    }

    pub fn get_desc(&self) -> &diligent_sys::PipelineStateDesc {
        // TODO
        unsafe {
            ((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.device_object.sys_ptr)
                as *const diligent_sys::PipelineStateDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }

    pub fn get_graphics_pipeline_desc(&self) -> &diligent_sys::GraphicsPipelineDesc {
        // TODO
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .GetGraphicsPipelineDesc
                .unwrap_unchecked()(self.sys_ptr)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    pub fn get_ray_tracing_pipeline_desc(&self) -> &diligent_sys::RayTracingPipelineDesc {
        // TODO
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .GetRayTracingPipelineDesc
                .unwrap_unchecked()(self.sys_ptr)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    pub fn get_tile_pipeline_desc(&self) -> &diligent_sys::TilePipelineDesc {
        // TODO
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .GetTilePipelineDesc
                .unwrap_unchecked()(self.sys_ptr)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    pub fn bind_static_resources(
        &mut self,
        shader_type: ShaderType,
        resource_mapping: &ResourceMapping,
        flags: BindShaderResourcesFlags,
    ) {
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .BindStaticResources
                .unwrap_unchecked()(
                self.sys_ptr,
                shader_type.into(),
                resource_mapping.sys_ptr,
                flags.bits(),
            )
        }
    }

    pub fn get_static_variable_by_name(
        &self,
        shader_type: ShaderType,
        name: impl AsRef<str>,
    ) -> Option<ShaderResourceVariable> {
        let name = CString::from_str(name.as_ref()).unwrap();

        let shader_resource_variable = unsafe {
            (*self.virtual_functions)
                .PipelineState
                .GetStaticVariableByName
                .unwrap_unchecked()(self.sys_ptr, shader_type.into(), name.as_ptr())
        };

        if shader_resource_variable.is_null() {
            None
        } else {
            let srv = ShaderResourceVariable::new(shader_resource_variable);
            srv.as_ref().add_ref();
            Some(srv)
        }
    }

    pub fn create_shader_resource_binding(
        &self,
        init_static_resources: bool,
    ) -> Result<ShaderResourceBinding, ()> {
        let mut shader_resource_binding_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .CreateShaderResourceBinding
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::addr_of_mut!(shader_resource_binding_ptr),
                init_static_resources,
            );
        }
        if shader_resource_binding_ptr.is_null() {
            Err(())
        } else {
            let srb = ShaderResourceBinding::new(shader_resource_binding_ptr);
            srb.as_ref().add_ref();
            Ok(srb)
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
                .unwrap_unchecked()(self.sys_ptr, shader_resource_binding.sys_ptr)
        }
    }

    pub fn copy_static_resources(&self, pipeline_state: &mut PipelineState) {
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .CopyStaticResources
                .unwrap_unchecked()(self.sys_ptr, pipeline_state.sys_ptr)
        }
    }

    pub fn is_compatible_with(&self, pipeline_state: &PipelineState) -> bool {
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .IsCompatibleWith
                .unwrap_unchecked()(self.sys_ptr, pipeline_state.sys_ptr)
        }
    }

    pub fn get_resource_signatures(&self) -> &[PipelineResourceSignature] {
        todo!()
    }

    pub fn get_status(&self, wait_for_completion: bool) -> diligent_sys::PIPELINE_STATE_STATUS {
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .GetStatus
                .unwrap_unchecked()(self.sys_ptr, wait_for_completion)
        }
    }
}
