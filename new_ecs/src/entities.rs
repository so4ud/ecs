use crate::ecs::Sigtype;

pub struct Entities {
    pub entity_info: Vec<Option<Sigtype>>,
}
impl Entities {
    pub fn new() -> Self {
        Self {
            entity_info: vec![],
        }
    }
}
