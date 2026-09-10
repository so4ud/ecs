#![cfg_attr(debug_assertions, allow(unused))]

use core::panic;
pub mod app;
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
    ecs::{ECS, EntityID},
    events::{Event, Startup, Update},
    plugins::{Plugins, wgpu_plugin::wgpu_plugin},
};
use bevy_ecs;
use components::Component;
use macros::{Component, Event};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    io::Write,
    ops::AddAssign,
    vec,
};
use wgpu;

fn main() {
    systemsex::main();
    let mut app = App::new();
    app.add_plugins(Plugins::default());

    app.run();
}
