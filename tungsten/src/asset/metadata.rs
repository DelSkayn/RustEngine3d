extern crate crossbeam;

use self::crossbeam::mem::epoch::{self,Atomic,Owned};

use std::sync::atomic::Ordering;

use std::any::{Any,TypeId};
use std::mem;

//used for unsafe as shit shit.
#[repr(C)]
pub struct TraitObject{
    pub data: *mut (),
    pub vtable: *mut (),
}

pub struct Container(Atomic<Box<MetaData>>);

impl Container{
    pub fn new() -> Self{
        Container(Atomic::null())
    }

    pub fn change<T: MetaData>(&self,data: T){
        if !self.is_owned(){
            panic!("Tried to change metadata with data still in place");
        }
        let data: Owned<Box<MetaData>> = Owned::new(Box::new(data));
        self.0.store(Some(data),Ordering::Release);
    }

    pub fn free(&self){
        self.0.store(None,Ordering::Release);
    }

    pub fn get<T: MetaData,F: FnOnce(&T)>(&self,func: F){
        let gaurd = epoch::pin();
        let data: &Box<MetaData> = *self.0.load(Ordering::Acquire,&gaurd).unwrap();
        // WHY?? (;>_<)
        let why: &MetaData = &(**data);
        func(into(why));
    }

    pub fn is_owned(&self) -> bool{
        self.0.cas(None,None,Ordering::Acquire).is_ok()
    }

}

pub trait MetaData: Any + Sync{

    #[doc(hidden)]
    fn id(&self) -> TypeId{
        TypeId::of::<Self>()
    }
}

fn into<'a,T: Any>(this: &MetaData) -> &'a T{
    if this.id() != TypeId::of::<T>(){
        panic!("Tried to confert metadata to wrong type.");
    }
    unsafe{
        let t: TraitObject = mem::transmute(this);
        mem::transmute(t.data)
    }
}


