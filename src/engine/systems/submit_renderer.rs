use bevy_ecs::system::Res;
use vulkanite::vk::{PipelineStageFlags2, PresentInfoKHR};

use crate::engine::{
    create_info::{
        create_command_buffer_submit_info, create_semaphore_submit_info, create_submit_info,
    },
    resources::{
        CurrentFrameDataResource, CurrentSwapchainImageResource, RenderQueueResource,
        SwapchainResource,
    },
};

pub fn submit_renderer_system(
    render_queue_resource: Res<RenderQueueResource>,
    swapchain_resource: Res<SwapchainResource>,
    current_swapchain_image_resource: Res<CurrentSwapchainImageResource>,
    current_frame_data_resource: Res<CurrentFrameDataResource>,
) {
    if let Some(frame_data) = current_frame_data_resource.current_frame_data {
        let command_buffer_submit_infos = [create_command_buffer_submit_info(
            &frame_data.command_buffer,
        )];

        let wait_semaphore_submit_infos = [create_semaphore_submit_info(
            &frame_data.swapchain_semaphore,
            PipelineStageFlags2::ColorAttachmentOutput,
        )];

        let signal_semaphore_submit_infos = [create_semaphore_submit_info(
            &frame_data.render_semaphore,
            PipelineStageFlags2::AllGraphics,
        )];

        let submit_infos2 = [create_submit_info(
            command_buffer_submit_infos.as_slice(),
            Some(signal_semaphore_submit_infos.iter().as_slice()),
            Some(wait_semaphore_submit_infos.as_slice()),
        )];

        render_queue_resource
            .queue
            .submit2(submit_infos2.as_slice(), Some(&frame_data.render_fence))
            .unwrap();

        let image_indices = [current_swapchain_image_resource.image_index as u32];
        let wait_semaphores = [frame_data.swapchain_semaphore];
        let present_infos = PresentInfoKHR::default()
            .wait_semaphores(wait_semaphores.as_slice())
            .swapchain(&swapchain_resource.swapchain, &image_indices, None::<()>);

        render_queue_resource.queue.present_khr(&present_infos);
    }
}
