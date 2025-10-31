use bevy_ecs::resource::Resource;
use vulkanite::vk::rs::*;

#[derive(Resource)]
pub struct VulkanContextResource {
    pub instance: Instance,
    pub debug_messsenger: Option<DebugUtilsMessengerEXT>,
    pub surface: SurfaceKHR,
    pub physical_device: PhysicalDevice,
    pub device: Device,
}
