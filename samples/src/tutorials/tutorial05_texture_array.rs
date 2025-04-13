use diligent::{
    accessories::linear_to_srgba,
    buffer::{Buffer, BufferDesc, BufferMode},
    device_context::{
        DeferredDeviceContext, DrawFlags, DrawIndexedAttribs, ImmediateDeviceContext,
        ResourceStateTransitionMode, SetVertexBufferFlags,
    },
    engine_factory::EngineFactory,
    geometry_primitives::GeometryPrimitiveVertexFlags,
    graphics_types::{
        BindFlags, CpuAccessFlags, MapFlags, SetShaderResourceFlags, ShaderType, ShaderTypes,
        TextureFormat, Usage, ValueType,
    },
    graphics_utilities::create_uniform_buffer,
    input_layout::{InputElementFrequency, LayoutElement},
    pipeline_state::PipelineState,
    render_device::RenderDevice,
    shader_resource_binding::ShaderResourceBinding,
    swap_chain::SwapChain,
    texture::{TextureDesc, TextureDimension, TextureSubResource},
    texture_view::{TextureView, TextureViewType},
};

use diligent_tools::native_app;

use diligent_samples::{
    sample::{get_adjusted_projection_matrix, get_surface_pretransform_matrix, SampleBase},
    sample_app::SampleApp,
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
    render_device: RenderDevice,
    immediate_context: ImmediateDeviceContext,

    textured_cube: TexturedCube,

    convert_ps_output_to_gamma: bool,

    pipeline_state: PipelineState,
    srb: ShaderResourceBinding,

    _texture_view: TextureView,

    rotation_matrix: glam::Mat4,

    grid_size: u32,

    instance_buffer: Buffer,

    vertex_shader_constants: Buffer,
}

