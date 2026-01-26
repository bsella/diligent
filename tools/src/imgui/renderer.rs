use bon::Builder;
use diligent::*;

use imgui::{
    TextureId,
    internal::RawWrapper,
    sys::{ImDrawIdx, ImDrawVert},
};

const GAMMA_TO_LINEAR: &str =
    "((Gamma) < 0.04045 ? (Gamma) / 12.92 : pow(max((Gamma) + 0.055, 0.0) / 1.055, 2.4))";
const SRGBA_TO_LINEAR: &str = r#"\
col.r = GAMMA_TO_LINEAR(col.r);\
col.g = GAMMA_TO_LINEAR(col.g);\
col.b = GAMMA_TO_LINEAR(col.b);\
col.a = 1.0 - GAMMA_TO_LINEAR(1.0 - col.a);"#;

#[cfg(any(feature = "d3d11", feature = "d3d12"))]
const VERTEX_SHADER_HLSL: &str = r#"
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
"#;

#[cfg(any(feature = "d3d11", feature = "d3d12"))]
const PIXEL_SHADER_HLSL: &str = r#"
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
"#;

#[cfg(feature = "opengl")]
const VERTEX_SHADER_GLSL: &str = r#"
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
"#;

#[cfg(feature = "opengl")]
const PIXEL_SHADER_GLSL: &str = r#"
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
"#;

#[cfg(feature = "webgpu")]
const VERTEX_SHADER_WGSL: &str = r#"
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
"#;

#[cfg(feature = "webgpu")]
const PIXEL_SHADER_WGSL: &str = r#"
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
"#;

#[cfg(feature = "webgpu")]
const PIXEL_SHADER_WGSL_GAMMA: &str = r#"
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
"#;

#[cfg(feature = "vulkan")]
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

#[cfg(feature = "vulkan")]
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

#[cfg(feature = "vulkan")]
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

struct ImguiRendererData {
    pipeline_state: Boxed<GraphicsPipelineState>,
    _font_texture_view: Boxed<TextureView>,
    texture_var: Boxed<ShaderResourceVariable>,
    shader_resource_binding: Boxed<ShaderResourceBinding>,

    vertex_constant_buffer: Boxed<Buffer>,

    base_vertex_supported: bool,

    vertex_buffer: Option<Boxed<Buffer>>,
    vertex_buffer_size: u32,
    index_buffer: Option<Boxed<Buffer>>,
    index_buffer_size: u32,
}

pub struct ImguiRenderer {
    context: imgui::SuspendedContext,
    data: ImguiRendererData,
}

pub struct ImguiFrame {
    context: imgui::Context,
    data: ImguiRendererData,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ColorConversionMode {
    Auto,
    SrgbToLinear,
    None,
}

#[derive(Builder)]
pub struct ImguiRendererCreateInfo<'device> {
    device: &'device RenderDevice,

    back_buffer_format: TextureFormat,
    depth_buffer_format: TextureFormat,

    initial_width: f32,
    initial_height: f32,

    #[builder(default = ColorConversionMode::Auto)]
    color_conversion: ColorConversionMode,
}

