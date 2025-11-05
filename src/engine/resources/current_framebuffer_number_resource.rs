use bevy_ecs::resource::Resource;

#[derive(Resource, Default)]
pub struct CurrentFramebufferNumberResource {
    pub frame_number: usize,
    pub index: usize,
}
