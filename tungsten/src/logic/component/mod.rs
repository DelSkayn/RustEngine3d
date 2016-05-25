
mod vec_storage;
pub use self::vec_storage::VecStorage;

use std::collections::HashMap;

use std::any::{Any, TypeId};

pub trait Component: Sized + Any {
    type Storage: ComponentStorage<Comp = Self>;
}

pub trait ComponentStorage: Any {
    type Comp: Component;

    fn new() -> Self;

    fn insert(&mut self, entity: usize, value: Self::Comp);

    fn get(&self, entity: usize) -> Option<&Self::Comp>;

    fn get_mut(&mut self, entity: usize) -> Option<&mut Self::Comp>;

    fn remove(&mut self, entity: usize);

    fn resize(&mut self);
}

// for remove entities without
// knowing the type
// Basicly a translation to a virtual type.
trait ComponentStorageType: Any {
    fn remove(&mut self, entity: usize);

    fn resize(&mut self);
}

impl<T: ComponentStorage> ComponentStorageType for T {
    fn remove(&mut self, entity: usize) {
        self.remove(entity);
    }

    fn resize(&mut self) {
        self.resize()
    }
}

pub struct Components {
    components: HashMap<TypeId, Box<ComponentStorageType>>,
}

impl Components {
    pub fn new() -> Self {
        Components { components: HashMap::new() }
    }

    pub fn register<T: Component>(&mut self) {
        let id = TypeId::of::<T>();
        let data = Box::new(T::Storage::new());
        self.components.insert(id, data);
    }

    pub fn add<T: Component>(&mut self, entity: usize, value: T) {
        let id = TypeId::of::<T>();
        let r: &mut Any = self.components
            .get_mut(&id)
            .expect("Added component before it was registered!");
        let r_down = r.downcast_mut::<T::Storage>().unwrap();
        r_down.insert(entity, value);
    }

    pub fn remove(&mut self, entity: usize) {
        for (_, comp) in &mut self.components {
            comp.remove(entity)
        }
    }

    pub fn add_mul<T: Component + Clone>(&mut self, entity: &[usize], value: T) {
        let id = TypeId::of::<T>();
        let r: &mut Any = self.components
            .get_mut(&id)
            .expect("Added component before it was registered!");
        let r_down = r.downcast_mut::<T::Storage>().unwrap();
        for i in entity {
            r_down.insert(*i, value.clone());
        }
    }

    pub fn remove_mul(&mut self, entity: &[usize]) {
        for (_, comp) in &mut self.components {
            for i in entity {
                comp.remove(*i);
            }
        }
    }

    pub fn resize(&mut self) {
        for (_, data) in &mut self.components {
            data.resize();
        }
    }
}
