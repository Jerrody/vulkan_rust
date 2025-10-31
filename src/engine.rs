use bevy_ecs::{schedule::Schedule, world::World};

mod components;
mod resources;
mod systems;

use components::*;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle};
use resources::*;
use systems::*;
use vulkanite::{
    DefaultAllocator, Dispatcher, DynamicDispatcher, flagbits, structure_chain,
    vk::{self, CommandBufferAllocateInfo, CommandPoolCreateFlags, CommandPoolCreateInfo, rs::*},
};
use winit::dpi::PhysicalSize;

extern "system" fn debug_callback(
    _severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    _ty: vk::DebugUtilsMessageTypeFlagsEXT,
    data: &vk::DebugUtilsMessengerCallbackDataEXT,
    _: *const (),
) -> vk::Bool32 {
    println!("Validation layer: {:?}", unsafe {
        std::ffi::CStr::from_ptr(data.p_message)
    });
    vk::FALSE
}

pub struct Engine {
    pub world: World,
    pub scheduler: Schedule,
}

impl Engine {
    const FRAMES_IN_FLIGHT: u32 = 2;

    pub fn new(window: &winit::window::Window) -> Self {
        let mut world = World::new();
        let scheduler = Schedule::default();

        Self::create_resources(window, &mut world);

        Self { world, scheduler }
    }

    fn create_resources(window: &winit::window::Window, world: &mut World) {
        let dispatcher = unsafe { DynamicDispatcher::new_loaded().unwrap() };
        let entry = vk::rs::Entry::new(dispatcher, DefaultAllocator);

        let raw_dispaly_handle = window.display_handle().unwrap().as_raw();
        let raw_window_handle = window.window_handle().unwrap().as_raw();

        let (instance, debug_messenger) = Self::create_instance(&entry, &raw_dispaly_handle);
        let surface = vulkanite::window::rs::create_surface(
            &instance,
            &raw_dispaly_handle,
            &raw_window_handle,
        )
        .unwrap();
        let (physical_device, device, queue, queue_family_index) =
            Self::create_device(&instance, &surface);

        let vulkan_context_resource = VulkanContextResource {
            instance: instance,
            debug_messsenger: debug_messenger,
            surface,
            physical_device,
            device,
        };
        let render_queue_resource = RenderQueueResource {
            queue_family_index,
            queue,
        };
        let swapchain_resource = Self::create_swapchain_resource(
            &physical_device,
            &device,
            &surface,
            window.inner_size(),
        );
        let frames_data = Self::create_frames_data(device, queue_family_index);
        let frames_data_resource = FramesDataResource { data: frames_data };

        world.insert_resource(vulkan_context_resource);
        world.insert_resource(render_queue_resource);
        world.insert_resource(swapchain_resource);
        world.insert_resource(frames_data_resource);
    }

    pub fn create_instance(
        entry: &vk::rs::Entry,
        display_handle: &RawDisplayHandle,
    ) -> (vk::rs::Instance, Option<vk::rs::DebugUtilsMessengerEXT>) {
        const VALIDATION_LAYER: &std::ffi::CStr = c"VK_LAYER_KHRONOS_validation";
        let layers: Vec<_> = entry.enumerate_instance_layer_properties().unwrap();
        let has_validation = layers
            .into_iter()
            .any(|layer| layer.get_layer_name() == VALIDATION_LAYER);
        let enabled_layers = has_validation.then_some(VALIDATION_LAYER.as_ptr());

        let mut enabled_extensions =
            Vec::from(vulkanite::window::enumerate_required_extensions(display_handle).unwrap());
        if has_validation {
            enabled_extensions.push(vk::EXT_DEBUG_UTILS.name);
        }

        let app_info = vk::ApplicationInfo::default()
            .application_name(Some(c"Hello Triangle"))
            .engine_name(Some(c"No Engine"))
            .api_version(vk::API_VERSION_1_0);

        let instance_info = vk::InstanceCreateInfo::default()
            .application_info(Some(&app_info))
            .enabled_extension(&enabled_extensions)
            .enabled_layer(enabled_layers.as_slice());

        let instance = entry.create_instance(&instance_info).unwrap();

        let debug_messenger = if has_validation {
            let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
                .message_severity(
                    flagbits!(vk::DebugUtilsMessageSeverityFlagsEXT::{Info | Warning | Error}),
                )
                .message_type(flagbits!(vk::DebugUtilsMessageTypeFlagsEXT::{General | Validation}))
                .pfn_user_callback(Some(debug_callback));
            Some(
                instance
                    .create_debug_utils_messenger_ext(&debug_info)
                    .unwrap(),
            )
        } else {
            None
        };

        (instance, debug_messenger)
    }

