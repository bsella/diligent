use diligent::{
    geometry_primitives::GeometryPrimitiveVertexFlags,
    graphics_utilities::{create_uniform_buffer, linear_to_srgba},
    *,
};

use diligent_samples::{
    sample_base::{
        sample::{SampleBase, get_adjusted_projection_matrix, get_surface_pretransform_matrix},
        sample_app::{self},
    },
    textured_cube::{CreatePSOInfo, TexturedCube},
};
use image::DynamicImage;
use rand::distr::uniform::{UniformFloat, UniformInt, UniformSampler};

const MAX_GRID_SIZE: u64 = 32;
const MAX_INSTANCES: u64 = MAX_GRID_SIZE * MAX_GRID_SIZE * MAX_GRID_SIZE;
const NUM_TEXTURES: usize = 4;

#[derive(Clone, Copy)]
#[repr(C)]
struct InstanceData {
    matrix: [f32; 4 * 4],
    texture_id: f32,
}

struct TextureArray {
    device: Boxed<RenderDevice>,

    textured_cube: TexturedCube,

    convert_ps_output_to_gamma: bool,

    pipeline_state: Boxed<GraphicsPipelineState>,
    srb: Boxed<ShaderResourceBinding>,

    _texture_view: Boxed<TextureView>,

    rotation_matrix: glam::Mat4,

    grid_size: u32,

    instance_buffer: Boxed<Buffer>,

    vertex_shader_constants: Boxed<Buffer>,
}

impl TextureArray {
    fn populate_instance_buffer(&mut self, context: &DeviceContext) {
        let mut instance_data = Vec::from_iter(std::iter::repeat_n(
            InstanceData {
                matrix: std::array::from_fn(|_| 0.0),
                texture_id: 0.0,
            },
            (self.grid_size * self.grid_size * self.grid_size) as usize,
        ));

        let mut rng = rand::rng();

        let scale_distr = UniformFloat::<f32>::new(0.3, 1.0).unwrap();
        let offset_distr = UniformFloat::<f32>::new(-0.15f32, 0.15f32).unwrap();
        let rot_distr =
            UniformFloat::<f32>::new(-std::f32::consts::PI, std::f32::consts::PI).unwrap();
        let tex_distr = UniformInt::<u32>::new(0, (NUM_TEXTURES - 1) as u32).unwrap();

        let base_scale = 0.6 / self.grid_size as f32;
        let mut inst_id: usize = 0;
        for x in 0..self.grid_size {
            for y in 0..self.grid_size {
                for z in 0..self.grid_size {
                    // Add random offset from central position in the grid
                    let x_offset = 2.0 * (x as f32 + 0.5 + offset_distr.sample(&mut rng))
                        / self.grid_size as f32
                        - 1.0;
                    let y_offset = 2.0 * (y as f32 + 0.5 + offset_distr.sample(&mut rng))
                        / self.grid_size as f32
                        - 1.0;
                    let z_offset = 2.0 * (z as f32 + 0.5 + offset_distr.sample(&mut rng))
                        / self.grid_size as f32
                        - 1.0;

                    // Random scale
                    let scale = base_scale * scale_distr.sample(&mut rng);

                    // Random rotation
                    let rotation = glam::Mat4::from_rotation_z(rot_distr.sample(&mut rng))
                        * glam::Mat4::from_rotation_y(rot_distr.sample(&mut rng))
                        * glam::Mat4::from_rotation_x(rot_distr.sample(&mut rng));

                    // Combine rotation, scale and translation
                    let matrix = glam::Mat4::from_translation(glam::Vec3 {
                        x: x_offset,
                        y: y_offset,
                        z: z_offset,
                    }) * glam::Mat4::from_scale(glam::Vec3 {
                        x: scale,
                        y: scale,
                        z: scale,
                    }) * rotation;
                    instance_data[inst_id] = InstanceData {
                        matrix: matrix.to_cols_array(),
                        texture_id: tex_distr.sample(&mut rng) as f32,
                    };
                    inst_id += 1;
                }
            }
        }

        // Update instance data buffer
        context.update_buffer_from_slice(
            &mut self.instance_buffer,
            instance_data.as_slice(),
            ResourceStateTransitionMode::Transition,
        );
    }
}

impl SampleBase for TextureArray {
    fn get_render_device(&self) -> &RenderDevice {
        &self.device
    }

