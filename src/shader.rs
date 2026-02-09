#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

use std::{
    ffi::{CStr, CString},
    ops::Deref,
    path::Path,
    str::FromStr,
};

use bitflags::bitflags;
use bon::Builder;
use static_assertions::const_assert_eq;

use crate::{
    PipelineResourceFlags,
    device_object::DeviceObject,
    graphics_types::{ShaderType, Version},
    object::Object,
};

#[repr(transparent)]
pub struct ShaderDesc(diligent_sys::ShaderDesc);

impl ShaderDesc {
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0._DeviceObjectAttribs.Name) }
    }
    pub fn shader_type(&self) -> Option<ShaderType> {
        ShaderType::from_sys(self.0.ShaderType)
    }
    pub fn use_combined_texture_samplers(&self) -> bool {
        self.0.UseCombinedTextureSamplers
    }

    pub fn combined_sampler_suffix(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.CombinedSamplerSuffix) }
    }
}

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
    ByteCode,
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
            ShaderLanguage::ByteCode => diligent_sys::SHADER_SOURCE_LANGUAGE_BYTECODE,
        }) as _
    }
}
const_assert_eq!(diligent_sys::SHADER_SOURCE_LANGUAGE_COUNT, 9);

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
const_assert_eq!(diligent_sys::SHADER_COMPILER_COUNT, 4);

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
const_assert_eq!(diligent_sys::SHADER_RESOURCE_TYPE_LAST, 8);

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

impl ShaderResourceType {
    pub fn get_valid_pipeline_resource_flags(&self) -> PipelineResourceFlags {
        const_assert_eq!(diligent_sys::SHADER_RESOURCE_TYPE_LAST, 8);
        match self {
            ShaderResourceType::ConstantBuffer => {
                PipelineResourceFlags::NoDynamicBuffers | PipelineResourceFlags::RuntimeArray
            }
            ShaderResourceType::TextureSRV => {
                PipelineResourceFlags::CombinedSampler | PipelineResourceFlags::RuntimeArray
            }
            ShaderResourceType::BufferSRV => {
                PipelineResourceFlags::NoDynamicBuffers
                    | PipelineResourceFlags::FormattedBuffer
                    | PipelineResourceFlags::RuntimeArray
            }
            ShaderResourceType::TextureUAV => PipelineResourceFlags::RuntimeArray,
            ShaderResourceType::BufferUAV => {
                PipelineResourceFlags::NoDynamicBuffers
                    | PipelineResourceFlags::FormattedBuffer
                    | PipelineResourceFlags::RuntimeArray
            }
            ShaderResourceType::Sampler => PipelineResourceFlags::RuntimeArray,
            ShaderResourceType::InputAttachment => PipelineResourceFlags::GeneralInputAttachment,
            ShaderResourceType::AccelStruct => PipelineResourceFlags::RuntimeArray,
        }
    }
}

