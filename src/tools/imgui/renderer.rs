use std::os::raw::c_void;

use crate::{
    bindings,
    core::{
        buffer::BufferDesc,
        device_context::DeviceContext,
        graphics_types::{
            BindFlags, CpuAccessFlags, PrimitiveTopology, RenderDeviceType, SetShaderResourceFlags,
            ShaderType, Usage, ValueType,
        },
        input_layout::LayoutElement,
        pipeline_state::{
            BlendFactor, BlendOperation, BlendStateDesc, ColorMask, CullMode,
            DepthStencilStateDesc, GraphicsPipelineDesc, GraphicsPipelineStateCreateInfo,
            PipelineState, RasterizerStateDesc, RenderTargetBlendDesc,
        },
        render_device::RenderDevice,
        shader::{ShaderCreateInfo, ShaderSource},
        texture::{TextureDesc, TextureDimension},
    },
};

const GAMMA_TO_LINEAR: &std::ffi::CStr =
    c"((Gamma) < 0.04045 ? (Gamma) / 12.92 : pow(max((Gamma) + 0.055, 0.0) / 1.055, 2.4))";
const SRGBA_TO_LINEAR: &std::ffi::CStr = cr#"
col.r = GAMMA_TO_LINEAR(col.r);
col.g = GAMMA_TO_LINEAR(col.g);
col.b = GAMMA_TO_LINEAR(col.b);
col.a = 1.0 - GAMMA_TO_LINEAR(1.0 - col.a);"#;

const VERTEX_SHADER_HLSL: &str = r#"(
cbuffer Constants
{
    float4x4 ProjectionMatrix;
}

struct VSInput
{
    float2 pos : ATTRIB0;
    float2 uv  : ATTRIB1;
    float4 col : ATTRIB2;
};

struct PSInput
{
    float4 pos : SV_POSITION;
    float4 col : COLOR;
    float2 uv  : TEXCOORD;
};

void main(in VSInput VSIn, out PSInput PSIn)
{
    PSIn.pos = mul(ProjectionMatrix, float4(VSIn.pos.xy, 0.0, 1.0));
    PSIn.col = VSIn.col;
    PSIn.uv  = VSIn.uv;
}
)"#;

const PIXEL_SHADER_HLSL: &str = r#"(
struct PSInput
{
    float4 pos : SV_POSITION;
    float4 col : COLOR;
    float2 uv  : TEXCOORD;
};

Texture2D    Texture;
SamplerState Texture_sampler;

float4 main(in PSInput PSIn) : SV_Target
{
    float4 col = Texture.Sample(Texture_sampler, PSIn.uv) * PSIn.col;
    col.rgb *= col.a;
    SRGBA_TO_LINEAR(col)
    return col;
}
)"#;

const VERTEX_SHADER_GLSL: &str = r#"(
#ifdef VULKAN
#   define BINDING(X) layout(binding=X)
#   define OUT_LOCATION(X) layout(location=X) // Requires separable programs
#else
#   define BINDING(X)
#   define OUT_LOCATION(X)
#endif
BINDING(0) uniform Constants
{
    mat4 ProjectionMatrix;
};

layout(location = 0) in vec2 in_pos;
layout(location = 1) in vec2 in_uv;
layout(location = 2) in vec4 in_col;

OUT_LOCATION(0) out vec4 vsout_col;
OUT_LOCATION(1) out vec2 vsout_uv;

#ifndef GL_ES
out gl_PerVertex
{
    vec4 gl_Position;
};
#endif

void main()
{
    gl_Position = ProjectionMatrix * vec4(in_pos.xy, 0.0, 1.0);
    vsout_col = in_col;
    vsout_uv  = in_uv;
}
)"#;

const PIXEL_SHADER_GLSL: &str = r#"(
#ifdef VULKAN
#   define BINDING(X) layout(binding=X)
#   define IN_LOCATION(X) layout(location=X) // Requires separable programs
#else
#   define BINDING(X)
#   define IN_LOCATION(X)
#endif
BINDING(0) uniform sampler2D Texture;

IN_LOCATION(0) in vec4 vsout_col;
IN_LOCATION(1) in vec2 vsout_uv;

layout(location = 0) out vec4 psout_col;

void main()
{
    vec4 col = vsout_col * texture(Texture, vsout_uv);
    col.rgb *= col.a;
    SRGBA_TO_LINEAR(col)
    psout_col = col;
}
)"#;

const VERTEX_SHADER_WGSL: &str = r#"(
struct Constants {
    ProjectionMatrix: mat4x4<f32>
};

@group(0) @binding(0) var<uniform> constants: Constants;

struct VSInput {
    @location(0) pos: vec2<f32>,
    @location(1) uv:  vec2<f32>,
    @location(2) col: vec4<f32>
};

struct PSInput {
    @builtin(position) pos: vec4<f32>,
    @location(0)       col: vec4<f32>,
    @location(1)       uv:  vec2<f32>
};

@vertex
fn main(in: VSInput) -> PSInput {
    var out: PSInput;
    out.pos = constants.ProjectionMatrix * vec4<f32>(in.pos, 0.0, 1.0);
    out.col = in.col;
    out.uv = in.uv;
    return out;
}
)"#;

