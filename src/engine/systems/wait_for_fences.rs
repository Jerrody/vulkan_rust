use std::u64;

use bevy_ecs::system::Res;

use crate::engine::resources::{CurrentFrameDataResource, VulkanContextResource};

pub fn wait_for_fences_system(
    vulkan_context_resource: Res<VulkanContextResource>,
    current_frame_data_resource: Res<CurrentFrameDataResource>,
) {
    println!("2");

    let frame_data = current_frame_data_resource.current_frame_data.unwrap();

    let device = &vulkan_context_resource.device;

    let fences = [frame_data.render_fence];
    device
        .wait_for_fences(fences.as_slice(), true, u64::MAX)
        .unwrap();
    device.reset_fences(fences.as_slice()).unwrap();
}
