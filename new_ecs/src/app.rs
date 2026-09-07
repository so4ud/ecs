use std::{any::TypeId, time::Instant};

use crate::{
    ecs::ECS,
    events::{Event, Startup, Update},
    systems::Systems,
};

pub struct App {
    pub ecs: ECS,
    systems: Systems,
    plugins: Plugins,
}
impl App {
    pub fn new() -> Self {
        let mut ses = Self {
            ecs: ECS::new(),
            systems: Systems::new(),
            plugins: Plugins::new(),
        };
        ses.ecs.recources.insert_recource(UpdateInfo {
            is_first: true,
            latest_update: Instant::now(),
        });

        ses
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
    pub fn run_plugins(&mut self) {
        self.plugins.clone().run(self);
        self.plugins.clear();
    }
    pub(crate) fn update(&mut self) {
        let update_info = self
            .ecs
            .recources
            .get_recource_mut::<UpdateInfo>()
            .expect("\n======= UPDATE INFO NOT PRESENT =======\n");

        if update_info.is_first {
            self.ecs.events.push_event(Startup {});
            update_info.latest_update = Instant::now();
            update_info.is_first = false;
        } else {
            self.ecs.events.push_event(Update {
                dv: update_info.latest_update - Instant::now(),
            });
            update_info.latest_update = Instant::now();
        }
        let event_type_ids: Vec<TypeId> = self.ecs.events.events.iter().map(|i| i.0).collect();

        for i in event_type_ids {
            for sys in &self.systems.systems {
                if &i == sys.0 {
                    (sys.1)(&mut self.ecs);
                }
            }
        }
        self.ecs.events.swap_and_clear();
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

struct UpdateInfo {
    is_first: bool,
    latest_update: Instant,
}
