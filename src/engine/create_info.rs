use vulkanite::vk::{
    raw::{CommandBuffer, Semaphore},
    *,
};

#[inline(always)]
pub fn create_fence_create_info<'a>() -> FenceCreateInfo<'a> {
    let fence_create_info = FenceCreateInfo::default().flags(FenceCreateFlags::Signaled);

    return fence_create_info;
}

#[inline(always)]
pub fn create_semaphore_create_info<'a>() -> SemaphoreCreateInfo<'a> {
    let semaphore_create_info = SemaphoreCreateInfo::default();

    return semaphore_create_info;
}

#[inline(always)]
pub fn create_semaphore_submit_info<'a>(
    semaphore: &'a Semaphore,
    pipeline_stage_mask: PipelineStageFlags2,
) -> SemaphoreSubmitInfo<'a> {
    let sempahore_submit_info = SemaphoreSubmitInfo::default()
        .semaphore(&semaphore)
        .value(1)
        .stage_mask(pipeline_stage_mask);

    sempahore_submit_info
}

#[inline(always)]
pub fn create_command_buffer_submit_info<'a>(
    command_buffer: &'a CommandBuffer,
) -> CommandBufferSubmitInfo<'a> {
    let command_buffer_submit_info =
        CommandBufferSubmitInfo::default().command_buffer(command_buffer);

    command_buffer_submit_info
}

#[inline(always)]
pub fn create_submit_info<'a>(
    commnad_buffer_submit_infos: &'a [CommandBufferSubmitInfo],
    signal_semaphores: Option<&'a [SemaphoreSubmitInfo]>,
    wait_semaphores: Option<&'a [SemaphoreSubmitInfo]>,
) -> SubmitInfo2<'a> {
    let mut submit_info2 = SubmitInfo2::default().command_buffer_infos(commnad_buffer_submit_infos);

    if let Some(signal_semaphores) = signal_semaphores {
        submit_info2 = submit_info2.signal_semaphore_infos(signal_semaphores);
    }

    if let Some(wait_semaphores) = wait_semaphores {
        submit_info2 = submit_info2.wait_semaphore_infos(wait_semaphores);
    }

    submit_info2
}
