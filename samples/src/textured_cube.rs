use std::path::PathBuf;

use diligent::{
    geometry_primitives::{
        GeometryPrimitive, GeometryPrimitiveAttributes, GeometryPrimitiveVertexFlags,
    },
    graphics_utilities::{create_geometry_primitive_buffers, GeometryPrimitiveBuffersCreateInfo},
    *,
};

pub struct TexturedCube {
    vertex_buffer: Boxed<Buffer>,
    index_buffer: Boxed<Buffer>,
    //texture: Option<Boxed<Texture>>,
}

pub struct CreatePSOInfo<'a> {
    device: &'a RenderDevice,
    rtv_format: TextureFormat,
    dsv_format: TextureFormat,
    shader_source_factory: &'a ShaderSourceInputStreamFactory,
    vs_file_path: PathBuf,
    ps_file_path: PathBuf,
    components: GeometryPrimitiveVertexFlags,
    extra_layout_elements: Vec<LayoutElement>,
    sample_count: u8,
}

impl<'a> CreatePSOInfo<'a> {
    pub fn new<P>(
        device: &'a RenderDevice,
        rtv_format: TextureFormat,
        dsv_format: TextureFormat,
        shader_source_factory: &'a ShaderSourceInputStreamFactory,
        vs_file_path: P,
        ps_file_path: P,
        components: GeometryPrimitiveVertexFlags,
        extra_layout_elements: impl Into<Vec<LayoutElement>>,
        sample_count: u8,
    ) -> Self
    where
        PathBuf: From<P>,
    {
        CreatePSOInfo {
            device,
            rtv_format,
            dsv_format,
            shader_source_factory,
            vs_file_path: vs_file_path.into(),
            ps_file_path: ps_file_path.into(),
            components,
            extra_layout_elements: extra_layout_elements.into(),
            sample_count,
        }
    }
}

impl TexturedCube {
    pub fn new(
        device: &RenderDevice,
        components: GeometryPrimitiveVertexFlags,
        vtx_buffer_bind_flags: BindFlags,
        vtx_buffer_mode: Option<BufferMode>,
        idx_buffer_bind_flags: BindFlags,
        idx_buffer_mode: Option<BufferMode>,
    ) -> Result<Self, ()> {
        let create_info = GeometryPrimitiveBuffersCreateInfo::builder()
            .vertex_buffer_bind_flags(vtx_buffer_bind_flags)
            .maybe_vertex_buffer_mode(vtx_buffer_mode)
            .index_buffer_bind_flags(idx_buffer_bind_flags)
            .maybe_index_buffer_mode(idx_buffer_mode)
            .build();

        let (vertex_buffer, index_buffer, info) = create_geometry_primitive_buffers(
            device,
            &GeometryPrimitiveAttributes::builder()
                .geometry_type(GeometryPrimitive::Cube { size: 2.0 })
                .vertex_flags(components)
                .build(),
            &create_info,
        )?;
        assert_eq!(info.num_vertices, 24);
        assert_eq!(info.num_indices, 36);
        Ok(TexturedCube {
            vertex_buffer,
            index_buffer,
        })
    }

