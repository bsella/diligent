use std::{ffi::CString, os::unix::ffi::OsStrExt, path::Path, str::FromStr};

use bitflags::bitflags;
use static_assertions::const_assert;

use super::{
    device_object::DeviceObject,
    graphics_types::{ShaderType, Version},
    object::Object,
};

pub enum ShaderSource<'a> {
    FilePath(&'a Path),
    SourceCode(&'a str),
    ByteCode(&'a [u8]),
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

impl From<&ShaderLanguage> for diligent_sys::SHADER_SOURCE_LANGUAGE {
    fn from(value: &ShaderLanguage) -> Self {
        (match value {
            ShaderLanguage::Default => diligent_sys::SHADER_SOURCE_LANGUAGE_DEFAULT,
            ShaderLanguage::HLSL => diligent_sys::SHADER_SOURCE_LANGUAGE_HLSL,
            ShaderLanguage::GLSL => diligent_sys::SHADER_SOURCE_LANGUAGE_GLSL,
            ShaderLanguage::GLSLVerbatim => diligent_sys::SHADER_SOURCE_LANGUAGE_GLSL_VERBATIM,
            ShaderLanguage::MSL => diligent_sys::SHADER_SOURCE_LANGUAGE_MSL,
            ShaderLanguage::MSLVerbatim => diligent_sys::SHADER_SOURCE_LANGUAGE_MSL_VERBATIM,
            ShaderLanguage::MTLB => diligent_sys::SHADER_SOURCE_LANGUAGE_MTLB,
            ShaderLanguage::WGSL => diligent_sys::SHADER_SOURCE_LANGUAGE_WGSL,
        }) as diligent_sys::SHADER_SOURCE_LANGUAGE
    }
}

pub enum ShaderCompiler {
    Default,
    GLSLANG,
    DXC,
    FXC,
}

impl From<&ShaderCompiler> for diligent_sys::SHADER_COMPILER {
    fn from(value: &ShaderCompiler) -> Self {
        (match value {
            ShaderCompiler::Default => diligent_sys::SHADER_COMPILER_DEFAULT,
            ShaderCompiler::GLSLANG => diligent_sys::SHADER_COMPILER_GLSLANG,
            ShaderCompiler::DXC => diligent_sys::SHADER_COMPILER_DXC,
            ShaderCompiler::FXC => diligent_sys::SHADER_COMPILER_FXC,
        }) as diligent_sys::SHADER_COMPILER
    }
}

pub enum ShaderResourceType {
    Unknown,
    ConstantBuffer,
    TextureSRV,
    BufferSRV,
    TextureUAV,
    BufferUAV,
    Sampler,
    InputAttachment,
    AccelStruct,
}
const_assert!(diligent_sys::SHADER_RESOURCE_TYPE_LAST == 8);

impl Into<ShaderResourceType> for diligent_sys::SHADER_RESOURCE_TYPE {
    fn into(self) -> ShaderResourceType {
        match self as diligent_sys::_SHADER_RESOURCE_TYPE {
            diligent_sys::SHADER_RESOURCE_TYPE_UNKNOWN => ShaderResourceType::Unknown,
            diligent_sys::SHADER_RESOURCE_TYPE_CONSTANT_BUFFER => {
                ShaderResourceType::ConstantBuffer
            }
            diligent_sys::SHADER_RESOURCE_TYPE_TEXTURE_SRV => ShaderResourceType::TextureSRV,
            diligent_sys::SHADER_RESOURCE_TYPE_BUFFER_SRV => ShaderResourceType::BufferSRV,
            diligent_sys::SHADER_RESOURCE_TYPE_TEXTURE_UAV => ShaderResourceType::TextureUAV,
            diligent_sys::SHADER_RESOURCE_TYPE_BUFFER_UAV => ShaderResourceType::BufferUAV,
            diligent_sys::SHADER_RESOURCE_TYPE_SAMPLER => ShaderResourceType::Sampler,
            diligent_sys::SHADER_RESOURCE_TYPE_INPUT_ATTACHMENT => {
                ShaderResourceType::InputAttachment
            }
            diligent_sys::SHADER_RESOURCE_TYPE_ACCEL_STRUCT => ShaderResourceType::AccelStruct,
            _ => panic!(),
        }
    }
}

pub struct ShaderResourceDesc {
    pub name: CString,
    pub resource_type: ShaderResourceType,
    pub array_size: usize,
}

impl From<diligent_sys::ShaderResourceDesc> for ShaderResourceDesc {
    fn from(value: diligent_sys::ShaderResourceDesc) -> Self {
        ShaderResourceDesc {
            name: unsafe { CString::from_raw(value.Name as _) },
            array_size: value.ArraySize as usize,
            resource_type: value.Type.into(),
        }
    }
}

bitflags! {
    pub struct ShaderCompileFlags : diligent_sys::SHADER_COMPILE_FLAGS {
        const None                  = diligent_sys::SHADER_COMPILE_FLAG_NONE as diligent_sys::SHADER_COMPILE_FLAGS;
        const EnableUnboundedArrays = diligent_sys::SHADER_COMPILE_FLAG_ENABLE_UNBOUNDED_ARRAYS as diligent_sys::SHADER_COMPILE_FLAGS;
        const SkipReflection        = diligent_sys::SHADER_COMPILE_FLAG_SKIP_REFLECTION as diligent_sys::SHADER_COMPILE_FLAGS;
        const Asynchronous          = diligent_sys::SHADER_COMPILE_FLAG_ASYNCHRONOUS as diligent_sys::SHADER_COMPILE_FLAGS;
        const PackMatrixRowMajor    = diligent_sys::SHADER_COMPILE_FLAG_PACK_MATRIX_ROW_MAJOR as diligent_sys::SHADER_COMPILE_FLAGS;
        const HlslToSpirvViaGlsl    = diligent_sys::SHADER_COMPILE_FLAG_HLSL_TO_SPIRV_VIA_GLSL as diligent_sys::SHADER_COMPILE_FLAGS;
    }
}
const_assert!(diligent_sys::SHADER_COMPILE_FLAG_LAST == 16);

pub struct ShaderDesc {
    name: CString,
    shader_type: ShaderType,
    use_combined_texture_samplers: bool,
    combined_sampler_suffix: CString,
}

pub struct ShaderCreateInfo<'a> {
    source: ShaderSource<'a>,
    shader_source_input_stream_factory: Option<&'a ShaderSourceInputStreamFactory>,
    entry_point: String,
    macros: Vec<(String, String)>,
    desc: ShaderDesc,
    source_language: ShaderLanguage,
    compiler: ShaderCompiler,
    language_version: Version,
    compile_flags: ShaderCompileFlags,
}

impl<'a> ShaderCreateInfo<'a> {
    pub fn new(name: impl AsRef<str>, source: ShaderSource<'a>, shader_type: ShaderType) -> Self {
        ShaderCreateInfo {
            source,
            shader_source_input_stream_factory: None,
            entry_point: "main".to_owned(),
            macros: Vec::new(),
            desc: ShaderDesc::new(name, shader_type),
            source_language: ShaderLanguage::Default,
            compiler: ShaderCompiler::Default,
            language_version: Version { major: 0, minor: 0 },
            compile_flags: ShaderCompileFlags::None,
        }
    }

    pub fn entry_point(mut self, entry_point: impl Into<String>) -> Self {
        self.entry_point = entry_point.into();
        self
    }

    pub fn set_macros(mut self, macros: Vec<(impl Into<String>, impl Into<String>)>) -> Self {
        self.macros = macros
            .into_iter()
            .map(|(name, def)| (name.into(), def.into()))
            .collect();
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
}

pub(crate) struct ShaderCreateInfoWrapper {
    _macro_strings: Vec<(CString, CString)>,
    _macros: Vec<diligent_sys::ShaderMacro>,
    _entry_point: CString,
    _shader_source_path: Option<CString>,
    sci: diligent_sys::ShaderCreateInfo,
}
impl ShaderCreateInfoWrapper {
    pub fn get(&self) -> &diligent_sys::ShaderCreateInfo {
        &self.sci
    }
}

impl From<&ShaderCreateInfo<'_>> for ShaderCreateInfoWrapper {
    fn from(value: &ShaderCreateInfo<'_>) -> Self {
        let macro_strings = Vec::from_iter(value.macros.iter().map(|(name, def)| {
            (
                CString::new(name.as_str()).unwrap(),
                CString::new(def.as_str()).unwrap(),
            )
        }));

        let macros =
            Vec::from_iter(
                macro_strings
                    .iter()
                    .map(|(name, def)| diligent_sys::ShaderMacro {
                        Name: name.as_ptr(),
                        Definition: def.as_ptr(),
                    }),
            );

        let entry_point = CString::from_str(value.entry_point.as_str()).unwrap();

        let mut shader_source_path = None;

        let sci = diligent_sys::ShaderCreateInfo {
            FilePath: match &value.source {
                &ShaderSource::FilePath(path) => {
                    #[cfg(unix)]
                    {
                        shader_source_path =
                            Some(CString::new(path.as_os_str().as_bytes()).unwrap());
                    };

                    #[cfg(windows)]
                    {
                        use std::os::windows::ffi::OsStrExt;
                        shader_source_path = Some(CString::new(
                            path.as_os_str()
                                .encode_wide()
                                .chain(Some(0))
                                .map(|b| {
                                    let b = b.to_ne_bytes();
                                    b.get(0).map(|s| *s).into_iter().chain(b.get(1).map(|s| *s))
                                })
                                .flatten(),
                        ));
                    };
                    shader_source_path
                        .as_ref()
                        .map_or(std::ptr::null(), |path| path.as_ptr())
                }
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
                ShaderSource::ByteCode(code) => code.as_ptr() as _,
                _ => std::ptr::null(),
            },
            __bindgen_anon_1: diligent_sys::ShaderCreateInfo__bindgen_ty_1 {
                ByteCodeSize: match value.source {
                    ShaderSource::ByteCode(code) => code.len(),
                    ShaderSource::SourceCode(code) => code.len(),
                    _ => 0,
                },
            },
            EntryPoint: entry_point.as_ptr(),
            Macros: diligent_sys::ShaderMacroArray {
                Elements: macros.as_ptr(),
                Count: macros.len() as u32,
            },
            Desc: diligent_sys::ShaderDesc {
                _DeviceObjectAttribs: {
                    diligent_sys::DeviceObjectAttribs {
                        Name: value.desc.name.as_ptr(),
                    }
                },
                ShaderType: diligent_sys::SHADER_TYPE::from(&value.desc.shader_type),
                UseCombinedTextureSamplers: value.desc.use_combined_texture_samplers,
                CombinedSamplerSuffix: value.desc.combined_sampler_suffix.as_ptr(),
            },
            SourceLanguage: diligent_sys::SHADER_SOURCE_LANGUAGE::from(&value.source_language),
            ShaderCompiler: diligent_sys::SHADER_COMPILER::from(&value.compiler),
            HLSLVersion: diligent_sys::ShaderVersion {
                Major: value.language_version.major,
                Minor: value.language_version.minor,
            },
            GLSLVersion: diligent_sys::ShaderVersion {
                Major: value.language_version.major,
                Minor: value.language_version.minor,
            },
            GLESSLVersion: diligent_sys::ShaderVersion {
                Major: value.language_version.major,
                Minor: value.language_version.minor,
            },
            MSLVersion: diligent_sys::ShaderVersion {
                Major: value.language_version.major,
                Minor: value.language_version.minor,
            },
            // TODO
            CompileFlags: value.compile_flags.bits(),
            LoadConstantBufferReflection: false,
            GLSLExtensions: std::ptr::null(),
            WebGPUEmulatedArrayIndexSuffix: std::ptr::null(),
        };

        ShaderCreateInfoWrapper {
            _macro_strings: macro_strings,
            _macros: macros,
            _entry_point: entry_point,
            _shader_source_path: shader_source_path,
            sci,
        }
    }
}

impl ShaderDesc {
    fn new(name: impl AsRef<str>, shader_type: ShaderType) -> Self {
        ShaderDesc {
            name: CString::new(name.as_ref()).unwrap(),
            shader_type,
            use_combined_texture_samplers: false,
            combined_sampler_suffix: std::ffi::CString::new("_sampler").unwrap(),
        }
    }
}

pub struct Shader {
    pub(crate) sys_ptr: *mut diligent_sys::IShader,
    virtual_functions: *mut diligent_sys::IShaderVtbl,

    device_object: DeviceObject,
}

impl AsRef<DeviceObject> for Shader {
    fn as_ref(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl Shader {
    pub(crate) fn new(shader_ptr: *mut diligent_sys::IShader) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IDeviceObject>()
                == std::mem::size_of::<diligent_sys::IShader>()
        );

        Shader {
            sys_ptr: shader_ptr,
            virtual_functions: unsafe { (*shader_ptr).pVtbl },
            device_object: DeviceObject::new(shader_ptr as *mut diligent_sys::IDeviceObject),
        }
    }

    pub fn get_desc(&self) -> diligent_sys::ShaderDesc {
        unsafe {
            *((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.device_object.sys_ptr)
                as *const diligent_sys::ShaderDesc)
        }
    }

    pub fn get_resources(&self) -> Vec<ShaderResourceDesc> {
        unsafe {
            let num_resources = (*self.virtual_functions)
                .Shader
                .GetResourceCount
                .unwrap_unchecked()(self.sys_ptr);

            let mut resources = Vec::with_capacity(num_resources as usize);

            for index in 0..num_resources {
                let resources_ptr = std::ptr::null_mut();
                (*self.virtual_functions)
                    .Shader
                    .GetResourceDesc
                    .unwrap_unchecked()(self.sys_ptr, index, resources_ptr);
                resources.push(ShaderResourceDesc::from(*resources_ptr));
            }
            resources
        }
    }

    pub fn get_constant_buffer_desc(
        &self,
        index: u32,
    ) -> Option<&diligent_sys::ShaderCodeBufferDesc> {
        unsafe {
            (*self.virtual_functions)
                .Shader
                .GetConstantBufferDesc
                .unwrap_unchecked()(self.sys_ptr, index)
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
                self.sys_ptr,
                bytecode as *mut *const _,
                std::ptr::addr_of_mut!(size),
            );
        }
        size
    }

    pub fn get_status(&self, wait_for_completion: bool) -> diligent_sys::SHADER_STATUS {
        unsafe {
            (*self.virtual_functions)
                .Shader
                .GetStatus
                .unwrap_unchecked()(self.sys_ptr, wait_for_completion)
        }
    }
}

pub struct ShaderSourceInputStreamFactory {
    pub(crate) factory_ptr: *mut diligent_sys::IShaderSourceInputStreamFactory,
    #[allow(dead_code)] // TODO : imlement methods of ShaderSourceInputStreamFactory
    virtual_functions: *mut diligent_sys::IShaderSourceInputStreamFactoryVtbl,

    object: Object,
}

impl AsRef<Object> for ShaderSourceInputStreamFactory {
    fn as_ref(&self) -> &Object {
        &self.object
    }
}

impl ShaderSourceInputStreamFactory {
    pub(crate) fn new(factory_ptr: *mut diligent_sys::IShaderSourceInputStreamFactory) -> Self {
        ShaderSourceInputStreamFactory {
            factory_ptr,
            virtual_functions: unsafe { (*factory_ptr).pVtbl },
            object: Object::new(factory_ptr as *mut diligent_sys::IObject),
        }
    }

    //pub fn create_input_stream(&self, name : impl AsRef<str>, IFileStream** ppStream);

    //pub fn create_input_stream2(&self, name : impl AsRef<str>, CREATE_SHADER_SOURCE_INPUT_STREAM_FLAGS Flags, IFileStream** ppStream);
}
