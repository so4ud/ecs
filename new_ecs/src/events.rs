use std::{
    any::{Any, TypeId},
    collections::{HashMap, VecDeque},
};

use macros::Event;
pub trait Event: Clone {}

pub struct Events {
    pub events: VecDeque<(TypeId, Box<dyn Any>)>,
}
impl Events {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
        }
    }
    pub fn push_event<T: Event + 'static>(&mut self, event: T) {
        self.events.push_back((TypeId::of::<T>(), Box::new(event)));
    }
    pub fn get_latest_event<T: Event + 'static>(&mut self) -> Option<T> {
        None
    }
}

#[derive(Debug, Clone, Copy, Event)]
pub(super) struct Update {
    /// time since last update
    pub(super) dv: std::time::Duration,
}