impl ImguiRenderer {
    pub fn new(create_info: &ImguiRendererCreateInfo) -> Self {
        let mut imgui_context = imgui::Context::create();

        let srgb_framebuffer = create_info
            .back_buffer_format
            .component_type()
            .is_some_and(|comp_type| comp_type == ComponentType::UnormSRGB);

        let manual_srgb = (create_info.color_conversion == ColorConversionMode::Auto
            && srgb_framebuffer)
            || (create_info.color_conversion == ColorConversionMode::SrgbToLinear);

        let device_info = create_info.device.get_device_info();
        let device_type = device_info.device_type();

        let macros = if manual_srgb {
            vec![
                ("GAMMA_TO_LINEAR(Gamma)", GAMMA_TO_LINEAR),
                ("SRGBA_TO_LINEAR(col)", SRGBA_TO_LINEAR),
            ]
        } else {
            vec![("SRGBA_TO_LINEAR(col)", "")]
        };

        let shader_ci = ShaderCreateInfo::builder()
            .use_combined_texture_samplers(true)
            .macros(macros);

        let vertex_shader = {
            let shader_source = match device_type {
                #[cfg(feature = "vulkan")]
                RenderDeviceType::VULKAN => ShaderSource::ByteCode(unsafe {
                    std::slice::from_raw_parts(
                        VERTEX_SHADER_SPIRV.as_ptr() as *const u8,
                        std::mem::size_of_val(VERTEX_SHADER_SPIRV),
                    )
                }),
                #[cfg(feature = "d3d11")]
                RenderDeviceType::D3D11 => ShaderSource::SourceCode(VERTEX_SHADER_HLSL),
                #[cfg(feature = "d3d12")]
                RenderDeviceType::D3D12 => ShaderSource::SourceCode(VERTEX_SHADER_HLSL),
                #[cfg(feature = "opengl")]
                RenderDeviceType::GL => ShaderSource::SourceCode(VERTEX_SHADER_GLSL),
                //RenderDeviceType::GLES => ShaderSource::SourceCode(VERTEX_SHADER_GLSL),
                #[cfg(feature = "webgpu")]
                RenderDeviceType::WEBGPU => ShaderSource::SourceCode(VERTEX_SHADER_WGSL),
                #[cfg(feature = "metal")]
                RenderDeviceType::METAL => {
                    todo!()
                }
            };

            let shader_ci = shader_ci
                .clone()
                .name("Imgui VS")
                .source(shader_source)
                .shader_type(ShaderType::Vertex)
                .build();

            create_info.device.create_shader(&shader_ci).unwrap()
        };

        let pixel_shader = {
            let shader_source = match device_type {
                #[cfg(feature = "vulkan")]
                RenderDeviceType::VULKAN => {
                    if manual_srgb {
                        ShaderSource::ByteCode(unsafe {
                            std::slice::from_raw_parts(
                                PIXEL_SHADER_GAMMA_SPIRV.as_ptr() as *const u8,
                                std::mem::size_of_val(PIXEL_SHADER_GAMMA_SPIRV),
                            )
                        })
                    } else {
                        ShaderSource::ByteCode(unsafe {
                            std::slice::from_raw_parts(
                                PIXEL_SHADER_SPIRV.as_ptr() as *const u8,
                                std::mem::size_of_val(PIXEL_SHADER_SPIRV),
                            )
                        })
                    }
                }
                #[cfg(feature = "d3d11")]
                RenderDeviceType::D3D11 => ShaderSource::SourceCode(PIXEL_SHADER_HLSL),
                #[cfg(feature = "d3d12")]
                RenderDeviceType::D3D12 => ShaderSource::SourceCode(PIXEL_SHADER_HLSL),
                #[cfg(feature = "opengl")]
                RenderDeviceType::GL => ShaderSource::SourceCode(PIXEL_SHADER_GLSL),
                //RenderDeviceType::GLES => ShaderSource::SourceCode(PIXEL_SHADER_GLSL),
                #[cfg(feature = "webgpu")]
                RenderDeviceType::WEBGPU => {
                    if manual_srgb {
                        ShaderSource::SourceCode(PIXEL_SHADER_WGSL_GAMMA)
                    } else {
                        ShaderSource::SourceCode(PIXEL_SHADER_WGSL)
                    }
                }
                #[cfg(feature = "metal")]
                RenderDeviceType::METAL => {
                    todo!()
                }
            };

            let shader_ci = shader_ci
                .clone()
                .name("Imgui PS")
                .source(shader_source)
                .shader_type(ShaderType::Pixel)
                .build();

            create_info.device.create_shader(&shader_ci).unwrap()
        };

        let sampler_desc = SamplerDesc::builder()
            .name(c"Texture Sampler")
            .address_u(TextureAddressMode::Wrap)
            .address_v(TextureAddressMode::Wrap)
            .address_w(TextureAddressMode::Wrap)
            .build();

        let mut render_targets = std::array::from_fn(|_| RenderTargetBlendDesc::default());

        render_targets[0] = RenderTargetBlendDesc::builder()
            .blend_enable(true)
            .src_blend(BlendFactor::One)
            .dest_blend(BlendFactor::InvSrcAlpha)
            .blend_op(BlendOperation::Add)
            .src_blend_alpha(BlendFactor::One)
            .dest_blend_alpha(BlendFactor::InvSrcAlpha)
            .blend_op_alpha(BlendOperation::Add)
            .render_target_write_mask(ColorMask::all())
            .build();

        let blend_state_desc = BlendStateDesc::builder()
            .render_targets(render_targets)
            .build();

        let rasterizer_state_desc = RasterizerStateDesc::builder()
            .cull_mode(CullMode::None)
            .scissor_enable(false)
            .build();

        let mut rtv_formats = std::array::from_fn(|_| None);

        rtv_formats[0] = Some(create_info.back_buffer_format);

        let shader_resource_variables = [ShaderResourceVariableDesc::builder()
            .name(c"Texture")
            .variable_type(ShaderResourceVariableType::Dynamic)
            .shader_stages(ShaderTypes::Pixel)
            .build()];

        let immutable_samplers = [ImmutableSamplerDesc::builder()
            .shader_stages(ShaderTypes::Pixel)
            .sampler_or_texture_name(c"Texture")
            .sampler_desc(&sampler_desc)
            .build()];

        let input_layouts = input_layouts![
            LayoutElement::builder().slot(0).f32_2(),
            LayoutElement::builder().slot(0).f32_2(),
            LayoutElement::builder().slot(0).u8_4(),
        ];

        let pipeline_state_ci = PipelineStateCreateInfo::builder()
            .shader_resource_variables(&shader_resource_variables)
            .immutable_samplers(&immutable_samplers)
            .name(c"ImGUI PSO")
            .graphics()
            .graphics_pipeline_desc(
                GraphicsPipelineDesc::builder()
                    .blend_desc(blend_state_desc)
                    .primitive_topology(PrimitiveTopology::TriangleList)
                    .input_layouts(&input_layouts)
                    .rasterizer_desc(rasterizer_state_desc)
                    .depth_stencil_desc(
                        DepthStencilStateDesc::builder().depth_enable(false).build(),
                    )
                    .output(
                        GraphicsPipelineRenderTargets::builder()
                            .num_render_targets(1)
                            .rtv_formats(rtv_formats)
                            .dsv_format(create_info.depth_buffer_format)
                            .build(),
                    )
                    .build(),
            )
            .vertex_shader(&vertex_shader)
            .pixel_shader(&pixel_shader)
            .build();

        let pipeline_state = create_info
            .device
            .create_graphics_pipeline_state(&pipeline_state_ci)
            .unwrap();

        let vertex_constant_buffer = {
            let buffer_desc = BufferDesc::builder()
                .name(c"Imgui Vertex Constant Buffer")
                .size((std::mem::size_of::<f32>() * 4 * 4) as u64)
                .usage(Usage::Dynamic)
                .bind_flags(BindFlags::UniformBuffer)
                .cpu_access_flags(CpuAccessFlags::Write)
                .build();
            create_info.device.create_buffer(&buffer_desc).unwrap()
        };

        pipeline_state
            .get_static_variable_by_name(ShaderType::Vertex, "Constants")
            .unwrap()
            .set(&vertex_constant_buffer, SetShaderResourceFlags::None);

        // Build texture atlas
        let font_atlas = imgui_context.fonts();
        let font_atlas_texture = font_atlas.build_rgba32_texture();

        let font_texture_desc = TextureDesc::builder()
            .name(c"Imgui font texture")
            .dimension(TextureDimension::Texture2D)
            .width(font_atlas_texture.width)
            .height(font_atlas_texture.height)
            .format(TextureFormat::RGBA8_UNORM)
            .bind_flags(BindFlags::ShaderResource)
            .usage(Usage::Immutable)
            .build();

        let subresource = TextureSubResource::builder()
            .from_host(font_atlas_texture.data, 4 * font_atlas_texture.width as u64)
            .build();

        let font_texture = create_info
            .device
            .create_texture(&font_texture_desc, &[&subresource], None)
            .unwrap();

        let font_texture_view = font_texture
            .get_default_view(TextureViewType::ShaderResource)
            .unwrap();

        let shader_resource_binding = pipeline_state.create_shader_resource_binding(true).unwrap();

        let texture_var = Boxed::from_ref(
            shader_resource_binding
                .get_variable_by_name("Texture", ShaderTypes::Pixel)
                .unwrap(),
        );

        // Store our identifier
        imgui_context.fonts().tex_id =
            TextureId::new(font_texture_view as *const TextureView as usize);

        imgui_context.io_mut().display_size =
            [create_info.initial_width, create_info.initial_height];

        ImguiRenderer {
            context: imgui_context.suspend(),
            data: ImguiRendererData {
                base_vertex_supported: create_info
                    .device
                    .get_adapter_info()
                    .draw_command()
                    .cap_flags()
                    .contains(DrawCommandCapFlags::BaseVertex),
                pipeline_state,
                _font_texture_view: Boxed::from_ref(font_texture_view),
                shader_resource_binding,
                texture_var,
                vertex_buffer: None,
                vertex_buffer_size: 1024,
                index_buffer: None,
                index_buffer_size: 2048,
                vertex_constant_buffer,
            },
        }
    }

