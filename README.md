# custom-engine
Learning graphics API 

## What is done
- Initialization of winit Window 
- Creation of Vulkan App Info
- Creation of Vulkan Instance
## What remains to be done
- [ ] Create Vulkan Surface
    - Create a function and use `ash_window::create_surface` to create the surface  
- [ ] Create Vulkan Physical Device
    - List all the physical devices with `instance.enumerate_physical_devices()`
    - Iter over the physical devices and get the one that supports `vk::QueueFlags::GRAPHICS` and `surface_loader.get_physical_device_surface_support()`
- [ ] Create Vulkan Queue Families Info
    - Get the queue family indices from the physical device
    - Use `vk::DeviceQueueCreateInfo` to create the queue info with the priorities beign `[1.0]`
- [ ] Create Vulkan Logical Device
    - Create the device with `instance.create_device()`
    - Store the presentation queue with `device.get_device_queue()`
    - Also store `surface_loader.get_physical_device_surface_formats()` and `surface_loader.get_physical_device_surface_capabilities()`
- [ ] Create presentation logic
    - Start with getting the present modes with `surface_loader.get_physical_device_surface_present_modes()`
    - Use *MAILBOX*
- [ ] Create Vulkan Swapchain
    - Start by creating the swapchain loader with `swapchain::Device::new()`
    - Create the swapchain info with `SwapchainCreateInfoKHR`
- [ ] Create the command buffer pool
    - Start with `device.create_command_pool()`
    - Create the command pool info with `CommandBufferAllocateInfo`
    - Allocate the command buffers with `device.allocate_command_buffers()`
- [ ] Create Vulkan Image Views
- [ ] Create Vulkan Render Pass
- [ ] Create Vulkan Graphics Pipeline

# References
- [Vulkan Tutorial](https://vulkan-tutorial.com/)
- [Ash Examples](https://github.com/ash-rs/ash/tree/master/ash-examples)