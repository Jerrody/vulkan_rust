use bevy_ecs::resource::Resource;
use vulkanite::vk::rs::*;

pub struct FrameData {
    pub command_pool: CommandPool,
    pub command_buffer: CommandBuffer,
}

impl FrameData {
    pub fn new(command_pool: CommandPool, command_buffer: CommandBuffer) -> Self {
        Self {
            command_pool,
            command_buffer,
        }
    }
}

#[derive(Resource)]
pub struct FramesDataResource {
    pub data: Vec<FrameData>,
}