    pub fn create_device(
        instance: &vk::rs::Instance,
        surface: &vk::rs::SurfaceKHR,
    ) -> (vk::rs::PhysicalDevice, vk::rs::Device, vk::rs::Queue, usize) {
        let physical_devices: Vec<_> = instance.enumerate_physical_devices().unwrap();

        let compute_device_score = |physical_device: &vk::rs::PhysicalDevice| {
            let properties = physical_device.get_properties();
            let is_discrete = properties.device_type == vk::PhysicalDeviceType::DiscreteGpu;
            let max_2d_dim = properties.limits.max_image_dimension2_d;

            // compute a score based on if the gpu is discrete and the maximal supported 2d image dimension
            (is_discrete as u32) * 10000 + max_2d_dim
        };

        let physical_device = physical_devices
            .into_iter()
            .max_by_key(compute_device_score)
            .ok_or_else(|| panic!("Failed to find a Vulkan compatible GPU"))
            .unwrap();

        let (queue_family_index, _) = physical_device
            .get_queue_family_properties::<Vec<_>>()
            .into_iter()
            .enumerate()
            .find(|(queue, props)| {
                props.queue_flags.contains(vk::QueueFlags::Graphics)
                    && physical_device
                        .get_surface_support_khr(*queue as u32, surface)
                        .is_ok_and(|supported| supported)
            })
            .ok_or_else(|| panic!("Failed to find a suitable GPU queue"))
            .unwrap();

        let features = vk::PhysicalDeviceFeatures::default();

        let required_extensions = [vk::KHR_SWAPCHAIN.name];
        let mut missing_extensions: std::collections::HashSet<&std::ffi::CStr> =
            required_extensions.iter().map(|ext| ext.get()).collect();
        for extension_prop in physical_device
            .enumerate_device_extension_properties::<Vec<_>>(None)
            .unwrap()
        {
            missing_extensions.remove(extension_prop.get_extension_name());
        }

        if !missing_extensions.is_empty() {
            panic!("The following required device extensions are missing : {missing_extensions:?}");
        }

        let queue_prio = 1.0f32;
        let queue_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index as u32)
            .queue_priorities(&queue_prio);

        let device_info = structure_chain!(
            vk::DeviceCreateInfo::default()
                .queue_create_infos(&queue_info)
                .enabled_features(Some(&features))
                .enabled_extension(&required_extensions),
            vk::PhysicalDeviceDynamicRenderingFeatures::default().dynamic_rendering(true),
            vk::PhysicalDeviceSynchronization2Features::default().synchronization2(true),
            vk::PhysicalDeviceVulkan12Features::default().descriptor_indexing(true)
        );

        let device = physical_device.create_device(device_info.as_ref()).unwrap();
        let queue = device.get_queue(queue_family_index as u32, 0);

