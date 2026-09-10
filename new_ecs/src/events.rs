use std::{
    any::{Any, TypeId},
    collections::{HashMap, VecDeque},
};

use macros::Event;
pub trait Event: Clone {}

#[derive(Debug)]
pub struct Events {
    pub events: VecDeque<(TypeId, Box<dyn Any>)>,
    pub next_tick_events: VecDeque<(TypeId, Box<dyn Any>)>,
}
impl Events {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            next_tick_events: VecDeque::new(),
        }
    }
    pub fn push_event<T: Event + 'static>(&mut self, event: T) {
        self.events.push_back((TypeId::of::<T>(), Box::new(event)));
    }
    pub fn push_next_tick_event<T: Event + 'static>(&mut self, event: T) {
        self.next_tick_events
            .push_back((TypeId::of::<T>(), Box::new(event)));
    }
    /// returns a reference to the latest event, if the type of the event doewsnt match with the provided generic returns `None`
    pub fn get_latest_event<T: Event + Clone + 'static>(&mut self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        if self.events.len() == 0 {
            return None;
        } else if self.events[0].0 != type_id {
            return None;
        }
        Some(self.events.get(0).unwrap().1.downcast_ref::<T>().unwrap())
    }
    pub fn swap_and_clear(&mut self) {
        std::mem::swap(&mut self.events, &mut self.next_tick_events);
        self.next_tick_events = VecDeque::new();
    }
}

#[derive(Debug, Clone, Copy, Event)]
pub(super) struct Update {
    /// time since last update
    pub(super) dv: std::time::Duration,
}
impl Update {
    pub(crate) fn as_secs(&self) -> f64 {
        self.dv.as_nanos().to_owned() as f64 / 1000_000_000.0
    }
}
#[derive(Debug, Clone, Copy, Event)]
pub(super) struct Startup {}
