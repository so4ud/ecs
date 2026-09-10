use std::{any::TypeId, time::Instant};

use crate::{
    ecs::ECS,
    events::{Event, Startup, Update},
    plugins::wgpu_plugin::Runtime,
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
    pub fn run(mut self) {
        self.run_plugins();
        if !self.ecs.recources.has_recource::<Runtime>() {
            self.run_plugins();
            loop {
                self.update();
            }
        } else {
            let mut runtime = self.ecs.recources.pop_recource::<Runtime>().unwrap();
            (runtime.runtime)(self);
        }
    }
    /// plugins are ran in the order that they are added
    pub(super) fn add_plugin<F: Fn(&mut App) + 'static>(&mut self, plugin: F) {
        self.plugins.plugins.push(Box::new(plugin));
    }
    pub fn add_plugins<T: AddPlugIn>(&mut self, plugins: T) {
        plugins.add_self_as_plugin(self);
    }
    /// systems are ran in the order that they are added
    pub(super) fn add_system<TriggerEvent: Event + 'static>(&mut self, system: fn(&mut ECS)) {
        let type_id = TypeId::of::<TriggerEvent>();
        self.systems.systems.insert(type_id, system);
    }
    pub fn run_plugins(&mut self) {
        let mut new_plugins = Plugins { plugins: vec![] };
        std::mem::swap(&mut self.plugins, &mut new_plugins);
        new_plugins.run(self);
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
                dv: Instant::now() - update_info.latest_update,
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

struct Plugins {
    plugins: Vec<Box<dyn Fn(&mut App)>>,
}
impl Plugins {
    fn new() -> Self {
        Self { plugins: vec![] }
    }
    fn run(&self, app: &mut App) {
        for plugin in &self.plugins {
            (*plugin)(app);
        }
    }
    fn clear(&mut self) {
        self.plugins.clear();
    }
}
pub trait AddPlugIn {
    fn add_self_as_plugin(self, app: &mut App);
}
impl<F: Fn(&mut App) + 'static> AddPlugIn for F {
    fn add_self_as_plugin(self, app: &mut App) {
        app.add_plugin(self);
    }
}
impl<F1: Fn(&mut App) + 'static, F2: Fn(&mut App) + 'static> AddPlugIn for (F1, F2) {
    fn add_self_as_plugin(self, app: &mut App) {
        self.0.add_self_as_plugin(app);
        self.1.add_self_as_plugin(app);
    }
}
impl<F1: Fn(&mut App) + 'static, F2: Fn(&mut App) + 'static, F3: Fn(&mut App) + 'static> AddPlugIn
    for (F1, F2, F3)
{
    fn add_self_as_plugin(self, app: &mut App) {
        self.0.add_self_as_plugin(app);
        self.1.add_self_as_plugin(app);
        self.2.add_self_as_plugin(app);
    }
}
struct UpdateInfo {
    is_first: bool,
    latest_update: Instant,
}
