#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

use std::{ffi::CString, ops::Deref, path::Path, str::FromStr};

use bitflags::bitflags;
use bon::Builder;
use static_assertions::const_assert;

use crate::{
    device_object::DeviceObject,
    graphics_types::{ShaderType, Version},
    object::Object,
};

#[derive(Clone, Copy)]
pub enum ShaderSource<'a> {
    FilePath(&'a Path),
    SourceCode(&'a str),
    ByteCode(&'a [u8]),
}

#[derive(Clone, Copy, Default)]
pub enum ShaderLanguage {
    #[default]
    Default,
    HLSL,
    GLSL,
    GLSLVerbatim,
    MSL,
    MSLVerbatim,
    MTLB,
    WGSL,
}

impl From<ShaderLanguage> for diligent_sys::SHADER_SOURCE_LANGUAGE {
    fn from(value: ShaderLanguage) -> Self {
        (match value {
            ShaderLanguage::Default => diligent_sys::SHADER_SOURCE_LANGUAGE_DEFAULT,
            ShaderLanguage::HLSL => diligent_sys::SHADER_SOURCE_LANGUAGE_HLSL,
            ShaderLanguage::GLSL => diligent_sys::SHADER_SOURCE_LANGUAGE_GLSL,
            ShaderLanguage::GLSLVerbatim => diligent_sys::SHADER_SOURCE_LANGUAGE_GLSL_VERBATIM,
            ShaderLanguage::MSL => diligent_sys::SHADER_SOURCE_LANGUAGE_MSL,
            ShaderLanguage::MSLVerbatim => diligent_sys::SHADER_SOURCE_LANGUAGE_MSL_VERBATIM,
            ShaderLanguage::MTLB => diligent_sys::SHADER_SOURCE_LANGUAGE_MTLB,
            ShaderLanguage::WGSL => diligent_sys::SHADER_SOURCE_LANGUAGE_WGSL,
        }) as _
    }
}

#[derive(Clone, Copy, Default)]
pub enum ShaderCompiler {
    #[default]
    Default,
    GLSLANG,
    DXC,
    FXC,
}

impl From<ShaderCompiler> for diligent_sys::SHADER_COMPILER {
    fn from(value: ShaderCompiler) -> Self {
        (match value {
            ShaderCompiler::Default => diligent_sys::SHADER_COMPILER_DEFAULT,
            ShaderCompiler::GLSLANG => diligent_sys::SHADER_COMPILER_GLSLANG,
            ShaderCompiler::DXC => diligent_sys::SHADER_COMPILER_DXC,
            ShaderCompiler::FXC => diligent_sys::SHADER_COMPILER_FXC,
        }) as _
    }
}

#[derive(Clone, Copy)]
pub enum ShaderResourceType {
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

impl From<diligent_sys::SHADER_RESOURCE_TYPE> for ShaderResourceType {
    fn from(value: diligent_sys::SHADER_RESOURCE_TYPE) -> Self {
        match value as diligent_sys::_SHADER_RESOURCE_TYPE {
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

impl From<ShaderResourceType> for diligent_sys::SHADER_RESOURCE_TYPE {
    fn from(value: ShaderResourceType) -> Self {
        (match value {
            ShaderResourceType::AccelStruct => diligent_sys::SHADER_RESOURCE_TYPE_ACCEL_STRUCT,
            ShaderResourceType::BufferSRV => diligent_sys::SHADER_RESOURCE_TYPE_BUFFER_SRV,
            ShaderResourceType::BufferUAV => diligent_sys::SHADER_RESOURCE_TYPE_BUFFER_UAV,
            ShaderResourceType::ConstantBuffer => {
                diligent_sys::SHADER_RESOURCE_TYPE_CONSTANT_BUFFER
            }
            ShaderResourceType::InputAttachment => {
                diligent_sys::SHADER_RESOURCE_TYPE_INPUT_ATTACHMENT
            }
            ShaderResourceType::Sampler => diligent_sys::SHADER_RESOURCE_TYPE_SAMPLER,
            ShaderResourceType::TextureSRV => diligent_sys::SHADER_RESOURCE_TYPE_TEXTURE_SRV,
            ShaderResourceType::TextureUAV => diligent_sys::SHADER_RESOURCE_TYPE_TEXTURE_UAV,
        }) as _
    }
}

pub struct ShaderResourceDesc {
    pub name: CString,
    pub resource_type: ShaderResourceType,
    pub array_size: usize,
}

impl From<&diligent_sys::ShaderResourceDesc> for ShaderResourceDesc {
    fn from(value: &diligent_sys::ShaderResourceDesc) -> Self {
        ShaderResourceDesc {
            name: unsafe { CString::from_raw(value.Name as _) },
            array_size: value.ArraySize as usize,
            resource_type: value.Type.into(),
        }
    }
}

bitflags! {
    #[derive(Clone,Copy)]
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

impl Default for ShaderCompileFlags {
    fn default() -> Self {
        ShaderCompileFlags::None
    }
}

#[derive(Builder)]
#[builder(derive(Clone))]
pub struct ShaderCreateInfo<'a> {
    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    name: CString,

    source: ShaderSource<'a>,

    shader_type: ShaderType,

    #[builder(default = false)]
    use_combined_texture_samplers: bool,

    #[builder(with =|suffix : impl AsRef<str>| CString::new(suffix.as_ref()).unwrap())]
    #[builder(default = CString::new("_sampler").unwrap())]
    combined_sampler_suffix: CString,

    shader_source_input_stream_factory: Option<&'a ShaderSourceInputStreamFactory>,

    #[builder(with =|ep : impl AsRef<str>| ep.as_ref().to_owned())]
    #[builder(default = "main".to_owned())]
    entry_point: String,

    #[builder(with =|macros: Vec<(impl Into<String>, impl Into<String>)>|
        macros
            .into_iter()
            .map(|(name, def)| (name.into(), def.into()))
            .collect())]
    #[builder(default = Vec::new())]
    macros: Vec<(String, String)>,

    #[builder(default)]
    source_language: ShaderLanguage,

    #[builder(default)]
    compiler: ShaderCompiler,

    #[builder(default = Version { major: 0, minor: 0 })]
    language_version: Version,

    #[builder(default)]
    compile_flags: ShaderCompileFlags,
}

pub(crate) struct ShaderCreateInfoWrapper {
    _macro_strings: Vec<(CString, CString)>,
    _macros: Vec<diligent_sys::ShaderMacro>,
    _entry_point: CString,
    _shader_source_path: Option<CString>,
    sci: diligent_sys::ShaderCreateInfo,
}

impl Deref for ShaderCreateInfoWrapper {
    type Target = diligent_sys::ShaderCreateInfo;
    fn deref(&self) -> &Self::Target {
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
                        shader_source_path =
                            Some(CString::new(path.to_string_lossy().as_bytes()).unwrap());
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
                        Name: value.name.as_ptr(),
                    }
                },
                ShaderType: value.shader_type.into(),
                UseCombinedTextureSamplers: value.use_combined_texture_samplers,
                CombinedSamplerSuffix: value.combined_sampler_suffix.as_ptr(),
            },
            SourceLanguage: value.source_language.into(),
            ShaderCompiler: value.compiler.into(),
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
                resources.push((&*resources_ptr).into());
            }
            resources
        }
    }

    pub fn get_constant_buffer_desc(
        &self,
        _index: u32,
    ) -> Result<diligent_sys::ShaderCodeBufferDesc, ()> {
        //unsafe {
        //    (*self.virtual_functions)
        //        .Shader
        //        .GetConstantBufferDesc
        //        .unwrap_unchecked()(self.sys_ptr, index)
        //    .as_ref()
        //}
        todo!()
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
