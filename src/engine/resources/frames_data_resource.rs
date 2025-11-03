use bevy_ecs::resource::Resource;
use vulkanite::vk::rs::*;

#[derive(Clone, Copy)]
pub struct FrameData {
    pub command_pool: CommandPool,
    pub command_buffer: CommandBuffer,
    pub swapchain_semaphore: Semaphore,
    pub render_semaphore: Semaphore,
    pub render_fence: Fence,
}

impl FrameData {
    pub fn new(
        command_pool: CommandPool,
        command_buffer: CommandBuffer,
        swapchain_semaphore: Semaphore,
        render_semaphore: Semaphore,
        render_fence: Fence,
    ) -> Self {
        Self {
            command_pool,
            command_buffer,
            swapchain_semaphore,
            render_semaphore,
            render_fence,
        }
    }
}

#[derive(Resource)]
pub struct FramesDataResource {
    pub data: Vec<FrameData>,
}

#[derive(Resource, Default)]
pub struct CurrentFrameDataResource {
    pub current_frame_data: Option<FrameData>,
}