    fn new(
        engine_factory: &EngineFactory,
        device: Boxed<RenderDevice>,
        main_context: &ImmediateDeviceContext,
        _immediate_contexts: Vec<Boxed<ImmediateDeviceContext>>,
        _deferred_contexts: Vec<Boxed<DeferredDeviceContext>>,
        swap_chain_descs: &[&SwapChainDesc],
    ) -> Self {
        // We are only using one swap chain
        let swap_chain_desc = swap_chain_descs[0];

        // If the swap chain color buffer format is a non-sRGB UNORM format,
        // we need to manually convert pixel shader output to gamma space.
        let convert_ps_output_to_gamma = matches!(
            swap_chain_desc.color_buffer_format(),
            TextureFormat::RGBA8_UNORM | TextureFormat::BGRA8_UNORM
        );

        // Create a shader source stream factory to load shaders from files.
        let shader_source_factory = engine_factory
            .create_default_shader_source_stream_factory(&[])
            .unwrap();

        // Define vertex shader input layout
        // This tutorial uses two types of input: per-vertex data and per-instance data.
        #[rustfmt::skip]
        let layout_elements = input_layouts![
            // Per-vertex data - first buffer slot
            // Attribute 0 - vertex position
            LayoutElement::builder().slot(0).f32_3(),
            // Attribute 1 - texture coordinates
            LayoutElement::builder().slot(0).f32_2(),

            // Per-instance data - second buffer slot
            // We will use four attributes to encode instance-specific 4x4 transformation matrix
            // Attribute 2 - first row
            LayoutElement::builder().slot(1).f32_4().frequency(InputElementFrequency::PerInstance),
            // Attribute 3 - second row
            LayoutElement::builder().slot(1).f32_4().frequency(InputElementFrequency::PerInstance),
            // Attribute 4 - third row
            LayoutElement::builder().slot(1).f32_4().frequency(InputElementFrequency::PerInstance),
            // Attribute 5 - fourth row
            LayoutElement::builder().slot(1).f32_4().frequency(InputElementFrequency::PerInstance),
            // Attribute 6 - texture array index
            LayoutElement::builder().slot(1).f32().frequency(InputElementFrequency::PerInstance),
        ];

        let cube_pso_ci = CreatePSOInfo::new(
            &device,
            swap_chain_desc.color_buffer_format(),
            swap_chain_desc.depth_buffer_format(),
            &shader_source_factory,
            "assets/cube_inst_tex_array.vsh",
            "assets/cube_inst_tex_array.psh",
            GeometryPrimitiveVertexFlags::None,
            layout_elements,
            1,
        );

        let pipeline_state =
            TexturedCube::create_pipeline_state(cube_pso_ci, convert_ps_output_to_gamma).unwrap();

        // Create dynamic uniform buffer that will store our transformation matrix
        // Dynamic buffers can be frequently updated by the CPU
        let vs_constants = create_uniform_buffer(
            &device,
            std::mem::size_of::<glam::Mat4>() as u64 * 2,
            c"VS constants CB",
            Usage::Dynamic,
            BindFlags::UniformBuffer,
            CpuAccessFlags::Write,
        )
        .unwrap();

        // Since we did not explicitly specify the type for 'Constants' variable, default
        // type (SHADER_RESOURCE_VARIABLE_TYPE_STATIC) will be used. Static variables
        // never change and are bound directly to the pipeline state object.
        pipeline_state
            .get_static_variable_by_name(ShaderType::Vertex, "Constants")
            .unwrap()
            .set(&vs_constants, SetShaderResourceFlags::None);

        // Since we are using mutable variable, we must create a shader resource binding object
        // http://diligentgraphics.com/2016/03/23/resource-binding-model-in-diligent-engine-2-0/
        let srb = pipeline_state.create_shader_resource_binding(true).unwrap();

        let textured_cube = TexturedCube::new(
            &device,
            GeometryPrimitiveVertexFlags::PosTex,
            BindFlags::VertexBuffer,
            None,
            BindFlags::IndexBuffer,
            None,
        )
        .unwrap();

        let texture_srv = {
            let images: [DynamicImage; NUM_TEXTURES] = std::array::from_fn(|i| i).map(|tex_id| {
                image::ImageReader::open(format!("assets/DGLogo{tex_id}.png"))
                    .unwrap()
                    .decode()
                    .unwrap()
            });

            let texture_desc = TextureDesc::builder()
                .name(c"DGLogo")
                .dimension(TextureDimension::Texture2DArray {
                    array_size: std::num::NonZero::new(NUM_TEXTURES).unwrap(),
                })
                .width(images[0].width())
                .height(images[0].height())
                .format(TextureFormat::RGBA8_UNORM_SRGB)
                .bind_flags(BindFlags::ShaderResource)
                .usage(Usage::Default)
                .build();

            let texture_data = images.each_ref().map(|image| {
                TextureSubResource::builder()
                    .from_host(
                        image.as_bytes(),
                        image.width() as u64 * std::mem::size_of::<[u8; 4]>() as u64,
                    )
                    .build()
            });

            let texture = device
                .create_texture(&texture_desc, &texture_data, None)
                .unwrap();

            // Get shader resource view from the texture array
            Boxed::<TextureView>::from_ref(
                texture
                    .get_default_view(TextureViewType::ShaderResource)
                    .unwrap(),
            )
        };

        srb.get_variable_by_name("g_Texture", ShaderTypes::Pixel)
            .unwrap()
            .set(&texture_srv, SetShaderResourceFlags::None);

        // Use default usage as this buffer will only be updated when grid size changes
        let inst_buff_desc = BufferDesc::builder()
            .name(c"Instance data buffer")
            .size(std::mem::size_of::<InstanceData>() as u64 * MAX_INSTANCES)
            .usage(Usage::Default)
            .bind_flags(BindFlags::VertexBuffer)
            .build();

        let inst_buff = device.create_buffer(&inst_buff_desc).unwrap();

        let mut sample = TextureArray {
            device,
            convert_ps_output_to_gamma,
            pipeline_state,
            textured_cube,
            srb,
            vertex_shader_constants: vs_constants,
            rotation_matrix: glam::Mat4::IDENTITY,
            grid_size: 5,
            instance_buffer: inst_buff,
            _texture_view: texture_srv,
        };

        sample.populate_instance_buffer(main_context);

        sample
    }

