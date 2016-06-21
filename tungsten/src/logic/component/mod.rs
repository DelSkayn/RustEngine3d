mod vec_storage;
pub use self::vec_storage::VecStorage;

mod components;
pub use super::component::components::{
    Components,
    ComponentStorageBorrowReadGuard,
    ComponentStorageBorrowWriteGuard,
};

use std::any::Any;

/// Component trait 
/// All components must implement this type in order 
/// to be used by an entity
pub trait Component: Sized + Any + Sync{
    /// The storage type this component will be stored in.
    type Storage: ComponentStorage<Comp = Self>;
}

/// A trait defining component storage type.
pub trait ComponentStorage: Any + Sync{
    type Comp: Component;

    /// Create a new type.
    fn new() -> Self;

    /// Insert a new component for entity.
    /// Will only be called if there is no existing component for the given entity.
    fn insert(&mut self, entity: usize, value: Self::Comp);

    /// return a reference to the component of a given entity.
    fn get(&self, entity: usize) -> Option<&Self::Comp>;

    /// return a mutable reference to the component of a given entity.
    fn get_mut(&mut self, entity: usize) -> Option<&mut Self::Comp>;

    /// remove the component of a given entity.
    /// might be called if there is no component.
    fn remove(&mut self, entity: usize);

    /// Called when freeing of memory is requested.
    /// Usefull when the storage is implemented as a vector so it can try to shrink.
    fn resize(&mut self);
}

// for remove entities without knowing the type.
// Basicly a translation to a virtual type.
trait ComponentStorageType: Any + Sync{
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