#[repr(transparent)]
pub struct ShaderResourceDesc(pub(crate) diligent_sys::ShaderResourceDesc);
impl ShaderResourceDesc {
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.Name) }
    }

    pub fn resource_type(&self) -> ShaderResourceType {
        match self.0.Type as _ {
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

    pub fn array_size(&self) -> usize {
        self.0.ArraySize as usize
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
const_assert_eq!(diligent_sys::SHADER_COMPILE_FLAG_LAST, 16);

impl Default for ShaderCompileFlags {
    fn default() -> Self {
        ShaderCompileFlags::None
    }
}

#[derive(Builder)]
#[builder(derive(Clone))]
pub struct ShaderCreateInfo<'a> {
    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    name: Option<CString>,

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

    #[builder(default = false)]
    load_constant_buffer_reflection: bool,

    #[builder(with =|cstr : impl AsRef<str>| CString::new(cstr.as_ref()).unwrap())]
    glsl_extensions: Option<CString>,

    #[cfg(feature = "webgpu")]
    #[builder(with =|suffix : impl AsRef<str>| CString::new(suffix.as_ref()).unwrap())]
    web_gpu_emulated_array_index_suffix: Option<CString>,
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
                    stream_factory.sys_ptr()
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
                        Name: value
                            .name
                            .as_ref()
                            .map_or(std::ptr::null(), |name| name.as_ptr()),
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
            CompileFlags: value.compile_flags.bits(),
            LoadConstantBufferReflection: value.load_constant_buffer_reflection,
            GLSLExtensions: value
                .glsl_extensions
                .as_ref()
                .map_or(std::ptr::null(), |cstr| cstr.as_ptr()),
            #[cfg(feature = "webgpu")]
            WebGPUEmulatedArrayIndexSuffix: value
                .web_gpu_emulated_array_index_suffix
                .as_ref()
                .map_or(std::ptr::null(), |cstr| cstr.as_ptr()),
            #[cfg(not(feature = "webgpu"))]
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

#[derive(Clone, Copy)]
pub enum ShaderCodeBasicType {
    Void,
    Bool,
    Int,
    Int8,
    Int16,
    Int64,
    Uint,
    Uint8,
    Uint16,
    Uint64,
    Float,
    Float16,
    Double,
    Min8Float,
    Min10Float,
    Min16Float,
    Min12Int,
    Min16Int,
    Min16Uint,
    String,
}
const_assert_eq!(diligent_sys::SHADER_CODE_BASIC_TYPE_COUNT, 21);

#[derive(Clone, Copy)]
pub enum ShaderCodeVariableClass {
    Scalar,
    Vector,
    MatrixRows,
    MatrixColumns,
    Struct,
}
const_assert_eq!(diligent_sys::SHADER_CODE_VARIABLE_CLASS_COUNT, 6);

#[derive(Clone, Copy)]
pub enum ShaderStatus {
    Uninitialized,
    Compiling,
    Ready,
    Failed,
}

#[repr(transparent)]
pub struct ShaderCodeVariableDesc(diligent_sys::ShaderCodeVariableDesc);

impl ShaderCodeVariableDesc {
    pub fn name(&self) -> Option<&str> {
        unsafe { CStr::from_ptr(self.0.Name).to_str().ok() }
    }

    pub fn type_name(&self) -> Option<&str> {
        unsafe { CStr::from_ptr(self.0.TypeName).to_str().ok() }
    }

    pub fn class(&self) -> Option<ShaderCodeVariableClass> {
        match self.0.Class as _ {
            diligent_sys::SHADER_CODE_VARIABLE_CLASS_UNKNOWN => None,
            diligent_sys::SHADER_CODE_VARIABLE_CLASS_SCALAR => {
                Some(ShaderCodeVariableClass::Scalar)
            }
            diligent_sys::SHADER_CODE_VARIABLE_CLASS_VECTOR => {
                Some(ShaderCodeVariableClass::Vector)
            }
            diligent_sys::SHADER_CODE_VARIABLE_CLASS_MATRIX_ROWS => {
                Some(ShaderCodeVariableClass::MatrixRows)
            }
            diligent_sys::SHADER_CODE_VARIABLE_CLASS_MATRIX_COLUMNS => {
                Some(ShaderCodeVariableClass::MatrixColumns)
            }
            diligent_sys::SHADER_CODE_VARIABLE_CLASS_STRUCT => {
                Some(ShaderCodeVariableClass::Struct)
            }
            _ => panic!("Unknown SHADER_CODE_VARIABLE_CLASS value"),
        }
    }

    pub fn basic_type(&self) -> Option<ShaderCodeBasicType> {
        match self.0.BasicType as _ {
            diligent_sys::SHADER_CODE_BASIC_TYPE_UNKNOWN => None,
            diligent_sys::SHADER_CODE_BASIC_TYPE_VOID => Some(ShaderCodeBasicType::Void),
            diligent_sys::SHADER_CODE_BASIC_TYPE_BOOL => Some(ShaderCodeBasicType::Bool),
            diligent_sys::SHADER_CODE_BASIC_TYPE_INT => Some(ShaderCodeBasicType::Int),
            diligent_sys::SHADER_CODE_BASIC_TYPE_INT8 => Some(ShaderCodeBasicType::Int8),
            diligent_sys::SHADER_CODE_BASIC_TYPE_INT16 => Some(ShaderCodeBasicType::Int16),
            diligent_sys::SHADER_CODE_BASIC_TYPE_INT64 => Some(ShaderCodeBasicType::Int64),
            diligent_sys::SHADER_CODE_BASIC_TYPE_UINT => Some(ShaderCodeBasicType::Uint),
            diligent_sys::SHADER_CODE_BASIC_TYPE_UINT8 => Some(ShaderCodeBasicType::Uint8),
            diligent_sys::SHADER_CODE_BASIC_TYPE_UINT16 => Some(ShaderCodeBasicType::Uint16),
            diligent_sys::SHADER_CODE_BASIC_TYPE_UINT64 => Some(ShaderCodeBasicType::Uint64),
            diligent_sys::SHADER_CODE_BASIC_TYPE_FLOAT => Some(ShaderCodeBasicType::Float),
            diligent_sys::SHADER_CODE_BASIC_TYPE_FLOAT16 => Some(ShaderCodeBasicType::Float16),
            diligent_sys::SHADER_CODE_BASIC_TYPE_DOUBLE => Some(ShaderCodeBasicType::Double),
            diligent_sys::SHADER_CODE_BASIC_TYPE_MIN8FLOAT => Some(ShaderCodeBasicType::Min8Float),
            diligent_sys::SHADER_CODE_BASIC_TYPE_MIN10FLOAT => {
                Some(ShaderCodeBasicType::Min10Float)
            }
            diligent_sys::SHADER_CODE_BASIC_TYPE_MIN16FLOAT => {
                Some(ShaderCodeBasicType::Min16Float)
            }
            diligent_sys::SHADER_CODE_BASIC_TYPE_MIN12INT => Some(ShaderCodeBasicType::Min12Int),
            diligent_sys::SHADER_CODE_BASIC_TYPE_MIN16INT => Some(ShaderCodeBasicType::Min16Int),
            diligent_sys::SHADER_CODE_BASIC_TYPE_MIN16UINT => Some(ShaderCodeBasicType::Min16Uint),
            diligent_sys::SHADER_CODE_BASIC_TYPE_STRING => Some(ShaderCodeBasicType::String),
            _ => panic!("Unknown SHADER_CODE_BASIC_TYPE value"),
        }
    }

    pub fn num_rows(&self) -> u8 {
        self.0.NumRows
    }
    pub fn num_columns(&self) -> u8 {
        self.0.NumColumns
    }
    pub fn offset(&self) -> u32 {
        self.0.Offset
    }

    pub fn array_size(&self) -> u32 {
        self.0.ArraySize
    }

    pub fn members(&self) -> &[ShaderCodeVariableDesc] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.pMembers as *const ShaderCodeVariableDesc,
                self.0.NumMembers as usize,
            )
        }
    }
}

