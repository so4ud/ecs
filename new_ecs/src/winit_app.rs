use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
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

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                // user's responsibility
                self.ecs.push_event(events::CloseRequested {});
            }
            WindowEvent::RedrawRequested => {
                // let mut state = self.ecs.recources.pop_recource::<State>().unwrap();
                // state.window.request_redraw();
                // self.ecs.insert_recource(state);
                self.update();
            }

            WindowEvent::Resized(size) => {
                // let state = self.ecs.recources.get_recource_mut::<State>().unwrap();
                // state.resize(size);
                self.ecs.push_event(events::WindowResized {
                    new_size: (size.width, size.height),
                });
            }
            _ => (),
        }
    }
}
