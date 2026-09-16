use crate::{components::Component, ecs::ECS};

// pub trait Querry {
//     type Item;
//     fn querry(ecs: &mut ECS) -> impl Iterator<Item = Self::Item>;
// }

// impl<T: Component + 'static> Querry for &T {
//     type Item = &'static T;

//     fn querry(ecs: &mut ECS) -> impl Iterator<Item = Self::Item> {

//     }
// }
// impl<T: Component + 'static> Querry for &mut T {
//     type Item = &'static mut T;
// }
