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
    plugins::{
        keys_plugin::{KeyboardInput, MouseMotion},
        wgpu_plugin::{self, Camera, read_fbx, state::State, vertex::Vertex},
    },
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
use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    keyboard::Key,
    window::Window,
};
// todo add app.query()
fn main() {
    let mut app = App::new();
    app.add_plugins(Plugins::default());
    app.add_system::<events::CloseRequested>(|_| {
        println!("blueh");
        std::process::exit(0);
    });
    app.add_system::<Startup>(|ecs| {
        let state = ecs.get_recource_mut::<State>().unwrap();
        let vertex_buffer = read_fbx(&state.device, "assets/models/блять.fbx".to_string());
        let vertex_buffer1 = read_fbx(&state.device, "assets/models/guy.fbx".to_string());
        let mesh_atlas = ecs.get_recource_mut::<wgpu_plugin::MeshAtlas>().unwrap();
        mesh_atlas.vertex_buffers.push(vertex_buffer);
        mesh_atlas.vertex_buffers.push(vertex_buffer1);
        // spawn renderable entity
        let entity_id = ecs.spawn_entity();
        ecs.attach_component(entity_id, wgpu_plugin::Mesh { mesh_id: 0 });
        ecs.attach_component(entity_id, wgpu_plugin::Texture { texture_id: 0 });
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
        ecs.insert_recource(true);
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
    app.add_system::<KeyboardInput>(|ecs| {
        let event = ecs.get_event::<KeyboardInput>().unwrap().clone();
        if !event.is_pressed {
            return;
        }
        match event.key {
            Key::Character(chr) => {
                let chr = chr.to_string();
                if &chr == "m" {
                    let window = ecs.get_recource_mut::<Arc<Window>>().unwrap().clone();
                    let is_shown = ecs.get_recource_mut::<bool>().unwrap();
                    if *is_shown == true {
                        *is_shown = false;
                        window.set_cursor_visible(*is_shown);
                    } else {
                        *is_shown = true;
                        window.set_cursor_visible(*is_shown);
                    }
                }
            }
            _ => (),
        }
    });
    app.add_system::<MouseMotion>(|ecs| {
        let event = ecs.get_event::<MouseMotion>().unwrap().clone();
        let is_shown = ecs.get_recource_ref::<bool>().unwrap().clone();
        if is_shown {
            return;
        }
        let window = ecs.get_recource_ref::<Arc<Window>>().unwrap().clone();
        if let Some(monitor) = window.current_monitor() {
            let monitor_size = monitor.size();
            let window_size = window.outer_size();

            let x = (monitor_size.width as i32 - window_size.width as i32) / 2;
            let y = (monitor_size.height as i32 - window_size.height as i32) / 2;

            window.set_cursor_position(PhysicalPosition::new(x, y));
        }
    });

    app.add_system::<MouseMotion>(|ecs| {
        if ecs.get_recource_ref::<bool>().unwrap() == &true {
            return;
        }
        let event = ecs.get_event::<MouseMotion>().unwrap().clone();
        let mut cam_id = None;
        for id in ecs.iter_over_alive_entity_ids() {
            if ecs.has_component_immut::<Camera>(id) && ecs.has_component_immut::<Camera>(id) {
                cam_id = Some(id);
                break;
            }
        }
        match cam_id {
            Some(id) => {
                let oreintation = ecs.get_component_mut::<Orientation>(id).unwrap();
                oreintation.x -= event.delta.1 as f32;
                oreintation.y -= event.delta.0 as f32;
                if oreintation.y > 360.0 {
                    oreintation.y -= 360.0;
                }
                if oreintation.x > 89.0 {
                    oreintation.x = 89.0;
                }
                if oreintation.x < -89.0 {
                    oreintation.x = -89.0;
                }
            }
            _ => (),
        }
    });
    app.run();
}
fn flip_sing(number: i128) -> i128 {
    !number + 1
}