    fn update_ui(&mut self, main_context: &ImmediateDeviceContext, ui: &mut imgui::Ui) {
        if let Some(_window_token) = ui
            .window("Settings")
            .always_auto_resize(true)
            .position([10.0, 10.0], imgui::Condition::Always)
            .begin()
            && ui.slider("Grid Size", 1, 32, &mut self.grid_size)
        {
            self.populate_instance_buffer(main_context);
        }
    }

    fn update(
        &mut self,
        _main_context: &ImmediateDeviceContext,
        current_time: f64,
        _elapsed_time: f64,
    ) {
        // Apply rotation
        self.rotation_matrix = glam::Mat4::from_rotation_y(current_time as f32)
            * glam::Mat4::from_rotation_x(-current_time as f32 * 0.25);
    }

    fn render(
        &self,
        main_context: Boxed<ImmediateDeviceContext>,
        swap_chain: &SwapChain,
    ) -> Boxed<ImmediateDeviceContext> {
        let view_proj_matrix = {
            let swap_chain_desc = swap_chain.desc();

            // Get pretransform matrix that rotates the scene according the surface orientation
            let srf_pre_transform = get_surface_pretransform_matrix(
                swap_chain_desc.pre_transform(),
                &glam::Vec3::new(0.0, 0.0, 1.0),
            );

            // Get projection matrix adjusted to the current screen orientation
            let proj = get_adjusted_projection_matrix(
                swap_chain_desc,
                std::f32::consts::PI / 4.0,
                0.1,
                100.0,
            );

            // Set cube view matrix
            let view = glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, 4.0))
                * glam::Mat4::from_rotation_x(-0.6);

            proj * srf_pre_transform * view
        };

        let rtv = swap_chain.get_current_back_buffer_rtv().unwrap();
        let dsv = swap_chain.get_depth_buffer_dsv().unwrap();

        // Clear the back buffer
        {
            let clear_color = {
                let clear_color = [0.350, 0.350, 0.350, 1.0];

                if self.convert_ps_output_to_gamma {
                    // If manual gamma correction is required, we need to clear the render target with sRGB color
                    linear_to_srgba(clear_color)
                } else {
                    clear_color
                }
            };

            main_context.clear_render_target::<f32>(
                rtv,
                &clear_color,
                ResourceStateTransitionMode::Transition,
            );
        }

        main_context.clear_depth(dsv, 1.0, ResourceStateTransitionMode::Transition);

        {
            // Map the buffer and write current world-view-projection matrix
            let mut cb_constants =
                main_context.map_buffer_write(&self.vertex_shader_constants, MapFlags::Discard);

            cb_constants[0] = view_proj_matrix;
            cb_constants[1] = self.rotation_matrix;
        }

        {
            // Bind vertex, instance and index buffers
            let buffers = [
                (self.textured_cube.get_vertex_buffer(), 0),
                (&self.instance_buffer, 0),
            ];
            main_context.set_vertex_buffers(
                &buffers,
                ResourceStateTransitionMode::Transition,
                SetVertexBufferFlags::Reset,
            );
            main_context.set_index_buffer(
                self.textured_cube.get_index_buffer(),
                0,
                ResourceStateTransitionMode::Transition,
            );
        }

        // Set the pipeline state
        let graphics = main_context.set_graphics_pipeline_state(&self.pipeline_state);

        // Commit shader resources. RESOURCE_STATE_TRANSITION_MODE_TRANSITION mode
        // makes sure that resources are transitioned to required states.
        graphics.commit_shader_resources(&self.srb, ResourceStateTransitionMode::Transition);

        let draw_attribs = DrawIndexedAttribs::builder()
            .num_indices(36)
            .index_type(ValueType::Uint32)
            .num_instances(self.grid_size * self.grid_size * self.grid_size)
            // Verify the state of vertex and index buffers
            .flags(DrawFlags::VerifyAll)
            .build();

        graphics.draw_indexed(&draw_attribs);

        graphics.finish()
    }

    fn get_name() -> &'static str {
        "Tutorial05: TextureArray"
    }
}

fn main() {
    sample_app::main::<TextureArray>().unwrap()
}