impl TextureArray {
    fn populate_instance_buffer(&mut self) {
        let mut instance_data = Vec::from_iter(std::iter::repeat_n(
            InstanceData {
                matrix: [(); 4 * 4].map(|_| 0.0),
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
        self.immediate_context.update_buffer_from_slice(
            &mut self.instance_buffer,
            instance_data.as_slice(),
            ResourceStateTransitionMode::Transition,
        );
    }
}

impl SampleBase for TextureArray {
    fn get_render_device(&self) -> &RenderDevice {
        &self.render_device
    }

    fn get_immediate_context(&self) -> &ImmediateDeviceContext {
        &self.immediate_context
    }

    fn new(
        engine_factory: &EngineFactory,
        render_device: RenderDevice,
        immediate_contexts: Vec<ImmediateDeviceContext>,
        _deferred_contexts: Vec<DeferredDeviceContext>,
        swap_chain: &SwapChain,
    ) -> Self {
        let swap_chain_desc = swap_chain.get_desc();

        // If the swap chain color buffer format is a non-sRGB UNORM format,
        // we need to manually convert pixel shader output to gamma space.
        let convert_ps_output_to_gamma = match swap_chain_desc.color_buffer_format {
            TextureFormat::RGBA8_UNORM | TextureFormat::BGRA8_UNORM => true,
            _ => false,
        };

        // Create a shader source stream factory to load shaders from files.
        let shader_source_factory = engine_factory
            .create_default_shader_source_stream_factory(&[])
            .unwrap();

        // Define vertex shader input layout
        // This tutorial uses two types of input: per-vertex data and per-instance data.
        #[rustfmt::skip]
        let layout_elements = vec![
            // Per-vertex data - first buffer slot
            // Attribute 0 - vertex position
            LayoutElement::new(0, 3, ValueType::Float32).is_normalized(false),
            // Attribute 1 - texture coordinates
            LayoutElement::new(0, 2, ValueType::Float32).is_normalized(false),

            // Per-instance data - second buffer slot
            // We will use four attributes to encode instance-specific 4x4 transformation matrix
            // Attribute 2 - first row
            LayoutElement::new(1, 4, ValueType::Float32).is_normalized(false).frequency(InputElementFrequency::PerInstance),
            // Attribute 3 - second row
            LayoutElement::new(1, 4, ValueType::Float32).is_normalized(false).frequency(InputElementFrequency::PerInstance),
            // Attribute 4 - third row
            LayoutElement::new(1, 4, ValueType::Float32).is_normalized(false).frequency(InputElementFrequency::PerInstance),
            // Attribute 5 - fourth row
            LayoutElement::new(1, 4, ValueType::Float32).is_normalized(false).frequency(InputElementFrequency::PerInstance),
            // Attribute 6 - texture array index
            LayoutElement::new(1, 1, ValueType::Float32).is_normalized(false).frequency(InputElementFrequency::PerInstance),
        ];

        let cube_pso_ci = CreatePSOInfo::new(
            &render_device,
            swap_chain_desc.color_buffer_format,
            swap_chain_desc.depth_buffer_format,
            &shader_source_factory,
            "cube_inst_tex_array.vsh",
            "cube_inst_tex_array.psh",
            GeometryPrimitiveVertexFlags::None,
            layout_elements,
            1,
        );

        let pipeline_state =
            TexturedCube::create_pipeline_state(&cube_pso_ci, convert_ps_output_to_gamma).unwrap();

        // Create dynamic uniform buffer that will store our transformation matrix
        // Dynamic buffers can be frequently updated by the CPU
        let vs_constants = create_uniform_buffer(
            &render_device,
            std::mem::size_of::<glam::Mat4>() as u64 * 2,
            "VS constants CB",
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
            &render_device,
            GeometryPrimitiveVertexFlags::PosTex,
            BindFlags::VertexBuffer,
            BufferMode::Undefined,
            BindFlags::IndexBuffer,
            BufferMode::Undefined,
        )
        .unwrap();

        let texture_srv = {
            let images: [DynamicImage; NUM_TEXTURES] = std::array::from_fn(|i| i).map(|tex_id| {
                image::ImageReader::open(format!("DGLogo{}.png", tex_id))
                    .unwrap()
                    .decode()
                    .unwrap()
            });

            let texture_desc = TextureDesc::new(
                "DGLogo",
                TextureDimension::Texture2DArray {
                    array_size: NUM_TEXTURES as u32,
                },
                images[0].width(),
                images[0].height(),
                TextureFormat::RGBA8_UNORM_SRGB,
            )
            .bind_flags(BindFlags::ShaderResource)
            .usage(Usage::Default);

            let texture_data = images.each_ref().map(|image| {
                TextureSubResource::new_cpu(
                    image.as_bytes(),
                    image.width() as u64 * std::mem::size_of::<[u8; 4]>() as u64,
                )
            });

            let texture = render_device
                .create_texture(&texture_desc, &texture_data.each_ref(), None)
                .unwrap();

            // Get shader resource view from the texture array
            texture
                .get_default_view(TextureViewType::ShaderResource)
                .unwrap()
        };

        srb.get_variable_by_name("g_Texture", ShaderTypes::Pixel)
            .unwrap()
            .set(&texture_srv, SetShaderResourceFlags::None);

        // Use default usage as this buffer will only be updated when grid size changes
        let inst_buff_desc = BufferDesc::new(
            "Instance data buffer",
            std::mem::size_of::<InstanceData>() as u64 * MAX_INSTANCES,
        )
        .usage(Usage::Default)
        .bind_flags(BindFlags::VertexBuffer);

        let inst_buff = render_device.create_buffer(&inst_buff_desc).unwrap();

        let mut sample = TextureArray {
            convert_ps_output_to_gamma,
            pipeline_state,
            immediate_context: immediate_contexts.into_iter().nth(0).unwrap(),
            textured_cube,
            render_device,
            srb,
            vertex_shader_constants: vs_constants,
            rotation_matrix: glam::Mat4::IDENTITY,
            grid_size: 5,
            instance_buffer: inst_buff,
            _texture_view: texture_srv,
        };

        sample.populate_instance_buffer();

        sample
    }

    fn update_ui(&mut self, ui: &mut imgui::Ui) {
        if let Some(_window_token) = ui
            .window("Settings")
            .always_auto_resize(true)
            .position([10.0, 10.0], imgui::Condition::Always)
            .begin()
        {
            if ui.slider("Grid Size", 1, 32, &mut self.grid_size) {
                self.populate_instance_buffer();
            }
        }
    }

    fn update(&mut self, current_time: f64, _elapsed_time: f64) {
        // Apply rotation
        self.rotation_matrix = glam::Mat4::from_rotation_y(current_time as f32)
            * glam::Mat4::from_rotation_x(-current_time as f32 * 0.25);
    }

    fn render(&self, swap_chain: &SwapChain) {
        let immediate_context = self.get_immediate_context();

        let view_proj_matrix = {
            let swap_chain_desc = swap_chain.get_desc();

            // Get pretransform matrix that rotates the scene according the surface orientation
            let srf_pre_transform =
                get_surface_pretransform_matrix(&swap_chain_desc, &glam::Vec3::new(0.0, 0.0, 1.0));

            // Get projection matrix adjusted to the current screen orientation
            let proj = get_adjusted_projection_matrix(
                &swap_chain_desc,
                std::f32::consts::PI / 4.0,
                0.1,
                100.0,
            );

            // Set cube view matrix
            let view = glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, 4.0))
                * glam::Mat4::from_rotation_x(-0.6);

            proj * srf_pre_transform * view
        };

        let mut rtv = swap_chain.get_current_back_buffer_rtv();
        let mut dsv = swap_chain.get_depth_buffer_dsv();

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

            immediate_context.clear_render_target::<f32>(
                &mut rtv,
                &clear_color,
                ResourceStateTransitionMode::Transition,
            );
        }

        immediate_context.clear_depth(&mut dsv, 1.0, ResourceStateTransitionMode::Transition);

        {
            // Map the buffer and write current world-view-projection matrix
            let mut cb_constants = immediate_context
                .map_buffer_write(&self.vertex_shader_constants, MapFlags::Discard);

            let buffer_write = unsafe { cb_constants.as_mut_slice(2, 0) };
            buffer_write[0] = view_proj_matrix;
            buffer_write[1] = self.rotation_matrix;
        }

        {
            // Bind vertex, instance and index buffers
            let buffers = [
                self.textured_cube.get_vertex_buffer(),
                &self.instance_buffer,
            ];
            immediate_context.set_vertex_buffers(
                &buffers,
                &[0, 0],
                ResourceStateTransitionMode::Transition,
                SetVertexBufferFlags::Reset,
            );
            immediate_context.set_index_buffer(
                self.textured_cube.get_index_buffer(),
                0,
                ResourceStateTransitionMode::Transition,
            );
        }

        // Set the pipeline state
        immediate_context.set_pipeline_state(&self.pipeline_state);

        // Commit shader resources. RESOURCE_STATE_TRANSITION_MODE_TRANSITION mode
        // makes sure that resources are transitioned to required states.
        immediate_context
            .commit_shader_resources(&self.srb, ResourceStateTransitionMode::Transition);

        let draw_attribs = DrawIndexedAttribs::new(36, ValueType::Uint32)
            .num_instances(self.grid_size * self.grid_size * self.grid_size)
            // Verify the state of vertex and index buffers
            .flags(DrawFlags::VerifyAll);

        immediate_context.draw_indexed(&draw_attribs);
    }

    fn get_name() -> &'static str {
        "Tutorial05: TextureArray"
    }
}

fn main() {
    native_app::main::<SampleApp<TextureArray>>().unwrap()
}
