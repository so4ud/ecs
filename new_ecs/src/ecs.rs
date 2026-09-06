use std::any::TypeId;

use crate::{
    Event,
    components::{self, Component},
    entities, events, recources,
};

pub type Sigtype = u128;
pub type EntityID = usize;

pub struct ECS {
    pub entities: entities::Entities,
    pub compoents: components::Components,
    pub recources: recources::Recources,
    pub events: events::Events,
}
impl ECS {
    pub fn new() -> Self {
        Self {
            entities: entities::Entities::new(),
            compoents: components::Components::new(),
            recources: recources::Recources::new(),
            events: events::Events::new(),
        }
    }
    pub(super) fn spawn_entity(&mut self) -> EntityID {
        for i in 0..self.entities.entity_info.len() {
            let info = self.entities.entity_info[i];
            match info {
                None => {
                    self.entities.entity_info[i] = Some(0);
                    return i;
                }
                Some(_) => (),
            }
        }
        self.entities.entity_info.push(Some(0));
        let entity_id = self.entities.entity_info.len() - 1;

        return entity_id;
    }
    pub(super) fn attach_component<T: components::Component + 'static>(
        &mut self,
        entity_id: EntityID,
        component: T,
    ) {
        let type_id = TypeId::of::<T>();
        self.compoents.initialize_component::<T>();
        let component_signature = self.compoents.type_to_sig[&type_id];

        let component_functions = self.compoents.sig_to_data.clone();
        for i in &mut self.compoents.component_lines {
            for func in &component_functions {
                (func.1.push_none)(i.1, entity_id + 1);
            }
        }

        unsafe {
            self.compoents.insert_component(component, entity_id);
        }

        match self.entities.entity_info[entity_id] {
            Option::Some(signature) => {
                self.entities.entity_info[entity_id] = Some(signature | component_signature)
            }

            Option::None => self.entities.entity_info[entity_id] = Some(component_signature),
        }
    }
    pub(super) fn pop_component<T: components::Component + 'static + Clone>(
        &mut self,
        entity_id: EntityID,
    ) -> Result<T, String> {
        self.compoents.initialize_component::<T>();
        let type_id = TypeId::of::<T>();
        let component_signature = self.compoents.type_to_sig[&type_id];
        let component;

        if self.entities.entity_info.len() < entity_id + 1 {
            return Err(String::from("trying to index entity outside lenth"));
        }

        let component_line = self
            .compoents
            .component_lines
            .get_mut(&type_id)
            .unwrap()
            .downcast_mut::<Vec<Option<T>>>()
            .unwrap();

        if component_line[entity_id].is_none() {
            return Err(String::from("entity doesnt have component"));
        }

        component = component_line[entity_id].clone().unwrap();
        component_line[entity_id] = None;

        match self.entities.entity_info[entity_id] {
            Option::Some(signature) => {
                self.entities.entity_info[entity_id] = Some(signature ^ component_signature)
            }

            Option::None => self.entities.entity_info[entity_id] = Some(0),
        }

        return Ok(component);
    }
    pub(super) fn despawn_entity(&mut self, entity_id: EntityID) -> Result<(), String> {
        if self.entities.entity_info.len() < entity_id + 1 {
            return Err(String::from("trying to index entity outside lenth"));
        }
        self.compoents.free_all_components_at_entity_id(entity_id);
        self.entities.entity_info[entity_id] = None;
        Ok(())
    }
    pub(super) fn get_component_ref<'a, T: components::Component + 'static>(
        &'a self,
        entity_id: EntityID,
    ) -> Option<&'a T> {
        let type_id = TypeId::of::<T>();
        let component_line = self
            .compoents
            .component_lines
            .get(&type_id)
            .unwrap()
            .downcast_ref::<Vec<Option<T>>>()
            .unwrap();

        let ses = component_line[entity_id].as_ref();
        return ses;
    }
    pub(super) fn get_component_mut<'a, T: components::Component + 'static>(
        &'a mut self,
        entity_id: EntityID,
    ) -> Option<&'a mut T> {
        let type_id = TypeId::of::<T>();
        let component_line = self
            .compoents
            .component_lines
            .get_mut(&type_id)
            .unwrap()
            .downcast_mut::<Vec<Option<T>>>()
            .unwrap();

        let ses = component_line[entity_id].as_mut();
        return ses;
    }
    pub fn initialize_component<T: Component + 'static>() {}
    pub(super) fn get_event<T: Event + 'static>(&mut self) -> Option<T> {
        None
    }
    pub fn push_event<T: Event + 'static>(&mut self, event: T) {
        self.events.push_event(event);
    }
}
