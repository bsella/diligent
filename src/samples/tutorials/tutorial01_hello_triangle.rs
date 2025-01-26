use diligent::bindings;
use diligent::core::device_context::DeviceContext;
use diligent::core::device_context::DrawAttribs;
use diligent::core::graphics_types::PrimitiveTopology;
use diligent::core::graphics_types::ShaderType;
use diligent::core::pipeline_state::CullMode;
use diligent::core::pipeline_state::GraphicsPipelineStateCreateInfo;
use diligent::core::pipeline_state::PipelineState;
use diligent::core::render_device::RenderDevice;
use diligent::core::shader::ShaderCreateInfo;
use diligent::core::shader::ShaderLanguage;
use diligent::core::shader::ShaderSource;
use diligent::core::swap_chain::SwapChain;
use diligent::samples::sample_base::sample_app::Sample;
use diligent::samples::sample_base::sample_app::SampleApp;
use diligent::samples::sample_base::sample_app::SampleBase;
use diligent::tools::native_app;

struct HelloTriangle {
    sample: Sample,

    pipeline_state: PipelineState,
}

impl SampleBase for HelloTriangle {
    fn get_immediate_context(&self) -> &DeviceContext {
        self.sample.get_immediate_context()
    }
    fn get_render_device(&self) -> &RenderDevice {
        self.sample.get_render_device()
    }

    fn new(
        render_device: RenderDevice,
        immediate_contexts: Vec<DeviceContext>,
        deferred_contexts: Vec<DeviceContext>,
        swap_chain: &SwapChain,
    ) -> Self {
        let mut pso_create_info = GraphicsPipelineStateCreateInfo::new(c"Simple triangle PSO");

        // This tutorial will render to a single render target
        pso_create_info.graphics_pipeline_desc.num_render_targets = 1;
        // Set render target format which is the format of the swap chain's color buffer
        pso_create_info.graphics_pipeline_desc.rtv_formats[0] =
            swap_chain.get_desc().ColorBufferFormat as u32;
        // Use the depth buffer format from the swap chain
        pso_create_info.graphics_pipeline_desc.dsv_format =
            swap_chain.get_desc().DepthBufferFormat as u32;
        // Primitive topology defines what kind of primitives will be rendered by this pipeline state
        pso_create_info.graphics_pipeline_desc.primitive_topology = PrimitiveTopology::TriangleList;
        // No back face culling for this tutorial
        pso_create_info
            .graphics_pipeline_desc
            .rasterizer_desc
            .cull_mode = CullMode::None;
        // Disable depth testing
        pso_create_info
            .graphics_pipeline_desc
            .depth_stencil_desc
            .depth_enable = false;

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
            let mut shader_create_info = ShaderCreateInfo::new(
                c"Triangle vertex shader",
                ShaderSource::SourceCode(&shader_source_code),
                ShaderType::Vertex,
            );

            // Tell the system that the shader source code is in HLSL.
            // For OpenGL, the engine will convert this into GLSL under the hood.
            shader_create_info.source_language = ShaderLanguage::HLSL;
            // OpenGL backend requires emulated combined HLSL texture samplers (g_Texture + g_Texture_sampler combination)
            shader_create_info.desc.use_combined_texture_samplers = true;

            render_device.create_shader(shader_create_info).unwrap()
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
            let mut shader_create_info = ShaderCreateInfo::new(
                c"Triangle pixel shader",
                ShaderSource::SourceCode(shader_source_code),
                ShaderType::Pixel,
            );
            // Tell the system that the shader source code is in HLSL.
            // For OpenGL, the engine will convert this into GLSL under the hood.
            shader_create_info.source_language = ShaderLanguage::HLSL;
            // OpenGL backend requires emulated combined HLSL texture samplers (g_Texture + g_Texture_sampler combination)
            shader_create_info.desc.use_combined_texture_samplers = true;

            render_device.create_shader(shader_create_info).unwrap()
        };

        // Finally, create the pipeline state
        pso_create_info.vertex_shader = Some(&vertex_shader);
        pso_create_info.pixel_shader = Some(&pixel_shader);

        let pipeline_state = render_device
            .create_graphics_pipeline_state(pso_create_info)
            .unwrap();

        HelloTriangle {
            sample: Sample::new(
                render_device,
                immediate_contexts,
                deferred_contexts,
                swap_chain,
            ),
            pipeline_state: pipeline_state,
        }
    }

    fn pre_window_resize(&mut self) {}

    fn window_resize(&mut self, _width: u32, _height: u32) {}
    fn render(&self, swap_chain: &SwapChain) {
        let immediate_context = self.get_immediate_context();

        let mut rtv = swap_chain.get_current_back_buffer_rtv();
        let mut dsv = swap_chain.get_depth_buffer_dsv();

        immediate_context.clear_render_target::<f32>(
            &mut rtv,
            &[0.350, 0.350, 0.350, 1.0],
            diligent::bindings::RESOURCE_STATE_TRANSITION_MODE_TRANSITION,
        );

        immediate_context.clear_depth_stencil(
            &mut dsv,
            bindings::CLEAR_DEPTH_FLAG,
            1.0,
            0,
            diligent::bindings::RESOURCE_STATE_TRANSITION_MODE_TRANSITION,
        );

        // Set the pipeline state in the immediate context
        immediate_context.set_pipeline_state(&self.pipeline_state);

        // Typically we should now call CommitShaderResources(), however shaders in this example don't
        // use any resources.

        let draw_attribs = DrawAttribs {
            first_instance_location: 0,
            flags: bindings::DRAW_FLAG_NONE as bindings::DRAW_FLAGS,
            num_vertices: 3,
            num_instances: 1,
            start_vertex_location: 0,
        };

        immediate_context.draw(draw_attribs);
    }
    fn update(&self, _current_time: f64, _elapsed_time: f64) {}
    fn get_name() -> &'static str {
        "Tutorial01: Hello Triangle"
    }
}

fn main() {
    native_app::main::<SampleApp<HelloTriangle>>().unwrap()
}
