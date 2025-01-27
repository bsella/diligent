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
    SourceCode(&'a str),
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

pub struct ShaderDesc<'a> {
    name: &'a std::ffi::CStr,
    shader_type: ShaderType,
    use_combined_texture_samplers: bool,
    combined_sampler_suffix: std::ffi::CString,
}

pub struct ShaderCreateInfo<'a> {
    source: ShaderSource<'a>,
    // TODO IShaderSourceInputStreamFactory
    entry_point: &'a std::ffi::CStr,
    macros: Vec<(&'a std::ffi::CStr, &'a std::ffi::CStr)>,
    desc: ShaderDesc<'a>,
    source_language: ShaderLanguage,
    compiler: ShaderCompiler,
    language_version: Version,
}

impl<'a> ShaderCreateInfo<'a> {
    pub fn new(
        name: &'a std::ffi::CStr,
        source: ShaderSource<'a>,
        shader_type: ShaderType,
    ) -> Self {
        ShaderCreateInfo {
            source: source,
            entry_point: c"main",
            macros: Vec::new(),
            desc: ShaderDesc::new(name, shader_type),
            source_language: ShaderLanguage::Default,
            compiler: ShaderCompiler::Default,
            language_version: Version { Major: 0, Minor: 0 },
        }
    }

    pub fn entry_point(mut self, entry_point: &'a std::ffi::CStr) -> ShaderCreateInfo<'a> {
        self.entry_point = entry_point;
        self
    }

    pub fn add_macro(
        mut self,
        name: &'a std::ffi::CStr,
        definition: &'a std::ffi::CStr,
    ) -> ShaderCreateInfo<'a> {
        self.macros.push((name, definition));
        self
    }

    pub fn use_combined_texture_samplers(
        mut self,
        use_combined_texture_samplers: bool,
    ) -> ShaderCreateInfo<'a> {
        self.desc.use_combined_texture_samplers = use_combined_texture_samplers;
        self
    }

    pub fn language(mut self, language: ShaderLanguage) -> ShaderCreateInfo<'a> {
        self.source_language = language;
        self
    }

    pub fn compiler(mut self, compiler: ShaderCompiler) -> ShaderCreateInfo<'a> {
        self.compiler = compiler;
        self
    }

    pub fn language_version(mut self, version: Version) -> ShaderCreateInfo<'a> {
        self.language_version = version;
        self
    }
}

impl<'a> Into<bindings::ShaderCreateInfo> for ShaderCreateInfo<'a> {
    fn into(self) -> bindings::ShaderCreateInfo {
        let macros = Vec::from_iter(self.macros.iter().map(|(name, def)| bindings::ShaderMacro {
            Name: name.as_ptr(),
            Definition: def.as_ptr(),
        }));
        bindings::ShaderCreateInfo {
            FilePath: match &self.source {
                ShaderSource::FilePath(path) => path.as_os_str().as_bytes().as_ptr() as *const i8,
                _ => std::ptr::null(),
            },
            pShaderSourceStreamFactory: std::ptr::null_mut(), // TODO
            Source: match self.source {
                ShaderSource::SourceCode(code) => code.as_ptr() as *const i8,
                _ => std::ptr::null(),
            },
            ByteCode: match self.source {
                ShaderSource::ByteCode(code, _) => code,
                _ => std::ptr::null(),
            },
            __bindgen_anon_1: bindings::ShaderCreateInfo__bindgen_ty_1 {
                ByteCodeSize: match self.source {
                    ShaderSource::ByteCode(_, Some(size)) => size,
                    ShaderSource::SourceCode(code) => code.len(),
                    _ => 0,
                },
            },
            EntryPoint: self.entry_point.as_ptr(),
            Macros: bindings::ShaderMacroArray {
                Elements: macros.as_ptr(),
                Count: macros.len() as u32,
            },
            Desc: bindings::ShaderDesc {
                _DeviceObjectAttribs: {
                    bindings::DeviceObjectAttribs {
                        Name: self.desc.name.as_ptr(),
                    }
                },
                ShaderType: self.desc.shader_type.into(),
                UseCombinedTextureSamplers: self.desc.use_combined_texture_samplers,
                CombinedSamplerSuffix: self.desc.combined_sampler_suffix.as_ptr(),
            },
            SourceLanguage: self.source_language.into(),
            ShaderCompiler: self.compiler.into(),
            HLSLVersion: bindings::ShaderVersion {
                Major: self.language_version.Major,
                Minor: self.language_version.Minor,
            },
            GLSLVersion: bindings::ShaderVersion {
                Major: self.language_version.Major,
                Minor: self.language_version.Minor,
            },
            GLESSLVersion: bindings::ShaderVersion {
                Major: self.language_version.Major,
                Minor: self.language_version.Minor,
            },
            MSLVersion: bindings::ShaderVersion {
                Major: self.language_version.Major,
                Minor: self.language_version.Minor,
            },
            // TODO
            CompileFlags: 0,
            LoadConstantBufferReflection: false,
            GLSLExtensions: std::ptr::null(),
            WebGPUEmulatedArrayIndexSuffix: std::ptr::null(),
        }
    }
}

