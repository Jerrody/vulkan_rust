mod engine;

use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{self, ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

use engine::Engine;

#[derive(Default)]
struct Application {
    window: Option<Window>,
    engine: Option<Engine>,
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        debug_assert!(
            self.engine.is_none(),
            "This example app does not expect the Window to be re-created"
        );

        let window_attributes = Window::default_attributes()
            .with_title("Vulkan")
            .with_inner_size(LogicalSize::new(1280, 720));
        let window = event_loop.create_window(window_attributes).unwrap();

        if self.engine.is_none() {
            self.engine = Some(Engine::new(&window));
            self.window = Some(window);
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: winit::event::StartCause) {
        if let Some(engine) = &mut self.engine {
            engine.update();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        /*         let Some(engine) = &mut self.engine else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::ScaleFactorChanged { .. } | WindowEvent::Resized(_) => {
                //vulkan_app.swapchain_objects.take();
            }
            _ => (),
        } */
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(event_loop::ControlFlow::Poll);
    let mut application = Application::default();
    event_loop.run_app(&mut application).unwrap();
}
