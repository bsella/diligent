use diligent::*;

use diligent_samples::sample_base::{
    sample::SampleBase,
    sample_app::{self},
};
use diligent_tools::native_app::app_settings::AppSettings;

const VERTEX_SHADER_SOURCE: &str = r#"
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

const PIXEL_SHADER_SOURCE: &str = r#"
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

struct MultipleWindows {
    device: Boxed<RenderDevice>,

    pipeline_state: Boxed<GraphicsPipelineState>,
}

impl SampleBase for MultipleWindows {
    fn get_render_device(&self) -> &RenderDevice {
        &self.device
    }

    fn make_swap_chains_create_info(_settings: &impl AppSettings) -> Vec<SwapChainCreateInfo> {
        vec![
            SwapChainCreateInfo::builder()
                .width(1024)
                .height(768)
                .is_primary(true)
                .build(),
            SwapChainCreateInfo::builder()
                .width(640)
                .height(480)
                .is_primary(false)
                .build(),
            SwapChainCreateInfo::builder()
                .width(480)
                .height(320)
                .is_primary(false)
                .build(),
        ]
    }

    fn new(
        _engine_factory: &EngineFactory,
        device: Boxed<RenderDevice>,
        _main_context: &ImmediateDeviceContext,
        _immediate_contexts: Vec<Boxed<ImmediateDeviceContext>>,
        _deferred_contexts: Vec<Boxed<DeferredDeviceContext>>,
        swap_chain_descs: &[&SwapChainDesc],
    ) -> Self {
        // We are only using one swap chain
        let swap_chain_desc = swap_chain_descs[0];

        let shader_create_info = ShaderCreateInfo::builder()
            // Tell the system that the shader source code is in HLSL.
            // For OpenGL, the engine will convert this into GLSL under the hood.
            .source_language(ShaderLanguage::HLSL)
            // OpenGL backend requires emulated combined HLSL texture samplers (g_Texture + g_Texture_sampler combination)
            .use_combined_texture_samplers(true);

        let vertex_shader = {
            device
                .create_shader(
                    &shader_create_info
                        .clone()
                        .name("Triangle vertex shader")
                        .source(ShaderSource::SourceCode(VERTEX_SHADER_SOURCE))
                        .shader_type(ShaderType::Vertex)
                        .build(),
                )
                .unwrap()
        };

        let pixel_shader = {
            device
                .create_shader(
                    &shader_create_info
                        .clone()
                        .name("Triangle pixel shader")
                        .source(ShaderSource::SourceCode(PIXEL_SHADER_SOURCE))
                        .shader_type(ShaderType::Pixel)
                        .build(),
                )
                .unwrap()
        };

        let mut rtv_formats = std::array::from_fn(|_| None);
        rtv_formats[0] = Some(swap_chain_desc.color_buffer_format());

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
            .dsv_format(swap_chain_desc.depth_buffer_format())
            .build();

        let graphics_pipeline_desc = GraphicsPipelineDesc::builder()
            .rasterizer_desc(rasterizer_desc)
            .depth_stencil_desc(depth_desc)
            .output(pipeline_output)
            // Primitive topology defines what kind of primitives will be rendered by this pipeline state
            .primitive_topology(PrimitiveTopology::TriangleList)
            .build();

        let pso_create_info = PipelineStateCreateInfo::builder()
            .name(c"Simple triangle PSO")
            .graphics()
            .graphics_pipeline_desc(graphics_pipeline_desc)
            .vertex_shader(&vertex_shader)
            .pixel_shader(&pixel_shader)
            .build();

        // Finally, create the pipeline state
        let pipeline_state = device
            .create_graphics_pipeline_state(&pso_create_info)
            .unwrap();

        MultipleWindows {
            device,
            pipeline_state,
        }
    }

    fn render(
        &self,
        main_context: Boxed<ImmediateDeviceContext>,
        swap_chain: &SwapChain,
    ) -> Boxed<ImmediateDeviceContext> {
        let rtv = swap_chain.get_current_back_buffer_rtv().unwrap();
        let dsv = swap_chain.get_depth_buffer_dsv().unwrap();

        // Clear the back buffer
        // Let the engine perform required state transitions
        main_context.clear_render_target::<f32>(
            rtv,
            &[0.350, 0.350, 0.350, 1.0],
            ResourceStateTransitionMode::Transition,
        );

        main_context.clear_depth(dsv, 1.0, ResourceStateTransitionMode::Transition);

        // Set the pipeline state in the immediate context
        let graphics = main_context.set_graphics_pipeline_state(&self.pipeline_state);

        // Typically we should now call CommitShaderResources(), however shaders in this example don't
        // use any resources.
        graphics.draw(&DrawAttribs::builder().num_vertices(3).build());

        graphics.finish()
    }

    fn get_name() -> &'static str {
        "Tutorial01: Hello Triangle"
    }
}

fn main() {
    sample_app::main::<MultipleWindows>().unwrap()
}
