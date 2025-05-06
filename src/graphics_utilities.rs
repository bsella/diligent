use crate::{
    buffer::{Buffer, BufferDesc, BufferMode},
    geometry_primitives::{
        GeometryPrimitive, GeometryPrimitiveAttributes, GeometryPrimitiveInfo,
        create_geometry_primitive, get_geometry_primitive_vertex_size,
    },
    graphics_types::{BindFlags, CpuAccessFlags, Usage},
    render_device::RenderDevice,
};

pub struct GeometryPrimitiveBuffersCreateInfo {
    vertex_buffer_usage: Usage,
    index_buffer_usage: Usage,
    vertex_buffer_bind_flags: BindFlags,
    index_buffer_bind_flags: BindFlags,
    vertex_buffer_mode: BufferMode,
    index_buffer_mode: BufferMode,
    vertex_buffer_cpu_access_flags: CpuAccessFlags,
    index_buffer_cpu_access_flags: CpuAccessFlags,
}

impl Default for GeometryPrimitiveBuffersCreateInfo {
    fn default() -> Self {
        GeometryPrimitiveBuffersCreateInfo {
            vertex_buffer_usage: Usage::Default,
            index_buffer_usage: Usage::Default,
            vertex_buffer_bind_flags: BindFlags::VertexBuffer,
            index_buffer_bind_flags: BindFlags::IndexBuffer,
            vertex_buffer_mode: BufferMode::Undefined,
            index_buffer_mode: BufferMode::Undefined,
            vertex_buffer_cpu_access_flags: CpuAccessFlags::None,
            index_buffer_cpu_access_flags: CpuAccessFlags::None,
        }
    }
}

impl GeometryPrimitiveBuffersCreateInfo {
    pub fn vertex_buffer_usage(mut self, vertex_buffer_usage: Usage) -> Self {
        self.vertex_buffer_usage = vertex_buffer_usage;
        self
    }
    pub fn index_buffer_usage(mut self, index_buffer_usage: Usage) -> Self {
        self.index_buffer_usage = index_buffer_usage;
        self
    }
    pub fn vertex_buffer_bind_flags(mut self, vertex_buffer_bind_flags: BindFlags) -> Self {
        self.vertex_buffer_bind_flags = vertex_buffer_bind_flags;
        self
    }
    pub fn index_buffer_bind_flags(mut self, index_buffer_bind_flags: BindFlags) -> Self {
        self.index_buffer_bind_flags = index_buffer_bind_flags;
        self
    }
    pub fn vertex_buffer_mode(mut self, vertex_buffer_mode: BufferMode) -> Self {
        self.vertex_buffer_mode = vertex_buffer_mode;
        self
    }
    pub fn index_buffer_mode(mut self, index_buffer_mode: BufferMode) -> Self {
        self.index_buffer_mode = index_buffer_mode;
        self
    }
    pub fn vertex_buffer_cpu_access_flags(
        mut self,
        vertex_buffer_cpu_access_flags: CpuAccessFlags,
    ) -> Self {
        self.vertex_buffer_cpu_access_flags = vertex_buffer_cpu_access_flags;
        self
    }
    pub fn index_buffer_cpu_access_flags(
        mut self,
        index_buffer_cpu_access_flags: CpuAccessFlags,
    ) -> Self {
        self.index_buffer_cpu_access_flags = index_buffer_cpu_access_flags;
        self
    }
}

static PRIMITIVE_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

pub fn create_geometry_primitive_buffers(
    device: &RenderDevice,
    attribs: &GeometryPrimitiveAttributes,
    buffer_ci: &GeometryPrimitiveBuffersCreateInfo,
) -> Result<(Buffer, Buffer, GeometryPrimitiveInfo), ()> {
    let (vertices, indices, info) = create_geometry_primitive(attribs)?;

    let primitive_id = PRIMITIVE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let primitive_type_str = match attribs.geometry_type {
        GeometryPrimitive::Cube { size: _ } => "Cube",
        GeometryPrimitive::Sphere { radius: _ } => "Sphere",
    };

    let name = format!("Geometry primitive {primitive_id} ({primitive_type_str})");

    let vb_desc = {
        let vb_desc = BufferDesc::new(&name, vertices.get_size() as u64)
            .bind_flags(buffer_ci.vertex_buffer_bind_flags)
            .usage(buffer_ci.vertex_buffer_usage)
            .cpu_access_flags(buffer_ci.vertex_buffer_cpu_access_flags)
            .mode(buffer_ci.vertex_buffer_mode);

        match buffer_ci.vertex_buffer_mode {
            BufferMode::Undefined => vb_desc,
            _ => vb_desc
                .element_byte_stride(get_geometry_primitive_vertex_size(&attribs.vertex_flags)),
        }
    };

    let vertex_buffer = device.create_buffer_with_data(
        &vb_desc,
        vertices.get_data_slice::<u8>(vertices.get_size(), 0),
        None,
    )?;

    let ib_desc = {
        let ib_desc = BufferDesc::new(&name, indices.get_size() as u64)
            .bind_flags(buffer_ci.index_buffer_bind_flags)
            .usage(buffer_ci.index_buffer_usage)
            .cpu_access_flags(buffer_ci.index_buffer_cpu_access_flags)
            .mode(buffer_ci.index_buffer_mode);

        match buffer_ci.index_buffer_mode {
            BufferMode::Undefined => ib_desc,
            _ => ib_desc.element_byte_stride(std::mem::size_of::<u32>() as u32),
        }
    };

    let index_buffer = device.create_buffer_with_data(
        &ib_desc,
        indices.get_data_slice::<u8>(indices.get_size(), 0),
        None,
    )?;

    Ok((vertex_buffer, index_buffer, info))
}

pub fn create_uniform_buffer(
    device: &RenderDevice,
    size: u64,
    name: impl AsRef<str>,
    usage: Usage,
    bind_flags: BindFlags,
    cpu_access_flags: CpuAccessFlags,
) -> Result<Buffer, ()> {
    let cpu_access_flags = match usage {
        Usage::Default | Usage::Immutable => CpuAccessFlags::None,
        _ => cpu_access_flags,
    };

    let cb_desc = BufferDesc::new(name, size)
        .usage(usage)
        .bind_flags(bind_flags)
        .cpu_access_flags(cpu_access_flags);

    device.create_buffer(&cb_desc)
}

pub fn create_uniform_buffer_with_data<T>(
    device: &RenderDevice,
    size: u64,
    name: impl AsRef<str>,
    usage: Usage,
    bind_flags: BindFlags,
    cpu_access_flags: CpuAccessFlags,
    data: &T,
) -> Result<Buffer, ()> {
    let cpu_access_flags = match usage {
        Usage::Default | Usage::Immutable => CpuAccessFlags::None,
        _ => cpu_access_flags,
    };

    let cb_desc = BufferDesc::new(name, size)
        .usage(usage)
        .bind_flags(bind_flags)
        .cpu_access_flags(cpu_access_flags);

    device.create_buffer_with_data(&cb_desc, data, None)
}

// https://en.wikipedia.org/wiki/SRGB
pub fn linear_to_gamma(x: f32) -> f32 {
    return if x <= 0.0031308 {
        x * 12.92
    } else {
        1.055 * f32::powf(x, 1.0 / 2.4) - 0.055
    };
}

pub fn linear_to_srgba(rgba: [f32; 4]) -> [f32; 4] {
    return [
        linear_to_gamma(rgba[0]),
        linear_to_gamma(rgba[1]),
        linear_to_gamma(rgba[2]),
        rgba[3],
    ];
}
