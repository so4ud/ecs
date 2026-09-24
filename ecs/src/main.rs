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
pub mod winit_app;
use crate::{
    app::{App, UpdateInfo},
    archetypes::{Orientation, Plugins, Position},
    ecs::{ECS, EntityID},
    events::{CloseRequested, Event, Startup, Update},
    plugins::{
        keys_plugin::{HeldKeys, KeyboardInput, MouseMotion},
        wgpu_plugin::{self, Camera, read_fbx, state::State, vertex::Vertex},
    },
};
// use bevy_ecs;
use asset_importer::{Importer, postprocess::PostProcessSteps};
use cgmath::InnerSpace;
use cgmath::{Deg, Matrix3, vec3};
use components::Component;
use macros::{Component, Event};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    io::Write,
    ops::AddAssign,
    sync::Arc,
    time::Instant,
    vec,
};
use wgpu::{self, Buffer, TextureView, util::DeviceExt};
use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    keyboard::{Key, NamedKey, SmolStr},
    window::Window,
};
// todo add app.query()
struct FrameCount {
    amount: u32,
    stamp: Instant,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(Plugins::default());
    app.add_system::<events::CloseRequested>(|_| {
        println!("blueh");
        std::process::exit(0);
    });
    app.add_system::<Startup>(|ecs| {
        ecs.insert_recource(FrameCount {
            amount: 0,
            stamp: Instant::now(),
        });
        let state = ecs.get_recource_mut::<State>().unwrap();
        let vertex_buffer = read_fbx(&state.device, "assets/models/блять.fbx".to_string());
        let vertex_buffer1 = read_fbx(&state.device, "assets/models/guy.fbx".to_string());
        let mesh_atlas = ecs.get_recource_mut::<wgpu_plugin::MeshAtlas>().unwrap();
        mesh_atlas.vertex_buffers.push(vertex_buffer);
        mesh_atlas.vertex_buffers.push(vertex_buffer1);
        // spawn renderable entity

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
                y: 180.0,
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

        ecs.insert_recource(true);
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
    app.add_system::<Update>(|ecs| {
        let mut cam_id = None;
        for id in ecs.iter_over_alive_entity_ids() {
            if ecs.has_component_immut::<Camera>(id) && ecs.has_component_immut::<Camera>(id) {
                cam_id = Some(id);
                break;
            }
        }
        let dv = ecs.get_event::<Update>().unwrap().as_secs() as f32;
        let held_keys = ecs.get_recource_ref::<HeldKeys>().unwrap();
        let velocity = 150.0f32;
        match cam_id {
            Some(id) => {
                let ori = ecs.get_component_ref::<Orientation>(id).unwrap().clone();
                let mut pos = ecs.get_component_ref::<Position>(id).unwrap().clone();

                if held_keys.is_pressed(&Key::Character(SmolStr::from("w"))) {
                    let forward = vec3(0.0, 0.0, 1.0f32);
                    let rot = Matrix3::from_angle_y(Deg(ori.y));
                    let forward = rot * forward * velocity * dv;

                    pos = Position {
                        x: pos.x + forward.x,
                        y: pos.y + forward.y,
                        z: pos.z + forward.z,
                    };
                }
                if held_keys.is_pressed(&Key::Character(SmolStr::from("s"))) {
                    let forward = vec3(0.0, 0.0, 1.0f32);
                    let rot = Matrix3::from_angle_y(Deg(ori.y + 180.0));
                    let forward = rot * forward * velocity * dv;

                    pos = Position {
                        x: pos.x + forward.x,
                        y: pos.y + forward.y,
                        z: pos.z + forward.z,
                    };
                }
                if held_keys.is_pressed(&Key::Character(SmolStr::from("a"))) {
                    let forward = vec3(0.0, 0.0, 1.0f32);
                    let rot = Matrix3::from_angle_y(Deg(ori.y + 90.0));
                    let forward = rot * forward * velocity * dv;
                    pos = Position {
                        x: pos.x + forward.x,
                        y: pos.y + forward.y,
                        z: pos.z + forward.z,
                    };
                }
                if held_keys.is_pressed(&Key::Character(SmolStr::from("d"))) {
                    let forward = vec3(0.0, 0.0, 1.0f32);
                    let rot = Matrix3::from_angle_y(Deg(ori.y + 260.0));
                    let forward = rot * forward * velocity * dv;
                    pos = Position {
                        x: pos.x + forward.x,
                        y: pos.y + forward.y,
                        z: pos.z + forward.z,
                    };
                }
                if held_keys.is_pressed(&Key::Named(NamedKey::Space)) {
                    let up = vec3(0.0, 1.0, 0.0f32);
                    let up = up * velocity * dv;
                    pos = Position {
                        x: pos.x + up.x,
                        y: pos.y + up.y,
                        z: pos.z + up.z,
                    };
                }
                if held_keys.is_pressed(&Key::Named(NamedKey::Control)) {
                    let down = vec3(0.0, -1.0, 0.0f32);
                    let down = down * velocity * dv;
                    pos = Position {
                        x: pos.x + down.x,
                        y: pos.y + down.y,
                        z: pos.z + down.z,
                    };
                }

                *ecs.get_component_mut::<Position>(id).unwrap() = pos;
            }
            _ => (),
        }
    });
    app.add_system::<KeyboardInput>(spawn_system);
    app.run();
}
fn spawn_system(ecs: &mut ECS) {
    let event = ecs.get_event::<KeyboardInput>().unwrap().clone();
    let held_keys = ecs.get_recource_ref::<HeldKeys>().unwrap().clone();
    if !event.is_pressed {
        return;
    }
    match event.key {
        Key::Character(c) => {
            if &c.to_string()[..] == "j" {
                let pos = ecs.get_component_ref::<Position>(0).unwrap().clone();

                spawn_blyat_at_pos(ecs, pos);
            }
        }
        _ => (),
    }
}

fn spawn_blyat_at_pos(ecs: &mut ECS, pos: Position) {
    let entity_id = ecs.spawn_entity();

    ecs.attach_component(
        entity_id,
        Position {
            x: pos.x,
            y: pos.y,
            z: pos.z,
        },
    );
    ecs.attach_component(entity_id, wgpu_plugin::Mesh { mesh_id: 0 });
    ecs.attach_component(entity_id, wgpu_plugin::Texture { texture_id: 0 });
}
