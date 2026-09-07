use std::marker::PhantomData;

use crate::ecs::ECS;
pub fn main() {}

struct SystemStorage {
    systems: Vec<Box<dyn System>>,
}
impl SystemStorage {
    fn new() -> Self {
        Self { systems: vec![] }
    }
    // fn add_system<F>(&mut self, system: F) {
    //     self.systems.push(Box::new(SystemImpl::new("gay", || {})));
    // }
}

trait System {
    fn run(&self, ecs: &mut ECS);
}

struct SystemImpl<F, Q> {
    name: &'static str,
    func: F,
    _marker: PhantomData<Q>,
}

impl<F, Q> SystemImpl<F, Q>
where
    F: Fn(&mut ECS, Q::Item) + Send + Sync + Clone + 'static,
    Q: Querry,
{
    fn new(name: &'static str, func: F) -> Self {
        Self {
            name,
            func,
            _marker: PhantomData,
        }
    }
}
impl<F, Q> System for SystemImpl<F, Q>
where
    F: Fn(&mut ECS, Q::Item) + Send + Sync + Clone + 'static,
    Q: Querry,
{
    fn run(&self, ecs: &mut ECS) {
        let data = Q::fetch(ecs);
        (self.func)(ecs, data);
    }
}

trait Querry {
    type Item;
    fn fetch(ecs: &mut ECS) -> Self::Item;
}
impl Querry for () {
    type Item = ();
    fn fetch(_: &mut ECS) -> Self::Item {
        ()
    }
}
