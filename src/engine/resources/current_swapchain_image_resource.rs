use bevy_ecs::resource::Resource;
use vulkanite::vk::rs::*;

#[derive(Resource, Default)]
pub struct CurrentSwapchainImageResource {
    pub image_index: usize,
    pub current_swapchain_image: Option<Image>,
}
