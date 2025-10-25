pub mod engine_factory_gl;

#[cfg(feature = "opengl_interop")]
#[path = ""]
mod gl_interop_modules {
    pub mod buffer_gl;
    pub mod device_context_gl;
    pub mod pipeline_state_gl;
    pub mod query_gl;
    pub mod render_device_gl;
    // TODO : RenderDeviceGLES
    pub mod shader_gl;
    pub mod swap_chain_gl;
    pub mod texture_gl;
}

#[cfg(feature = "opengl_interop")]
pub use gl_interop_modules::*;
