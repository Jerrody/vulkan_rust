use std::u64;

use bevy_ecs::system::{Res, ResMut};
use vulkanite::vk::Status;

use crate::engine::resources::{
    CurrentFrameDataResource, CurrentSwapchainImageResource, SwapchainResource,
    VulkanContextResource,
};

pub fn acquire_next_swapchain_image_system(
    vulkan_contex_resource: Res<VulkanContextResource>,
    swapchain_resource: Res<SwapchainResource>,
    current_frame_data_resource: Res<CurrentFrameDataResource>,
    mut current_swapchain_image_resource: ResMut<CurrentSwapchainImageResource>,
) {
    let frame_data = current_frame_data_resource.current_frame_data.unwrap();
    let (result, image_index) = vulkan_contex_resource
        .device
        .acquire_next_image_khr(
            &swapchain_resource.swapchain,
            u64::MAX,
            Some(&frame_data.render_semaphore),
            None,
        )
        .unwrap();

    if result != Status::Success {
        panic!("{result}");
    }

    current_swapchain_image_resource.image_index = image_index;
    current_swapchain_image_resource.current_swapchain_image =
        Some(swapchain_resource.images[image_index as usize]);
}
