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
    name: &'a std::ffi::CStr,
    pipeline_type: bindings::_PIPELINE_TYPE,
    srb_allocation_granularity: u32,
    immediate_context_mask: u64,
    resource_layout: PipelineResourceLayoutDesc<'a>,
}

pub struct PipelineStateCreateInfo<'a> {
    pub pso_desc: PipelineStateDesc<'a>,
    pub flags: PipelineStateObjectCreateFlags,
    pub resource_signatures: &'a [&'a PipelineResourceSignature],
    //TODO
    //pub pPSOCache: *mut IPipelineStateCache,
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

pub struct BlendStateDesc {
    pub alpha_to_coverage_enable: bool,
    pub independent_blend_enable: bool,
    pub render_targets: [RenderTargetBlendDesc; 8usize],
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

pub struct GraphicsPipelineDesc {
    blend_desc: BlendStateDesc,
    sample_mask: u32,
    rasterizer_desc: RasterizerStateDesc,
    depth_stencil_desc: DepthStencilStateDesc,
    input_layouts: Vec<bindings::LayoutElement>,
    primitive_topology: PrimitiveTopology,
    num_viewports: u8,
    num_render_targets: u8,
    subpass_index: u8,
    shading_rate_flags: PipelineShadingRateFlags,
    rtv_formats: [bindings::_TEXTURE_FORMAT; 8usize],
    dsv_format: bindings::_TEXTURE_FORMAT,
    read_only_dsv: bool,
    sample_desc: SampleDesc,
    // TODO
    // pub render_pass: Option<&RenderPass>,
    node_mask: u32,
}

impl Default for GraphicsPipelineDesc {
    fn default() -> Self {
        GraphicsPipelineDesc::new(
            BlendStateDesc::default(),
            RasterizerStateDesc::default(),
            DepthStencilStateDesc::default(),
        )
    }
}

impl GraphicsPipelineDesc {
    pub fn new(
        blend_desc: BlendStateDesc,
        rasterizer_desc: RasterizerStateDesc,
        depth_stencil_desc: DepthStencilStateDesc,
    ) -> Self {
        GraphicsPipelineDesc {
            blend_desc: blend_desc,
            sample_mask: 0xFFFFFFFF,
            rasterizer_desc: rasterizer_desc,
            depth_stencil_desc: depth_stencil_desc,
            input_layouts: Vec::default(),
            primitive_topology: PrimitiveTopology::TriangleList,
            num_viewports: 1,
            num_render_targets: 0,
            subpass_index: 0,
            shading_rate_flags: PipelineShadingRateFlags::None,
            rtv_formats: [
                bindings::TEX_FORMAT_UNKNOWN,
                bindings::TEX_FORMAT_UNKNOWN,
                bindings::TEX_FORMAT_UNKNOWN,
                bindings::TEX_FORMAT_UNKNOWN,
                bindings::TEX_FORMAT_UNKNOWN,
                bindings::TEX_FORMAT_UNKNOWN,
                bindings::TEX_FORMAT_UNKNOWN,
                bindings::TEX_FORMAT_UNKNOWN,
            ],
            dsv_format: bindings::TEX_FORMAT_UNKNOWN,
            read_only_dsv: false,
            node_mask: 0,
            sample_desc: SampleDesc {
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
    pub fn rtv_format(mut self, index: usize, value: bindings::_TEXTURE_FORMAT) -> Self {
        self.rtv_formats[index] = value;
        self
    }
    pub fn dsv_format(mut self, dsv_format: bindings::_TEXTURE_FORMAT) -> Self {
        self.dsv_format = dsv_format;
        self
    }
    pub fn read_only_dsv(mut self, read_only_dsv: bool) -> Self {
        self.read_only_dsv = read_only_dsv;
        self
    }
}

pub struct GraphicsPipelineStateCreateInfo<'a> {
    pipeline_state_create_info: PipelineStateCreateInfo<'a>,
    graphics_pipeline_desc: GraphicsPipelineDesc,
    vertex_shader: Option<&'a Shader>,
    pixel_shader: Option<&'a Shader>,
    domain_shader: Option<&'a Shader>,
    hull_shader: Option<&'a Shader>,
    geometry_shader: Option<&'a Shader>,
    amplification_shader: Option<&'a Shader>,
    mesh_shader: Option<&'a Shader>,
}

impl<'a> GraphicsPipelineStateCreateInfo<'a> {
    pub fn new(name: &'a std::ffi::CStr, graphics_pipeline_desc: GraphicsPipelineDesc) -> Self {
        GraphicsPipelineStateCreateInfo {
            pipeline_state_create_info: PipelineStateCreateInfo::new(
                name,
                bindings::PIPELINE_TYPE_GRAPHICS,
            ),
            graphics_pipeline_desc: graphics_pipeline_desc,
            vertex_shader: None,
            pixel_shader: None,
            domain_shader: None,
            hull_shader: None,
            geometry_shader: None,
            amplification_shader: None,
            mesh_shader: None,
        }
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

    pub fn get_desc(&self) -> &bindings::PipelineStateDesc {
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

    pub fn get_graphics_pipeline_desc(&self) -> &bindings::GraphicsPipelineDesc {
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .GetGraphicsPipelineDesc
                .unwrap_unchecked()(self.pipeline_state)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    pub fn get_ray_tracing_pipeline_desc(&self) -> &bindings::RayTracingPipelineDesc {
        unsafe {
            (*self.virtual_functions)
                .PipelineState
                .GetRayTracingPipelineDesc
                .unwrap_unchecked()(self.pipeline_state)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    pub fn get_tile_pipeline_desc(&self) -> &bindings::TilePipelineDesc {
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

    pub fn get_static_variables(
        &self,
        _shader_type: bindings::SHADER_TYPE,
    ) -> Option<&[ShaderResourceVariable]> {
        todo!()
    }

    pub fn create_shader_resource_binding(
        &self,
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

    pub fn get_status(&self, wait_for_completion: Option<bool>) -> bindings::PIPELINE_STATE_STATUS {
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
