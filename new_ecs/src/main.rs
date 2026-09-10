#![cfg_attr(debug_assertions, allow(unused))]

use core::panic;
pub mod app;
pub mod components;
pub mod ecs;
pub mod entities;
pub mod events;
pub mod recources;
pub mod systems;
pub mod systemsex;
use crate::{
    app::App,
    ecs::{ECS, EntityID},
    events::{Event, Startup, Update},
};
use bevy_ecs;
use components::Component;
use macros::{Component, Event};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    vec,
};

#[derive(Debug, Clone, Copy, Component)]
struct Sex {
    sex: &'static str,
}

#[derive(Debug, Clone, Event)]
struct Evv {}
fn main() {
    systemsex::main();
    let mut app = App::new();

    app.add_system::<Startup>(setup_system);
    app.add_system::<Update>(move_system);
    app.run_plugins();
    loop {
        app.update();
    }
}

#[derive(Debug, Component)]
struct Position {
    x: f64,
}
#[derive(Debug, Component)]
struct Velocity {
    x: f64,
}

fn setup_system(ecs: &mut ECS) {
    ecs.spawn_entity();
    ecs.attach_component(0, Velocity { x: 0.5 });
    ecs.attach_component(0, Position { x: 0.0 });
    ecs.spawn_entity();
    ecs.attach_component(1, Velocity { x: -0.5 });
    ecs.attach_component(1, Position { x: 0.0 });
}

fn move_system(ecs: &mut ECS) {
    let update = ecs.get_event::<Update>().unwrap().clone();
    let pos1 = ecs.get_component_ref::<Position>(0).unwrap().x.clone();
    let pos2 = ecs.get_component_ref::<Position>(1).unwrap().x.clone();
    let vel1 = ecs.get_component_ref::<Velocity>(0).unwrap().x.clone();
    let vel2 = ecs.get_component_ref::<Velocity>(1).unwrap().x.clone();

    println!("position 1: {}", &pos1);
    println!("position 2: {}", &pos2);
    print!("\x1b[1A\x1b[1A\r");

    ecs.get_component_mut::<Position>(0).unwrap().x = pos1 + vel1 * update.as_secs();
    ecs.get_component_mut::<Position>(1).unwrap().x = pos2 + vel2 * update.as_secs();
}
