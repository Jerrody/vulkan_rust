use bevy_ecs::system::Res;

use crate::engine::resources::{CurrentFrameDataResource, VulkanContextResource};
use vulkanite::vk::*;

pub fn begin_command_buffer_system(current_frame_data_resource: Res<CurrentFrameDataResource>) {
    let commnad_buffer_begin_info = get_command_buffer_begin_info();

    current_frame_data_resource
        .current_frame_data
        .unwrap()
        .command_buffer
        .begin(&commnad_buffer_begin_info)
        .unwrap();
}

#[inline(always)]
fn get_command_buffer_begin_info<'a>() -> CommandBufferBeginInfo<'a> {
    let command_buffer_begin_info =
        CommandBufferBeginInfo::default().flags(CommandBufferUsageFlags::OneTimeSubmit);

    command_buffer_begin_info
}