impl<'a> ShaderDesc<'a> {
    fn new(name: &'a std::ffi::CStr, shader_type: ShaderType) -> Self {
        ShaderDesc {
            name: name,
            shader_type: shader_type,
            use_combined_texture_samplers: false,
            combined_sampler_suffix: std::ffi::CString::new("_sampler").unwrap(),
        }
    }
}

pub struct Shader {
    pub(crate) shader: *mut bindings::IShader,
    virtual_functions: *mut bindings::IShaderVtbl,

    device_object: DeviceObject,
}

impl AsDeviceObject for Shader {
    fn as_device_object(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl Shader {
    pub(crate) fn new(shader_ptr: *mut bindings::IShader) -> Self {
        Shader {
            shader: shader_ptr,
            virtual_functions: unsafe { (*shader_ptr).pVtbl },
            device_object: DeviceObject::new(shader_ptr as *mut bindings::IDeviceObject),
        }
    }

    pub fn get_desc(&self) -> bindings::ShaderDesc {
        unsafe {
            *((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.shader as *mut bindings::IDeviceObject)
                as *const bindings::ShaderDesc)
        }
    }

    pub fn get_resources(&self) -> Vec<bindings::ShaderResourceDesc> {
        unsafe {
            let num_resources = (*self.virtual_functions)
                .Shader
                .GetResourceCount
                .unwrap_unchecked()(self.shader);

            let mut resources = Vec::with_capacity(num_resources as usize);

            for index in 0..num_resources {
                let resources_ptr = std::ptr::null_mut();
                (*self.virtual_functions)
                    .Shader
                    .GetResourceDesc
                    .unwrap_unchecked()(self.shader, index, resources_ptr);
                resources.push(*resources_ptr);
            }
            resources
        }
    }

    pub fn get_constant_buffer_desc(&self, index: u32) -> Option<&bindings::ShaderCodeBufferDesc> {
        unsafe {
            (*self.virtual_functions)
                .Shader
                .GetConstantBufferDesc
                .unwrap_unchecked()(self.shader, index)
            .as_ref()
        }
    }

    pub fn get_bytecode(&self, bytecode: *mut *const u8) -> u64 {
        let mut size: u64 = 0;
        unsafe {
            (*self.virtual_functions)
                .Shader
                .GetBytecode
                .unwrap_unchecked()(
                self.shader,
                bytecode as *mut *const c_void,
                std::ptr::addr_of_mut!(size),
            );
        }
        size
    }

    pub fn get_status(&self, wait_for_completion: bool) -> bindings::SHADER_STATUS {
        unsafe {
            (*self.virtual_functions)
                .Shader
                .GetStatus
                .unwrap_unchecked()(self.shader, wait_for_completion)
        }
    }
}
