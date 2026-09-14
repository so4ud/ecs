use std::any::Any;
use std::rc::Rc;
use std::sync::Arc;
use std::vec;

use macros::Component;
use wgpu::wgt::TextureDescriptor;
use wgpu::{Buffer, Extent3d, RenderPipelineDescriptor};
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
use crate::{events, wgpu};

pub fn wgpu_plugin(app: &mut App, event_loop: &winit::event_loop::ActiveEventLoop) {
    let window = app.ecs.get_recource_mut::<Arc<Window>>().unwrap();
    let mut state = pollster::block_on(State::new(
        event_loop.owned_display_handle(),
        window.clone(),
    ));
    app.ecs.insert_recource(TextureAtlas::new(&mut state));
    app.ecs.recources.insert_recource(state);
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

#[derive(Debug, Clone, Copy, Component)]
pub struct Camera {
    fov: f32,
    range: f32,
}

pub type TextureID = usize;
pub type MeshID = usize;

#[derive(Debug, Clone, Copy, Component)]
pub struct Mesh {
    /// index into the `MeshAtlas` recource
    pub mesh_id: MeshID,
}
#[derive(Debug, Clone, Copy, Component)]
pub struct Texture {
    /// index into the `TextureAtlas` recource
    pub texture_id: TextureID,
}
pub struct MeshAtlas {
    /// vertex buffer and amount of vertecies in it
    vertex_buffers: Vec<(wgpu::Buffer, u32)>,
}
pub struct TextureAtlas {
    atlas: wgpu::Texture,
    texture_views: wgpu::TextureView,
    // ! have to use this
    texture_info: Vec<TextureInfo>,
}
impl TextureAtlas {
    fn new(state: &mut State) -> Self {
        let texture = state.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("main texture atlas"),
            size: wgpu::Extent3d {
                width: 4096,
                height: 4096,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb, // Standard for sRGB PNGs
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut pih = Self {
            atlas: texture,
            texture_views: texture_view,
            texture_info: vec![],
        };
        pih.load_texture(
            state,
            "assets/textures/guy.png",
            Some("defoult texture".to_string()),
        );
        return pih;
    }
    /// png only
    pub fn load_texture(
        &mut self,
        state: &mut State,
        file_path: impl AsRef<std::path::Path>,
        name: Option<String>,
    ) -> TextureID {
        let img_bytes = std::fs::read(file_path).unwrap();
        let img = image::load_from_memory(&img_bytes[..]).unwrap().to_rgba8();
        let dimensions = img.dimensions();

        // 1. Define texture size and descriptor
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        // 2. Write the image data to the queue
        state.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.atlas,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &img,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        self.texture_info.push(TextureInfo {
            name,
            size: dimensions,
            start_position: (0, 0),
            index: 0,
        });
        return self.texture_info.len() - 1;
    }
    pub fn get_texture(&self, texture_id: TextureID) -> (&wgpu::TextureView, &TextureInfo) {
        if texture_id > self.texture_info.len() - 1 {
            return (&self.texture_views, &self.texture_info[0]);
        }
        return (&self.texture_views, &self.texture_info[texture_id]);
    }
}
// ! encoder.copy_texture_to_texture(source, destination, Extent3d {depth_or_array_layers});

pub struct TextureInfo {
    pub name: Option<String>,
    /// size in pixels
    pub size: (u32, u32),
    /// start position in the `TextureAtlas`, top-left corner
    pub start_position: (u32, u32),
    /// index into `TextureAtlas.atlas`
    pub index: usize,
    // format: wgpu::TextureFormat,
}
