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
    events::{Event, Update},
};
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
    let mut app = App::new();
    app.add_plugin(|app| {
        let id = app.ecs.spawn_entity();

        app.ecs.attach_component(id, Sex { sex: "sesx" });
    });

    app.add_system::<Update>(|ecs| {
        ecs.push_event(Evv {});
        ecs.push_event(Evv {});
        let comp = ecs.get_component_ref::<Sex>(0).unwrap();
        println!("hi: {}", comp.sex);
    });
    app.add_system::<Evv>(|_| {
        dbg!("evv spoted");
    });
    app.run_plugins();
    loop {
        app.update();
    }
}
