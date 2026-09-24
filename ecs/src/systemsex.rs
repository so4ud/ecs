use std::any::Any;

fn main() {}

struct Storage {
    infos: Vec<usize>,
    systems: Vec<Box<dyn Any>>,
}
impl Storage {}

trait Querry {
    type Item;

    fn fetch() -> Self::Item;
}

trait System {}
impl<Q> System for fn(Q) {}
