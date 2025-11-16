use std::ffi::{CStr, CString};

use bon::Builder;

use crate::{
    Boxed,
    buffer::{Buffer, BufferDesc, BufferMode},
    geometry_primitives::{
        GeometryPrimitive, GeometryPrimitiveAttributes, GeometryPrimitiveInfo,
        create_geometry_primitive, get_geometry_primitive_vertex_size,
    },
    graphics_types::{BindFlags, CpuAccessFlags, Usage},
    render_device::RenderDevice,
};

#[derive(Builder)]
pub struct GeometryPrimitiveBuffersCreateInfo {
    #[builder(default)]
    vertex_buffer_usage: Usage,

    #[builder(default)]
    index_buffer_usage: Usage,

    vertex_buffer_bind_flags: BindFlags,

    index_buffer_bind_flags: BindFlags,

    vertex_buffer_mode: Option<BufferMode>,

    index_buffer_mode: Option<BufferMode>,

    #[builder(default)]
    vertex_buffer_cpu_access_flags: CpuAccessFlags,

    #[builder(default)]
    index_buffer_cpu_access_flags: CpuAccessFlags,
}

impl Default for GeometryPrimitiveBuffersCreateInfo {
    fn default() -> Self {
        GeometryPrimitiveBuffersCreateInfo {
            vertex_buffer_usage: Usage::Default,
            index_buffer_usage: Usage::Default,
            vertex_buffer_bind_flags: BindFlags::VertexBuffer,
            index_buffer_bind_flags: BindFlags::IndexBuffer,
            vertex_buffer_mode: None,
            index_buffer_mode: None,
            vertex_buffer_cpu_access_flags: CpuAccessFlags::None,
            index_buffer_cpu_access_flags: CpuAccessFlags::None,
        }
    }
}

static PRIMITIVE_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

pub fn create_geometry_primitive_buffers(
    device: &RenderDevice,
    attribs: &GeometryPrimitiveAttributes,
    buffer_ci: &GeometryPrimitiveBuffersCreateInfo,
) -> Result<(Boxed<Buffer>, Boxed<Buffer>, GeometryPrimitiveInfo), ()> {
    let (vertices, indices, info) = create_geometry_primitive(attribs)?;

    let primitive_id = PRIMITIVE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let primitive_type_str = match attribs.geometry_type {
        GeometryPrimitive::Cube { size: _ } => "Cube",
        GeometryPrimitive::Sphere { radius: _ } => "Sphere",
    };

    let name = format!("Geometry primitive {primitive_id} ({primitive_type_str})");
    let name = CString::new(name).unwrap();

    let vb_desc = {
        let vb_desc = BufferDesc::builder()
            .name(&name)
            .size(vertices.get_size() as _)
            .bind_flags(buffer_ci.vertex_buffer_bind_flags)
            .usage(buffer_ci.vertex_buffer_usage)
            .cpu_access_flags(buffer_ci.vertex_buffer_cpu_access_flags)
            .maybe_mode(buffer_ci.vertex_buffer_mode);

        if buffer_ci.vertex_buffer_mode.is_some() {
            vb_desc
                .element_byte_stride(get_geometry_primitive_vertex_size(attribs.vertex_flags))
                .build()
        } else {
            vb_desc.build()
        }
    };

    let vertex_buffer = device.create_buffer_with_data(
        &vb_desc,
        vertices.get_data_slice::<u8>(vertices.get_size(), 0),
        None,
    )?;

    let ib_desc = {
        let ib_desc = BufferDesc::builder()
            .name(&name)
            .size(indices.get_size() as _)
            .bind_flags(buffer_ci.index_buffer_bind_flags)
            .usage(buffer_ci.index_buffer_usage)
            .cpu_access_flags(buffer_ci.index_buffer_cpu_access_flags)
            .maybe_mode(buffer_ci.index_buffer_mode);

        if buffer_ci.index_buffer_mode.is_some() {
            ib_desc
                .element_byte_stride(std::mem::size_of::<u32>() as u32)
                .build()
        } else {
            ib_desc.build()
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
    name: impl AsRef<CStr>,
    usage: Usage,
    bind_flags: BindFlags,
    cpu_access_flags: CpuAccessFlags,
) -> Result<Boxed<Buffer>, ()> {
    let cpu_access_flags = match usage {
        Usage::Default | Usage::Immutable => CpuAccessFlags::None,
        _ => cpu_access_flags,
    };

    let cb_desc = BufferDesc::builder()
        .name(name.as_ref())
        .size(size)
        .usage(usage)
        .bind_flags(bind_flags)
        .cpu_access_flags(cpu_access_flags)
        .build();

    device.create_buffer(&cb_desc)
}

pub fn create_uniform_buffer_with_data<T>(
    device: &RenderDevice,
    size: u64,
    name: impl AsRef<CStr>,
    usage: Usage,
    bind_flags: BindFlags,
    cpu_access_flags: CpuAccessFlags,
    data: &T,
) -> Result<Boxed<Buffer>, ()> {
    let cpu_access_flags = match usage {
        Usage::Default | Usage::Immutable => CpuAccessFlags::None,
        _ => cpu_access_flags,
    };

    let cb_desc = BufferDesc::builder()
        .name(name.as_ref())
        .size(size)
        .usage(usage)
        .bind_flags(bind_flags)
        .cpu_access_flags(cpu_access_flags)
        .build();

    device.create_buffer_with_data(&cb_desc, data, None)
}

// https://en.wikipedia.org/wiki/SRGB
pub fn linear_to_gamma(x: f32) -> f32 {
    if x <= 0.0031308 {
        x * 12.92
    } else {
        1.055 * f32::powf(x, 1.0 / 2.4) - 0.055
    }
}

pub fn linear_to_srgba(rgba: [f32; 4]) -> [f32; 4] {
    [
        linear_to_gamma(rgba[0]),
        linear_to_gamma(rgba[1]),
        linear_to_gamma(rgba[2]),
        rgba[3],
    ]
}
