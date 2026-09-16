use std::{
    any::{Any, TypeId},
    time::Instant,
};

use crate::{
    ecs::ECS,
    events::{Event, Startup, Update, WindowResized},
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
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        event_loop.run_app(&mut self).unwrap();
    }
    /// plugins are ran in the order that they are added
    pub fn add_plugin<F: Fn(&mut App) + 'static>(&mut self, plugin: F) {
        self.plugins.plugins.push((Box::new(plugin), false));
    }
    pub fn add_plugin_active_event_loop(
        &mut self,
        plugin: fn(&mut App, &winit::event_loop::ActiveEventLoop),
    ) {
        self.plugins.plugins.push((Box::new(plugin), true));
    }
    pub fn add_plugins<T: AddPlugIn>(&mut self, plugins: T) {
        plugins.add_self_as_plugin(self);
    }
    /// systems are ran in the order that they are added
    pub(super) fn add_system<TriggerEvent: Event + 'static>(&mut self, system: fn(&mut ECS)) {
        let type_id = TypeId::of::<TriggerEvent>();
        self.systems.systems.push((type_id, system));
    }
    pub fn run_plugins(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let mut new_plugins = Plugins { plugins: vec![] };
        std::mem::swap(&mut self.plugins, &mut new_plugins);
        new_plugins.run(self, event_loop);
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
        let mut event_type_ids: Vec<TypeId> = self.ecs.events.events.iter().map(|i| i.0).collect();

        for i in &event_type_ids {
            for sys in &self.systems.systems {
                if i == &sys.0 {
                    (sys.1)(&mut self.ecs);
                }
            }
            self.ecs.events.events.pop_front();
        }
        self.ecs.events.swap_and_clear();
    }
}

enum AppErr {}

struct Plugins {
    plugins: Vec<(Box<dyn Any>, bool)>,
}
impl Plugins {
    fn new() -> Self {
        Self { plugins: vec![] }
    }
    fn run(&self, app: &mut App, event_loop: &winit::event_loop::ActiveEventLoop) {
        for plugin in &self.plugins {
            if plugin.1 == false {
                let plug = plugin.0.downcast_ref::<fn(&mut App)>().unwrap();
                plug(app);
            } else {
                let plug = plugin
                    .0
                    .downcast_ref::<fn(&mut App, &winit::event_loop::ActiveEventLoop)>()
                    .unwrap();
                plug(app, event_loop);
            }
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
// impl<T: Fn(&mut App, &winit::event_loop::ActiveEventLoop) + 'static> AddPlugIn for T {
//     fn add_self_as_plugin(self, app: &mut App) {
//         app.add_plugin_active_event_loop(self);
//     }
// }
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
