#define DILIGENT_C_INTERFACE 1

#include <DiligentCore/Graphics/GraphicsEngine/interface/EngineFactory.h>

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