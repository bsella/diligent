#define DILIGENT_C_INTERFACE 1

#ifdef VULKAN_SUPPORTED

#include <Graphics/GraphicsEngineVulkan/interface/EngineFactoryVk.h>

#ifdef VULKAN_INTEROP
#   define VK_NO_PROTOTYPES
#   include <vulkan/vulkan.h>

#   include <Graphics/GraphicsEngineVulkan/interface/BottomLevelASVk.h>
#   include <Graphics/GraphicsEngineVulkan/interface/BufferViewVk.h>
#   include <Graphics/GraphicsEngineVulkan/interface/BufferVk.h>
#   include <Graphics/GraphicsEngineVulkan/interface/CommandQueueVk.h>
#   include <Graphics/GraphicsEngineVulkan/interface/DeviceContextVk.h>
#   include <Graphics/GraphicsEngineVulkan/interface/DeviceMemoryVk.h>
#   include <Graphics/GraphicsEngineVulkan/interface/FenceVk.h>
#   include <Graphics/GraphicsEngineVulkan/interface/FramebufferVk.h>
#   include <Graphics/GraphicsEngineVulkan/interface/PipelineStateCacheVk.h>
#   include <Graphics/GraphicsEngineVulkan/interface/PipelineStateVk.h>
#   include <Graphics/GraphicsEngineVulkan/interface/QueryVk.h>
#   include <Graphics/GraphicsEngineVulkan/interface/RenderDeviceVk.h>
#   include <Graphics/GraphicsEngineVulkan/interface/RenderPassVk.h>
#   include <Graphics/GraphicsEngineVulkan/interface/SamplerVk.h>
#   include <Graphics/GraphicsEngineVulkan/interface/ShaderBindingTableVk.h>
#   include <Graphics/GraphicsEngineVulkan/interface/ShaderResourceBindingVk.h>
#   include <Graphics/GraphicsEngineVulkan/interface/ShaderVk.h>
#   include <Graphics/GraphicsEngineVulkan/interface/SwapChainVk.h>
#   include <Graphics/GraphicsEngineVulkan/interface/TextureViewVk.h>
#   include <Graphics/GraphicsEngineVulkan/interface/TextureVk.h>
#   include <Graphics/GraphicsEngineVulkan/interface/TopLevelASVk.h>
#endif

#endif

#ifdef OPENGL_SUPPORTED

#include <Graphics/GraphicsEngineOpenGL/interface/EngineFactoryOpenGL.h>

#ifdef OPENGL_INTEROP
    // TODO
#endif

#endif