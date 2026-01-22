use std::ffi::CString;
use std::marker::PhantomData;
use std::str::FromStr;
use std::{ffi::CStr, ops::Deref};

use bitflags::bitflags;
use bon::Builder;
use static_assertions::const_assert_eq;

use crate::device_object::DeviceObject;
use crate::pipeline_state_cache::PipelineStateCache;
use crate::{Boxed, BoxedFromNulError, PipelineResourceFlags, PipelineType, Ported};
use crate::{
    graphics_types::{PrimitiveTopology, ShaderType, ShaderTypes, TextureFormat},
    input_layout::LayoutElement,
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
const_assert_eq!(diligent_sys::BLEND_OPERATION_NUM_OPERATIONS, 6);

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
const_assert_eq!(diligent_sys::BLEND_OPERATION_NUM_OPERATIONS, 6);

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
const_assert_eq!(diligent_sys::LOGIC_OP_NUM_OPERATIONS, 16);

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
const_assert_eq!(diligent_sys::STENCIL_OP_NUM_OPS, 9);

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
const_assert_eq!(diligent_sys::COMPARISON_FUNC_NUM_FUNCTIONS, 9);

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
const_assert_eq!(diligent_sys::SHADER_VARIABLE_FLAG_LAST, 8);

impl Default for ShaderVariableFlags {
    fn default() -> Self {
        ShaderVariableFlags::None
    }
}

impl ShaderVariableFlags {
    pub fn to_pipeline_resource_flags(&self) -> PipelineResourceFlags {
        const_assert_eq!(diligent_sys::SHADER_VARIABLE_FLAG_LAST, 1 << 3);

        let mut result = PipelineResourceFlags::None;

        if self.contains(ShaderVariableFlags::NoDynamicBuffers) {
            result |= PipelineResourceFlags::NoDynamicBuffers;
        }

        if self.contains(ShaderVariableFlags::GeneralInputAttachmentVk) {
            result |= PipelineResourceFlags::GeneralInputAttachment;
        }

        result
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
const_assert_eq!(diligent_sys::PSO_CREATE_FLAG_LAST, 8);

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
const_assert_eq!(diligent_sys::PIPELINE_SHADING_RATE_FLAG_LAST, 2);

impl Default for PipelineShadingRateFlags {
    fn default() -> Self {
        PipelineShadingRateFlags::None
    }
}

#[derive(Clone, Copy)]
pub enum PipelineStateStatus {
    Uninitialized,
    Compiling,
    Ready,
    Failed,
}

#[repr(transparent)]
pub struct PipelineResourceLayoutDesc(diligent_sys::PipelineResourceLayoutDesc);

impl PipelineResourceLayoutDesc {
    pub fn default_variable_type(&self) -> ShaderResourceVariableType {
        self.0.DefaultVariableType.into()
    }
    pub fn default_variable_merge_stages(&self) -> ShaderTypes {
        ShaderTypes::from_bits_retain(self.0.DefaultVariableMergeStages)
    }
    pub fn variables(&self) -> &[ShaderResourceVariableDesc<'_>] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.Variables as *const ShaderResourceVariableDesc,
                self.0.NumVariables as usize,
            )
        }
    }
    pub fn immutable_samplers(&self) -> &[ImmutableSamplerDesc<'_>] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.ImmutableSamplers as *const ImmutableSamplerDesc,
                self.0.NumImmutableSamplers as usize,
            )
        }
    }
}

#[repr(transparent)]
pub struct PipelineStateDesc(diligent_sys::PipelineStateDesc);

impl PipelineStateDesc {
    pub fn pipeline_type(&self) -> PipelineType {
        self.0.PipelineType.into()
    }

    pub fn srb_allocation_granularity(&self) -> u32 {
        self.0.SRBAllocationGranularity
    }

    pub fn immediate_context_mask(&self) -> u64 {
        self.0.ImmediateContextMask
    }

