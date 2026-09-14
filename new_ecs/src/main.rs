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
    archetypes::Plugins,
    ecs::{ECS, EntityID},
    events::{CloseRequested, Event, Startup, Update},
    plugins::wgpu_plugin::{self, state::State, vertex::Vertex},
};
// use bevy_ecs;
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
        if !ecs.has_recource::<State>() {
            return;
        }
        let state = ecs.get_recource_mut::<State>().unwrap();
        let vertecies = [
            Vertex::new([0.3, 0.3, 0.0], [0.0, 0.0, 0.0], [1.0, 0.0]),
            Vertex::new([-0.3, 0.3, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
            Vertex::new([-0.3, -0.3, 1.0], [0.0, 0.0, 0.0], [0.0, 1.0]),
            // trig
            Vertex::new([0.3, 0.3, 0.0], [0.0, 0.0, 0.0], [1.0, 0.0]),
            Vertex::new([-0.3, -0.3, 0.0], [0.0, 0.0, 0.0], [0.0, 1.0]),
            Vertex::new([0.3, -0.3, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0]),
        ];
        let vertex_buffer: wgpu::Buffer =
            state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&vertecies),
                    usage: wgpu::BufferUsages::VERTEX,
                });
        // state.render(Some(&vertex_buffer), None);
        ecs.insert_recource(vertex_buffer);
    });
    app.add_system::<Update>(|ecs| {
        let mut state = ecs.pop_recource::<State>().unwrap();
        let vertex_buffer = ecs.get_recource_ref::<Buffer>().unwrap();
        let texture_view = ecs.get_recource_ref::<wgpu_plugin::TextureAtlas>().unwrap();
        state.render(Some(vertex_buffer), Some(texture_view.get_texture(0).0));
        ecs.insert_recource(state);
    });
    app.add_system::<Startup>(|ecs| {
        ecs.push_event(E1 {});
        ecs.push_event(E2 {});
        ecs.push_event(E3 {});
    });
    app.add_system::<E1>(|_| {
        dbg!(1);
    });
    app.add_system::<E2>(|_| {
        dbg!(2);
    });
    app.add_system::<E3>(|_| {
        dbg!(3);
    });
    app.run();
}

#[derive(Clone, Copy, Event)]
struct E1 {}
#[derive(Clone, Copy, Event)]
struct E2 {}
#[derive(Clone, Copy, Event)]
struct E3 {}