const PIXEL_SHADER_WGSL: &str = r#"(
struct PSInput {
    @builtin(position) pos: vec4f,
    @location(0)       col: vec4f,
    @location(1)       uv:  vec2f
};

@group(0) @binding(1) var Texture: texture_2d<f32>;
@group(0) @binding(2) var Texture_sampler: sampler;

@fragment
fn main(in: PSInput) -> @location(0) vec4f {
    var col: vec4f = textureSample(Texture, Texture_sampler, in.uv) * in.col;
    col.r *= col.a;
    col.g *= col.a;
    col.b *= col.a;
    return col;
}
)"#;

const PIXEL_SHADER_WGSL_GAMMA: &str = r#"(
struct PSInput {
    @builtin(position) pos: vec4f,
    @location(0)       col: vec4f,
    @location(1)       uv:  vec2f
};

@group(0) @binding(1) var Texture: texture_2d<f32>;
@group(0) @binding(2) var Texture_sampler: sampler;

fn SRGBToLinear(sRGB: vec4f) -> vec4f {
    let threshold  = vec4f(0.04045);
    let linearPart = sRGB / 12.92;
    let gammaPart  = pow(max(vec4f(0.0), sRGB + vec4f(0.055)) / 1.055, vec4f(2.4));
    return mix(linearPart, gammaPart, step(threshold, sRGB));
}

@fragment
fn main(in: PSInput) -> @location(0) vec4f {
    var col: vec4f = textureSample(Texture, Texture_sampler, in.uv) * in.col;
    col.r *= col.a;
    col.g *= col.a;
    col.b *= col.a;
    col = SRGBToLinear(vec4f(col.rgb, 1.0 - col.a));
    col.a = 1.0 - col.a;
    return col;
}
)"#;

const VERTEX_SHADER_SPIRV: &[u32] = &[
    0x07230203, 0x00010000, 0x0008000a, 0x00000028, 0x00000000, 0x00020011, 0x00000001, 0x0006000b,
    0x00000001, 0x4c534c47, 0x6474732e, 0x3035342e, 0x00000000, 0x0003000e, 0x00000000, 0x00000001,
    0x000b000f, 0x00000000, 0x00000004, 0x6e69616d, 0x00000000, 0x0000000a, 0x00000016, 0x00000020,
    0x00000022, 0x00000025, 0x00000026, 0x00030003, 0x00000002, 0x000001a4, 0x00040005, 0x00000004,
    0x6e69616d, 0x00000000, 0x00060005, 0x00000008, 0x505f6c67, 0x65567265, 0x78657472, 0x00000000,
    0x00060006, 0x00000008, 0x00000000, 0x505f6c67, 0x7469736f, 0x006e6f69, 0x00030005, 0x0000000a,
    0x00000000, 0x00050005, 0x0000000e, 0x736e6f43, 0x746e6174, 0x00000073, 0x00080006, 0x0000000e,
    0x00000000, 0x6a6f7250, 0x69746365, 0x614d6e6f, 0x78697274, 0x00000000, 0x00030005, 0x00000010,
    0x00000000, 0x00040005, 0x00000016, 0x705f6e69, 0x0000736f, 0x00050005, 0x00000020, 0x756f7376,
    0x6f635f74, 0x0000006c, 0x00040005, 0x00000022, 0x635f6e69, 0x00006c6f, 0x00050005, 0x00000025,
    0x756f7376, 0x76755f74, 0x00000000, 0x00040005, 0x00000026, 0x755f6e69, 0x00000076, 0x00050048,
    0x00000008, 0x00000000, 0x0000000b, 0x00000000, 0x00030047, 0x00000008, 0x00000002, 0x00040048,
    0x0000000e, 0x00000000, 0x00000005, 0x00050048, 0x0000000e, 0x00000000, 0x00000023, 0x00000000,
    0x00050048, 0x0000000e, 0x00000000, 0x00000007, 0x00000010, 0x00030047, 0x0000000e, 0x00000002,
    0x00040047, 0x00000010, 0x00000022, 0x00000000, 0x00040047, 0x00000010, 0x00000021, 0x00000000,
    0x00040047, 0x00000016, 0x0000001e, 0x00000000, 0x00040047, 0x00000020, 0x0000001e, 0x00000000,
    0x00040047, 0x00000022, 0x0000001e, 0x00000002, 0x00040047, 0x00000025, 0x0000001e, 0x00000001,
    0x00040047, 0x00000026, 0x0000001e, 0x00000001, 0x00020013, 0x00000002, 0x00030021, 0x00000003,
    0x00000002, 0x00030016, 0x00000006, 0x00000020, 0x00040017, 0x00000007, 0x00000006, 0x00000004,
    0x0003001e, 0x00000008, 0x00000007, 0x00040020, 0x00000009, 0x00000003, 0x00000008, 0x0004003b,
    0x00000009, 0x0000000a, 0x00000003, 0x00040015, 0x0000000b, 0x00000020, 0x00000001, 0x0004002b,
    0x0000000b, 0x0000000c, 0x00000000, 0x00040018, 0x0000000d, 0x00000007, 0x00000004, 0x0003001e,
    0x0000000e, 0x0000000d, 0x00040020, 0x0000000f, 0x00000002, 0x0000000e, 0x0004003b, 0x0000000f,
    0x00000010, 0x00000002, 0x00040020, 0x00000011, 0x00000002, 0x0000000d, 0x00040017, 0x00000014,
    0x00000006, 0x00000002, 0x00040020, 0x00000015, 0x00000001, 0x00000014, 0x0004003b, 0x00000015,
    0x00000016, 0x00000001, 0x0004002b, 0x00000006, 0x00000018, 0x00000000, 0x0004002b, 0x00000006,
    0x00000019, 0x3f800000, 0x00040020, 0x0000001e, 0x00000003, 0x00000007, 0x0004003b, 0x0000001e,
    0x00000020, 0x00000003, 0x00040020, 0x00000021, 0x00000001, 0x00000007, 0x0004003b, 0x00000021,
    0x00000022, 0x00000001, 0x00040020, 0x00000024, 0x00000003, 0x00000014, 0x0004003b, 0x00000024,
    0x00000025, 0x00000003, 0x0004003b, 0x00000015, 0x00000026, 0x00000001, 0x00050036, 0x00000002,
    0x00000004, 0x00000000, 0x00000003, 0x000200f8, 0x00000005, 0x00050041, 0x00000011, 0x00000012,
    0x00000010, 0x0000000c, 0x0004003d, 0x0000000d, 0x00000013, 0x00000012, 0x0004003d, 0x00000014,
    0x00000017, 0x00000016, 0x00050051, 0x00000006, 0x0000001a, 0x00000017, 0x00000000, 0x00050051,
    0x00000006, 0x0000001b, 0x00000017, 0x00000001, 0x00070050, 0x00000007, 0x0000001c, 0x0000001a,
    0x0000001b, 0x00000018, 0x00000019, 0x00050091, 0x00000007, 0x0000001d, 0x00000013, 0x0000001c,
    0x00050041, 0x0000001e, 0x0000001f, 0x0000000a, 0x0000000c, 0x0003003e, 0x0000001f, 0x0000001d,
    0x0004003d, 0x00000007, 0x00000023, 0x00000022, 0x0003003e, 0x00000020, 0x00000023, 0x0004003d,
    0x00000014, 0x00000027, 0x00000026, 0x0003003e, 0x00000025, 0x00000027, 0x000100fd, 0x00010038,
];

