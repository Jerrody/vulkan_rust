use bevy_ecs::system::Res;

use crate::engine::resources::{CurrentFramebufferNumberResource, SwapchainResource};

pub fn clear_value_system(
    current_framebuffer_number_resource: Res<CurrentFramebufferNumberResource>,
    swapchain_resource: Res<SwapchainResource>,
) {
}
