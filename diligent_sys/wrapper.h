#define DILIGENT_C_INTERFACE 1

#include <DiligentCore/Common/interface/GeometryPrimitives.h>

extern unsigned int Diligent_GetGeometryPrimitiveVertexSize(GEOMETRY_PRIMITIVE_VERTEX_FLAGS VertexFlags);

extern void Diligent_CreateGeometryPrimitive(const GeometryPrimitiveAttributes* Attribs, IDataBlob** ppVertices, IDataBlob** ppIndices, GeometryPrimitiveInfo* pInfo);

#ifdef VULKAN_SUPPORTED

#include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/EngineFactoryVk.h>

#ifdef VULKAN_INTEROP
#   define VK_NO_PROTOTYPES
#   include <vulkan/vulkan.h>

#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/BottomLevelASVk.h>
#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/BufferViewVk.h>
#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/BufferVk.h>
#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/CommandQueueVk.h>
#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/DeviceContextVk.h>
#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/DeviceMemoryVk.h>
#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/FenceVk.h>
#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/FramebufferVk.h>
#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/PipelineStateCacheVk.h>
#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/PipelineStateVk.h>
#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/QueryVk.h>
#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/RenderDeviceVk.h>
#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/RenderPassVk.h>
#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/SamplerVk.h>
#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/ShaderBindingTableVk.h>
#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/ShaderResourceBindingVk.h>
#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/ShaderVk.h>
#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/SwapChainVk.h>
#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/TextureViewVk.h>
#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/TextureVk.h>
#   include <DiligentCore/Graphics/GraphicsEngineVulkan/interface/TopLevelASVk.h>
#endif

#endif

#ifdef OPENGL_SUPPORTED

#include <DiligentCore/Graphics/GraphicsEngineOpenGL/interface/EngineFactoryOpenGL.h>

#ifdef OPENGL_INTEROP
#   include <GL/gl.h>

#   include <DiligentCore/Graphics/GraphicsEngineOpenGL/interface/BaseInterfacesGL.h>
#   include <DiligentCore/Graphics/GraphicsEngineOpenGL/interface/BufferGL.h>
#   include <DiligentCore/Graphics/GraphicsEngineOpenGL/interface/BufferViewGL.h>
#   include <DiligentCore/Graphics/GraphicsEngineOpenGL/interface/DeviceContextGL.h>
#   include <DiligentCore/Graphics/GraphicsEngineOpenGL/interface/FenceGL.h>
#   include <DiligentCore/Graphics/GraphicsEngineOpenGL/interface/PipelineStateGL.h>
#   include <DiligentCore/Graphics/GraphicsEngineOpenGL/interface/QueryGL.h>
// TODO #   include <DiligentCore/Graphics/GraphicsEngineOpenGL/interface/RenderDeviceGLES.h>
#   include <DiligentCore/Graphics/GraphicsEngineOpenGL/interface/RenderDeviceGL.h>
#   include <DiligentCore/Graphics/GraphicsEngineOpenGL/interface/SamplerGL.h>
#   include <DiligentCore/Graphics/GraphicsEngineOpenGL/interface/ShaderGL.h>
#   include <DiligentCore/Graphics/GraphicsEngineOpenGL/interface/ShaderResourceBindingGL.h>
#   include <DiligentCore/Graphics/GraphicsEngineOpenGL/interface/SwapChainGL.h>
#   include <DiligentCore/Graphics/GraphicsEngineOpenGL/interface/TextureGL.h>
#   include <DiligentCore/Graphics/GraphicsEngineOpenGL/interface/TextureViewGL.h>
#endif

#endif

#ifdef D3D11_SUPPORTED
#include <DiligentCore/Graphics/GraphicsEngineD3D11/interface/EngineFactoryD3D11.h>
#ifdef D3D11_INTEROP
    // TODO
#endif
#endif

#ifdef D3D12_SUPPORTED
#include <DiligentCore/Graphics/GraphicsEngineD3D12/interface/EngineFactoryD3D12.h>
#ifdef D3D12_INTEROP
    // TODO
#endif
#endif