const PIXEL_SHADER_SPIRV: &[u32] = &[
    0x07230203, 0x00010000, 0x0008000a, 0x00000023, 0x00000000, 0x00020011, 0x00000001, 0x0006000b,
    0x00000001, 0x4c534c47, 0x6474732e, 0x3035342e, 0x00000000, 0x0003000e, 0x00000000, 0x00000001,
    0x0008000f, 0x00000004, 0x00000004, 0x6e69616d, 0x00000000, 0x00000009, 0x0000000b, 0x00000014,
    0x00030010, 0x00000004, 0x00000007, 0x00030003, 0x00000002, 0x000001a4, 0x00040005, 0x00000004,
    0x6e69616d, 0x00000000, 0x00050005, 0x00000009, 0x756f7370, 0x6f635f74, 0x0000006c, 0x00050005,
    0x0000000b, 0x756f7376, 0x6f635f74, 0x0000006c, 0x00040005, 0x00000010, 0x74786554, 0x00657275,
    0x00050005, 0x00000014, 0x756f7376, 0x76755f74, 0x00000000, 0x00040047, 0x00000009, 0x0000001e,
    0x00000000, 0x00040047, 0x0000000b, 0x0000001e, 0x00000000, 0x00040047, 0x00000010, 0x00000022,
    0x00000000, 0x00040047, 0x00000010, 0x00000021, 0x00000000, 0x00040047, 0x00000014, 0x0000001e,
    0x00000001, 0x00020013, 0x00000002, 0x00030021, 0x00000003, 0x00000002, 0x00030016, 0x00000006,
    0x00000020, 0x00040017, 0x00000007, 0x00000006, 0x00000004, 0x00040020, 0x00000008, 0x00000003,
    0x00000007, 0x0004003b, 0x00000008, 0x00000009, 0x00000003, 0x00040020, 0x0000000a, 0x00000001,
    0x00000007, 0x0004003b, 0x0000000a, 0x0000000b, 0x00000001, 0x00090019, 0x0000000d, 0x00000006,
    0x00000001, 0x00000000, 0x00000000, 0x00000000, 0x00000001, 0x00000000, 0x0003001b, 0x0000000e,
    0x0000000d, 0x00040020, 0x0000000f, 0x00000000, 0x0000000e, 0x0004003b, 0x0000000f, 0x00000010,
    0x00000000, 0x00040017, 0x00000012, 0x00000006, 0x00000002, 0x00040020, 0x00000013, 0x00000001,
    0x00000012, 0x0004003b, 0x00000013, 0x00000014, 0x00000001, 0x00040015, 0x00000018, 0x00000020,
    0x00000000, 0x0004002b, 0x00000018, 0x00000019, 0x00000003, 0x00040020, 0x0000001a, 0x00000003,
    0x00000006, 0x00040017, 0x0000001d, 0x00000006, 0x00000003, 0x00050036, 0x00000002, 0x00000004,
    0x00000000, 0x00000003, 0x000200f8, 0x00000005, 0x0004003d, 0x00000007, 0x0000000c, 0x0000000b,
    0x0004003d, 0x0000000e, 0x00000011, 0x00000010, 0x0004003d, 0x00000012, 0x00000015, 0x00000014,
    0x00050057, 0x00000007, 0x00000016, 0x00000011, 0x00000015, 0x00050085, 0x00000007, 0x00000017,
    0x0000000c, 0x00000016, 0x0003003e, 0x00000009, 0x00000017, 0x00050041, 0x0000001a, 0x0000001b,
    0x00000009, 0x00000019, 0x0004003d, 0x00000006, 0x0000001c, 0x0000001b, 0x0004003d, 0x00000007,
    0x0000001e, 0x00000009, 0x0008004f, 0x0000001d, 0x0000001f, 0x0000001e, 0x0000001e, 0x00000000,
    0x00000001, 0x00000002, 0x0005008e, 0x0000001d, 0x00000020, 0x0000001f, 0x0000001c, 0x0004003d,
    0x00000007, 0x00000021, 0x00000009, 0x0009004f, 0x00000007, 0x00000022, 0x00000021, 0x00000020,
    0x00000004, 0x00000005, 0x00000006, 0x00000003, 0x0003003e, 0x00000009, 0x00000022, 0x000100fd,
    0x00010038,
];

