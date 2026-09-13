use std::any::Any;
use std::rc::Rc;
use std::sync::Arc;

use macros::Component;
use wgpu::{Buffer, RenderPipelineDescriptor};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, OwnedDisplayHandle};
use winit::window::{Window, WindowId};

use crate::app::App;
use crate::archetypes::{self};
use crate::components::Component;
use crate::ecs::ECS;
use crate::events::WindowResized;
use crate::plugins::wgpu_plugin::state::State;
use crate::{events, wgpu};

pub fn wgpu_plugin(app: &mut App, event_loop: &winit::event_loop::ActiveEventLoop) {
    let window = app.ecs.get_recource_mut::<Arc<Window>>().unwrap();
    let mut state = pollster::block_on(State::new(
        event_loop.owned_display_handle(),
        window.clone(),
    ));
    app.ecs.recources.insert_recource(state);
    app.add_system::<WindowResized>(resize_sys);
}

fn resize_sys(ecs: &mut ECS) {
    let new_size = ecs.get_event::<WindowResized>().unwrap().clone();
    // dbg!(&new_size);
    let state = ecs.get_recource_mut::<State>().unwrap();

    state.resize(winit::dpi::PhysicalSize {
        width: new_size.new_size.0,
        height: new_size.new_size.1,
    });
}

#[derive(Debug, Clone, Copy, Component)]
pub struct Mesh {
    /// index into the `MeshAtlas` recource
    mesh_id: usize,
}
#[derive(Debug, Clone, Copy, Component)]
pub struct Texture {
    /// index into the `TextureAtlas` recource
    texture_id: usize,
}
struct MeshAtlas {}
struct TextureAtlas {
    atlas: Vec<wgpu::Texture>,
    texture_info: Vec<TextureInfo>,
}
// ! encoder.copy_texture_to_texture(source, destination, Extent3d {depth_or_array_layers});

struct TextureInfo {
    name: Option<String>,
    /// size in pixels
    size: (u32, u32),
    /// start position in the `TextureAtlas`, top-left corner
    start_position: (u32, u32),
    /// index into `TextureAtlas.atlas`
    index: usize,
}
