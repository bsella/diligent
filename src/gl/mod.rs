pub mod engine_factory_gl;

#[cfg(feature = "opengl_interop")]
#[path = ""]
mod gl_interop_modules {
    // TODO
    // BaseInterfacesGL
    // BufferGL
    // BufferViewGL
    // DeviceContextGL
    // EngineFactoryOpenGL
    // FenceGL
    // PipelineStateGL
    // QueryGL
    // RenderDeviceGL
    // RenderDeviceGLES
    // SamplerGL
    // ShaderGL
    // ShaderResourceBindingGL
    // SwapChainGL
    // TextureGL
    // TextureViewG
}

#[cfg(feature = "opengl_interop")]
pub use gl_interop_modules::*;
