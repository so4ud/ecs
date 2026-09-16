use std::marker::PhantomData;

use crate::ecs::ECS;
pub fn main() {
    let mut die = SystemStorage::new();
    die.add_system(|_: ()| {});
}

struct SystemStorage {
    systems: Vec<Box<dyn System>>,
}
impl SystemStorage {
    fn new() -> Self {
        Self { systems: vec![] }
    }
    fn add_system<F: IntoBoxedSys<F, Q> + FnMut(Q::Item) + 'static, Q: Qerry + 'static>(
        &mut self,
        f: F,
    ) {
        let ses: BoxedSys<F, Q> = f.into_boxed();
        self.systems.push(Box::new(ses));
    }
}
trait System {
    fn run(&mut self, ecs: &mut ECS);
}
trait IntoBoxedSys<F, Q> {
    fn into_boxed(self) -> BoxedSys<F, Q>;
}
impl<F: FnMut(Q), Q: Qerry> IntoBoxedSys<F, Q> for F {
    fn into_boxed(self) -> BoxedSys<F, Q> {
        BoxedSys {
            f: self,
            _p: PhantomData,
        }
    }
}
impl<F: FnMut(Q::Item), Q: Qerry> System for BoxedSys<F, Q> {
    fn run(&mut self, ecs: &mut ECS) {
        (self.f)(Q::fetch(ecs));
    }
}

struct BoxedSys<F, Q> {
    f: F,
    _p: PhantomData<Q>,
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

impl Qerry for () {
    type Item = ();
    fn fetch(ecs: &mut ECS) -> Self::Item {
        ()
    }
}