const PIXEL_SHADER_GAMMA_SPIRV: &[u32] = &[
    0x07230203, 0x00010000, 0x0008000a, 0x0000007b, 0x00000000, 0x00020011, 0x00000001, 0x0006000b,
    0x00000001, 0x4c534c47, 0x6474732e, 0x3035342e, 0x00000000, 0x0003000e, 0x00000000, 0x00000001,
    0x0008000f, 0x00000004, 0x00000004, 0x6e69616d, 0x00000000, 0x00000009, 0x0000000b, 0x00000014,
    0x00030010, 0x00000004, 0x00000007, 0x00030003, 0x00000002, 0x000001a4, 0x00040005, 0x00000004,
    0x6e69616d, 0x00000000, 0x00050005, 0x00000009, 0x756f7370, 0x6f635f74, 0x0000006c, 0x00050005,
    0x0000000b, 0x756f7376, 0x6f635f74, 0x0000006c, 0x00040005, 0x00000010, 0x74786554, 0x00657275,
    0x00050005, 0x00000014, 0x756f7376, 0x76755f74, 0x00000000, 0x00040047, 0x00000009, 0x0000001e,
    0x00000000, 0x00040047, 0x0000000b, 0x0000001e, 0x00000000, 0x00040047, 0x00000010, 0x00000022,
    0x00000000, 0x00040047, 0x00000010, 0x00000021, 0x00000000, 0x00040047, 0x00000014, 0x0000001e,
    0x00000001, 0x00020013, 0x00000002, 0x00030021, 0x00000003, 0x00000002, 0x00030016, 0x00000006,
    0x00000020, 0x00040017, 0x00000007, 0x00000006, 0x00000004, 0x00040020, 0x00000008, 0x00000003,
    0x00000007, 0x0004003b, 0x00000008, 0x00000009, 0x00000003, 0x00040020, 0x0000000a, 0x00000001,
    0x00000007, 0x0004003b, 0x0000000a, 0x0000000b, 0x00000001, 0x00090019, 0x0000000d, 0x00000006,
    0x00000001, 0x00000000, 0x00000000, 0x00000000, 0x00000001, 0x00000000, 0x0003001b, 0x0000000e,
    0x0000000d, 0x00040020, 0x0000000f, 0x00000000, 0x0000000e, 0x0004003b, 0x0000000f, 0x00000010,
    0x00000000, 0x00040017, 0x00000012, 0x00000006, 0x00000002, 0x00040020, 0x00000013, 0x00000001,
    0x00000012, 0x0004003b, 0x00000013, 0x00000014, 0x00000001, 0x00040015, 0x00000018, 0x00000020,
    0x00000000, 0x0004002b, 0x00000018, 0x00000019, 0x00000003, 0x00040020, 0x0000001a, 0x00000003,
    0x00000006, 0x00040017, 0x0000001d, 0x00000006, 0x00000003, 0x0004002b, 0x00000018, 0x00000023,
    0x00000000, 0x0004002b, 0x00000006, 0x00000026, 0x3d25aee6, 0x00020014, 0x00000027, 0x00040020,
    0x00000029, 0x00000007, 0x00000006, 0x0004002b, 0x00000006, 0x0000002f, 0x414eb852, 0x0004002b,
    0x00000006, 0x00000034, 0x3d6147ae, 0x0004002b, 0x00000006, 0x00000036, 0x00000000, 0x0004002b,
    0x00000006, 0x00000038, 0x3f870a3d, 0x0004002b, 0x00000006, 0x0000003a, 0x4019999a, 0x0004002b,
    0x00000018, 0x0000003e, 0x00000001, 0x0004002b, 0x00000018, 0x00000051, 0x00000002, 0x0004002b,
    0x00000006, 0x00000064, 0x3f800000, 0x00050036, 0x00000002, 0x00000004, 0x00000000, 0x00000003,
    0x000200f8, 0x00000005, 0x0004003b, 0x00000029, 0x0000002a, 0x00000007, 0x0004003b, 0x00000029,
    0x00000042, 0x00000007, 0x0004003b, 0x00000029, 0x00000055, 0x00000007, 0x0004003b, 0x00000029,
    0x00000069, 0x00000007, 0x0004003d, 0x00000007, 0x0000000c, 0x0000000b, 0x0004003d, 0x0000000e,
    0x00000011, 0x00000010, 0x0004003d, 0x00000012, 0x00000015, 0x00000014, 0x00050057, 0x00000007,
    0x00000016, 0x00000011, 0x00000015, 0x00050085, 0x00000007, 0x00000017, 0x0000000c, 0x00000016,
    0x0003003e, 0x00000009, 0x00000017, 0x00050041, 0x0000001a, 0x0000001b, 0x00000009, 0x00000019,
    0x0004003d, 0x00000006, 0x0000001c, 0x0000001b, 0x0004003d, 0x00000007, 0x0000001e, 0x00000009,
    0x0008004f, 0x0000001d, 0x0000001f, 0x0000001e, 0x0000001e, 0x00000000, 0x00000001, 0x00000002,
    0x0005008e, 0x0000001d, 0x00000020, 0x0000001f, 0x0000001c, 0x0004003d, 0x00000007, 0x00000021,
    0x00000009, 0x0009004f, 0x00000007, 0x00000022, 0x00000021, 0x00000020, 0x00000004, 0x00000005,
    0x00000006, 0x00000003, 0x0003003e, 0x00000009, 0x00000022, 0x00050041, 0x0000001a, 0x00000024,
    0x00000009, 0x00000023, 0x0004003d, 0x00000006, 0x00000025, 0x00000024, 0x000500b8, 0x00000027,
    0x00000028, 0x00000025, 0x00000026, 0x000300f7, 0x0000002c, 0x00000000, 0x000400fa, 0x00000028,
    0x0000002b, 0x00000031, 0x000200f8, 0x0000002b, 0x00050041, 0x0000001a, 0x0000002d, 0x00000009,
    0x00000023, 0x0004003d, 0x00000006, 0x0000002e, 0x0000002d, 0x00050088, 0x00000006, 0x00000030,
    0x0000002e, 0x0000002f, 0x0003003e, 0x0000002a, 0x00000030, 0x000200f9, 0x0000002c, 0x000200f8,
    0x00000031, 0x00050041, 0x0000001a, 0x00000032, 0x00000009, 0x00000023, 0x0004003d, 0x00000006,
    0x00000033, 0x00000032, 0x00050081, 0x00000006, 0x00000035, 0x00000033, 0x00000034, 0x0007000c,
    0x00000006, 0x00000037, 0x00000001, 0x00000028, 0x00000035, 0x00000036, 0x00050088, 0x00000006,
    0x00000039, 0x00000037, 0x00000038, 0x0007000c, 0x00000006, 0x0000003b, 0x00000001, 0x0000001a,
    0x00000039, 0x0000003a, 0x0003003e, 0x0000002a, 0x0000003b, 0x000200f9, 0x0000002c, 0x000200f8,
    0x0000002c, 0x0004003d, 0x00000006, 0x0000003c, 0x0000002a, 0x00050041, 0x0000001a, 0x0000003d,
    0x00000009, 0x00000023, 0x0003003e, 0x0000003d, 0x0000003c, 0x00050041, 0x0000001a, 0x0000003f,
    0x00000009, 0x0000003e, 0x0004003d, 0x00000006, 0x00000040, 0x0000003f, 0x000500b8, 0x00000027,
    0x00000041, 0x00000040, 0x00000026, 0x000300f7, 0x00000044, 0x00000000, 0x000400fa, 0x00000041,
    0x00000043, 0x00000048, 0x000200f8, 0x00000043, 0x00050041, 0x0000001a, 0x00000045, 0x00000009,
    0x0000003e, 0x0004003d, 0x00000006, 0x00000046, 0x00000045, 0x00050088, 0x00000006, 0x00000047,
    0x00000046, 0x0000002f, 0x0003003e, 0x00000042, 0x00000047, 0x000200f9, 0x00000044, 0x000200f8,
    0x00000048, 0x00050041, 0x0000001a, 0x00000049, 0x00000009, 0x0000003e, 0x0004003d, 0x00000006,
    0x0000004a, 0x00000049, 0x00050081, 0x00000006, 0x0000004b, 0x0000004a, 0x00000034, 0x0007000c,
    0x00000006, 0x0000004c, 0x00000001, 0x00000028, 0x0000004b, 0x00000036, 0x00050088, 0x00000006,
    0x0000004d, 0x0000004c, 0x00000038, 0x0007000c, 0x00000006, 0x0000004e, 0x00000001, 0x0000001a,
    0x0000004d, 0x0000003a, 0x0003003e, 0x00000042, 0x0000004e, 0x000200f9, 0x00000044, 0x000200f8,
    0x00000044, 0x0004003d, 0x00000006, 0x0000004f, 0x00000042, 0x00050041, 0x0000001a, 0x00000050,
    0x00000009, 0x0000003e, 0x0003003e, 0x00000050, 0x0000004f, 0x00050041, 0x0000001a, 0x00000052,
    0x00000009, 0x00000051, 0x0004003d, 0x00000006, 0x00000053, 0x00000052, 0x000500b8, 0x00000027,
    0x00000054, 0x00000053, 0x00000026, 0x000300f7, 0x00000057, 0x00000000, 0x000400fa, 0x00000054,
    0x00000056, 0x0000005b, 0x000200f8, 0x00000056, 0x00050041, 0x0000001a, 0x00000058, 0x00000009,
    0x00000051, 0x0004003d, 0x00000006, 0x00000059, 0x00000058, 0x00050088, 0x00000006, 0x0000005a,
    0x00000059, 0x0000002f, 0x0003003e, 0x00000055, 0x0000005a, 0x000200f9, 0x00000057, 0x000200f8,
    0x0000005b, 0x00050041, 0x0000001a, 0x0000005c, 0x00000009, 0x00000051, 0x0004003d, 0x00000006,
    0x0000005d, 0x0000005c, 0x00050081, 0x00000006, 0x0000005e, 0x0000005d, 0x00000034, 0x0007000c,
    0x00000006, 0x0000005f, 0x00000001, 0x00000028, 0x0000005e, 0x00000036, 0x00050088, 0x00000006,
    0x00000060, 0x0000005f, 0x00000038, 0x0007000c, 0x00000006, 0x00000061, 0x00000001, 0x0000001a,
    0x00000060, 0x0000003a, 0x0003003e, 0x00000055, 0x00000061, 0x000200f9, 0x00000057, 0x000200f8,
    0x00000057, 0x0004003d, 0x00000006, 0x00000062, 0x00000055, 0x00050041, 0x0000001a, 0x00000063,
    0x00000009, 0x00000051, 0x0003003e, 0x00000063, 0x00000062, 0x00050041, 0x0000001a, 0x00000065,
    0x00000009, 0x00000019, 0x0004003d, 0x00000006, 0x00000066, 0x00000065, 0x00050083, 0x00000006,
    0x00000067, 0x00000064, 0x00000066, 0x000500b8, 0x00000027, 0x00000068, 0x00000067, 0x00000026,
    0x000300f7, 0x0000006b, 0x00000000, 0x000400fa, 0x00000068, 0x0000006a, 0x00000070, 0x000200f8,
    0x0000006a, 0x00050041, 0x0000001a, 0x0000006c, 0x00000009, 0x00000019, 0x0004003d, 0x00000006,
    0x0000006d, 0x0000006c, 0x00050083, 0x00000006, 0x0000006e, 0x00000064, 0x0000006d, 0x00050088,
    0x00000006, 0x0000006f, 0x0000006e, 0x0000002f, 0x0003003e, 0x00000069, 0x0000006f, 0x000200f9,
    0x0000006b, 0x000200f8, 0x00000070, 0x00050041, 0x0000001a, 0x00000071, 0x00000009, 0x00000019,
    0x0004003d, 0x00000006, 0x00000072, 0x00000071, 0x00050083, 0x00000006, 0x00000073, 0x00000064,
    0x00000072, 0x00050081, 0x00000006, 0x00000074, 0x00000073, 0x00000034, 0x0007000c, 0x00000006,
    0x00000075, 0x00000001, 0x00000028, 0x00000074, 0x00000036, 0x00050088, 0x00000006, 0x00000076,
    0x00000075, 0x00000038, 0x0007000c, 0x00000006, 0x00000077, 0x00000001, 0x0000001a, 0x00000076,
    0x0000003a, 0x0003003e, 0x00000069, 0x00000077, 0x000200f9, 0x0000006b, 0x000200f8, 0x0000006b,
    0x0004003d, 0x00000006, 0x00000078, 0x00000069, 0x00050083, 0x00000006, 0x00000079, 0x00000064,
    0x00000078, 0x00050041, 0x0000001a, 0x0000007a, 0x00000009, 0x00000019, 0x0003003e, 0x0000007a,
    0x00000079, 0x000100fd, 0x00010038,
];

