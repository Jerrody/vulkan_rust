use bevy_ecs::system::Res;

use vulkanite::vk::*;

use crate::engine::{
    create_info,
    resources::{
        CurrentFrameDataResource, CurrentFramebufferNumberResource, CurrentSwapchainImageResource,
    },
    utils,
};

pub fn clear_value_system(
    current_frame_data_resource: Res<CurrentFrameDataResource>,
    current_swapchain_image_resource: Res<CurrentSwapchainImageResource>,
    current_framebuffer_number_resource: Res<CurrentFramebufferNumberResource>,
) {
    println!("5");
    let frame_data = current_frame_data_resource.current_frame_data.unwrap();
    let command_buffer = frame_data.command_buffer;

    println!("5.1");
    let image = current_swapchain_image_resource
        .current_swapchain_image
        .unwrap();
    utils::transition_image(
        &command_buffer,
        &image,
        ImageLayout::Undefined,
        ImageLayout::General,
    );
    println!("5.2");

    let flash = f32::abs(f32::sin(current_framebuffer_number_resource.frame_number as _) / 120.0);
    let mut clear_color_value = ClearColorValue::default();
    clear_color_value.float32 = [0.0, 0.0, flash, 1.0];
    println!("5.3");

    let image_clear_range = create_info::create_image_subresource_range(ImageAspectFlags::Color);

    command_buffer.clear_color_image(
        &image,
        ImageLayout::General,
        &clear_color_value,
        &image_clear_range,
    );
    println!("5.4");

    utils::transition_image(
        &command_buffer,
        &image,
        ImageLayout::General,
        ImageLayout::PresentSrcKHR,
    );
    println!("5.5");
}
