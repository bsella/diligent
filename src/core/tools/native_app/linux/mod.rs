#[cfg(feature = "VULKAN_SUPPORTED")]
mod linux_xcb;

#[cfg(not(feature = "VULKAN_SUPPORTED"))]
mod linux_xlib;

#[cfg(feature = "VULKAN_SUPPORTED")]
pub fn main() -> Result<(), std::io::Error> {
    linux_xcb::main()
}