    pub fn new_frame(self) -> ImguiFrame {
        ImguiFrame {
            context: self.context.activate().unwrap(),
            data: self.data,
        }
    }
}

impl ImguiFrame {
    pub fn render(
        &mut self,
        device_context: &ImmediateDeviceContext,
        render_device: &RenderDevice,
    ) {
        let _debug_group = device_context.debug_group(c"ImGui", None);

        let draw_data = self.context.render();

        // Avoid rendering when minimized
        if draw_data.display_size[0] <= 0.0
            || draw_data.display_size[1] <= 0.0
            || draw_data.draw_lists_count() == 0
        {
            return;
        }

        // Resize the vertex buffer if needed
        let vertex_buffer = if let Some(ref vertex_buffer) = self.data.vertex_buffer
            && self.data.vertex_buffer_size >= draw_data.total_vtx_count as u32
        {
            vertex_buffer
        } else {
            while self.data.vertex_buffer_size < draw_data.total_vtx_count as u32 {
                self.data.vertex_buffer_size *= 2
            }

            let buffer_desc = BufferDesc::builder()
                .name(c"Imgui vertex buffer")
                .size(
                    (self.data.vertex_buffer_size as usize * std::mem::size_of::<imgui::DrawVert>())
                        as u64,
                )
                .usage(Usage::Dynamic)
                .cpu_access_flags(CpuAccessFlags::Write)
                .bind_flags(BindFlags::VertexBuffer)
                .build();

            self.data
                .vertex_buffer
                .insert(render_device.create_buffer(&buffer_desc).unwrap())
        };

        // Resize the index buffer if needed
        let index_buffer = if let Some(ref index_buffer) = self.data.index_buffer
            && self.data.index_buffer_size >= draw_data.total_idx_count as u32
        {
            index_buffer
        } else {
            while self.data.index_buffer_size < draw_data.total_idx_count as u32 {
                self.data.index_buffer_size *= 2
            }

            let buffer_desc = BufferDesc::builder()
                .name(c"Imgui index buffer")
                .size(
                    (self.data.index_buffer_size as usize * std::mem::size_of::<imgui::DrawIdx>())
                        as u64,
                )
                .usage(Usage::Dynamic)
                .cpu_access_flags(CpuAccessFlags::Write)
                .bind_flags(BindFlags::IndexBuffer)
                .build();

            self.data
                .index_buffer
                .insert(render_device.create_buffer(&buffer_desc).unwrap())
        };

        // Transfer the vertex and index buffer from imgui data into our GPU buffers
        {
            let mut vb_access = device_context.map_buffer_write(vertex_buffer, MapFlags::Discard);
            let mut ib_access = device_context.map_buffer_write(index_buffer, MapFlags::Discard);

            let mut vtx_offset = 0;
            let mut idx_offset = 0;

            for draw_list in draw_data.draw_lists() {
                let vtx_buffer = draw_list.vtx_buffer();
                let idx_buffer = draw_list.idx_buffer();

                vb_access[vtx_offset..][..vtx_buffer.len()].copy_from_slice(vtx_buffer);
                ib_access[idx_offset..][..idx_buffer.len()].copy_from_slice(idx_buffer);

                vtx_offset += vtx_buffer.len();
                idx_offset += idx_buffer.len();
            }
        }

        // Setup orthographic projection matrix into our constant buffer
        // Our visible imgui space lies from pDrawData->DisplayPos (top left) to pDrawData->DisplayPos+data_data->DisplaySize (bottom right).
        // DisplayPos is (0,0) for single viewport apps.
        {
            // DisplaySize always refers to the logical dimensions that account for pre-transform, hence
            // the aspect ratio will be correct after applying appropriate rotation.
            let l = draw_data.display_pos[0];
            let r = draw_data.display_pos[0] + draw_data.display_size[0];
            let t = draw_data.display_pos[1];
            let b = draw_data.display_pos[1] + draw_data.display_size[1];

            #[rustfmt::skip]
            let projection = [
                    2.0 / (r - l),               0.0, 0.0, 0.0,
                              0.0,     2.0 / (t - b), 0.0, 0.0,
                              0.0,               0.0, 0.5, 0.0,
                (r + l) / (l - r), (t + b) / (b - t), 0.5, 1.0,
            ];

            {
                let mut projection_data = device_context.map_buffer_write::<[f32; 16]>(
                    &self.data.vertex_constant_buffer,
                    MapFlags::Discard,
                );

                (*projection_data)[0] = projection;
            }
        }

        // Setup the render state
        let pipeline_token = {
            // Setup shader and vertex buffers
            device_context.set_vertex_buffers(
                &[(vertex_buffer, 0)],
                ResourceStateTransitionMode::Transition,
                SetVertexBufferFlags::Reset,
            );
            device_context.set_index_buffer(
                index_buffer,
                0,
                ResourceStateTransitionMode::Transition,
            );

            let pipeline_token =
                device_context.set_graphics_pipeline_state(&self.data.pipeline_state);

            device_context.set_blend_factors(Some(&[0.0, 0.0, 0.0, 0.0]));

            let viewport = Viewport::builder()
                .top_left_x(0.0)
                .top_left_y(0.0)
                .width(draw_data.display_size[0])
                .height(draw_data.display_size[1])
                .min_depth(0.0)
                .max_depth(1.0)
                .build();

            device_context.set_viewports(
                &[viewport],
                draw_data.display_size[0] as u32,
                draw_data.display_size[1] as u32,
            );

            pipeline_token
        };

        // Render command lists
        // (Because we merged all buffers into a single one, we maintain our own offset into them)
        let mut global_idx_offset: u32 = 0;
        let mut global_vtx_offset: u32 = 0;

        let mut last_texture_view: *const TextureView = std::ptr::null();
        for cmd_list in draw_data.draw_lists() {
            for cmd in cmd_list.commands() {
                match cmd {
                    imgui::DrawCmd::Elements { count, cmd_params } => {
                        if count == 0 {
                            continue;
                        }

                        // Apply scissor/clipping rectangle
                        #[rustfmt::skip]
                        let clip_rect = [
                            (cmd_params.clip_rect[0] - draw_data.display_pos[0]) * draw_data.framebuffer_scale[0],
                            (cmd_params.clip_rect[1] - draw_data.display_pos[1]) * draw_data.framebuffer_scale[1],
                            (cmd_params.clip_rect[2] - draw_data.display_pos[0]) * draw_data.framebuffer_scale[0],
                            (cmd_params.clip_rect[3] - draw_data.display_pos[1]) * draw_data.framebuffer_scale[1],
                        ];

                        // Apply pretransform
                        //clip_rect = TransformClipRect(draw_data.display_size, clip_rect);

                        let scissor = Rect::builder()
                            .left(clip_rect[0].max(0.0) as i32)
                            .top(clip_rect[1].max(0.0) as i32)
                            .right(clip_rect[2].min(draw_data.display_size[0]) as i32)
                            .bottom(clip_rect[3].min(draw_data.display_size[1]) as i32)
                            .build();

                        if !scissor.is_valid() {
                            continue;
                        }

                        device_context.set_scissor_rects(
                            &[scissor],
                            draw_data.display_size[0] as u32,
                            draw_data.display_size[1] as u32,
                        );

                        // Bind texture
                        let texture_view = cmd_params.texture_id.id() as *const TextureView;

                        if texture_view != last_texture_view {
                            last_texture_view = texture_view;

                            let texture_view = unsafe { texture_view.as_ref().unwrap() };

                            self.data
                                .texture_var
                                .set(texture_view, SetShaderResourceFlags::None);

                            device_context.commit_shader_resources(
                                &self.data.shader_resource_binding,
                                ResourceStateTransitionMode::Transition,
                            );
                        }

                        let draw_attribs = {
                            let draw_attribs = DrawIndexedAttribs::builder()
                                .num_indices(count as u32)
                                .index_type(
                                    if std::mem::size_of::<ImDrawIdx>()
                                        == std::mem::size_of::<u16>()
                                    {
                                        ValueType::Uint16
                                    } else {
                                        ValueType::Uint32
                                    },
                                )
                                .flags(DrawFlags::VerifyStates)
                                .first_index_location(
                                    cmd_params.idx_offset as u32 + global_idx_offset,
                                );

                            if self.data.base_vertex_supported {
                                draw_attribs
                                    .base_vertex(cmd_params.vtx_offset as u32 + global_vtx_offset)
                                    .build()
                            } else {
                                let offset = std::mem::size_of::<ImDrawVert>()
                                    * (cmd_params.vtx_offset + global_vtx_offset as usize);
                                device_context.set_vertex_buffers(
                                    &[(vertex_buffer, offset as u64)],
                                    ResourceStateTransitionMode::Transition,
                                    SetVertexBufferFlags::None,
                                );
                                draw_attribs.build()
                            }
                        };
                        pipeline_token.draw_indexed(&draw_attribs);
                    }
                    imgui::DrawCmd::RawCallback { callback, raw_cmd } => unsafe {
                        callback(cmd_list.raw(), raw_cmd);
                    },
                    imgui::DrawCmd::ResetRenderState => {
                        // User callback, registered via ImDrawList::AddCallback()
                        // (ImDrawCallback_ResetRenderState is a special callback value used by the user to request the renderer to reset render state.)
                        //if (pCmd->UserCallback == ImDrawCallback_ResetRenderState)
                        //    SetupRenderState();
                    }
                }
            }

            global_idx_offset += cmd_list.idx_buffer().len() as u32;
            global_vtx_offset += cmd_list.vtx_buffer().len() as u32;
        }
    }

    pub fn ui_mut(&mut self) -> &mut imgui::Ui {
        self.context.frame()
    }

    pub fn io_mut(&mut self) -> &mut imgui::Io {
        self.context.io_mut()
    }

    pub fn finish(self) -> ImguiRenderer {
        ImguiRenderer {
            context: self.context.suspend(),
            data: self.data,
        }
    }
}
