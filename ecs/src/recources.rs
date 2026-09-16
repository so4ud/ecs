use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct Recources {
    recourses: HashMap<TypeId, Box<dyn Any>>,
}
impl Recources {
    pub fn new() -> Self {
        Self {
            recourses: HashMap::new(),
        }
    }
    pub fn insert_recource<T: 'static>(&mut self, recource: T) {
        let type_id = TypeId::of::<T>();
        self.recourses.insert(type_id, Box::new(recource));
    }
    pub fn has_recource<T: 'static>(&self) -> bool {
        self.recourses.contains_key(&TypeId::of::<T>())
    }
    pub fn pop_recource<T: 'static>(&mut self) -> Option<T> {
        let type_id = TypeId::of::<T>();
        if self.recourses.contains_key(&type_id) != true {
            return None;
        }
        let recource = self.recourses.remove_entry(&type_id).unwrap().1;
        let recource = recource.downcast::<T>().unwrap();
        return Some(*recource);
    }
    pub fn get_recource_ref<'a, T: 'static>(&'a self) -> Option<&'a T> {
        let type_id = TypeId::of::<T>();
        if !self.has_recource::<T>() {
            return None;
        }

        let recource = self.recourses.get(&type_id).unwrap().downcast_ref::<T>();
        return recource;
    }
    pub fn get_recource_mut<'a, T: 'static>(&'a mut self) -> Option<&'a mut T> {
        let type_id = TypeId::of::<T>();
        if !self.has_recource::<T>() {
            return None;
        }

        let recource = self
            .recourses
            .get_mut(&type_id)
            .unwrap()
            .downcast_mut::<T>();
        return recource;
    }
}
