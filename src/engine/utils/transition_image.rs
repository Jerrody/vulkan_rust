use vulkanite::vk::{rs::*, *};

use crate::engine::create_info;

#[inline(always)]
pub fn transition_image(
    command_buffer: &CommandBuffer,
    image: Image,
    current_layout: ImageLayout,
    new_layout: ImageLayout,
) {
    let aspect_mask: ImageAspectFlags;
    if new_layout == ImageLayout::DepthAttachmentOptimal {
        aspect_mask = ImageAspectFlags::Depth;
    } else {
        aspect_mask = ImageAspectFlags::Color;
    }

    let image_subresource_range = create_info::create_image_subresource_range(aspect_mask);

    let image_memory_barriers = [ImageMemoryBarrier2::default()
        .src_stage_mask(PipelineStageFlags2::AllCommands)
        .src_access_mask(AccessFlags2::MemoryWrite)
        .dst_stage_mask(PipelineStageFlags2::AllCommands)
        .dst_access_mask(AccessFlags2::MemoryWrite | AccessFlags2::MemoryRead)
        .old_layout(current_layout)
        .new_layout(new_layout)
        .subresource_range(image_subresource_range)
        .image(&image)];

    let dependency_info = DependencyInfo::default().image_memory_barriers(&image_memory_barriers);

    command_buffer.pipeline_barrier2(&dependency_info);
}
