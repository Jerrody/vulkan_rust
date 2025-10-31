use bevy_ecs::resource::Resource;
use vulkanite::vk::{Extent2D, Format, rs::*};

#[derive(Resource)]
pub struct SwapchainResource {
    pub swapchain: SwapchainKHR,
    pub images: Vec<Image>,
    pub image_views: Vec<ImageView>,
    pub format: Format,
    pub extent: Extent2D,
}
