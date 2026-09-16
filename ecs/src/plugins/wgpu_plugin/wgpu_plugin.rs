use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::vec;

use macros::Component;
use wgpu::wgt::TextureDescriptor;
use wgpu::{BindGroupLayout, Buffer, Extent3d, Origin3d, RenderPipeline, RenderPipelineDescriptor};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, OwnedDisplayHandle};
use winit::window::{Window, WindowId};

use crate::app::App;
use crate::archetypes::{self};
use crate::components::Component;
use crate::ecs::ECS;
use crate::events::{Update, WindowResized};
use crate::plugins::wgpu_plugin::render_system;
use crate::plugins::wgpu_plugin::state::State;
use crate::wgpu_plugin::texture_atlas::*;
use crate::{events, wgpu};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Uniforms {
    pub(crate) m: [[f32; 4]; 4],
    pub(crate) v: [[f32; 4]; 4],
    pub(crate) p: [[f32; 4]; 4],
}

pub(crate) struct RenderingPipelinesAndBinds {
    pub(crate) rendering_infos: HashMap<String, RenderingPipelineAndBind>,
}
pub(crate) struct RenderingPipelineAndBind {
    pub(crate) bind_group_layout: BindGroupLayout,
    pub(crate) render_pipeline: RenderPipeline,
}

pub fn wgpu_plugin(app: &mut App, event_loop: &winit::event_loop::ActiveEventLoop) {
    let window = app.ecs.get_recource_mut::<Arc<Window>>().unwrap();
    let mut state = pollster::block_on(State::new(
        event_loop.owned_display_handle(),
        window.clone(),
    ));
    let mut texture_atlas = TextureAtlas::new(&mut state);
    texture_atlas.read_texture_data();
    let mesh_atlas = MeshAtlas::new();
    state.init_rendering_pipeline(&mut app.ecs);
    app.ecs.insert_recource(texture_atlas);
    app.ecs.recources.insert_recource(state);
    app.ecs.recources.insert_recource(mesh_atlas);
    app.add_system::<WindowResized>(resize_sys);
    app.add_system::<Update>(render_system::render_system);
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
fn init_shaders_bindgroups(ecs: &mut ECS) {}
#[derive(Debug, Clone, Copy, Component)]
pub struct Camera {
    pub fov: f32,
    pub range: f32,
}

pub type MeshID = usize;

#[derive(Debug, Clone, Copy, Component)]
pub struct Mesh {
    /// index into the `MeshAtlas` recource
    pub mesh_id: MeshID,
}

pub struct MeshAtlas {
    /// vertex buffer and amount of vertecies in it
    pub(crate) vertex_buffers: Vec<(wgpu::Buffer, u32)>,
}
impl MeshAtlas {
    pub(crate) fn new() -> Self {
        Self {
            vertex_buffers: vec![],
        }
    }
    pub(crate) fn get_mesh(&self, mesh_id: usize) -> (&wgpu::Buffer, u32) {
        if mesh_id > self.vertex_buffers.len() - 1 {
            return (&self.vertex_buffers[0].0, self.vertex_buffers[0].1.clone());
        }
        return (
            &self.vertex_buffers[mesh_id].0,
            self.vertex_buffers[mesh_id].1.clone(),
        );
    }
}
