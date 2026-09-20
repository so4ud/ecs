use crate::{components::Component, ecs::ECS};
struct UnsafeECSCell {
    ptr: *mut ECS,
}
impl UnsafeECSCell {}
