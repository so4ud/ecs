use std::{
    any::{Any, TypeId},
    collections::{HashMap, VecDeque},
    marker::PhantomData,
};

use crate::{Event, ecs::ECS, events::Update};

pub struct Systems {
    /// type id of the triger recived event
    pub systems: HashMap<TypeId, fn(&mut ECS)>,
}
impl Systems {
    pub fn new() -> Self {
        Self {
            systems: HashMap::new(),
        }
    }
}
