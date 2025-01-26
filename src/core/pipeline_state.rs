use bitflags::bitflags;
use static_assertions::const_assert;

use super::device_object::{AsDeviceObject, DeviceObject};
use super::graphics_types::{PrimitiveTopology, ShaderTypes};
use super::pipeline_resource_signature::{ImmutableSamplerDesc, PipelineResourceSignature};
use super::resource_mapping::ResourceMapping;
use super::shader::Shader;
use super::shader_resource_binding::ShaderResourceBinding;
use super::shader_resource_variable::{
    ShaderResourceVariable, ShaderResourceVariableDesc, ShaderResourceVariableType,
};
use crate::bindings::{self, SampleDesc};

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
const_assert!(bindings::BLEND_OPERATION_NUM_OPERATIONS == 6);

impl Into<bindings::BLEND_FACTOR> for BlendFactor {
    fn into(self) -> bindings::BLEND_FACTOR {
        (match self {
            BlendFactor::Zero => bindings::BLEND_FACTOR_ZERO,
            BlendFactor::One => bindings::BLEND_FACTOR_ONE,
            BlendFactor::SrcColor => bindings::BLEND_FACTOR_SRC_COLOR,
            BlendFactor::InvSrcColor => bindings::BLEND_FACTOR_INV_SRC_COLOR,
            BlendFactor::SrcAlpha => bindings::BLEND_FACTOR_SRC_ALPHA,
            BlendFactor::InvSrcAlpha => bindings::BLEND_FACTOR_INV_SRC_ALPHA,
            BlendFactor::DestAlpha => bindings::BLEND_FACTOR_DEST_ALPHA,
            BlendFactor::InvDestAlpha => bindings::BLEND_FACTOR_INV_DEST_ALPHA,
            BlendFactor::DestColor => bindings::BLEND_FACTOR_DEST_COLOR,
            BlendFactor::InvDestColor => bindings::BLEND_FACTOR_INV_DEST_COLOR,
            BlendFactor::SrcAlphaSat => bindings::BLEND_FACTOR_SRC_ALPHA_SAT,
            BlendFactor::BlendFactor => bindings::BLEND_FACTOR_BLEND_FACTOR,
            BlendFactor::InvBlendFactor => bindings::BLEND_FACTOR_INV_BLEND_FACTOR,
            BlendFactor::Src1Color => bindings::BLEND_FACTOR_SRC1_COLOR,
            BlendFactor::InvSrc1Color => bindings::BLEND_FACTOR_INV_SRC1_COLOR,
            BlendFactor::Src1Alpha => bindings::BLEND_FACTOR_SRC1_ALPHA,
            BlendFactor::InvSrc1Alpha => bindings::BLEND_FACTOR_INV_SRC1_ALPHA,
        }) as bindings::BLEND_FACTOR
    }
}

pub enum BlendOperation {
    Add,
    Subtract,
    RevSubtract,
    Min,
    Max,
}
const_assert!(bindings::BLEND_OPERATION_NUM_OPERATIONS == 6);