pub struct ImguiRenderer {
    context: imgui::Context,
    pipeline_state: PipelineState,
}

#[derive(Eq, PartialEq)]
pub enum ColorConversionMode {
    Auto,
    SrgbToLinear,
    None,
}

pub struct ImguiRendererCreateInfo<'a> {
    device: &'a RenderDevice,

    back_buffer_format: bindings::TEXTURE_FORMAT,
    depth_buffer_format: bindings::TEXTURE_FORMAT,

    color_conversion: ColorConversionMode,

    initial_width: f32,
    initial_height: f32,
}

impl<'a> ImguiRendererCreateInfo<'a> {
    pub fn new(
        device: &'a RenderDevice,
        back_buffer_format: bindings::TEXTURE_FORMAT,
        depth_buffer_format: bindings::TEXTURE_FORMAT,
        initial_width: u16,
        initial_height: u16,
    ) -> Self {
        ImguiRendererCreateInfo {
            device: &device,
            back_buffer_format,
            depth_buffer_format,
            color_conversion: ColorConversionMode::Auto,
            initial_height: initial_height as f32,
            initial_width: initial_width as f32,
        }
    }

    pub fn color_conversion(mut self, color_conversion: ColorConversionMode) -> Self {
        self.color_conversion = color_conversion;
        self
    }
}

