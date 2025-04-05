use std::path::PathBuf;

use diligent::{
    buffer::{Buffer, BufferMode},
    geometry_primitives::{
        GeometryPrimitive, GeometryPrimitiveAttributes, GeometryPrimitiveVertexFlags,
    },
    graphics_types::{
        BindFlags, FilterType, PrimitiveTopology, ShaderType, ShaderTypes, TextureAddressMode,
        TextureFormat, ValueType,
    },
    graphics_utilities::{create_geometry_primitive_buffers, GeometryPrimitiveBuffersCreateInfo},
    input_layout::LayoutElement,
    pipeline_resource_signature::ImmutableSamplerDesc,
    pipeline_state::{
        BlendStateDesc, CullMode, DepthStencilStateDesc, GraphicsPipelineDesc,
        GraphicsPipelineStateCreateInfo, PipelineState, RasterizerStateDesc,
    },
    render_device::RenderDevice,
    sampler::SamplerDesc,
    //shader::ShaderCompileFlags,
    shader::{ShaderCreateInfo, ShaderLanguage, ShaderSource, ShaderSourceInputStreamFactory},
    shader_resource_variable::{ShaderResourceVariableDesc, ShaderResourceVariableType},
};

pub struct TexturedCube {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    //texture: Option<Texture>,
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
        vtx_buffer_mode: BufferMode,
        idx_buffer_bind_flags: BindFlags,
        idx_buffer_mode: BufferMode,
    ) -> Option<Self> {
        if let Some((vertex_buffer, index_buffer, info)) = create_geometry_primitive_buffers(
            device,
            &GeometryPrimitiveAttributes::new(GeometryPrimitive::Cube { size: 2.0 })
                .vertex_flags(components),
            &GeometryPrimitiveBuffersCreateInfo::default()
                .vertex_buffer_bind_flags(vtx_buffer_bind_flags)
                .vertex_buffer_mode(vtx_buffer_mode)
                .index_buffer_bind_flags(idx_buffer_bind_flags)
                .index_buffer_mode(idx_buffer_mode),
        ) {
            assert_eq!(info.num_vertices, 24);
            assert_eq!(info.num_indices, 36);
            Some(TexturedCube {
                vertex_buffer,
                index_buffer,
            })
        } else {
            None
        }
    }

    pub fn create_pipeline_state(
        create_info: &CreatePSOInfo,
        convert_output_to_gamma: bool,
    ) -> Option<PipelineState> {
        let mut input_layouts = Vec::new();

        if create_info
            .components
            .contains(GeometryPrimitiveVertexFlags::Position)
        {
            LayoutElement::new(0, 3, ValueType::Float32).is_normalized(false);
        }
        if create_info
            .components
            .contains(GeometryPrimitiveVertexFlags::Normal)
        {
            LayoutElement::new(0, 3, ValueType::Float32).is_normalized(false);
        }
        if create_info
            .components
            .contains(GeometryPrimitiveVertexFlags::TexCoord)
        {
            LayoutElement::new(0, 2, ValueType::Float32).is_normalized(false);
        }

        input_layouts.extend(create_info.extra_layout_elements.clone());

        let graphics_pipeline_desc = GraphicsPipelineDesc::new(
            BlendStateDesc::default(),
            // Cull back faces
            RasterizerStateDesc::default().cull_mode(CullMode::Back),
            // Enable depth testing
            DepthStencilStateDesc::default().depth_enable(true),
        )
        // This tutorial will render to a single render target
        .num_render_targets(1)
        // Set render target format which is the format of the swap chain's color buffer
        .rtv_format::<0>(create_info.rtv_format)
        // Set depth buffer format which is the format of the swap chain's back buffer
        .dsv_format(create_info.dsv_format)
        // Set the desired number of samples
        .sample_count(create_info.sample_count)
        // Primitive topology defines what kind of primitives will be rendered by this pipeline state
        .primitive_topology(PrimitiveTopology::TriangleList)
        .set_input_layouts(input_layouts);

        fn common_shader_ci<'a>(
            name: &str,
            source: ShaderSource<'a>,
            shader_type: ShaderType,
            convert_output_to_gamma: bool,
            create_info: &'a CreatePSOInfo,
        ) -> ShaderCreateInfo<'a> {
            ShaderCreateInfo::new(name, source, shader_type)
                // Tell the system that the shader source code is in HLSL.
                // For OpenGL, the engine will convert this into GLSL under the hood.
                .language(ShaderLanguage::HLSL)
                // OpenGL backend requires emulated combined HLSL texture samplers (g_Texture + g_Texture_sampler combination)
                .use_combined_texture_samplers(true)
                // Pack matrices in row-major order
                //.compile_flags(ShaderCompileFlags::PackMatrixRowMajor)
                // Presentation engine always expects input in gamma space. Normally, pixel shader output is
                // converted from linear to gamma space by the GPU. However, some platforms (e.g. Android in GLES mode,
                // or Emscripten in WebGL mode) do not support gamma-correction. In this case the application
                // has to do the conversion manually.
                .set_macros(vec![(
                    "CONVERT_PS_OUTPUT_TO_GAMMA",
                    if convert_output_to_gamma { "1" } else { "0" },
                )])
                .shader_source_input_stream_factory(Some(create_info.shader_source_factory))
                .entry_point("main")
        }

        // Create a vertex shader
        let vertex_shader = create_info
            .device
            .create_shader(&common_shader_ci(
                "Cube VS",
                ShaderSource::FilePath(&create_info.vs_file_path),
                ShaderType::Vertex,
                convert_output_to_gamma,
                create_info,
            ))
            .unwrap();

        // Create a pixel shader
        let pixel_shader = create_info
            .device
            .create_shader(&common_shader_ci(
                "Cube PS",
                ShaderSource::FilePath(&create_info.ps_file_path),
                ShaderType::Pixel,
                convert_output_to_gamma,
                create_info,
            ))
            .unwrap();

        let sampler_desc = SamplerDesc::new("Textured Cube Sampler")
            .min_filter(FilterType::Linear)
            .mag_filter(FilterType::Linear)
            .mip_filter(FilterType::Linear)
            .address_u(TextureAddressMode::Clamp)
            .address_v(TextureAddressMode::Clamp)
            .address_w(TextureAddressMode::Clamp);

        // Pipeline state name is used by the engine to report issues.
        // It is always a good idea to give objects descriptive names.
        let pso_create_info =
            GraphicsPipelineStateCreateInfo::new("Cube PSO", graphics_pipeline_desc)
                .vertex_shader(&vertex_shader)
                .pixel_shader(&pixel_shader)
                // Define variable type that will be used by default
                .default_variable_type(ShaderResourceVariableType::Static)
                // Shader variables should typically be mutable, which means they are expected
                // to change on a per-instance basis
                .set_shader_resource_variables([ShaderResourceVariableDesc::new(
                    "g_Texture",
                    ShaderResourceVariableType::Mutable,
                    ShaderTypes::Pixel,
                )])
                // Define immutable sampler for g_Texture. Immutable samplers should be used whenever possible
                .set_immutable_samplers([ImmutableSamplerDesc::new(
                    ShaderTypes::Pixel,
                    "g_Texture",
                    &sampler_desc,
                )]);

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
