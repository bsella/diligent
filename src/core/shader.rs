use std::{
    os::{raw::c_void, unix::ffi::OsStrExt},
    path::PathBuf,
};

use crate::bindings::{self, Version};

use super::{
    device_object::{AsDeviceObject, DeviceObject},
    graphics_types::ShaderType,
};

pub enum ShaderSource<'a> {
    FilePath(PathBuf), // TODO Option<IShaderSourceInputStreamFactory>
    SourceCode(&'a str, Option<usize>),
    ByteCode(*const c_void, Option<usize>),
}

pub enum ShaderLanguage {
    Default,
    HLSL,
    GLSL,
    GLSLVerbatim,
    MSL,
    MSLVerbatim,
    MTLB,
    WGSL,
}

impl Into<bindings::SHADER_SOURCE_LANGUAGE> for ShaderLanguage {
    fn into(self) -> bindings::SHADER_SOURCE_LANGUAGE {
        (match self {
            ShaderLanguage::Default => bindings::SHADER_SOURCE_LANGUAGE_DEFAULT,
            ShaderLanguage::HLSL => bindings::SHADER_SOURCE_LANGUAGE_HLSL,
            ShaderLanguage::GLSL => bindings::SHADER_SOURCE_LANGUAGE_GLSL,
            ShaderLanguage::GLSLVerbatim => bindings::SHADER_SOURCE_LANGUAGE_GLSL_VERBATIM,
            ShaderLanguage::MSL => bindings::SHADER_SOURCE_LANGUAGE_MSL,
            ShaderLanguage::MSLVerbatim => bindings::SHADER_SOURCE_LANGUAGE_MSL_VERBATIM,
            ShaderLanguage::MTLB => bindings::SHADER_SOURCE_LANGUAGE_MTLB,
            ShaderLanguage::WGSL => bindings::SHADER_SOURCE_LANGUAGE_WGSL,
        }) as bindings::SHADER_SOURCE_LANGUAGE
    }
}

pub enum ShaderCompiler {
    Default,
    GLSLANG,
    DXC,
    FXC,
}

impl Into<bindings::SHADER_COMPILER> for ShaderCompiler {
    fn into(self) -> bindings::SHADER_COMPILER {
        (match self {
            ShaderCompiler::Default => bindings::SHADER_COMPILER_DEFAULT,
            ShaderCompiler::GLSLANG => bindings::SHADER_COMPILER_GLSLANG,
            ShaderCompiler::DXC => bindings::SHADER_COMPILER_DXC,
            ShaderCompiler::FXC => bindings::SHADER_COMPILER_FXC,
        }) as bindings::SHADER_COMPILER
    }
}

impl Default for bindings::ShaderResourceDesc {
    fn default() -> Self {
        bindings::ShaderResourceDesc {
            Name: std::ptr::null(),
            Type: bindings::SHADER_RESOURCE_TYPE_UNKNOWN as u8,
            ArraySize: 0,
        }
    }
}

pub struct ShaderDesc {
    pub name: String,
    pub shader_type: ShaderType,
    pub use_combined_texture_samplers: bool,
    pub combined_sampler_suffix: String,
}

pub struct ShaderCreateInfo<'a> {
    pub source: ShaderSource<'a>,
    // TODO IShaderSourceInputStreamFactory
    pub entry_point: &'a str,
    pub macros: Vec<(&'a str, &'a str)>,
    pub desc: ShaderDesc,
    pub source_language: ShaderLanguage,
    pub compiler: ShaderCompiler,
    pub hlsl_version: Version,
    pub glsl_version: Version,
    pub glessl_version: Version,
    pub msl_version: Version,
}

impl<'a> ShaderCreateInfo<'a> {
    pub fn new(name: &str, source: ShaderSource<'a>, shader_type: ShaderType) -> Self {
        ShaderCreateInfo {
            source: source,
            entry_point: "main",
            macros: Vec::new(),
            desc: ShaderDesc::new(name, shader_type),
            source_language: ShaderLanguage::Default,
            compiler: ShaderCompiler::Default,
            hlsl_version: Version { Major: 0, Minor: 0 },
            glsl_version: Version { Major: 0, Minor: 0 },
            glessl_version: Version { Major: 0, Minor: 0 },
            msl_version: Version { Major: 0, Minor: 0 },
        }
    }
}