impl ImguiRenderer {
    pub fn new(create_info: ImguiRendererCreateInfo) -> Self {
        let mut imgui_context = imgui::Context::create();

        let srgb_framebuffer = true;
        //let srgb_framebuffer = GetTextureFormatAttribs(create_info.back_buffer_format)
        //    .ComponentType
        //    == bindings::COMPONENT_TYPE_UNORM_SRGB;
        let manual_srgb = (create_info.color_conversion == ColorConversionMode::Auto
            && srgb_framebuffer)
            || (create_info.color_conversion == ColorConversionMode::SrgbToLinear);

        let device_info = create_info.device.get_device_info();
        let device_type = device_info.device_type();

        let vertex_shader = {
            let shader_ci = {
                let shader_source = match device_type {
                    RenderDeviceType::VULKAN => ShaderSource::ByteCode(
                        VERTEX_SHADER_SPIRV.as_ptr() as *const c_void,
                        std::mem::size_of_val(VERTEX_SHADER_SPIRV),
                    ),
                    RenderDeviceType::D3D11 | RenderDeviceType::D3D12 => {
                        ShaderSource::SourceCode(VERTEX_SHADER_HLSL)
                    }
                    RenderDeviceType::GL | RenderDeviceType::GLES => {
                        ShaderSource::SourceCode(VERTEX_SHADER_GLSL)
                    }
                    RenderDeviceType::WEBGPU => ShaderSource::SourceCode(VERTEX_SHADER_WGSL),
                    RenderDeviceType::METAL => {
                        todo!()
                    }
                };

                let shader_ci =
                    ShaderCreateInfo::new(c"Imgui VS", shader_source, ShaderType::Vertex);

                if manual_srgb {
                    shader_ci
                        .add_macro(c"GAMMA_TO_LINEAR(Gamma)", GAMMA_TO_LINEAR)
                        .add_macro(c"SRGBA_TO_LINEAR(col)", SRGBA_TO_LINEAR)
                } else {
                    shader_ci.add_macro(c"SRGBA_TO_LINEAR(col)", c"")
                }
            };
            create_info.device.create_shader(shader_ci).unwrap()
        };

        let pixel_shader = {
            let shader_source = match device_type {
                RenderDeviceType::VULKAN => {
                    if manual_srgb {
                        ShaderSource::ByteCode(
                            PIXEL_SHADER_GAMMA_SPIRV.as_ptr() as *const c_void,
                            std::mem::size_of_val(PIXEL_SHADER_GAMMA_SPIRV),
                        )
                    } else {
                        ShaderSource::ByteCode(
                            PIXEL_SHADER_SPIRV.as_ptr() as *const c_void,
                            std::mem::size_of_val(PIXEL_SHADER_SPIRV),
                        )
                    }
                }
                RenderDeviceType::D3D11 | RenderDeviceType::D3D12 => {
                    ShaderSource::SourceCode(PIXEL_SHADER_HLSL)
                }
                RenderDeviceType::GL | RenderDeviceType::GLES => {
                    ShaderSource::SourceCode(PIXEL_SHADER_GLSL)
                }
                RenderDeviceType::WEBGPU => {
                    if manual_srgb {
                        ShaderSource::SourceCode(PIXEL_SHADER_WGSL_GAMMA)
                    } else {
                        ShaderSource::SourceCode(PIXEL_SHADER_WGSL)
                    }
                }
                RenderDeviceType::METAL => {
                    todo!()
                }
            };

            let shader_ci = ShaderCreateInfo::new(c"Imgui PS", shader_source, ShaderType::Pixel);
            create_info.device.create_shader(shader_ci).unwrap()
        };

        let pipeline_state_ci = GraphicsPipelineStateCreateInfo::new(
            c"ImGUI PSO",
            GraphicsPipelineDesc::new(
                BlendStateDesc::default().render_target_blend_desc::<0>(
                    RenderTargetBlendDesc::default()
                        .blend_enable(true)
                        .src_blend(BlendFactor::One)
                        .dest_blend(BlendFactor::InvSrcAlpha)
                        .blend_op(BlendOperation::Add)
                        .src_blend_alpha(BlendFactor::One)
                        .dest_blend_alpha(BlendFactor::InvSrcAlpha)
                        .blend_op_alpha(BlendOperation::Add)
                        .render_target_write_mask(ColorMask::all()),
                ),
                RasterizerStateDesc::default()
                    .cull_mode(CullMode::None)
                    .scissor_enable(false),
                DepthStencilStateDesc::default().depth_enable(false),
            )
            .num_render_targets(1)
            .rtv_format::<0>(create_info.back_buffer_format as bindings::_TEXTURE_FORMAT)
            .dsv_format(create_info.depth_buffer_format as bindings::_TEXTURE_FORMAT)
            .primitive_topology(PrimitiveTopology::TriangleList)
            .add_input_layout(LayoutElement::new(0, 0, 2, ValueType::Float32))
            .add_input_layout(LayoutElement::new(1, 0, 2, ValueType::Float32))
            .add_input_layout(LayoutElement::new(2, 0, 4, ValueType::Uint8).is_normalized(true)),
        )
        .vertex_shader(&vertex_shader)
        .pixel_shader(&pixel_shader);

        // ShaderResourceVariableDesc Variables[] =
        //    {
        //        {SHADER_TYPE_PIXEL, "Texture", SHADER_RESOURCE_VARIABLE_TYPE_DYNAMIC} //
        //    };
        // PSOCreateInfo.PSODesc.ResourceLayout.Variables    = Variables;
        // PSOCreateInfo.PSODesc.ResourceLayout.NumVariables = _countof(Variables);

        // SamplerDesc SamLinearWrap;
        // SamLinearWrap.AddressU = TEXTURE_ADDRESS_WRAP;
        // SamLinearWrap.AddressV = TEXTURE_ADDRESS_WRAP;
        // SamLinearWrap.AddressW = TEXTURE_ADDRESS_WRAP;
        // ImmutableSamplerDesc ImtblSamplers[] =
        //    {
        //        {SHADER_TYPE_PIXEL, "Texture", SamLinearWrap} //
        //    };
        // PSOCreateInfo.PSODesc.ResourceLayout.ImmutableSamplers    = ImtblSamplers;
        // PSOCreateInfo.PSODesc.ResourceLayout.NumImmutableSamplers = _countof(ImtblSamplers);

        let pipeline_state = create_info
            .device
            .create_graphics_pipeline_state(pipeline_state_ci)
            .unwrap();

        let vertex_constant_buffer = {
            let buffer_desc = BufferDesc::new(
                c"Imgui Vertex Constant Buffer",
                (std::mem::size_of::<f32>() * 4 * 4) as u64,
            )
            .usage(Usage::Dynamic)
            .bind_flags(BindFlags::UniformBuffer)
            .cpu_access_flags(CpuAccessFlags::Write);
            create_info.device.create_buffer(buffer_desc, None).unwrap()
        };

        pipeline_state
            .get_static_variable_by_name(ShaderType::Vertex, c"Constants")
            .unwrap()
            .set(&vertex_constant_buffer, SetShaderResourceFlags::None);

        // Build texture atlas
        let font_atlas = imgui_context.fonts();
        let font_atlas_texture = font_atlas.build_rgba32_texture();

        // unsigned char* pData  = nullptr;
        // int            Width  = 0;
        // int            Weight = 0;
        // IO.Fonts->GetTexDataAsRGBA32(&pData, &Width, &Weight);

        let font_texture_desc = TextureDesc::new(
            c"Imgui font texture",
            TextureDimension::Texture2D,
            font_atlas_texture.width,
            font_atlas_texture.height,
            bindings::TEX_FORMAT_RGBA8_UNORM,
        )
        .bind_flags(BindFlags::ShaderResourcec)
        .usage(Usage::Immutable);

        // TextureSubResData Mip0Data[] = {{pData, 4 * Uint64{FontTexDesc.Width}}};
        // TextureData       InitData(Mip0Data, _countof(Mip0Data));

        // RefCntAutoPtr<ITexture> pFontTex;
        // m_pDevice->CreateTexture(FontTexDesc, &InitData, &pFontTex);
        // m_pFontSRV = pFontTex->GetDefaultView(TEXTURE_VIEW_SHADER_RESOURCE);

        // m_pSRB.Release();
        // m_pPSO->CreateShaderResourceBinding(&m_pSRB, true);
        // m_pTextureVar = m_pSRB->GetVariableByName(SHADER_TYPE_PIXEL, "Texture");
        // VERIFY_EXPR(m_pTextureVar != nullptr);

        // // Store our identifier
        // IO.Fonts->TexID = (ImTextureID)m_pFontSRV;

        imgui_context.io_mut().display_size =
            [create_info.initial_width, create_info.initial_height];

        ImguiRenderer {
            context: imgui_context,
            pipeline_state,
        }
    }

    #[inline]
    pub fn context(&self) -> &imgui::Context {
        &self.context
    }

    #[inline]
    pub fn context_mut(&mut self) -> &mut imgui::Context {
        &mut self.context
    }

    pub fn render(&mut self, device_context: &DeviceContext) {
        let draw_data = self.context.render();
        // TODO : RenderDrawData
    }
}
