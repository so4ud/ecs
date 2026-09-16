#![cfg_attr(debug_assertions, allow(unused))]

use core::panic;
pub mod app;
pub mod archetypes;
pub mod components;
pub mod ecs;
pub mod entities;
pub mod events;
pub mod plugins;
pub mod query;
pub mod recources;
pub mod systems;
pub mod systemsex;
mod winit_app;
use crate::{
    app::App,
    archetypes::{Orientation, Plugins, Position},
    ecs::{ECS, EntityID},
    events::{CloseRequested, Event, Startup, Update},
    plugins::wgpu_plugin::{self, Camera, read_fbx, state::State, vertex::Vertex},
};
// use bevy_ecs;
use asset_importer::{Importer, postprocess::PostProcessSteps};
use components::Component;
use macros::{Component, Event};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    io::Write,
    ops::AddAssign,
    sync::Arc,
    vec,
};
use wgpu::{self, Buffer, TextureView, util::DeviceExt};
use winit::{dpi::LogicalSize, window::Window};

fn main() {
    let mut app = App::new();
    app.add_plugins(Plugins::default());
    app.add_system::<events::CloseRequested>(|_| {
        println!("blueh");
        std::process::exit(0);
    });
    app.add_system::<Startup>(|ecs| {
        let state = ecs.get_recource_mut::<State>().unwrap();
        let vertex_buffer = read_fbx(&state.device, "assets/models/guy.fbx".to_string());
        let mesh_atlas = ecs.get_recource_mut::<wgpu_plugin::MeshAtlas>().unwrap();
        mesh_atlas.vertex_buffers.push(vertex_buffer);
        // spawn renderable entity
        let entity_id = ecs.spawn_entity();
        ecs.attach_component(entity_id, wgpu_plugin::Mesh { mesh_id: 0 });
        ecs.attach_component(entity_id, wgpu_plugin::Texture { texture_id: 2 });
        ecs.attach_component(
            entity_id,
            Position {
                x: 0.0,
                y: 0.0,
                z: 1.7,
            },
        );
        ecs.attach_component(
            entity_id,
            Orientation {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        );
        // spawn camera
        let entity_id = ecs.spawn_entity();
        ecs.attach_component(
            entity_id,
            Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        );
        ecs.attach_component(
            entity_id,
            Orientation {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        );
        ecs.attach_component(
            entity_id,
            Camera {
                fov: 90.0,
                range: 1024.0,
            },
        );
        ecs.insert_recource(0f32);
    });
    app.add_system::<Update>(|ecs| {
        let mut counter = ecs.pop_recource::<f32>().unwrap();
        let mut cam_id = 1;

        let pos = ecs.get_component_mut::<Orientation>(0).unwrap();
        pos.y = counter * 50.0;
        counter += 0.01;
        if counter > 360.0 {
            counter -= 360.0;
        }
        ecs.insert_recource(counter);
    });
    app.run();
}