    pub fn create_pipeline_state(
        create_info: CreatePSOInfo,
        convert_output_to_gamma: bool,
    ) -> Result<Boxed<GraphicsPipelineState>, ()> {
        let mut input_layouts = Vec::new();

        if create_info
            .components
            .contains(GeometryPrimitiveVertexFlags::Position)
        {
            input_layouts.push(LayoutElement::builder().slot(0).f32_3().build());
        }
        if create_info
            .components
            .contains(GeometryPrimitiveVertexFlags::Normal)
        {
            input_layouts.push(LayoutElement::builder().slot(0).f32_3().build());
        }
        if create_info
            .components
            .contains(GeometryPrimitiveVertexFlags::TexCoord)
        {
            input_layouts.push(LayoutElement::builder().slot(0).f32_2().build());
        }

        input_layouts.extend(create_info.extra_layout_elements);

        let mut rtv_formats = std::array::from_fn(|_| None);

        rtv_formats[0] = Some(create_info.rtv_format);

        let graphics_pipeline_desc = GraphicsPipelineDesc::builder()
            .rasterizer_desc(
                RasterizerStateDesc::builder()
                    // Cull back faces
                    .cull_mode(CullMode::Back)
                    .build(),
            )
            .depth_stencil_desc(
                // Enable depth testing
                DepthStencilStateDesc::builder().depth_enable(true).build(),
            )
            .output(
                GraphicsPipelineRenderTargets::builder()
                    // This tutorial will render to a single render target
                    .num_render_targets(1)
                    // Set render target format which is the format of the swap chain's color buffer
                    .rtv_formats(rtv_formats)
                    // Set depth buffer format which is the format of the swap chain's back buffer
                    .dsv_format(create_info.dsv_format)
                    .build(),
            )
            // Set the desired number of samples
            .sample_count(create_info.sample_count)
            // Primitive topology defines what kind of primitives will be rendered by this pipeline state
            .primitive_topology(PrimitiveTopology::TriangleList)
            .input_layouts(input_layouts)
            .build();

        let shader_ci = ShaderCreateInfo::builder()
            // Tell the system that the shader source code is in HLSL.
            // For OpenGL, the engine will convert this into GLSL under the hood.
            .source_language(ShaderLanguage::HLSL)
            // OpenGL backend requires emulated combined HLSL texture samplers (g_Texture + g_Texture_sampler combination)
            .use_combined_texture_samplers(true)
            // Pack matrices in row-major order
            .compile_flags(ShaderCompileFlags::PackMatrixRowMajor)
            // Presentation engine always expects input in gamma space. Normally, pixel shader output is
            // converted from linear to gamma space by the GPU. However, some platforms (e.g. Android in GLES mode,
            // or Emscripten in WebGL mode) do not support gamma-correction. In this case the application
            // has to do the conversion manually.
            .macros(vec![(
                "CONVERT_PS_OUTPUT_TO_GAMMA",
                if convert_output_to_gamma { "1" } else { "0" },
            )])
            .shader_source_input_stream_factory(create_info.shader_source_factory)
            .entry_point("main");

        // Create a vertex shader
        let vertex_shader = create_info
            .device
            .create_shader(
                &shader_ci
                    .clone()
                    .name("Cube VS")
                    .source(ShaderSource::FilePath(&create_info.vs_file_path))
                    .shader_type(ShaderType::Vertex)
                    .build(),
            )
            .unwrap();

        // Create a pixel shader
        let pixel_shader = create_info
            .device
            .create_shader(
                &shader_ci
                    .clone()
                    .name("Cube PS")
                    .source(ShaderSource::FilePath(&create_info.ps_file_path))
                    .shader_type(ShaderType::Pixel)
                    .build(),
            )
            .unwrap();

        let sampler_desc = SamplerDesc::builder()
            .name("Textured Cube Sampler")
            .min_filter(FilterType::Linear)
            .mag_filter(FilterType::Linear)
            .mip_filter(FilterType::Linear)
            .address_u(TextureAddressMode::Clamp)
            .address_v(TextureAddressMode::Clamp)
            .address_w(TextureAddressMode::Clamp)
            .build();

        // Pipeline state name is used by the engine to report issues.
        // It is always a good idea to give objects descriptive names.
        let pso_create_info = PipelineStateCreateInfo::builder()
            // Define variable type that will be used by default
            .default_variable_type(ShaderResourceVariableType::Static)
            // Shader variables should typically be mutable, which means they are expected
            // to change on a per-instance basis
            .shader_resource_variables([ShaderResourceVariableDesc::builder()
                .name("g_Texture")
                .variable_type(ShaderResourceVariableType::Mutable)
                .shader_stages(ShaderTypes::Pixel)
                .build()])
            // Define immutable sampler for g_Texture. Immutable samplers should be used whenever possible
            .immutable_samplers([ImmutableSamplerDesc::new(
                ShaderTypes::Pixel,
                "g_Texture",
                &sampler_desc,
            )])
            .name("Cube PSO")
            .graphics()
            .graphics_pipeline_desc(graphics_pipeline_desc)
            .vertex_shader(&vertex_shader)
            .pixel_shader(&pixel_shader)
            .build();

        create_info
            .device
            .create_graphics_pipeline_state(&pso_create_info)
    }

    pub fn get_vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    pub fn get_index_buffer(&self) -> &Buffer {
        &self.index_buffer
    }
}
