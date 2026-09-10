use std::marker::PhantomData;

use crate::ecs::ECS;
pub fn main() {
    let die = SystemStorage::new();
}

struct SystemStorage {
    systems: Vec<Box<dyn System>>,
}
impl SystemStorage {
    fn new() -> Self {
        Self { systems: vec![] }
    }
    fn add_system(&mut self, f: impl AddSys) {
        f.add(self);
    }
}
trait System {}
trait AddSys {
    fn add(self, s: &mut SystemStorage);
}
// impl<F: FnMut(Q) + 'static, Q: 'static> AddSys for F {
//     fn add(self, s: &mut SystemStorage) {}
// }
// impl<F: FnMut(Q::Item) + 'static, Q: Qerry + 'static> AddSys for F {
//     fn add(self, s: &mut SystemStorage) {}
// }

trait Qerry {
    type Item;
    fn fetch(ecs: &mut ECS) -> Self::Item;
}