impl Into<bindings::BLEND_OPERATION> for BlendOperation {
    fn into(self) -> bindings::BLEND_OPERATION {
        (match self {
            BlendOperation::Add => bindings::BLEND_OPERATION_ADD,
            BlendOperation::Subtract => bindings::BLEND_OPERATION_SUBTRACT,
            BlendOperation::RevSubtract => bindings::BLEND_OPERATION_REV_SUBTRACT,
            BlendOperation::Min => bindings::BLEND_OPERATION_MIN,
            BlendOperation::Max => bindings::BLEND_OPERATION_MAX,
        }) as bindings::BLEND_OPERATION
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
const_assert!(bindings::LOGIC_OP_NUM_OPERATIONS == 16);

impl Into<bindings::LOGIC_OPERATION> for LogicOperation {
    fn into(self) -> bindings::LOGIC_OPERATION {
        (match self {
            LogicOperation::Clear => bindings::LOGIC_OP_CLEAR,
            LogicOperation::Set => bindings::LOGIC_OP_SET,
            LogicOperation::Copy => bindings::LOGIC_OP_COPY,
            LogicOperation::CopyInverted => bindings::LOGIC_OP_COPY_INVERTED,
            LogicOperation::NoOp => bindings::LOGIC_OP_NOOP,
            LogicOperation::Invert => bindings::LOGIC_OP_INVERT,
            LogicOperation::And => bindings::LOGIC_OP_AND,
            LogicOperation::Nand => bindings::LOGIC_OP_NAND,
            LogicOperation::Or => bindings::LOGIC_OP_OR,
            LogicOperation::Nor => bindings::LOGIC_OP_NOR,
            LogicOperation::Xor => bindings::LOGIC_OP_XOR,
            LogicOperation::Equiv => bindings::LOGIC_OP_EQUIV,
            LogicOperation::AndReverse => bindings::LOGIC_OP_AND_REVERSE,
            LogicOperation::AndInverted => bindings::LOGIC_OP_AND_INVERTED,
            LogicOperation::OrReverse => bindings::LOGIC_OP_OR_REVERSE,
            LogicOperation::OrInverted => bindings::LOGIC_OP_OR_INVERTED,
        }) as bindings::LOGIC_OPERATION
    }
}

pub enum FillMode {
    Wireframe,
    Solid,
}

impl Into<bindings::FILL_MODE> for FillMode {
    fn into(self) -> bindings::FILL_MODE {
        (match self {
            FillMode::Wireframe => bindings::FILL_MODE_WIREFRAME,
            FillMode::Solid => bindings::FILL_MODE_SOLID,
        }) as bindings::FILL_MODE
    }
}

pub enum CullMode {
    None,
    Front,
    Back,
}

impl Into<bindings::CULL_MODE> for CullMode {
    fn into(self) -> bindings::CULL_MODE {
        (match self {
            CullMode::None => bindings::CULL_MODE_NONE,
            CullMode::Front => bindings::CULL_MODE_FRONT,
            CullMode::Back => bindings::CULL_MODE_BACK,
        }) as bindings::CULL_MODE
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
const_assert!(bindings::STENCIL_OP_NUM_OPS == 9);

impl Into<bindings::STENCIL_OP> for StencilOperation {
    fn into(self) -> bindings::STENCIL_OP {
        (match self {
            StencilOperation::Keep => bindings::STENCIL_OP_KEEP,
            StencilOperation::Zero => bindings::STENCIL_OP_ZERO,
            StencilOperation::Replace => bindings::STENCIL_OP_REPLACE,
            StencilOperation::IncrSat => bindings::STENCIL_OP_INCR_SAT,
            StencilOperation::DecrSat => bindings::STENCIL_OP_DECR_SAT,
            StencilOperation::Invert => bindings::STENCIL_OP_INVERT,
            StencilOperation::IncrWrap => bindings::STENCIL_OP_INCR_WRAP,
            StencilOperation::DecrWrap => bindings::STENCIL_OP_DECR_WRAP,
        }) as bindings::STENCIL_OP
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
const_assert!(bindings::COMPARISON_FUNC_NUM_FUNCTIONS == 9);

impl Into<bindings::COMPARISON_FUNCTION> for ComparisonFunction {
    fn into(self) -> bindings::COMPARISON_FUNCTION {
        (match self {
            ComparisonFunction::Never => bindings::COMPARISON_FUNC_NEVER,
            ComparisonFunction::Less => bindings::COMPARISON_FUNC_LESS,
            ComparisonFunction::Equal => bindings::COMPARISON_FUNC_EQUAL,
            ComparisonFunction::LessEqual => bindings::COMPARISON_FUNC_LESS_EQUAL,
            ComparisonFunction::Greater => bindings::COMPARISON_FUNC_GREATER,
            ComparisonFunction::NotEqual => bindings::COMPARISON_FUNC_NOT_EQUAL,
            ComparisonFunction::GreaterEqual => bindings::COMPARISON_FUNC_GREATER_EQUAL,
            ComparisonFunction::Always => bindings::COMPARISON_FUNC_ALWAYS,
        }) as bindings::COMPARISON_FUNCTION
    }
}

bitflags! {
    pub struct ShaderVariableFlags: bindings::_SHADER_VARIABLE_FLAGS {
        const None                           = bindings::SHADER_VARIABLE_FLAG_NONE;
        const NoDynamicBuffers               = bindings::SHADER_VARIABLE_FLAG_NO_DYNAMIC_BUFFERS;
        const GeneralInputAttachmentVk       = bindings::SHADER_VARIABLE_FLAG_GENERAL_INPUT_ATTACHMENT_VK;
        const UnfilterableFloatTextureWebgpu = bindings::SHADER_VARIABLE_FLAG_UNFILTERABLE_FLOAT_TEXTURE_WEBGPU;
        const NonFilteringSamplerWebgpu      = bindings::SHADER_VARIABLE_FLAG_NON_FILTERING_SAMPLER_WEBGPU;
    }
}
const_assert!(bindings::SHADER_VARIABLE_FLAG_LAST == 8);

bitflags! {
    pub struct ColorMask: bindings::_COLOR_MASK {
        const NONE  = bindings::COLOR_MASK_NONE;
        const RED   = bindings::COLOR_MASK_RED;
        const GREEN = bindings::COLOR_MASK_GREEN;
        const BLUE  = bindings::COLOR_MASK_BLUE;
        const ALPHA = bindings::COLOR_MASK_ALPHA;
        const RGB   = bindings::COLOR_MASK_RGB;
        const RGBA  = bindings::COLOR_MASK_ALL;
    }
}

bitflags! {
    pub struct PipelineStateObjectCreateFlags: bindings::_PSO_CREATE_FLAGS {
        const None                           = bindings::PSO_CREATE_FLAG_NONE;
        const IgnoreMissingVariables         = bindings::PSO_CREATE_FLAG_IGNORE_MISSING_VARIABLES;
        const IgnoreMissingImmutableSamplers = bindings::PSO_CREATE_FLAG_IGNORE_MISSING_IMMUTABLE_SAMPLERS;
        const DontRemapShaderResources       = bindings::PSO_CREATE_FLAG_DONT_REMAP_SHADER_RESOURCES;
        const Asynchronous                   = bindings::PSO_CREATE_FLAG_ASYNCHRONOUS;
    }
}
const_assert!(bindings::PSO_CREATE_FLAG_LAST == 8);

bitflags! {
    pub struct PipelineShadingRateFlags: bindings::_PIPELINE_SHADING_RATE_FLAGS {
        const None         = bindings::PIPELINE_SHADING_RATE_FLAG_NONE;
        const PerPrimitive = bindings::PIPELINE_SHADING_RATE_FLAG_PER_PRIMITIVE;
        const TextureBased = bindings::PIPELINE_SHADING_RATE_FLAG_TEXTURE_BASED;
    }
}
const_assert!(bindings::PIPELINE_SHADING_RATE_FLAG_LAST == 2);

pub struct PipelineResourceLayoutDesc<'a> {
    pub default_variable_type: ShaderResourceVariableType,
    pub default_variable_merge_stages: ShaderTypes,
    pub variables: Vec<ShaderResourceVariableDesc<'a>>,
    pub immutable_samplers: Vec<ImmutableSamplerDesc<'a>>,
}

pub struct PipelineStateDesc<'a> {
    pub name: &'a std::ffi::CStr,
    pipeline_type: bindings::_PIPELINE_TYPE,
    pub srb_allocation_granularity: u32,
    pub immediate_context_mask: u64,
    pub resource_layout: PipelineResourceLayoutDesc<'a>,
}

pub struct PipelineStateCreateInfo<'a> {
    pub pso_desc: PipelineStateDesc<'a>,
    pub flags: PipelineStateObjectCreateFlags,
    pub resource_signatures: &'a [&'a PipelineResourceSignature],
    //TODO
    //pub pPSOCache: *mut IPipelineStateCache,
}

pub struct RenderTargetBlendDesc {
    pub blend_enable: bool,
    pub logic_operation_enable: bool,
    pub src_blend: BlendFactor,
    pub dest_blend: BlendFactor,
    pub blend_op: BlendOperation,
    pub src_blend_alpha: BlendFactor,
    pub dest_blend_alpha: BlendFactor,
    pub blend_op_alpha: BlendOperation,
    pub logic_op: LogicOperation,
    pub render_target_write_mask: ColorMask,
}

pub struct BlendStateDesc {
    pub alpha_to_coverage_enable: bool,
    pub independent_blend_enable: bool,
    pub render_targets: [RenderTargetBlendDesc; 8usize],
}

pub struct RasterizerStateDesc {
    pub fill_mode: FillMode,
    pub cull_mode: CullMode,
    pub front_counter_clockwise: bool,
    pub depth_clip_enable: bool,
    pub scissor_enable: bool,
    pub antialiased_line_enable: bool,
    pub depth_bias: i32,
    pub depth_bias_clamp: f32,
    pub slope_scaled_depth_bias: f32,
}

pub struct StencilOperationsDesc {
    pub stencil_fail_op: StencilOperation,
    pub stencil_depth_fail_op: StencilOperation,
    pub stencil_pass_op: StencilOperation,
    pub stencil_func: ComparisonFunction,
}

pub struct DepthStencilStateDesc {
    pub depth_enable: bool,
    pub depth_write_enable: bool,
    pub depth_func: ComparisonFunction,
    pub stencil_enable: bool,
    pub stencil_read_mask: u8,
    pub stencil_write_mask: u8,
    pub front_face: StencilOperationsDesc,
    pub back_face: StencilOperationsDesc,
}

pub struct GraphicsPipelineDesc {
    pub blend_desc: BlendStateDesc,
    pub sample_mask: u32,
    pub rasterizer_desc: RasterizerStateDesc,
    pub depth_stencil_desc: DepthStencilStateDesc,
    pub input_layouts: Vec<bindings::LayoutElement>,
    pub primitive_topology: PrimitiveTopology,
    pub num_viewports: u8,
    pub num_render_targets: u8,
    pub subpass_index: u8,
    pub shading_rate_flags: PipelineShadingRateFlags,
    pub rtv_formats: [bindings::_TEXTURE_FORMAT; 8usize],
    pub dsv_format: bindings::_TEXTURE_FORMAT,
    pub read_only_dsv: bool,
    pub sample_desc: SampleDesc,
    // TODO
    // pub render_pass: Option<&RenderPass>,
    pub node_mask: u32,
}

pub struct GraphicsPipelineStateCreateInfo<'a> {
    pub pipeline_state_create_info: PipelineStateCreateInfo<'a>,
    pub graphics_pipeline_desc: GraphicsPipelineDesc,
    pub vertex_shader: Option<&'a Shader>,
    pub pixel_shader: Option<&'a Shader>,
    pub domain_shader: Option<&'a Shader>,
    pub hull_shader: Option<&'a Shader>,
    pub geometry_shader: Option<&'a Shader>,
    pub amplification_shader: Option<&'a Shader>,
    pub mesh_shader: Option<&'a Shader>,
}

impl<'a> GraphicsPipelineStateCreateInfo<'a> {
    pub fn new(name: &'a std::ffi::CStr) -> Self {
        GraphicsPipelineStateCreateInfo {
            pipeline_state_create_info: PipelineStateCreateInfo::new(
                name,
                bindings::PIPELINE_TYPE_GRAPHICS,
            ),
            graphics_pipeline_desc: GraphicsPipelineDesc::default(),
            vertex_shader: None,
            pixel_shader: None,
            domain_shader: None,
            hull_shader: None,
            geometry_shader: None,
            amplification_shader: None,
            mesh_shader: None,
        }
    }
}

impl<'a> Into<bindings::GraphicsPipelineStateCreateInfo> for GraphicsPipelineStateCreateInfo<'a> {
    fn into(self) -> bindings::GraphicsPipelineStateCreateInfo {
        bindings::GraphicsPipelineStateCreateInfo {
            _PipelineStateCreateInfo: self.pipeline_state_create_info.into(),
            GraphicsPipeline: self.graphics_pipeline_desc.into(),
            pVS: self
                .vertex_shader
                .map_or(std::ptr::null_mut(), |shader| shader.shader),
            pPS: self
                .pixel_shader
                .map_or(std::ptr::null_mut(), |shader| shader.shader),
            pDS: self
                .domain_shader
                .map_or(std::ptr::null_mut(), |shader| shader.shader),
            pHS: self
                .hull_shader
                .map_or(std::ptr::null_mut(), |shader| shader.shader),
            pGS: self
                .geometry_shader
                .map_or(std::ptr::null_mut(), |shader| shader.shader),
            pAS: self
                .amplification_shader
                .map_or(std::ptr::null_mut(), |shader| shader.shader),
            pMS: self
                .mesh_shader
                .map_or(std::ptr::null_mut(), |shader| shader.shader),
        }
    }
}

impl<'a> Into<bindings::PipelineStateCreateInfo> for PipelineStateCreateInfo<'a> {
    fn into(self) -> bindings::PipelineStateCreateInfo {
        bindings::PipelineStateCreateInfo {
            PSODesc: self.pso_desc.into(),
            Flags: self.flags.bits(),
            ResourceSignaturesCount: self.resource_signatures.len() as u32,
            ppResourceSignatures: if self.resource_signatures.is_empty() {
                std::ptr::null_mut()
            } else {
                self.resource_signatures
                    .iter()
                    .map(|rs| rs.pipeline_resource_signature)
                    .collect::<Vec<*mut bindings::IPipelineResourceSignature>>()
                    .as_mut_ptr()
            },
            pPSOCache: std::ptr::null_mut(), // TODO
            pInternalData: std::ptr::null_mut(),
        }
    }
}
impl<'a> Into<bindings::PipelineStateDesc> for PipelineStateDesc<'a> {
    fn into(self) -> bindings::PipelineStateDesc {
        bindings::PipelineStateDesc {
            _DeviceObjectAttribs: bindings::DeviceObjectAttribs {
                Name: self.name.as_ptr(),
            },
            PipelineType: self.pipeline_type as u8,
            SRBAllocationGranularity: self.srb_allocation_granularity,
            ImmediateContextMask: self.immediate_context_mask,
            ResourceLayout: self.resource_layout.into(),
        }
    }
}

impl Into<bindings::GraphicsPipelineDesc> for GraphicsPipelineDesc {
    fn into(self) -> bindings::GraphicsPipelineDesc {
        bindings::GraphicsPipelineDesc {
            BlendDesc: self.blend_desc.into(),
            SampleMask: self.sample_mask,
            RasterizerDesc: self.rasterizer_desc.into(),
            DepthStencilDesc: self.depth_stencil_desc.into(),
            InputLayout: bindings::InputLayoutDesc {
                LayoutElements: if self.input_layouts.is_empty() {
                    std::ptr::null()
                } else {
                    self.input_layouts.as_ptr()
                },
                NumElements: self.input_layouts.len() as u32,
            },
            PrimitiveTopology: self.primitive_topology.into(),
            NumViewports: self.num_viewports,
            NumRenderTargets: self.num_render_targets,
            SubpassIndex: self.subpass_index,
            ShadingRateFlags: self.shading_rate_flags.bits()
                as bindings::PIPELINE_SHADING_RATE_FLAGS,
            RTVFormats: self
                .rtv_formats
                .map(|format| format as bindings::TEXTURE_FORMAT),
            DSVFormat: self.dsv_format as bindings::TEXTURE_FORMAT,
            ReadOnlyDSV: self.read_only_dsv,
            SmplDesc: self.sample_desc.into(),
            pRenderPass: std::ptr::null_mut(),
            NodeMask: self.node_mask,
        }
    }
}

impl Into<bindings::BlendStateDesc> for BlendStateDesc {
    fn into(self) -> bindings::BlendStateDesc {
        bindings::BlendStateDesc {
            AlphaToCoverageEnable: self.alpha_to_coverage_enable,
            IndependentBlendEnable: self.independent_blend_enable,
            RenderTargets: self.render_targets.map(|rt| rt.into()),
        }
    }
}

impl<'a> Into<bindings::PipelineResourceLayoutDesc> for PipelineResourceLayoutDesc<'a> {
    fn into(self) -> bindings::PipelineResourceLayoutDesc {
        bindings::PipelineResourceLayoutDesc {
            DefaultVariableType: self.default_variable_type.into(),
            DefaultVariableMergeStages: self.default_variable_merge_stages.bits(),
            NumVariables: self.variables.len() as u32,
            Variables: if self.variables.is_empty() {
                std::ptr::null()
            } else {
                self.variables
                    .into_iter()
                    .map(|var| var.into())
                    .collect::<Vec<bindings::ShaderResourceVariableDesc>>()
                    .as_ptr()
            },
            NumImmutableSamplers: self.immutable_samplers.len() as u32,
            ImmutableSamplers: if self.immutable_samplers.is_empty() {
                std::ptr::null()
            } else {
                self.immutable_samplers
                    .into_iter()
                    .map(|var| var.into())
                    .collect::<Vec<bindings::ImmutableSamplerDesc>>()
                    .as_ptr()
            },
        }
    }
}

impl Into<bindings::RasterizerStateDesc> for RasterizerStateDesc {
    fn into(self) -> bindings::RasterizerStateDesc {
        bindings::RasterizerStateDesc {
            FillMode: self.fill_mode.into(),
            CullMode: self.cull_mode.into(),
            FrontCounterClockwise: self.front_counter_clockwise,
            DepthClipEnable: self.depth_clip_enable,
            ScissorEnable: self.scissor_enable,
            AntialiasedLineEnable: self.antialiased_line_enable,
            DepthBias: self.depth_bias,
            DepthBiasClamp: self.depth_bias_clamp,
            SlopeScaledDepthBias: self.slope_scaled_depth_bias,
        }
    }
}

impl Into<bindings::DepthStencilStateDesc> for DepthStencilStateDesc {
    fn into(self) -> bindings::DepthStencilStateDesc {
        bindings::DepthStencilStateDesc {
            DepthEnable: self.depth_enable,
            DepthWriteEnable: self.depth_write_enable,
            DepthFunc: self.depth_func.into(),
            StencilEnable: self.stencil_enable,
            StencilReadMask: self.stencil_read_mask,
            StencilWriteMask: self.stencil_write_mask,
            FrontFace: self.front_face.into(),
            BackFace: self.back_face.into(),
        }
    }
}

impl Into<bindings::StencilOpDesc> for StencilOperationsDesc {
    fn into(self) -> bindings::StencilOpDesc {
        bindings::StencilOpDesc {
            StencilFailOp: self.stencil_fail_op.into(),
            StencilDepthFailOp: self.stencil_depth_fail_op.into(),
            StencilPassOp: self.stencil_pass_op.into(),
            StencilFunc: self.stencil_func.into(),
        }
    }
}

impl Into<bindings::RenderTargetBlendDesc> for RenderTargetBlendDesc {
    fn into(self) -> bindings::RenderTargetBlendDesc {
        bindings::RenderTargetBlendDesc {
            BlendEnable: self.blend_enable,
            LogicOperationEnable: self.logic_operation_enable,
            SrcBlend: self.src_blend.into(),
            DestBlend: self.dest_blend.into(),
            BlendOp: self.blend_op.into(),
            SrcBlendAlpha: self.src_blend_alpha.into(),
            DestBlendAlpha: self.dest_blend_alpha.into(),
            BlendOpAlpha: self.blend_op_alpha.into(),
            LogicOp: self.logic_op.into(),
            RenderTargetWriteMask: self.render_target_write_mask.bits() as bindings::COLOR_MASK,
        }
    }
}

impl<'a> PipelineStateCreateInfo<'a> {
    fn new(name: &'a std::ffi::CStr, pipeline_type: bindings::_PIPELINE_TYPE) -> Self {
        PipelineStateCreateInfo {
            pso_desc: PipelineStateDesc::new(name, pipeline_type),
            flags: PipelineStateObjectCreateFlags::None,
            resource_signatures: &[],
        }
    }
}

impl Default for GraphicsPipelineDesc {
    fn default() -> Self {
        GraphicsPipelineDesc {
            blend_desc: BlendStateDesc::default(),
            sample_mask: 0xFFFFFFFF,
            rasterizer_desc: RasterizerStateDesc::default(),
            depth_stencil_desc: DepthStencilStateDesc::default(),
            input_layouts: Vec::default(),
            primitive_topology: PrimitiveTopology::TriangleList,
            num_viewports: 1,
            num_render_targets: 0,
            subpass_index: 0,
            shading_rate_flags: PipelineShadingRateFlags::None,
            rtv_formats: [
                bindings::TEX_FORMAT_RGBA8_UNORM_SRGB,
                bindings::TEX_FORMAT_RGBA32_FLOAT,
                bindings::TEX_FORMAT_RGBA32_FLOAT,
                bindings::TEX_FORMAT_RGBA32_FLOAT,
                bindings::TEX_FORMAT_RGBA32_FLOAT,
                bindings::TEX_FORMAT_RGBA32_FLOAT,
                bindings::TEX_FORMAT_RGBA32_FLOAT,
                bindings::TEX_FORMAT_RGBA32_FLOAT,
            ],
            dsv_format: bindings::TEX_FORMAT_D32_FLOAT,
            read_only_dsv: false,
            node_mask: 0,
            sample_desc: SampleDesc {
                Count: 1,
                Quality: 0,
            },
        }
    }
}

impl<'a> PipelineStateDesc<'a> {
    fn new(name: &'a std::ffi::CStr, pipeline_type: bindings::_PIPELINE_TYPE) -> Self {
        PipelineStateDesc {
            name: name,
            pipeline_type: pipeline_type,
            srb_allocation_granularity: 1,
            immediate_context_mask: 1,
            resource_layout: PipelineResourceLayoutDesc::new(pipeline_type),
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

impl<'a> PipelineResourceLayoutDesc<'a> {
    fn new(pipeline_type: bindings::_PIPELINE_TYPE) -> Self {
        PipelineResourceLayoutDesc {
            default_variable_type: ShaderResourceVariableType::Static,
            default_variable_merge_stages: match pipeline_type {
                bindings::PIPELINE_TYPE_GRAPHICS => ShaderTypes::AllGraphics,
                bindings::PIPELINE_TYPE_COMPUTE => ShaderTypes::Compute,
                bindings::PIPELINE_TYPE_MESH => ShaderTypes::AllMesh,
                bindings::PIPELINE_TYPE_RAY_TRACING => ShaderTypes::AllRayTracing,
                bindings::PIPELINE_TYPE_TILE => ShaderTypes::Tile,
                _ => panic!("Unknown shader type"),
            },
            variables: Vec::new(),
            immutable_samplers: Vec::new(),
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

pub struct PipelineState {
    pub(crate) pipeline_state: *mut bindings::IPipelineState,
    virtual_functions: *mut bindings::IPipelineStateVtbl,

    device_object: DeviceObject,
}

impl AsDeviceObject for PipelineState {
    fn as_device_object(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl PipelineState {
    pub(crate) fn new(pipeline_state_ptr: *mut bindings::IPipelineState) -> Self {
        PipelineState {
            pipeline_state: pipeline_state_ptr,
            virtual_functions: unsafe { (*pipeline_state_ptr).pVtbl },
            device_object: DeviceObject::new(pipeline_state_ptr as *mut bindings::IDeviceObject),
        }
    }

    fn get_desc(&self) -> &bindings::PipelineStateDesc {
        unsafe {
            ((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.pipeline_state as *mut bindings::IDeviceObject)
                as *const bindings::PipelineStateDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }

    fn get_graphics_pipeline_desc(&self) -> &bindings::GraphicsPipelineDesc {
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .GetGraphicsPipelineDesc
                .unwrap_unchecked()(self.pipeline_state)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    fn get_ray_tracing_pipeline_desc(&self) -> &bindings::RayTracingPipelineDesc {
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .GetRayTracingPipelineDesc
                .unwrap_unchecked()(self.pipeline_state)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    fn get_tile_pipeline_desc(&self) -> &bindings::TilePipelineDesc {
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .GetTilePipelineDesc
                .unwrap_unchecked()(self.pipeline_state)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    fn bind_static_resources(
        &mut self,
        shader_type: bindings::SHADER_TYPE,
        resource_mapping: &ResourceMapping,
        flags: bindings::BIND_SHADER_RESOURCES_FLAGS,
    ) {
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .BindStaticResources
                .unwrap_unchecked()(
                self.pipeline_state,
                shader_type,
                resource_mapping.resource_mapping,
                flags,
            )
        }
    }

    fn get_static_variables(
        &self,
        shader_type: bindings::SHADER_TYPE,
    ) -> Option<&[ShaderResourceVariable]> {
        todo!()
    }

    fn create_shader_resource_binding(
        &mut self,
        init_static_resources: Option<bool>,
    ) -> Option<ShaderResourceBinding> {
        let mut shader_resource_binding_ptr: *mut bindings::IShaderResourceBinding =
            std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .CreateShaderResourceBinding
                .unwrap_unchecked()(
                self.pipeline_state,
                std::ptr::addr_of_mut!(shader_resource_binding_ptr),
                init_static_resources.unwrap_or(false),
            );
        }
        if shader_resource_binding_ptr.is_null() {
            None
        } else {
            Some(ShaderResourceBinding::new(shader_resource_binding_ptr))
        }
    }

    fn initialize_static_srb_resources(&self, shader_resource_binding: &mut ShaderResourceBinding) {
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

    fn copy_static_resources(&self, pipeline_state: &mut PipelineState) {
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .CopyStaticResources
                .unwrap_unchecked()(self.pipeline_state, pipeline_state.pipeline_state)
        }
    }

    fn is_compatible_with(&self, pipeline_state: &PipelineState) -> bool {
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .IsCompatibleWith
                .unwrap_unchecked()(self.pipeline_state, pipeline_state.pipeline_state)
        }
    }

    fn get_resource_signatures(&self) -> &[PipelineResourceSignature] {
        todo!()
    }

    fn get_status(&self, wait_for_completion: Option<bool>) -> bindings::PIPELINE_STATE_STATUS {
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
