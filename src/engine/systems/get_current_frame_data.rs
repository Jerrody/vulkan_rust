use bevy_ecs::system::{Res, ResMut};

use crate::engine::resources::{
    CurrentFrameDataResource, CurrentFramebufferNumberResource, FramesDataResource,
};

pub fn get_current_frame_data_system(
    current_framebuffer_number_resource: Res<CurrentFramebufferNumberResource>,
    frames_data_resource: Res<FramesDataResource>,
    mut current_frame_data_resource: ResMut<CurrentFrameDataResource>,
) {
    current_frame_data_resource.current_frame_data =
        Some(frames_data_resource.data[current_framebuffer_number_resource.index]);
}
