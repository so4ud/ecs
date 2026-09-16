use crate::{
    app::{AddPlugIn, App},
    components::Component,
    ecs::EntityID,
};
use macros::Component;

/// ! add more when more default plugins are developed
pub struct Plugins(fn(&mut App, &winit::event_loop::ActiveEventLoop));
impl Default for Plugins {
    fn default() -> Self {
        Self(crate::plugins::wgpu_plugin::wgpu_plugin)
    }
}
impl AddPlugIn for Plugins {
    fn add_self_as_plugin(self, app: &mut App) {
        app.add_plugin_active_event_loop(self.0);
    }
}
/// `runtime` is a function that takes ownership over the `App` once `App::run()` is called, some plugins may add their own runtime.
/// If left empty then `App::run()` resorts to updating the `App` in a loop
/// If you want to add your own `Runtime`, add it as a recource
// pub struct Runtime {
//     pub runtime: Box<dyn FnMut(App)>,
// }
// impl Runtime {
//     pub fn new(f: impl FnMut(App) + 'static) -> Self {
//         Self {
//             runtime: Box::new(f),
//         }
//     }
// }

#[derive(Debug, Clone, Copy, Component)]
pub struct Parent {
    pub entity_id: EntityID,
}
#[derive(Debug, Clone, Copy, Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Debug, Clone, Copy, Component)]
pub struct Orientation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
