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
use crate::{
    app::App,
    archetypes::Plugins,
    ecs::{ECS, EntityID},
    events::{CloseRequested, Event, Startup, Update},
    plugins::wgpu_plugin::{state::State, vertex::Vertex},
};
// use bevy_ecs;
use components::Component;
use macros::{Component, Event};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    io::Write,
    ops::AddAssign,
    vec,
};
use wgpu::{self, Buffer, util::DeviceExt};

fn main() {
    systemsex::main();
    let mut app = App::new();
    app.add_plugins(Plugins::default());
    app.add_system::<events::CloseRequested>(|ecs: &mut ECS| {
        dbg!("blueh");
        std::process::exit(0);
    });
    app.add_system::<Update>(|ecs| {
        dbg!();
        if !ecs.has_recource::<State>() {
            return;
        }
        let state = ecs.get_recource_mut::<State>().unwrap();
        let vertecies = [
            Vertex::new([1.0, 0.0 - 0.5, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
            Vertex::new([0.0, 1.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
            Vertex::new([-1.0, 0.0 - 0.5, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        ];
        let vertex_buffer: wgpu::Buffer =
            state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&vertecies),
                    usage: wgpu::BufferUsages::VERTEX, // Mark it specifically as a vertex buffer
                });
        ecs.push_recource(vertex_buffer);
    });
    app.add_system::<Update>(|ecs| {
        if !ecs.has_recource::<Buffer>() {
            return;
        }
        let buffer = ecs.get_recource_mut::<Buffer>().unwrap();
        dbg!(buffer.slice(0..3));
    });
    app.run();
}
