use crate::ecs::{EntityID, Sigtype};

pub struct Entities {
    pub entity_info: Vec<Option<Sigtype>>,
}
impl Entities {
    pub fn new() -> Self {
        Self {
            entity_info: vec![],
        }
    }
    pub fn is_alive(&self, entity_id: EntityID) -> bool {
        if self.entity_info.len() > 0 && (entity_id == 0) {
        } else if self.entity_info.len() <= entity_id + 1 {
            return false;
        }
        match &self.entity_info[entity_id] {
            Some(_) => return true,
            None => return false,
        }
    }
    pub fn get_entity_signature(&self, entity_id: EntityID) -> Option<Sigtype> {
        if !self.is_alive(entity_id) {
            return None;
        }

        self.entity_info[entity_id].clone()
    }
}
