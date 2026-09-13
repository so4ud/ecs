use std::any::Any;
use std::rc::Rc;
use std::sync::Arc;

use wgpu::{Buffer, RenderPipelineDescriptor};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, OwnedDisplayHandle};
use winit::window::{Window, WindowId};

use crate::app::App;
use crate::archetypes::{self};
use crate::plugins::wgpu_plugin::state::State;
use crate::{events, wgpu};

pub fn wgpu_plugin(app: &mut App, event_loop: &winit::event_loop::ActiveEventLoop) {
    let window = app.ecs.get_recource_mut::<Arc<Window>>().unwrap();
    let state = pollster::block_on(State::new(
        event_loop.owned_display_handle(),
        window.clone(),
    ));
    app.ecs.recources.insert_recource(state);
}
