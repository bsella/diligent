use diligent::{
    device_context::{
        DeferredDeviceContext, DrawAttribs, ImmediateDeviceContext, ResourceStateTransitionMode,
    },
    engine_factory::EngineFactory,
    graphics_types::{PrimitiveTopology, ShaderType},
    pipeline_state::{
        CullMode, DepthStencilStateDesc, GraphicsPipelineDesc, GraphicsPipelineRenderTargets,
        PipelineState, PipelineStateCreateInfo, RasterizerStateDesc,
    },
    render_device::RenderDevice,
    shader::{ShaderCreateInfo, ShaderLanguage, ShaderSource},
    swap_chain::SwapChain,
};

use diligent_samples::sample_base::{sample::SampleBase, sample_app::SampleApp};
use diligent_tools::native_app;

struct HelloTriangle {
    immediate_context: ImmediateDeviceContext,

    pipeline_state: PipelineState,
}

impl SampleBase for HelloTriangle {
    fn get_immediate_context(&self) -> &ImmediateDeviceContext {
        &self.immediate_context
    }

    fn new(
        _engine_factory: &EngineFactory,
        device: &RenderDevice,
        immediate_contexts: Vec<ImmediateDeviceContext>,
        _deferred_contexts: Vec<DeferredDeviceContext>,
        swap_chain: &SwapChain,
    ) -> Self {
        let swap_chain_desc = swap_chain.get_desc();

        let shader_create_info = ShaderCreateInfo::builder()
            // Tell the system that the shader source code is in HLSL.
            // For OpenGL, the engine will convert this into GLSL under the hood.
            .source_language(ShaderLanguage::HLSL)
            // OpenGL backend requires emulated combined HLSL texture samplers (g_Texture + g_Texture_sampler combination)
            .use_combined_texture_samplers(true);

        let vertex_shader = {
            let shader_source_code = r#"
struct PSInput
{
    float4 Pos   : SV_POSITION;
    float3 Color : COLOR;
};

void main(in uint VertId : SV_VertexID, out PSInput PSIn)
{
    float4 Pos[3];
    Pos[0] = float4(-0.5, -0.5, 0.0, 1.0);
    Pos[1] = float4( 0.0, +0.5, 0.0, 1.0);
    Pos[2] = float4(+0.5, -0.5, 0.0, 1.0);

    float3 Col[3];
    Col[0] = float3(1.0, 0.0, 0.0); // red
    Col[1] = float3(0.0, 1.0, 0.0); // green
    Col[2] = float3(0.0, 0.0, 1.0); // blue

    PSIn.Pos   = Pos[VertId];
    PSIn.Color = Col[VertId];
}
"#;

            device
                .create_shader(
                    &shader_create_info
                        .clone()
                        .name("Triangle vertex shader")
                        .source(ShaderSource::SourceCode(shader_source_code))
                        .shader_type(ShaderType::Vertex)
                        .build(),
                )
                .unwrap()
        };

        let pixel_shader = {
            let shader_source_code = r#"
struct PSInput
{ 
    float4 Pos   : SV_POSITION;
    float3 Color : COLOR;
};

struct PSOutput
{ 
    float4 Color : SV_TARGET;
};

void main(in PSInput PSIn, out PSOutput PSOut)
{
    PSOut.Color = float4(PSIn.Color.rgb, 1.0);
}
"#;
            device
                .create_shader(
                    &shader_create_info
                        .clone()
                        .name("Triangle pixel shader")
                        .source(ShaderSource::SourceCode(shader_source_code))
                        .shader_type(ShaderType::Pixel)
                        .build(),
                )
                .unwrap()
        };

        let mut rtv_formats = std::array::from_fn(|_| None);
        rtv_formats[0] = Some(swap_chain_desc.color_buffer_format);

        let rasterizer_desc = RasterizerStateDesc::builder()
            // No back face culling for this tutorial
            .cull_mode(CullMode::None)
            .build();

        let depth_desc = DepthStencilStateDesc::builder()
            // Disable depth testing
            .depth_enable(false)
            .build();

        let pipeline_output = GraphicsPipelineRenderTargets::builder()
            // This tutorial will render to a single render target
            .num_render_targets(1)
            // Set render target format which is the format of the swap chain's color buffer
            .rtv_formats(rtv_formats)
            // Use the depth buffer format from the swap chain
            .dsv_format(swap_chain_desc.depth_buffer_format)
            .build();

        let graphics_pipeline_desc = GraphicsPipelineDesc::builder()
            .rasterizer_desc(rasterizer_desc)
            .depth_stencil_desc(depth_desc)
            .output(pipeline_output)
            // Primitive topology defines what kind of primitives will be rendered by this pipeline state
            .primitive_topology(PrimitiveTopology::TriangleList)
            .build();

        let pso_create_info = PipelineStateCreateInfo::builder()
            .graphics("Simple triangle PSO")
            .graphics_pipeline_desc(graphics_pipeline_desc)
            .vertex_shader(&vertex_shader)
            .pixel_shader(&pixel_shader)
            .build();

        // Finally, create the pipeline state
        let pipeline_state = device
            .create_graphics_pipeline_state(&pso_create_info)
            .unwrap();

        HelloTriangle {
            immediate_context: immediate_contexts.into_iter().nth(0).unwrap(),
            pipeline_state,
        }
    }

    fn render(&self, swap_chain: &SwapChain) {
        let immediate_context = self.get_immediate_context();

        let mut rtv = swap_chain.get_current_back_buffer_rtv();
        let mut dsv = swap_chain.get_depth_buffer_dsv();

        // Clear the back buffer
        // Let the engine perform required state transitions
        immediate_context.clear_render_target::<f32>(
            &mut rtv,
            &[0.350, 0.350, 0.350, 1.0],
            ResourceStateTransitionMode::Transition,
        );

        immediate_context.clear_depth(&mut dsv, 1.0, ResourceStateTransitionMode::Transition);

        // Set the pipeline state in the immediate context
        immediate_context.set_pipeline_state(&self.pipeline_state);

        // Typically we should now call CommitShaderResources(), however shaders in this example don't
        // use any resources.
        immediate_context.draw(&DrawAttribs::builder().num_vertices(3).build());
    }

    fn get_name() -> &'static str {
        "Tutorial01: Hello Triangle"
    }
}

fn main() {
    native_app::main::<SampleApp<HelloTriangle>>().unwrap()
}
