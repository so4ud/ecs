use std::any::{Any, TypeId};

use crate::{
    Event,
    components::{self, Component},
    entities, events, recources,
};

pub type Sigtype = u128;
pub type EntityID = usize;

pub struct ECS {
    pub(crate) entities: entities::Entities,
    pub(crate) compoents: components::Components,
    pub(crate) recources: recources::Recources,
    pub(crate) events: events::Events,
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
    pub fn spawn_entity(&mut self) -> EntityID {
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
    pub fn push_recource<T: 'static>(&mut self, recource: T) {
        self.recources.insert_recource(recource);
    }
    pub fn attach_component<T: components::Component + 'static>(
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
                if self.compoents.sig_to_type[func.0] == *i.0 {
                    (func.1.push_none)(i.1, entity_id + 1);
                }
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
    pub fn has_recource<T: 'static>(&self) -> bool {
        self.recources.has_recource::<T>()
    }
    pub fn pop_component<T: components::Component + 'static + Clone>(
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
    pub fn despawn_entity(&mut self, entity_id: EntityID) -> Result<(), String> {
        if self.entities.entity_info.len() < entity_id + 1 {
            return Err(String::from("trying to index entity outside lenth"));
        }
        self.compoents.free_all_components_at_entity_id(entity_id);
        self.entities.entity_info[entity_id] = None;
        Ok(())
    }
    pub fn get_component_ref<'a, T: components::Component + 'static>(
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
    pub fn get_component_mut<'a, T: components::Component + 'static>(
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
    pub fn pop_recource<T: 'static>(&mut self) -> Option<T> {
        self.recources.pop_recource()
    }
    pub fn get_recource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.recources.get_recource_mut()
    }
    pub fn get_recource_ref<T: 'static>(&self) -> Option<&T> {
        self.recources.get_recource_ref()
    }
    pub(crate) fn initialize_component<T: Component + 'static>() {}
    pub fn get_event<T: Event + 'static>(&mut self) -> Option<&T> {
        self.events.get_latest_event::<T>()
    }
    pub fn push_event<T: Event + 'static>(&mut self, event: T) {
        self.events.push_next_tick_event(event);
    }
    pub fn has_component_immut<T: Component + 'static>(&self, entity_id: EntityID) -> bool {
        if !self.entities.is_alive(entity_id) {
            return false;
        }
        let type_id = TypeId::of::<T>();
        if !self.compoents.type_to_sig.contains_key(&type_id) {
            return false;
        }
        let component_signature = self.compoents.type_to_sig[&type_id].clone();
        let entity_signature = self.entities.get_entity_signature(entity_id).unwrap();

        return (entity_signature & component_signature) == component_signature;
    }
    pub fn has_component<T: Component + 'static>(&mut self, entity_id: EntityID) -> bool {
        self.compoents.initialize_component::<T>();
        self.has_component_immut::<T>(entity_id)
    }
    pub fn iter_over_alive_entity_ids(&self) -> impl Iterator<Item = EntityID> {
        let mut pih = vec![];
        for i in 0..self.entities.entity_info.len() {
            match &self.entities.entity_info[i] {
                Some(_) => pih.push(i),
                None => (),
            }
        }

        return pih.into_iter();
    }
}

pub(crate) enum EntityGetErr {
    NoEntityAtEntityId,
}
