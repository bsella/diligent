use std::{
    os::{raw::c_void, unix::ffi::OsStrExt},
    path::PathBuf,
};

use bitflags::bitflags;
use static_assertions::const_assert;

use crate::bindings::{self, ShaderMacro, Version};

use super::{
    device_object::{AsDeviceObject, DeviceObject},
    graphics_types::ShaderType,
    object::{AsObject, Object},
};

pub enum ShaderSource<'a> {
    FilePath(PathBuf),
    SourceCode(&'a str),
    ByteCode(*const c_void, usize),
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

impl From<&ShaderLanguage> for bindings::SHADER_SOURCE_LANGUAGE {
    fn from(value: &ShaderLanguage) -> Self {
        (match value {
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

impl From<&ShaderCompiler> for bindings::SHADER_COMPILER {
    fn from(value: &ShaderCompiler) -> Self {
        (match value {
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

bitflags! {
    pub struct ShaderCompileFlags : bindings::_SHADER_COMPILE_FLAGS {
        const None                  = bindings::SHADER_COMPILE_FLAG_NONE;
        const EnableUnboundedArrays = bindings::SHADER_COMPILE_FLAG_ENABLE_UNBOUNDED_ARRAYS;
        const SkipReflection        = bindings::SHADER_COMPILE_FLAG_SKIP_REFLECTION;
        const Asynchronous          = bindings::SHADER_COMPILE_FLAG_ASYNCHRONOUS;
        const PackMatrixRowMajor    = bindings::SHADER_COMPILE_FLAG_PACK_MATRIX_ROW_MAJOR;
    }
}
const_assert!(bindings::SHADER_COMPILE_FLAG_LAST == 8);

pub struct ShaderDesc<'a> {
    name: &'a std::ffi::CStr,
    shader_type: ShaderType,
    use_combined_texture_samplers: bool,
    combined_sampler_suffix: std::ffi::CString,
}

pub struct ShaderCreateInfo<'a> {
    source: ShaderSource<'a>,
    shader_source_input_stream_factory: Option<&'a ShaderSourceInputStreamFactory>,
    entry_point: &'a std::ffi::CStr,
    macros: Vec<(&'a std::ffi::CStr, &'a std::ffi::CStr)>,
    desc: ShaderDesc<'a>,
    source_language: ShaderLanguage,
    compiler: ShaderCompiler,
    language_version: Version,
    compile_flags: ShaderCompileFlags,
}

impl<'a> ShaderCreateInfo<'a> {
    pub fn new(
        name: &'a std::ffi::CStr,
        source: ShaderSource<'a>,
        shader_type: ShaderType,
    ) -> Self {
        ShaderCreateInfo {
            source,
            shader_source_input_stream_factory: None,
            entry_point: c"main",
            macros: Vec::new(),
            desc: ShaderDesc::new(name, shader_type),
            source_language: ShaderLanguage::Default,
            compiler: ShaderCompiler::Default,
            language_version: Version { Major: 0, Minor: 0 },
            compile_flags: ShaderCompileFlags::None,
        }
    }

    pub fn entry_point(mut self, entry_point: &'a std::ffi::CStr) -> Self {
        self.entry_point = entry_point;
        self
    }

    pub fn add_macro(mut self, name: &'a std::ffi::CStr, definition: &'a std::ffi::CStr) -> Self {
        self.macros.push((name, definition));
        self
    }

    pub fn use_combined_texture_samplers(mut self, use_combined_texture_samplers: bool) -> Self {
        self.desc.use_combined_texture_samplers = use_combined_texture_samplers;
        self
    }

    pub fn language(mut self, language: ShaderLanguage) -> Self {
        self.source_language = language;
        self
    }

    pub fn compiler(mut self, compiler: ShaderCompiler) -> Self {
        self.compiler = compiler;
        self
    }

    pub fn language_version(mut self, version: Version) -> Self {
        self.language_version = version;
        self
    }

    pub fn compile_flags(mut self, compile_flags: ShaderCompileFlags) -> Self {
        self.compile_flags = compile_flags;
        self
    }

    pub fn shader_source_input_stream_factory(
        mut self,
        shader_source_input_stream_factory: Option<&'a ShaderSourceInputStreamFactory>,
    ) -> Self {
        self.shader_source_input_stream_factory = shader_source_input_stream_factory;
        self
    }

    pub fn name(mut self, name: &'a std::ffi::CStr) -> Self {
        self.desc.name = name;
        self
    }
    pub fn source(mut self, source: ShaderSource<'a>) -> Self {
        self.source = source;
        self
    }

    pub fn shader_type(mut self, shader_type: ShaderType) -> Self {
        self.desc.shader_type = shader_type;
        self
    }
}

pub(crate) struct ShaderCreateInfoWrapper {
    _macros: Vec<ShaderMacro>,
    sci: bindings::ShaderCreateInfo,
}
impl ShaderCreateInfoWrapper {
    pub fn get(&self) -> &bindings::ShaderCreateInfo {
        &self.sci
    }
}

impl From<&ShaderCreateInfo<'_>> for ShaderCreateInfoWrapper {
    fn from(value: &ShaderCreateInfo<'_>) -> Self {
        let macros = Vec::from_iter(
            value
                .macros
                .iter()
                .map(|(name, def)| bindings::ShaderMacro {
                    Name: name.as_ptr(),
                    Definition: def.as_ptr(),
                }),
        );

        let sci = bindings::ShaderCreateInfo {
            FilePath: match &value.source {
                ShaderSource::FilePath(path) => path.as_os_str().as_bytes().as_ptr() as *const i8,
                _ => std::ptr::null(),
            },
            pShaderSourceStreamFactory: value
                .shader_source_input_stream_factory
                .map_or(std::ptr::null_mut(), |stream_factory| {
                    stream_factory.factory_ptr
                }),
            Source: match value.source {
                ShaderSource::SourceCode(code) => code.as_ptr() as *const i8,
                _ => std::ptr::null(),
            },
            ByteCode: match value.source {
                ShaderSource::ByteCode(code, _) => code,
                _ => std::ptr::null(),
            },
            __bindgen_anon_1: bindings::ShaderCreateInfo__bindgen_ty_1 {
                ByteCodeSize: match value.source {
                    ShaderSource::ByteCode(_, size) => size,
                    ShaderSource::SourceCode(code) => code.len(),
                    _ => 0,
                },
            },
            EntryPoint: value.entry_point.as_ptr(),
            Macros: bindings::ShaderMacroArray {
                Elements: macros.as_ptr(),
                Count: macros.len() as u32,
            },
            Desc: bindings::ShaderDesc {
                _DeviceObjectAttribs: {
                    bindings::DeviceObjectAttribs {
                        Name: value.desc.name.as_ptr(),
                    }
                },
                ShaderType: bindings::SHADER_TYPE::from(&value.desc.shader_type),
                UseCombinedTextureSamplers: value.desc.use_combined_texture_samplers,
                CombinedSamplerSuffix: value.desc.combined_sampler_suffix.as_ptr(),
            },
            SourceLanguage: bindings::SHADER_SOURCE_LANGUAGE::from(&value.source_language),
            ShaderCompiler: bindings::SHADER_COMPILER::from(&value.compiler),
            HLSLVersion: bindings::ShaderVersion {
                Major: value.language_version.Major,
                Minor: value.language_version.Minor,
            },
            GLSLVersion: bindings::ShaderVersion {
                Major: value.language_version.Major,
                Minor: value.language_version.Minor,
            },
            GLESSLVersion: bindings::ShaderVersion {
                Major: value.language_version.Major,
                Minor: value.language_version.Minor,
            },
            MSLVersion: bindings::ShaderVersion {
                Major: value.language_version.Major,
                Minor: value.language_version.Minor,
            },
            // TODO
            CompileFlags: value.compile_flags.bits(),
            LoadConstantBufferReflection: false,
            GLSLExtensions: std::ptr::null(),
            WebGPUEmulatedArrayIndexSuffix: std::ptr::null(),
        };

        ShaderCreateInfoWrapper {
            _macros: macros,
            sci,
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

pub struct ShaderSourceInputStreamFactory {
    pub(crate) factory_ptr: *mut bindings::IShaderSourceInputStreamFactory,
    #[allow(dead_code)] // TODO : imlement methods of ShaderSourceInputStreamFactory
    virtual_functions: *mut bindings::IShaderSourceInputStreamFactoryVtbl,

    object: Object,
}

impl AsObject for ShaderSourceInputStreamFactory {
    fn as_object(&self) -> &Object {
        &self.object
    }
}

impl ShaderSourceInputStreamFactory {
    pub(crate) fn new(factory_ptr: *mut bindings::IShaderSourceInputStreamFactory) -> Self {
        ShaderSourceInputStreamFactory {
            factory_ptr,
            virtual_functions: unsafe { (*factory_ptr).pVtbl },
            object: Object::new(factory_ptr as *mut bindings::IObject),
        }
    }

    //pub fn create_input_stream(&self, name : &std::ffi::CStr, IFileStream** ppStream);

    //pub fn create_input_stream2(&self, name : &std::ffi::CStr, CREATE_SHADER_SOURCE_INPUT_STREAM_FLAGS Flags, IFileStream** ppStream);
}
