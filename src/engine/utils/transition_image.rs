use vulkanite::vk::{rs::*, *};

#[inline(always)]
pub fn transition_image(
    command_buffer: CommandBuffer,
    image: Image,
    current_layout: ImageLayout,
    new_layout: ImageLayout,
) {
    let aspect_mask: ImageAspectFlags;
    if (new_layout == ImageLayout::DepthAttachmentOptimal) {
        aspect_mask = ImageAspectFlags::Depth;
    } else {
        aspect_mask = ImageAspectFlags::Color;
    }

    let image_subresource_range = ImageSubresourceRange::default()
        .aspect_mask(aspect_mask)
        .base_array_layer(Default::default())
        .base_mip_level(Default::default())
        .layer_count(REMAINING_MIP_LEVELS)
        .level_count(REMAINING_MIP_LEVELS);

    let image_memory_barrier = ImageMemoryBarrier2::default()
        .src_stage_mask(PipelineStageFlags2::AllCommands)
        .src_access_mask(AccessFlags2::MemoryWrite)
        .dst_stage_mask(PipelineStageFlags2::AllCommands)
        .dst_access_mask(AccessFlags2::MemoryWrite | AccessFlags2::MemoryRead)
        .old_layout(current_layout)
        .new_layout(new_layout)
        .subresource_range(image_subresource_range)
        .image(&image);

    let image_memory_barriers = [image_memory_barrier];
    let dependency_info =
        DependencyInfo::default().image_memory_barriers(image_memory_barriers.as_slice());

    command_buffer.pipeline_barrier2(&dependency_info);
}
