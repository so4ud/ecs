use std::any::TypeId;

use crate::{ecs::ECS, events::Event, systems::Systems};

pub struct App {
    pub ecs: ECS,
    systems: Systems,
    plugins: Plugins,
}
impl App {
    pub fn new() -> Self {
        Self {
            ecs: ECS::new(),
            systems: Systems::new(),
            plugins: Plugins::new(),
        }
    }
    /// plugins are ran in the order that they are added
    pub(super) fn add_plugin(&mut self, plugin: fn(&mut App)) {
        self.plugins.plugins.push(plugin);
    }
    /// systems are ran in the order that they are added
    pub(super) fn add_system<TriggerEvent: Event + 'static>(&mut self, system: fn(&mut ECS)) {
        let type_id = TypeId::of::<TriggerEvent>();
        self.systems.systems.insert(type_id, system);
    }
    pub(super) fn run(mut self) {
        self.plugins.clone().run(&mut self);
        self.plugins.clear();

        loop {
            for system in &self.systems.systems {
                (system.1)(&mut self.ecs);
            }
        }
    }
}

enum AppErr {}

#[derive(Debug, Clone)]
struct Plugins {
    plugins: Vec<fn(&mut App)>,
}
impl Plugins {
    fn new() -> Self {
        Self { plugins: vec![] }
    }
    fn run(&self, app: &mut App) {
        for plugin in &self.plugins {
            plugin(app);
        }
    }
    fn clear(&mut self) {
        self.plugins.clear();
    }
}