    pub fn resource_layout(&self) -> &PipelineResourceLayoutDesc {
        unsafe { std::mem::transmute(&self.0.ResourceLayout) }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct PipelineStateCreateInfo<'a>(
    pub(crate) diligent_sys::PipelineStateCreateInfo,
    PhantomData<&'a ()>,
);

#[bon::bon]
impl<'a> PipelineStateCreateInfo<'a> {
    #[builder(derive(Clone))]
    pub fn new(
        #[builder(setters(vis = ""))] pipeline_type: diligent_sys::PIPELINE_TYPE,

        name: Option<&'a CStr>,

        #[builder(default = 1)] srb_allocation_granularity: u32,

        #[builder(default = 1)] immediate_context_mask: u64,

        #[builder(default = ShaderResourceVariableType::Static)]
        default_variable_type: ShaderResourceVariableType,

        default_variable_merge_stages: Option<ShaderTypes>,

        #[builder(default = &[])] shader_resource_variables: &[ShaderResourceVariableDesc<'a>],

        #[builder(default = &[])] immutable_samplers: &[ImmutableSamplerDesc<'a>],

        #[builder(default)] flags: PipelineStateObjectCreateFlags,

        #[builder(default = &[])] resource_signatures: &[&'a PipelineResourceSignature],

        pso_cache: Option<&'a PipelineStateCache>,
    ) -> Self {
        PipelineStateCreateInfo(
            diligent_sys::PipelineStateCreateInfo {
                PSODesc: diligent_sys::PipelineStateDesc {
                    _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                        Name: name.map_or(std::ptr::null(), |name| name.as_ptr()),
                    },
                    ResourceLayout: diligent_sys::PipelineResourceLayoutDesc {
                        DefaultVariableType: default_variable_type.into(),
                        DefaultVariableMergeStages: default_variable_merge_stages
                            .map_or(diligent_sys::SHADER_TYPE_UNKNOWN as _, |stages| {
                                stages.bits()
                            }),
                        NumVariables: shader_resource_variables.len() as u32,
                        Variables: shader_resource_variables
                            .first()
                            .map_or(std::ptr::null(), |v| &v.0),
                        NumImmutableSamplers: immutable_samplers.len() as u32,
                        ImmutableSamplers: immutable_samplers
                            .first()
                            .map_or(std::ptr::null(), |sampler| &sampler.0),
                    },
                    ImmediateContextMask: immediate_context_mask,
                    PipelineType: pipeline_type,
                    SRBAllocationGranularity: srb_allocation_granularity,
                },
                Flags: flags.bits(),
                ResourceSignaturesCount: resource_signatures.len() as u32,
                ppResourceSignatures: if resource_signatures.is_empty() {
                    std::ptr::null_mut()
                } else {
                    resource_signatures.as_ptr() as _
                },
                pPSOCache: if let Some(pso_cache) = pso_cache {
                    pso_cache.sys_ptr()
                } else {
                    std::ptr::null_mut()
                },
                pInternalData: std::ptr::null_mut(),
            },
            PhantomData,
        )
    }
}

impl<
    'a,
    'shader_resource_variables,
    'immutable_samplers,
    'resource_signatures,
    S: pipeline_state_create_info_builder::State,
>
    PipelineStateCreateInfoBuilder<
        'a,
        'shader_resource_variables,
        'immutable_samplers,
        'resource_signatures,
        S,
    >
