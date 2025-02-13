use diligent::bindings;
use diligent::core::device_context::DeviceContext;
use diligent::core::device_context::DrawAttribs;
use diligent::core::device_context::ResourceStateTransitionMode;
use diligent::core::graphics_types::PrimitiveTopology;
use diligent::core::graphics_types::ShaderType;
use diligent::core::pipeline_state::BlendStateDesc;
use diligent::core::pipeline_state::CullMode;
use diligent::core::pipeline_state::DepthStencilStateDesc;
use diligent::core::pipeline_state::GraphicsPipelineDesc;
use diligent::core::pipeline_state::GraphicsPipelineStateCreateInfo;
use diligent::core::pipeline_state::PipelineState;
use diligent::core::pipeline_state::RasterizerStateDesc;
use diligent::core::render_device::RenderDevice;
use diligent::core::shader::ShaderCreateInfo;
use diligent::core::shader::ShaderLanguage;
use diligent::core::shader::ShaderSource;
use diligent::core::swap_chain::SwapChain;
use diligent::samples::sample::SampleBase;
use diligent::samples::sample_app::SampleApp;
use diligent::tools::native_app;

struct HelloTriangle {
    render_device: RenderDevice,
    immediate_contexts: Vec<DeviceContext>,
    _deferred_contexts: Vec<DeviceContext>,

    pipeline_state: PipelineState,
}

impl SampleBase for HelloTriangle {
    fn get_render_device(&self) -> &RenderDevice {
        &self.render_device
    }

    fn get_immediate_context(&self) -> &DeviceContext {
        self.immediate_contexts.first().unwrap()
    }

    fn new(
        render_device: RenderDevice,
        immediate_contexts: Vec<DeviceContext>,
        deferred_contexts: Vec<DeviceContext>,
        swap_chain: &SwapChain,
    ) -> Self {
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
            let shader_create_info = ShaderCreateInfo::new(
                c"Triangle vertex shader",
                ShaderSource::SourceCode(&shader_source_code),
                ShaderType::Vertex,
            )
            // Tell the system that the shader source code is in HLSL.
            // For OpenGL, the engine will convert this into GLSL under the hood.
            .language(ShaderLanguage::HLSL)
            // OpenGL backend requires emulated combined HLSL texture samplers (g_Texture + g_Texture_sampler combination)
            .use_combined_texture_samplers(true);

            render_device.create_shader(&shader_create_info).unwrap()
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
            let shader_create_info = ShaderCreateInfo::new(
                c"Triangle pixel shader",
                ShaderSource::SourceCode(shader_source_code),
                ShaderType::Pixel,
            )
            // Tell the system that the shader source code is in HLSL.
            // For OpenGL, the engine will convert this into GLSL under the hood.
            .language(ShaderLanguage::HLSL)
            // OpenGL backend requires emulated combined HLSL texture samplers (g_Texture + g_Texture_sampler combination)
            .use_combined_texture_samplers(true);

            render_device.create_shader(&shader_create_info).unwrap()
        };

        let graphics_pipeline_desc = GraphicsPipelineDesc::new(
            BlendStateDesc::default(),
            RasterizerStateDesc::default()
                // No back face culling for this tutorial
                .cull_mode(CullMode::None),
            DepthStencilStateDesc::default()
                // Disable depth testing
                .depth_enable(false),
        )
        // This tutorial will render to a single render target
        .num_render_targets(1)
        // Set render target format which is the format of the swap chain's color buffer
        .rtv_format::<0>(swap_chain.get_desc().ColorBufferFormat as bindings::_TEXTURE_FORMAT)
        // Use the depth buffer format from the swap chain
        .dsv_format(swap_chain.get_desc().DepthBufferFormat as bindings::_TEXTURE_FORMAT)
        // Primitive topology defines what kind of primitives will be rendered by this pipeline state
        .primitive_topology(PrimitiveTopology::TriangleList);

        let pso_create_info =
            GraphicsPipelineStateCreateInfo::new(c"Simple triangle PSO", graphics_pipeline_desc)
                .vertex_shader(&vertex_shader)
                .pixel_shader(&pixel_shader);

        // Finally, create the pipeline state
        let pipeline_state = render_device
            .create_graphics_pipeline_state(&pso_create_info)
            .unwrap();

        HelloTriangle {
            render_device,
            immediate_contexts,
            _deferred_contexts: deferred_contexts,
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
        immediate_context.draw(&DrawAttribs::new(3));
    }

    fn get_name() -> &'static str {
        "Tutorial01: Hello Triangle"
    }
}

fn main() {
    native_app::main::<SampleApp<HelloTriangle>>().unwrap()
}
