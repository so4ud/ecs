use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{
        DeviceEvent::{self, Button, MouseMotion},
        WindowEvent,
    },
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::{app::App, events, plugins::wgpu_plugin::state::State};

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window object
        if !self.ecs.has_recource::<Arc<Window>>() {
            let window = Arc::new(
                event_loop
                    .create_window(Window::default_attributes())
                    .unwrap(),
            );

            window.request_redraw();
            self.ecs.insert_recource(window);
            self.run_plugins(event_loop);
        }
    }
    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                let event = crate::plugins::keys_plugin::MouseMotion { delta };
                self.ecs.push_event(event);
            }
            _ => (),
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                // user's responsibility
                self.ecs.push_event(events::CloseRequested {});
            }
            WindowEvent::Resized(size) => {
                self.ecs.push_event(events::WindowResized {
                    new_size: (size.width, size.height),
                });
                // dbg!(&size);
            }
            WindowEvent::RedrawRequested => {
                self.ecs
                    .get_recource_ref::<Arc<Window>>()
                    .unwrap()
                    .request_redraw();
                self.update();
            }

            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                let key = event.logical_key;
                let is_pressed = event.state.is_pressed();
                let event = crate::plugins::keys_plugin::KeyboardInput { key, is_pressed };
                self.ecs.push_event(event);
            }
            _ => (),
        }
    }
}
