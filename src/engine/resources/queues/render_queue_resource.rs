use bevy_ecs::resource::Resource;
use vulkanite::vk::rs::*;

#[derive(Resource)]
pub struct RenderQueueResource {
    pub queue_family_index: usize,
    pub queue: Queue,
}