#[repr(transparent)]
pub struct ShaderCodeBufferDesc(diligent_sys::ShaderCodeBufferDesc);

impl ShaderCodeBufferDesc {
    pub fn size(&self) -> u32 {
        self.0.Size
    }

    pub fn variables(&self) -> &[ShaderCodeVariableDesc] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.pVariables as *const ShaderCodeVariableDesc,
                self.0.NumVariables as usize,
            )
        }
    }
}

pub struct ShaderResourceDescIterator<'shader> {
    shader: &'shader Shader,
    count: usize,
    current_index: usize,
}

impl Iterator for ShaderResourceDescIterator<'_> {
    type Item = ShaderResourceDesc;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.count {
            return None;
        }

        let mut desc = std::mem::MaybeUninit::uninit();

        unsafe_member_call!(
            self.shader,
            Shader,
            GetResourceDesc,
            self.current_index as u32,
            desc.as_mut_ptr()
        );

        self.current_index += 1;

        Some(ShaderResourceDesc(unsafe { desc.assume_init() }))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.count
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        if self.count == 0 {
            return None;
        }

        let mut desc = std::mem::MaybeUninit::uninit();

        unsafe_member_call!(
            self.shader,
            Shader,
            GetResourceDesc,
            self.count as u32 - 1,
            desc.as_mut_ptr()
        );

        Some(ShaderResourceDesc(unsafe { desc.assume_init() }))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.count - self.current_index;
        (remaining, Some(remaining))
    }
}

define_ported!(
    Shader,
    diligent_sys::IShader,
    diligent_sys::IShaderMethods : 5,
    DeviceObject
);

impl Shader {
    pub fn desc(&self) -> &ShaderDesc {
        let desc_ptr = unsafe_member_call!(self, DeviceObject, GetDesc);
        unsafe { &*(desc_ptr as *const ShaderDesc) }
    }

    pub fn resources(&self) -> ShaderResourceDescIterator<'_> {
        ShaderResourceDescIterator {
            count: unsafe_member_call!(self, Shader, GetResourceCount) as usize,
            current_index: 0,
            shader: self,
        }
    }

    pub fn get_constant_buffer_desc(&self, index: u32) -> Option<&ShaderCodeBufferDesc> {
        let desc_ptr = unsafe_member_call!(self, Shader, GetConstantBufferDesc, index);
        if desc_ptr.is_null() {
            None
        } else {
            Some(unsafe { &*(desc_ptr as *const ShaderCodeBufferDesc) })
        }
    }

    pub fn get_bytecode(&self) -> Option<&[u8]> {
        let mut size = 0;

        let mut bytecode: *const u8 = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            Shader,
            GetBytecode,
            std::ptr::from_mut(&mut bytecode) as *mut *const std::ffi::c_void,
            &mut size
        );

        if bytecode.is_null() {
            None
        } else {
            Some(unsafe { std::slice::from_raw_parts(bytecode, size as usize) })
        }
    }

    pub fn get_status(&self, wait_for_completion: bool) -> ShaderStatus {
        let status = unsafe_member_call!(self, Shader, GetStatus, wait_for_completion);
        match status as _ {
            diligent_sys::SHADER_STATUS_UNINITIALIZED => ShaderStatus::Uninitialized,
            diligent_sys::SHADER_STATUS_COMPILING => ShaderStatus::Compiling,
            diligent_sys::SHADER_STATUS_READY => ShaderStatus::Ready,
            diligent_sys::SHADER_STATUS_FAILED => ShaderStatus::Failed,
            _ => panic!("Unknown SHADER_STATUS value"),
        }
    }
}

define_ported!(
    ShaderSourceInputStreamFactory,
    diligent_sys::IShaderSourceInputStreamFactory,
    diligent_sys::IShaderSourceInputStreamFactoryMethods : 2,
    Object
);

impl ShaderSourceInputStreamFactory {
    //pub fn create_input_stream(&self, name : impl AsRef<str>, IFileStream** ppStream);

    //pub fn create_input_stream2(&self, name : impl AsRef<str>, CREATE_SHADER_SOURCE_INPUT_STREAM_FLAGS Flags, IFileStream** ppStream);
}