        (physical_device, device, queue, queue_family_index)
    }

    fn create_swapchain_resource(
        physical_device: &vk::rs::PhysicalDevice,
        device: &vk::rs::Device,
        surface: &vk::rs::SurfaceKHR,
        window_size: PhysicalSize<u32>,
    ) -> SwapchainResource {
        let capabilities = physical_device
            .get_surface_capabilities_khr(surface)
            .unwrap();

        let format = physical_device
            .get_surface_formats_khr::<Vec<_>>(Some(surface))
            .unwrap()
            .into_iter()
            .max_by_key(|fmt| match fmt {
                // we have one pair of format/color_space that we prefer
                vk::SurfaceFormatKHR {
                    format: vk::Format::B8G8R8A8Srgb,
                    color_space: vk::ColorSpaceKHR::SrgbNonlinear,
                } => 1,
                _ => 0,
            })
            .ok_or_else(|| panic!("No swapchain format is available"))
            .unwrap();

        // Only use FIFO for the time being
        // The Vulkan spec guarantees that if the swapchain extension is supported
        // then the FIFO present mode is too
        if !physical_device
            .get_surface_present_modes_khr::<Vec<_>>(Some(surface))
            .unwrap()
            .contains(&vk::PresentModeKHR::Fifo)
        {
            panic!("FIFO present mode is missing");
        }

        let extent = if capabilities.current_extent.width != u32::MAX {
            capabilities.current_extent
        } else {
            let min_ex = capabilities.min_image_extent;
            let max_ex = capabilities.max_image_extent;
            vk::Extent2D {
                width: window_size.width.clamp(min_ex.width, max_ex.width),
                height: window_size.height.clamp(min_ex.height, max_ex.height),
            }
        };

        let max_swap_count = if capabilities.max_image_count != 0 {
            capabilities.max_image_count
        } else {
            u32::MAX
        };
        let swapchain_count = (capabilities.min_image_count + 1).min(max_swap_count);

        let swapchain_info = vk::SwapchainCreateInfoKHR::default()
            .surface(&surface)
            .min_image_count(swapchain_count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::ColorAttachment)
            .image_sharing_mode(vk::SharingMode::Exclusive)
            .pre_transform(capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::Opaque)
            .present_mode(vk::PresentModeKHR::Fifo)
            .clipped(true);

        let swapchain = device.create_swapchain_khr(&swapchain_info).unwrap();
        let swapchain_images: Vec<_> = device.get_swapchain_images_khr(&swapchain).unwrap();
        let swapchain_views: Vec<_> = swapchain_images
            .iter()
            .map(|img| {
                device.create_image_view(
                    &vk::ImageViewCreateInfo::default()
                        .image(img)
                        .view_type(vk::ImageViewType::Type2D)
                        .format(format.format)
                        .subresource_range(vk::ImageSubresourceRange {
                            aspect_mask: vk::ImageAspectFlags::Color,
                            base_mip_level: 0,
                            level_count: 1,
                            base_array_layer: 0,
                            layer_count: 1,
                        }),
                )
            })
            .collect::<vk::Result<_>>()
            .unwrap();

        let swapchain_resource = SwapchainResource {
            swapchain,
            images: swapchain_images,
            image_views: swapchain_views,
            format: format.format,
            extent,
        };

        swapchain_resource
    }

    pub fn create_frames_data(device: Device, queue_family_index: usize) -> Vec<FrameData> {
        let mut frames_data: Vec<FrameData> = Vec::new();

        let command_pool_create_info = CommandPoolCreateInfo::default()
            .flags(CommandPoolCreateFlags::ResetCommandBuffer)
            .queue_family_index(queue_family_index as _);

        for _ in 0..Self::FRAMES_IN_FLIGHT {
            let command_pool = device
                .create_command_pool(&command_pool_create_info)
                .unwrap();

            let command_buffer_allocate_info = CommandBufferAllocateInfo::default()
                .level(vk::CommandBufferLevel::Primary)
                .command_pool(&command_pool)
                .command_buffer_count(1);

            let command_buffers: Vec<_> = device
                .allocate_command_buffers(&command_buffer_allocate_info)
                .unwrap();

            let command_buffer = command_buffers[0];

            let frame_data = FrameData::new(command_pool, command_buffer);
            frames_data.push(frame_data);
        }

        frames_data
    }
}
