use crate::{
    app::{AddPlugIn, App},
    plugins::wgpu_plugin::wgpu_plugin,
};

pub mod wgpu_plugin;

pub struct Plugins(fn(&mut App));

impl Default for Plugins {
    fn default() -> Self {
        Self(wgpu_plugin)
    }
}

impl AddPlugIn for Plugins {
    fn add_self_as_plugin(self, app: &mut App) {
        self.0.add_self_as_plugin(app);
    }
}