impl<'a> Into<bindings::ShaderCreateInfo> for ShaderCreateInfo<'a> {
    fn into(self) -> bindings::ShaderCreateInfo {
        let macros = Vec::from_iter(self.macros.iter().map(|(name, def)| bindings::ShaderMacro {
            Name: name.as_ptr() as *const i8,
            Definition: def.as_ptr() as *const i8,
        }));
        bindings::ShaderCreateInfo {
            FilePath: match &self.source {
                ShaderSource::FilePath(path) => path.as_os_str().as_bytes().as_ptr() as *const i8,
                _ => std::ptr::null(),
            },
            pShaderSourceStreamFactory: std::ptr::null_mut(), // TODO
            Source: match self.source {
                ShaderSource::SourceCode(code, _) => code.as_ptr() as *const i8,
                _ => std::ptr::null(),
            },
            ByteCode: match self.source {
                ShaderSource::ByteCode(code, _) => code,
                _ => std::ptr::null(),
            },
            __bindgen_anon_1: bindings::ShaderCreateInfo__bindgen_ty_1 {
                ByteCodeSize: match self.source {
                    ShaderSource::ByteCode(_, Some(size)) => size,
                    ShaderSource::SourceCode(_, Some(size)) => size,
                    _ => 0,
                },
            },
            EntryPoint: self.entry_point.as_ptr() as *const i8,
            Macros: bindings::ShaderMacroArray {
                Elements: macros.as_ptr(),
                Count: macros.len() as u32,
            },
            Desc: bindings::ShaderDesc {
                _DeviceObjectAttribs: {
                    bindings::DeviceObjectAttribs {
                        Name: self.desc.name.as_ptr() as *const i8,
                    }
                },
                ShaderType: self.desc.shader_type.into(),
                UseCombinedTextureSamplers: self.desc.use_combined_texture_samplers,
                CombinedSamplerSuffix: self.desc.combined_sampler_suffix.as_ptr() as *const i8,
            },
            SourceLanguage: self.source_language.into(),
            ShaderCompiler: self.compiler.into(),
            HLSLVersion: bindings::ShaderVersion {
                Major: self.hlsl_version.Major,
                Minor: self.hlsl_version.Minor,
            },
            GLSLVersion: bindings::ShaderVersion {
                Major: self.hlsl_version.Major,
                Minor: self.hlsl_version.Minor,
            },
            GLESSLVersion: bindings::ShaderVersion {
                Major: self.hlsl_version.Major,
                Minor: self.hlsl_version.Minor,
            },
            MSLVersion: bindings::ShaderVersion {
                Major: self.hlsl_version.Major,
                Minor: self.hlsl_version.Minor,
            },
            // TODO
            CompileFlags: 0,
            LoadConstantBufferReflection: false,
            GLSLExtensions: std::ptr::null(),
            WebGPUEmulatedArrayIndexSuffix: std::ptr::null(),
        }
    }
}

impl ShaderDesc {
    fn new(name: &str, shader_type: ShaderType) -> Self {
        ShaderDesc {
            name: name.to_string(),
            shader_type: shader_type,
            use_combined_texture_samplers: false,
            combined_sampler_suffix: "_sampler".to_string(),
        }
    }
}

pub struct Shader {
    pub(crate) m_shader: *mut bindings::IShader,
    m_virtual_functions: *mut bindings::IShaderVtbl,

    m_device_object: DeviceObject,
}

impl AsDeviceObject for Shader {
    fn as_device_object(&self) -> &DeviceObject {
        &self.m_device_object
    }
}

impl Shader {
    pub(crate) fn new(shader_ptr: *mut bindings::IShader) -> Self {
        Shader {
            m_shader: shader_ptr,
            m_virtual_functions: unsafe { (*shader_ptr).pVtbl },
            m_device_object: DeviceObject::new(shader_ptr as *mut bindings::IDeviceObject),
        }
    }

    fn get_desc(&self) -> bindings::ShaderDesc {
        unsafe {
            *((*self.m_virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.m_shader as *mut bindings::IDeviceObject)
                as *const bindings::ShaderDesc)
        }
    }

    fn get_resources(&self) -> Vec<bindings::ShaderResourceDesc> {
        unsafe {
            let num_resources = (*self.m_virtual_functions)
                .Shader
                .GetResourceCount
                .unwrap_unchecked()(self.m_shader);

            let mut resources = Vec::with_capacity(num_resources as usize);

            for index in 0..num_resources {
                let resources_ptr = std::ptr::null_mut();
                (*self.m_virtual_functions)
                    .Shader
                    .GetResourceDesc
                    .unwrap_unchecked()(self.m_shader, index, resources_ptr);
                resources.push(*resources_ptr);
            }
            resources
        }
    }

    fn get_constant_buffer_desc(&self, index: u32) -> Option<&bindings::ShaderCodeBufferDesc> {
        unsafe {
            (*self.m_virtual_functions)
                .Shader
                .GetConstantBufferDesc
                .unwrap_unchecked()(self.m_shader, index)
            .as_ref()
        }
    }

    fn get_bytecode(&self, bytecode: *mut *const u8) -> u64 {
        unsafe {
            let mut size: u64 = 0;
            (*self.m_virtual_functions)
                .Shader
                .GetBytecode
                .unwrap_unchecked()(
                self.m_shader,
                bytecode as *mut *const c_void,
                std::ptr::addr_of_mut!(size),
            );
            size
        }
    }

    fn get_status(&self, wait_for_completion: bool) -> bindings::SHADER_STATUS {
        unsafe {
            (*self.m_virtual_functions)
                .Shader
                .GetStatus
                .unwrap_unchecked()(self.m_shader, wait_for_completion)
        }
    }
}