where
    S::PipelineType: pipeline_state_create_info_builder::IsUnset,
{
    pub fn graphics(
        self,
    ) -> GraphicsPipelineStateCreateInfoBuilder<
        'a,
        graphics_pipeline_state_create_info_builder::SetPipelineStateCreateInfo,
    > {
        GraphicsPipelineStateCreateInfo::builder().pipeline_state_create_info(
            self.pipeline_type(diligent_sys::PIPELINE_TYPE_GRAPHICS as _)
                .build(),
        )
    }

    pub fn mesh(
        self,
    ) -> GraphicsPipelineStateCreateInfoBuilder<
        'a,
        graphics_pipeline_state_create_info_builder::SetPipelineStateCreateInfo,
    > {
        GraphicsPipelineStateCreateInfo::builder().pipeline_state_create_info(
            self.pipeline_type(diligent_sys::PIPELINE_TYPE_MESH as _)
                .build(),
        )
    }

    #[cfg(feature = "d3d12")]
    pub fn raytracing<
        'general_shaders,
        'triangle_hit_shaders,
        'procedural_hit_shaders,
        'shader_record_name,
    >(
        self,
    ) -> RayTracingPipelineStateCreateInfoBuilder<
        'a,
        'general_shaders,
        'triangle_hit_shaders,
        'procedural_hit_shaders,
        'shader_record_name,
        ray_tracing_pipeline_state_create_info_builder::SetPipelineStateCreateInfo,
    > {
        RayTracingPipelineStateCreateInfo::builder().pipeline_state_create_info(
            self.pipeline_type(diligent_sys::PIPELINE_TYPE_RAY_TRACING as _)
                .build(),
        )
    }
    #[cfg(not(feature = "d3d12"))]
    pub fn raytracing<'general_shaders, 'triangle_hit_shaders, 'procedural_hit_shaders>(
        self,
    ) -> RayTracingPipelineStateCreateInfoBuilder<
        'a,
        'general_shaders,
        'triangle_hit_shaders,
        'procedural_hit_shaders,
        ray_tracing_pipeline_state_create_info_builder::SetPipelineStateCreateInfo,
    > {
        RayTracingPipelineStateCreateInfo::builder().pipeline_state_create_info(
            self.pipeline_type(diligent_sys::PIPELINE_TYPE_RAY_TRACING as _)
                .build(),
        )
    }

    pub fn tile(
        self,
    ) -> TilePipelineStateCreateInfoBuilder<
        'a,
        tile_pipeline_state_create_info_builder::SetPipelineStateCreateInfo,
    > {
        TilePipelineStateCreateInfo::builder().pipeline_state_create_info(
            self.pipeline_type(diligent_sys::PIPELINE_TYPE_TILE as _)
                .build(),
        )
    }

    pub fn compute<'shader>(
        self,
    ) -> ComputePipelineStateCreateInfoBuilder<
        'a,
        'shader,
        compute_pipeline_state_create_info_builder::SetPipelineStateCreateInfo,
    > {
        ComputePipelineStateCreateInfo::builder().pipeline_state_create_info(
            self.pipeline_type(diligent_sys::PIPELINE_TYPE_COMPUTE as _)
                .build(),
        )
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

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct BlendStateDesc(pub(crate) diligent_sys::BlendStateDesc);

#[bon::bon]
impl BlendStateDesc {
    #[builder]
    pub fn new(
        #[builder(default = false)] alpha_to_coverage_enable: bool,

        #[builder(default = false)] independent_blend_enable: bool,

        #[builder(default = std::array::from_fn(|_| RenderTargetBlendDesc::default()))]
        render_targets: [RenderTargetBlendDesc;
            diligent_sys::DILIGENT_MAX_RENDER_TARGETS as usize],
    ) -> Self {
        BlendStateDesc(diligent_sys::BlendStateDesc {
            AlphaToCoverageEnable: alpha_to_coverage_enable,
            IndependentBlendEnable: independent_blend_enable,
            RenderTargets: render_targets.each_ref().map(|rt| rt.into()),
        })
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct RasterizerStateDesc(pub(crate) diligent_sys::RasterizerStateDesc);

#[bon::bon]
impl RasterizerStateDesc {
    #[builder]
    pub fn new(
        #[builder(default = FillMode::Solid)] fill_mode: FillMode,

        #[builder(default = CullMode::Back)] cull_mode: CullMode,

        #[builder(default = false)] front_counter_clockwise: bool,

        #[builder(default = true)] depth_clip_enable: bool,

        #[builder(default = false)] scissor_enable: bool,

        #[builder(default = false)] antialiased_line_enable: bool,

        #[builder(default = 0)] depth_bias: i32,

        #[builder(default = 0.0)] depth_bias_clamp: f32,

        #[builder(default = 0.0)] slope_scaled_depth_bias: f32,
    ) -> Self {
        RasterizerStateDesc(diligent_sys::RasterizerStateDesc {
            FillMode: fill_mode.into(),
            CullMode: cull_mode.into(),
            FrontCounterClockwise: front_counter_clockwise,
            DepthClipEnable: depth_clip_enable,
            ScissorEnable: scissor_enable,
            AntialiasedLineEnable: antialiased_line_enable,
            DepthBias: depth_bias,
            DepthBiasClamp: depth_bias_clamp,
            SlopeScaledDepthBias: slope_scaled_depth_bias,
        })
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct StencilOperationsDesc(diligent_sys::StencilOpDesc);

#[bon::bon]
impl StencilOperationsDesc {
    #[builder]
    pub fn new(
        #[builder(default = StencilOperation::Keep)] stencil_fail_op: StencilOperation,

        #[builder(default = StencilOperation::Keep)] stencil_depth_fail_op: StencilOperation,

        #[builder(default = StencilOperation::Keep)] stencil_pass_op: StencilOperation,

        #[builder(default = ComparisonFunction::Always)] stencil_func: ComparisonFunction,
    ) -> Self {
        Self(diligent_sys::StencilOpDesc {
            StencilFailOp: stencil_fail_op.into(),
            StencilDepthFailOp: stencil_depth_fail_op.into(),
            StencilPassOp: stencil_pass_op.into(),
            StencilFunc: stencil_func.into(),
        })
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct DepthStencilStateDesc(pub(crate) diligent_sys::DepthStencilStateDesc);

#[bon::bon]
impl DepthStencilStateDesc {
    #[builder]
    pub fn new(
        #[builder(default = true)] depth_enable: bool,

        #[builder(default = true)] depth_write_enable: bool,

        #[builder(default = ComparisonFunction::Less)] depth_func: ComparisonFunction,

        #[builder(default = false)] stencil_enable: bool,

        #[builder(default = 0xff)] stencil_read_mask: u8,

        #[builder(default = 0xff)] stencil_write_mask: u8,

        #[builder(default = StencilOperationsDesc::builder().build())]
        front_face: StencilOperationsDesc,

        #[builder(default = StencilOperationsDesc::builder().build())]
        back_face: StencilOperationsDesc,
    ) -> Self {
        DepthStencilStateDesc(diligent_sys::DepthStencilStateDesc {
            DepthEnable: depth_enable,
            DepthWriteEnable: depth_write_enable,
            DepthFunc: depth_func.into(),
            StencilEnable: stencil_enable,
            StencilReadMask: stencil_read_mask,
            StencilWriteMask: stencil_write_mask,
            FrontFace: front_face.0,
            BackFace: back_face.0,
        })
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

impl<'a> From<GraphicsPipelineRenderTargets> for GraphicsPipelineOutput<'a> {
    fn from(value: GraphicsPipelineRenderTargets) -> Self {
        GraphicsPipelineOutput::RenderTargets(value)
    }
}

impl<'a> From<GraphicsPipelineRenderPass<'a>> for GraphicsPipelineOutput<'a> {
    fn from(value: GraphicsPipelineRenderPass<'a>) -> Self {
        GraphicsPipelineOutput::RenderPass(value)
    }
}

impl Default for GraphicsPipelineOutput<'_> {
    fn default() -> Self {
        GraphicsPipelineOutput::RenderTargets(GraphicsPipelineRenderTargets::default())
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct GraphicsPipelineDesc<'a>(
    pub(crate) diligent_sys::GraphicsPipelineDesc,
    PhantomData<&'a ()>,
);

#[bon::bon]
impl<'a> GraphicsPipelineDesc<'a> {
    #[builder]
    pub fn new(
        #[builder(default = BlendStateDesc::builder().build())] blend_desc: BlendStateDesc,

        #[builder(default = RasterizerStateDesc::builder().build())]
        rasterizer_desc: RasterizerStateDesc,

        #[builder(default = DepthStencilStateDesc::builder().build())]
        depth_stencil_desc: DepthStencilStateDesc,

        #[builder(into)]
        #[builder(default)]
        output: GraphicsPipelineOutput<'a>,

        #[builder(default = 0xFFFFFFFF)] sample_mask: u32,

        #[builder(default = &[])] input_layouts: &[LayoutElement],

        #[builder(default = PrimitiveTopology::TriangleList)] primitive_topology: PrimitiveTopology,

        #[builder(default = 1)] num_viewports: u8,

        #[builder(default)] shading_rate_flags: PipelineShadingRateFlags,

        #[builder(default = 1)] sample_count: u8,

        #[builder(default = 0)] sample_quality: u8,

        #[builder(default = 0)] node_mask: u32,
    ) -> Self {
        GraphicsPipelineDesc(
            diligent_sys::GraphicsPipelineDesc {
                BlendDesc: blend_desc.0,
                SampleMask: sample_mask,
                RasterizerDesc: rasterizer_desc.0,
                DepthStencilDesc: depth_stencil_desc.0,
                InputLayout: diligent_sys::InputLayoutDesc {
                    LayoutElements: input_layouts
                        .first()
                        .map_or(std::ptr::null(), |layout| &layout.0),
                    NumElements: input_layouts.len() as u32,
                },
                PrimitiveTopology: primitive_topology.into(),
                NumViewports: num_viewports,
                NumRenderTargets: match &output {
                    GraphicsPipelineOutput::RenderPass(_) => 0,
                    GraphicsPipelineOutput::RenderTargets(render_targets) => {
                        render_targets.num_render_targets
                    }
                },
                ShadingRateFlags: shading_rate_flags.bits(),
                RTVFormats: match &output {
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
                DSVFormat: match &output {
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
                ReadOnlyDSV: match &output {
                    GraphicsPipelineOutput::RenderPass(_) => false,
                    GraphicsPipelineOutput::RenderTargets(render_targets) => {
                        render_targets.read_only_dsv
                    }
                },
                SmplDesc: diligent_sys::SampleDesc {
                    Count: sample_count,
                    Quality: sample_quality,
                },
                pRenderPass: match &output {
                    GraphicsPipelineOutput::RenderPass(render_pass) => {
                        render_pass.render_pass.sys_ptr()
                    }
                    GraphicsPipelineOutput::RenderTargets(_) => std::ptr::null_mut(),
                },
                SubpassIndex: match &output {
                    GraphicsPipelineOutput::RenderPass(render_pass) => render_pass.subpass_index,
                    GraphicsPipelineOutput::RenderTargets(_) => 0,
                },
                NodeMask: node_mask,
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct GraphicsPipelineStateCreateInfo<'a>(
    pub(crate) diligent_sys::GraphicsPipelineStateCreateInfo,
    PhantomData<&'a ()>,
);

#[bon::bon]
impl<'a> GraphicsPipelineStateCreateInfo<'a> {
    #[builder(derive(Clone))]
    pub fn new(
        #[builder(setters(vis = ""))] pipeline_state_create_info: PipelineStateCreateInfo<'a>,

        graphics_pipeline_desc: GraphicsPipelineDesc<'a>,

        vertex_shader: Option<&'a Shader>,

        pixel_shader: Option<&'a Shader>,

        domain_shader: Option<&'a Shader>,

        hull_shader: Option<&'a Shader>,

        geometry_shader: Option<&'a Shader>,

        amplification_shader: Option<&'a Shader>,

        mesh_shader: Option<&'a Shader>,
    ) -> Self {
        GraphicsPipelineStateCreateInfo(
            diligent_sys::GraphicsPipelineStateCreateInfo {
                _PipelineStateCreateInfo: pipeline_state_create_info.0,
                GraphicsPipeline: graphics_pipeline_desc.0,
                pVS: vertex_shader.map_or(std::ptr::null_mut(), |shader| shader.sys_ptr()),
                pPS: pixel_shader.map_or(std::ptr::null_mut(), |shader| shader.sys_ptr()),
                pDS: domain_shader.map_or(std::ptr::null_mut(), |shader| shader.sys_ptr()),
                pHS: hull_shader.map_or(std::ptr::null_mut(), |shader| shader.sys_ptr()),
                pGS: geometry_shader.map_or(std::ptr::null_mut(), |shader| shader.sys_ptr()),
                pAS: amplification_shader.map_or(std::ptr::null_mut(), |shader| shader.sys_ptr()),
                pMS: mesh_shader.map_or(std::ptr::null_mut(), |shader| shader.sys_ptr()),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct RayTracingGeneralShaderGroup<'a>(
    pub(crate) diligent_sys::RayTracingGeneralShaderGroup,
    PhantomData<&'a ()>,
);

#[bon::bon]
impl<'a> RayTracingGeneralShaderGroup<'a> {
    #[builder]
    pub fn new(name: &CStr, shader: &Shader) -> Self {
        Self(
            diligent_sys::RayTracingGeneralShaderGroup {
                Name: name.as_ptr(),
                pShader: shader.sys_ptr(),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct RayTracingTriangleHitShaderGroup<'a>(
    pub(crate) diligent_sys::RayTracingTriangleHitShaderGroup,
    PhantomData<&'a ()>,
);

#[bon::bon]
impl<'a> RayTracingTriangleHitShaderGroup<'a> {
    #[builder]
    pub fn new(name: &CStr, closest: &Shader, any: Option<&'a Shader>) -> Self {
        Self(
            diligent_sys::RayTracingTriangleHitShaderGroup {
                Name: name.as_ptr(),
                pClosestHitShader: closest.sys_ptr(),
                pAnyHitShader: any.map_or(std::ptr::null_mut(), Shader::sys_ptr),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct RayTracingProceduralHitShaderGroup<'a>(
    pub(crate) diligent_sys::RayTracingProceduralHitShaderGroup,
    PhantomData<&'a ()>,
);

#[bon::bon]
impl<'a> RayTracingProceduralHitShaderGroup<'a> {
    #[builder]
    pub fn new(
        name: &CStr,
        intersection: &Shader,
        closest: Option<&'a Shader>,
        any: Option<&'a Shader>,
    ) -> Self {
        Self(
            diligent_sys::RayTracingProceduralHitShaderGroup {
                Name: name.as_ptr(),
                pIntersectionShader: intersection.sys_ptr(),
                pClosestHitShader: closest.map_or(std::ptr::null_mut(), Shader::sys_ptr),
                pAnyHitShader: any.map_or(std::ptr::null_mut(), Shader::sys_ptr),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct RayTracingPipelineStateCreateInfo<'a>(
    pub(crate) diligent_sys::RayTracingPipelineStateCreateInfo,
    PhantomData<&'a ()>,
);

#[bon::bon]
impl<'a> RayTracingPipelineStateCreateInfo<'a> {
    #[builder]
    pub fn new(
        #[builder(setters(vis = ""))] pipeline_state_create_info: PipelineStateCreateInfo<'a>,

        shader_record_size: u16,

        max_recursion_depth: u8,

        general_shaders: &[RayTracingGeneralShaderGroup<'a>],

        triangle_hit_shaders: Option<&[RayTracingTriangleHitShaderGroup<'a>]>,

        procedural_hit_shaders: Option<&[RayTracingProceduralHitShaderGroup<'a>]>,

        #[cfg(feature = "d3d12")] shader_record_name: Option<&CStr>,

        #[cfg(feature = "d3d12")]
        #[builder(default = 0)]
        max_attribute_size: u32,

        #[cfg(feature = "d3d12")]
        #[builder(default = 0)]
        max_payload_size: u32,
    ) -> Self {
        RayTracingPipelineStateCreateInfo(
            diligent_sys::RayTracingPipelineStateCreateInfo {
                _PipelineStateCreateInfo: pipeline_state_create_info.0,
                RayTracingPipeline: diligent_sys::RayTracingPipelineDesc {
                    ShaderRecordSize: shader_record_size,
                    MaxRecursionDepth: max_recursion_depth,
                },
                pGeneralShaders: general_shaders
                    .first()
                    .map_or(std::ptr::null(), |shader| &shader.0),
                GeneralShaderCount: general_shaders.len() as u32,
                pTriangleHitShaders: triangle_hit_shaders
                    .map(|shaders| shaders.first().map_or(std::ptr::null(), |shader| &shader.0))
                    .unwrap_or(std::ptr::null()),
                TriangleHitShaderCount: triangle_hit_shaders.map_or(0, |shaders| shaders.len())
                    as u32,
                pProceduralHitShaders: procedural_hit_shaders
                    .map(|shaders| shaders.first().map_or(std::ptr::null(), |shader| &shader.0))
                    .unwrap_or(std::ptr::null()),
                ProceduralHitShaderCount: procedural_hit_shaders.map_or(0, |shaders| shaders.len())
                    as u32,
                #[cfg(feature = "d3d12")]
                pShaderRecordName: shader_record_name
                    .map_or(std::ptr::null(), |name| name.as_ptr()),
                #[cfg(feature = "d3d12")]
                MaxAttributeSize: max_attribute_size,
                #[cfg(feature = "d3d12")]
                MaxPayloadSize: max_payload_size,

                #[cfg(not(feature = "d3d12"))]
                pShaderRecordName: std::ptr::null(),
                #[cfg(not(feature = "d3d12"))]
                MaxAttributeSize: 0,
                #[cfg(not(feature = "d3d12"))]
                MaxPayloadSize: 0,
            },
            PhantomData,
        )
    }
}

pub struct PipelineResourceSignatureIterator<'pipeline> {
    pipeline: &'pipeline PipelineState,
    signatures_count: usize,
    current_index: usize,
}

impl<'pipeline> Iterator for PipelineResourceSignatureIterator<'pipeline> {
    type Item = &'pipeline PipelineResourceSignature;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.signatures_count {
            return None;
        }

        let signature = unsafe_member_call!(
            self.pipeline,
            PipelineState,
            GetResourceSignature,
            self.current_index as u32
        ) as *const PipelineResourceSignature;

        self.current_index += 1;

        Some(unsafe { &*signature })
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.signatures_count
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        if self.signatures_count == 0 {
            return None;
        }

        let signature = unsafe_member_call!(
            self.pipeline,
            PipelineState,
            GetResourceSignature,
            self.signatures_count as u32 - 1
        ) as *const PipelineResourceSignature;

        Some(unsafe { &*signature })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.signatures_count - self.current_index;
        (remaining, Some(remaining))
    }
}

define_ported!(
    PipelineState,
    diligent_sys::IPipelineState,
    diligent_sys::IPipelineStateMethods : 14,
    DeviceObject
);

impl PipelineState {
    pub fn desc(&self) -> &PipelineStateDesc {
        let desc_ptr = unsafe_member_call!(self, DeviceObject, GetDesc);
        unsafe { &*(desc_ptr as *const PipelineStateDesc) }
    }

    pub fn bind_static_resources(
        &mut self,
        shader_type: ShaderType,
        resource_mapping: &ResourceMapping,
        flags: BindShaderResourcesFlags,
    ) {
        unsafe_member_call!(
            self,
            PipelineState,
            BindStaticResources,
            shader_type.into(),
            resource_mapping.sys_ptr(),
            flags.bits()
        )
    }

    pub fn get_static_variable_by_name(
        &self,
        shader_type: ShaderType,
        name: impl AsRef<str>,
    ) -> Option<&ShaderResourceVariable> {
        let name = CString::from_str(name.as_ref()).unwrap();

        let shader_resource_variable = unsafe_member_call!(
            self,
            PipelineState,
            GetStaticVariableByName,
            shader_type.into(),
            name.as_ptr()
        );

        if shader_resource_variable.is_null() {
            None
        } else {
            Some(unsafe { &*(shader_resource_variable as *const ShaderResourceVariable) })
        }
    }

    pub fn create_shader_resource_binding(
        &self,
        init_static_resources: bool,
    ) -> Result<Boxed<ShaderResourceBinding>, BoxedFromNulError> {
        let mut shader_resource_binding_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            PipelineState,
            CreateShaderResourceBinding,
            &mut shader_resource_binding_ptr,
            init_static_resources
        );

        Boxed::new(shader_resource_binding_ptr)
    }

    pub fn initialize_static_srb_resources(
        &self,
        shader_resource_binding: &mut ShaderResourceBinding,
    ) {
        unsafe_member_call!(
            self,
            PipelineState,
            InitializeStaticSRBResources,
            shader_resource_binding.sys_ptr()
        )
    }

    pub fn copy_static_resources(&self, pipeline_state: &mut PipelineState) {
        unsafe_member_call!(
            self,
            PipelineState,
            CopyStaticResources,
            pipeline_state.sys_ptr()
        )
    }

    pub fn is_compatible_with(&self, pipeline_state: &PipelineState) -> bool {
        unsafe_member_call!(
            self,
            PipelineState,
            IsCompatibleWith,
            pipeline_state.sys_ptr()
        )
    }

    pub fn resource_signatures(&self) -> PipelineResourceSignatureIterator<'_> {
        let signatures_count =
            unsafe_member_call!(self, PipelineState, GetResourceSignatureCount) as usize;
        PipelineResourceSignatureIterator {
            current_index: 0,
            pipeline: self,
            signatures_count,
        }
    }

    pub fn get_status(&self, wait_for_completion: bool) -> PipelineStateStatus {
        let status = unsafe_member_call!(self, PipelineState, GetStatus, wait_for_completion);
        match status as _ {
            diligent_sys::PIPELINE_STATE_STATUS_UNINITIALIZED => PipelineStateStatus::Uninitialized,
            diligent_sys::PIPELINE_STATE_STATUS_COMPILING => PipelineStateStatus::Compiling,
            diligent_sys::PIPELINE_STATE_STATUS_READY => PipelineStateStatus::Ready,
            diligent_sys::PIPELINE_STATE_STATUS_FAILED => PipelineStateStatus::Failed,
            _ => panic!("Unknown PIPELINE_STATE_STATUS value"),
        }
    }
}

#[repr(transparent)]
pub struct GraphicsPipelineState(PipelineState);

impl Deref for GraphicsPipelineState {
    type Target = PipelineState;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl GraphicsPipelineState {
    pub fn get_graphics_pipeline_desc(&self) -> &GraphicsPipelineDesc<'_> {
        let desc = unsafe_member_call!(self.0, PipelineState, GetGraphicsPipelineDesc)
            as *const GraphicsPipelineDesc;
        unsafe { &*desc }
    }
}

#[repr(transparent)]
pub struct ComputePipelineState(PipelineState);

impl Deref for ComputePipelineState {
    type Target = PipelineState;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[repr(transparent)]
pub struct RayTracingPipelineState(PipelineState);

impl Deref for RayTracingPipelineState {
    type Target = PipelineState;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[repr(transparent)]
pub struct RayTracingPipelineDesc(diligent_sys::RayTracingPipelineDesc);

impl RayTracingPipelineDesc {
    pub fn shader_record_size(&self) -> u16 {
        self.0.ShaderRecordSize
    }
    pub fn max_recursion_depth(&self) -> u8 {
        self.0.MaxRecursionDepth
    }
}

impl RayTracingPipelineState {
    pub fn get_raytracing_pipeline_desc(&self) -> &RayTracingPipelineDesc {
        let desc = unsafe_member_call!(self.0, PipelineState, GetRayTracingPipelineDesc)
            as *const RayTracingPipelineDesc;
        unsafe { &*desc }
    }
}

#[repr(transparent)]
pub struct TilePipelineStateCreateInfo<'a>(
    pub(crate) diligent_sys::TilePipelineStateCreateInfo,
    PhantomData<&'a ()>,
);

#[bon::bon]
impl<'a> TilePipelineStateCreateInfo<'a> {
    #[builder]
    pub fn new(
        #[builder(setters(vis = ""))] pipeline_state_create_info: PipelineStateCreateInfo<'a>,
        render_target_formats: Vec<TextureFormat>,
        sample_count: u8,

        shader: &'a Shader,
    ) -> Self {
        Self(
            diligent_sys::TilePipelineStateCreateInfo {
                _PipelineStateCreateInfo: pipeline_state_create_info.0,
                TilePipeline: diligent_sys::TilePipelineDesc {
                    NumRenderTargets: render_target_formats.len() as u8,
                    SampleCount: sample_count,
                    RTVFormats: {
                        let mut formats = [diligent_sys::TEX_FORMAT_UNKNOWN
                            as diligent_sys::TEXTURE_FORMAT;
                            diligent_sys::MAX_RENDER_TARGETS as usize];

                        render_target_formats
                            .iter()
                            .enumerate()
                            .for_each(|(index, &fmt)| formats[index] = fmt.into());
                        formats
                    },
                },
                pTS: shader.sys_ptr(),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct TilePipelineState(PipelineState);

impl Deref for TilePipelineState {
    type Target = PipelineState;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[repr(transparent)]
pub struct TilePipelineDesc(diligent_sys::TilePipelineDesc);

impl TilePipelineDesc {
    pub fn num_render_targets(&self) -> u8 {
        self.0.NumRenderTargets
    }
    pub fn sample_count(&self) -> u8 {
        self.0.SampleCount
    }
    pub fn rtv_formats(&self) -> [TextureFormat; 8usize] {
        self.0.RTVFormats.map(TextureFormat::from)
    }
}

impl TilePipelineState {
    pub fn get_tile_pipeline_desc(&self) -> &TilePipelineDesc {
        let desc = unsafe_member_call!(self.0, PipelineState, GetTilePipelineDesc)
            as *const TilePipelineDesc;
        unsafe { &*desc }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct ComputePipelineStateCreateInfo<'a>(
    pub(crate) diligent_sys::ComputePipelineStateCreateInfo,
    PhantomData<&'a ()>,
);

#[bon::bon]
impl<'a> ComputePipelineStateCreateInfo<'a> {
    #[builder]
    pub fn new(
        #[builder(setters(vis = ""))] pipeline_state_create_info: PipelineStateCreateInfo<'a>,
        shader: &Shader,
    ) -> Self {
        ComputePipelineStateCreateInfo(
            diligent_sys::ComputePipelineStateCreateInfo {
                _PipelineStateCreateInfo: pipeline_state_create_info.0,
                pCS: shader.sys_ptr(),
            },
            PhantomData,
        )
    }
}

impl Ported for GraphicsPipelineState {
    type SysType = diligent_sys::IPipelineState;
}

impl Ported for ComputePipelineState {
    type SysType = diligent_sys::IPipelineState;
}

impl Ported for RayTracingPipelineState {
    type SysType = diligent_sys::IPipelineState;
}

impl Ported for TilePipelineState {
    type SysType = diligent_sys::IPipelineState;
}
