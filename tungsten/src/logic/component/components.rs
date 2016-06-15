
use super::ComponentStorageType;

use super::*;

use std::collections::HashMap;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::any::TypeId;
use std::ops::{Deref,DerefMut};
use std::mem;

//used for unsafe as shit shit.
#[repr(C)]
pub struct TraitObject{
    pub data: *mut (),
    pub vtable: *mut (),
}

/// Gaurd for the borrowing components storage.
pub struct ComponentStorageBorrowReadGuard<'a,T:ComponentStorage>{
    borrow: &'a T,
    gaurd: BorrowMarkerGuardRead<'a>,
}

impl<'a,T:ComponentStorage> Deref for ComponentStorageBorrowReadGuard<'a,T>{
    type Target = T;

    fn deref(&self) -> &Self::Target{
        self.borrow
    }
}

/// Gaurd for the borrowing components storage mutable.
pub struct ComponentStorageBorrowWriteGuard<'a,T:ComponentStorage>{
    borrow: &'a mut T,
    gaurd: BorrowMarkerGuardWrite<'a>,
}

impl<'a,T:ComponentStorage> Deref for ComponentStorageBorrowWriteGuard<'a,T>{
    type Target = T;

    fn deref(&self) -> &Self::Target{
        self.borrow
    }
}

impl<'a,T:ComponentStorage> DerefMut for ComponentStorageBorrowWriteGuard<'a,T>{
    fn deref_mut(&mut self) -> &mut Self::Target{
        self.borrow
    }
}

/// Utility struct for making sure components are not borrowed already.
struct BorrowMarker(AtomicIsize);

//TODO fix nameing
/// Gaurd for the borrow Utility.
struct BorrowMarkerGuardRead<'a>(&'a BorrowMarker);

impl<'a> Drop for BorrowMarkerGuardRead<'a>{
    fn drop(&mut self){
        (self.0).0.fetch_sub(1,Ordering::AcqRel);
    }
}

/// Gaurd for the borrow utility mutable.
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

    /// Create a new `Components` object.
    pub fn new() -> Self {
        Components { components: HashMap::new() }
    }

    /// Registers a components so it can be request for adding 
    /// removing and borrowing.
    pub fn register<T: Component>(&mut self) {
        let id = TypeId::of::<T>();
        let data = ComponentsData{
            storage: Box::new(T::Storage::new()),
            borrow_marker: BorrowMarker::new(),
        };
        self.components.insert(id, data);
    }

    /// Add a component to a specific entity.
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

    /// remove all the components of an entity 
    pub fn remove(&mut self, entity: usize) {
        for (_, comp) in &mut self.components {
            comp.storage.remove(entity)
        }
    }

    /// Add multiple components with the same date at once.
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

    /// Remove multiple entities at once.
    pub fn remove_mul(&mut self, entity: &[usize]) {
        for (_, comp) in &mut self.components {
            for i in entity {
                comp.storage.remove(*i);
            }
        }
    }

    /// Request that component container free memory.
    pub fn resize(&mut self) {
        for (_, data) in &mut self.components {
            data.storage.resize();
        }
    }

    /// Borrow component storage if it has not been mutable borrowed already.
    /// returns `None` if it could not borrow the component.
    pub fn borrow<'a,T: Component>(&'a self) -> Option<ComponentStorageBorrowReadGuard<'a,T::Storage>>{
        let id = TypeId::of::<T>();
        let borrow = self.components
            .get(&id)
            .expect("Added component before it was registered!");

        if let Some(x) = borrow.borrow_marker.borrow(){
            //downcast
            let bow: &ComponentStorageType = &(*borrow.storage);
            let r_down: &T::Storage = unsafe{
                let to: TraitObject = mem::transmute(bow);
                mem::transmute(to.data)
            };

            Some(ComponentStorageBorrowReadGuard{
                borrow: r_down,
                gaurd: x,
            })
        }else{
            None
        }

    }

    /// Borrow component storage if it has not been borrowed already.
    /// returns `None` if it could not borrow the component.
    pub fn borrow_mut<'a,T: Component>(&'a self) -> Option<ComponentStorageBorrowWriteGuard<'a,T::Storage>>{
        let id = TypeId::of::<T>();
        // TODO find a way to not transmute a const to a mut
        let borrow = self.components
            .get(&id)
            .expect("Added component before it was registered!");

        if let Some(x) = borrow.borrow_marker.borrow_mut(){
            //downcast
            let bow: &ComponentStorageType = &(*borrow.storage);
            let r_down: &mut T::Storage = unsafe{
                let to: TraitObject = mem::transmute(bow);
                mem::transmute(to.data)
            };

            Some(ComponentStorageBorrowWriteGuard{
                borrow: r_down,
                gaurd: x,
            })
        }else{
            None
        }
    }
}
