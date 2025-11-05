use bevy_ecs::system::Res;

use crate::engine::resources::CurrentFrameDataResource;

pub fn end_command_buffer_system(current_frame_data_resource: Res<CurrentFrameDataResource>) {
    println!("6");
    current_frame_data_resource
        .current_frame_data
        .unwrap()
        .command_buffer
        .end()
        .unwrap();
}
