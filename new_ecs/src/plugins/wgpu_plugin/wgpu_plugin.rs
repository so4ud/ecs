use std::sync::Arc;

use wgpu::{Buffer, RenderPipelineDescriptor};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, OwnedDisplayHandle};
use winit::window::{Window, WindowId};

use crate::app::App;
use crate::archetypes::{self, Runtime};
use crate::plugins::wgpu_plugin::state::State;
use crate::{events, wgpu};

pub fn wgpu_plugin(app: &mut App) {
    app.ecs
        .recources
        .insert_recource(Runtime::new(winit_runtime));
}

fn winit_runtime(mut app: App) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window object
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let state = pollster::block_on(State::new(
            event_loop.owned_display_handle(),
            window.clone(),
        ));
        self.ecs.recources.insert_recource(state);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                // user's responsibility
                self.ecs.push_event(events::CloseRequested {});
            }
            WindowEvent::RedrawRequested => {
                let mut state = self.ecs.recources.pop_recource::<State>().unwrap();
                if self.ecs.has_recource::<Buffer>() {
                    let buffer = self.ecs.get_recource_ref::<Buffer>().unwrap();
                    state.render(Some(&buffer));
                } else {
                    state.render(None);
                }
                state.window.request_redraw();
                self.ecs.push_recource(state);
                self.update();
            }

            WindowEvent::Resized(size) => {
                let state = self.ecs.recources.get_recource_mut::<State>().unwrap();
                state.resize(size);
            }
            _ => (),
        }
    }
}
