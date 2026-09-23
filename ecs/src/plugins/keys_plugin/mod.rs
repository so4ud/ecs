use std::collections::HashMap;

use crate::{app::App, events::Event};
use macros::Event;
use winit::keyboard::Key;

pub(crate) fn keys_plugin(app: &mut App) {
    let held_keys = HeldKeys::new();
    app.ecs.insert_recource(held_keys);
}

#[derive(Debug, Clone, Event)]
pub struct KeyboardInput {
    pub key: winit::keyboard::Key,
    pub is_pressed: bool,
}
#[derive(Debug, Copy, Clone, Event)]
pub struct MouseMotion {
    pub delta: (f64, f64),
}

pub struct HeldKeys {
    pub(crate) held_keys: HashMap<Key, bool>,
}
impl HeldKeys {
    pub(crate) fn new() -> Self {
        Self {
            held_keys: HashMap::new(),
        }
    }
    pub fn is_pressed(&self, key: &Key) -> bool {
        if !self.held_keys.contains_key(&key) {
            return false;
        }
        self.held_keys[&key]
    }
    pub fn press_key(&mut self, key: &Key) {
        self.held_keys.insert(key.clone(), true);
    }
    pub fn release_key(&mut self, key: &Key) {
        self.held_keys.insert(key.clone(), false);
    }
}
