use std::{
    any::{Any, TypeId, type_name},
    collections::HashMap,
};

use sparseset::SparseSet;

use crate::ecs::{EntityID, Sigtype, U512};
pub(crate) type ComponentLine<T> = SparseSet<Option<T>>;

pub struct Components {
    /// HasMap<TypeId, Box<Vec<Option<T: Component>>>,
    pub component_lines: HashMap<TypeId, Box<dyn Any>>,
    pub type_to_sig: HashMap<TypeId, Sigtype>,
    pub sig_to_type: HashMap<Sigtype, TypeId>,
    pub sig_to_data: HashMap<Sigtype, ComponentData>,
    pub sig_offset: usize,
}
impl Components {
    pub fn new() -> Self {
        Self {
            component_lines: HashMap::new(),
            type_to_sig: HashMap::new(),
            sig_to_type: HashMap::new(),
            sig_to_data: HashMap::new(),
            sig_offset: 0,
        }
    }
    pub fn initialize_component<T: Component + 'static>(&mut self) {
        let id = TypeId::of::<T>();
        if self.type_to_sig.contains_key(&id) {
            return;
        }

        // asing the type its signature
        if self.sig_offset >= 511 {
            panic!("TOO MANY COMPONENTS");
        }
        let sig: Sigtype = U512::from(1u8) << U512::from(self.sig_offset);
        self.sig_offset += 1;
        // get metadata in order
        self.sig_to_type.insert(sig, id);
        self.type_to_sig.insert(id, sig);
        self.sig_to_data.insert(sig, ComponentData::new(T::die));

        // insert the "component line" :)
        let component_line: ComponentLine<T> = SparseSet::with_capacity(0xFFF);
        self.component_lines.insert(id, Box::new(component_line));
    }

    pub unsafe fn insert_component<T: Component + 'static>(
        &mut self,
        component: T,
        entity_id: EntityID,
    ) {
        self.initialize_component::<T>();
        let id = TypeId::of::<T>();

        let component_line = self
            .component_lines
            .get_mut(&id)
            .unwrap()
            .downcast_mut::<ComponentLine<T>>()
            .unwrap();

        if component_line.len() == entity_id {
            component_line.insert(component_line.len(), Some(component));
            return;
        }
        if component_line.capacity() < entity_id {
            eprint!("component capacity: {}\n", component_line.capacity());
            eprint!("entity id: {}\n", entity_id);
            panic!("ran out of component capcity");
        }

        component_line.insert(entity_id, Some(component));
    }
    pub fn pop_component<T: Component + Clone + 'static>(&self, entity_id: EntityID) -> Option<T> {
        let id = TypeId::of::<T>();
        let component_line = self
            .component_lines
            .get(&id)
            .unwrap()
            .downcast_ref::<ComponentLine<T>>()
            .unwrap();

        if component_line.len() < entity_id + 1 {
            return None;
        }

        component_line[entity_id].value.clone()
    }

    /// clears any owend stuff too... i hope
    pub fn free_all_components_at_entity_id(&mut self, entity_id: EntityID) {
        for i in &mut self.component_lines {
            ((self.sig_to_data[&self.type_to_sig[i.0]]).dealloc)(i.1, entity_id);
        }
    }
}

pub trait Component {
    fn die(component_line: &mut Box<dyn Any>, index: usize)
    where
        Self: Sized + 'static,
    {
        let pih = component_line
            .downcast_mut::<ComponentLine<Self>>()
            .unwrap();
        pih.insert(index, None);
    }
    // fn add_nones_to_reach_index(component_line: &mut Box<dyn Any>, index: usize)
    // where
    //     Self: Sized + 'static,
    // {
    //     // dbg!(component_line.is::<Vec<Option<Self>>>())
    //     // dbg!(type_name::<Self>());
    //     let pih = component_line.downcast_mut::<Vec<Option<Self>>>().unwrap();
    //     let len = pih.len();
    //     if len == 0 {
    //     } else if index < len - 1 {
    //         return;
    //     }

    //     for _ in 0..index - len {
    //         pih.push(None);
    //     }
    // }
}

#[derive(Debug, Clone, Copy)]
pub struct ComponentData {
    /// takes a compoenent line and an index and drops data at that addres
    pub dealloc: fn(&mut Box<dyn Any>, usize),
    // /// pushes `None` to the component line
    // pub push_none: fn(&mut Box<dyn Any>, usize),
}
impl ComponentData {
    fn new(dealloc: fn(&mut Box<dyn Any>, usize)) -> Self {
        Self { dealloc }
    }
}
