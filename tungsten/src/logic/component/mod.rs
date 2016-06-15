mod vec_storage;
pub use self::vec_storage::VecStorage;

use std::collections::HashMap;

use std::sync::atomic::{AtomicIsize, Ordering};

use std::any::{Any, TypeId};

use std::mem;

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

//used for unsafe a shit shit.
#[repr(C)]
pub struct TraitObject{
    pub data: *mut (),
    pub vtable: *mut (),
}

pub struct ComponentStorageBorrowReadGuard<'a,T:ComponentStorage>{
    borrow: &'a T,
}

pub struct ComponentStorageBorrowWriteGuard<'a,T:ComponentStorage>{
    borrow: &'a mut T,
}

struct BorrowMarker(AtomicIsize);

//TODO fixe nameing
struct BorrowMarkerGuardRead<'a>(&'a BorrowMarker);

impl<'a> Drop for BorrowMarkerGuardRead<'a>{
    fn drop(&mut self){
        (self.0).0.fetch_sub(1,Ordering::AcqRel);
    }
}

struct BorrowMarkerGuardWrite<'a>(&'a BorrowMarker);

impl<'a> Drop for BorrowMarkerGuardWrite<'a>{
    fn drop(&mut self){
        (self.0).0.store(0,Ordering::Release);
    }
}

impl BorrowMarker{
    fn new() -> Self {
        BorrowMarker(AtomicIsize::new(0))
    }

    //try to borrow the marker.
    pub fn borrow<'a>(&'a self) -> Option<BorrowMarkerGuardRead<'a>>{
        loop{
            let cur = self.0.load(Ordering::Acquire);
            if cur < 0{
                return None;
            }
            let new = cur + 1;
            if self.0.compare_and_swap(cur,new,Ordering::AcqRel) == cur{
                return Some(BorrowMarkerGuardRead(self));
            }
        }
    }

    pub fn borrow_mut<'a>(&'a self) -> Option<BorrowMarkerGuardWrite<'a>>{
        loop{
            let cur = self.0.load(Ordering::Acquire);
            if cur > 0{
                return None;
            }
            if self.0.compare_and_swap(cur,-1,Ordering::AcqRel) == cur{
                return Some(BorrowMarkerGuardWrite(self));
            }
        }
    }
}

pub struct ComponentsData{
    storage: Box<ComponentStorageType>,
    borrow_marker: BorrowMarker,
}

pub struct Components {
    components: HashMap<TypeId, ComponentsData>,
}

impl Components {
    pub fn new() -> Self {
        Components { components: HashMap::new() }
    }

    pub fn register<T: Component>(&mut self) {
        let id = TypeId::of::<T>();
        let data = ComponentsData{
            storage: Box::new(T::Storage::new()),
            borrow_marker: BorrowMarker::new(),
        };
        self.components.insert(id, data);
    }

    pub fn add<T: Component>(&mut self, entity: usize, value: T) {
        let id = TypeId::of::<T>();
        let borrow = self.components
            .get_mut(&id)
            .expect("Added component before it was registered!");

        //downcast
        let bow: &mut ComponentStorageType = &mut(*borrow.storage);
        let r_down: &mut T::Storage = unsafe{
            let to: TraitObject = mem::transmute(bow);
            mem::transmute(to.data)
        };

        r_down.insert(entity, value);
    }

    pub fn remove(&mut self, entity: usize) {
        for (_, comp) in &mut self.components {
            comp.storage.remove(entity)
        }
    }

    pub fn add_mul<T: Component + Clone>(&mut self, entity: &[usize], value: T) {
        let id = TypeId::of::<T>();
        let borrow = self.components
            .get_mut(&id)
            .expect("Added component before it was registered!");

        //downcast
        let bow: &mut ComponentStorageType = &mut(*borrow.storage);
        let r_down: &mut T::Storage = unsafe{
            let to: TraitObject = mem::transmute(bow);
            mem::transmute(to.data)
        };

        for i in entity {
            r_down.insert(*i, value.clone());
        }
    }

    pub fn remove_mul(&mut self, entity: &[usize]) {
        for (_, comp) in &mut self.components {
            for i in entity {
                comp.storage.remove(*i);
            }
        }
    }

    pub fn resize(&mut self) {
        for (_, data) in &mut self.components {
            data.storage.resize();
        }
    }

    pub fn borrow<'a,T: Component>(&'a self) -> Option<ComponentStorageBorrowReadGuard<'a,T::Storage>>{
        unimplemented!();
    }

    pub fn borrow_mut<'a,T: Component>(&'a self) -> Option<ComponentStorageBorrowWriteGuard<'a,T::Storage>>{
        unimplemented!();
    }
}
