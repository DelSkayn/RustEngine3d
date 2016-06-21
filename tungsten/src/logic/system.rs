
use super::Component;
use super::component::{
    Components,
    ComponentStorageBorrowReadGuard,
    ComponentStorageBorrowWriteGuard,
};

use task;

use std::sync::atomic::{AtomicUsize, Ordering};

use std::cell::{Cell,UnsafeCell};
use std::sync::Arc;

pub type ArgReadGaurd<'a,T> = ComponentStorageBorrowReadGuard<'a,T>;
pub type ArgWriteGaurd<'a,T> = ComponentStorageBorrowWriteGuard<'a,T>;

/// Struct for handeling system data needs.
#[derive(Clone)]
pub struct Args<'a>{
    components: &'a Components,
}

impl<'a> Args<'a>{
    fn borrow<T: Component>(&'a self) -> Option<ArgReadGaurd<'a,T::Storage>>{
        self.components.borrow::<T>()
    }

    fn borrow_mut<T: Component>(&self) -> Option<ArgWriteGaurd<'a,T::Storage>>{
        self.components.borrow_mut::<T>()
    }
}

pub trait System: Sync + 'static {
    /// Executes an system returns wether the system was properly
    /// executed or needs to be executed again.
    /// If system was not properly executed the state of the world was not changed,
    fn execute<'a>(&'a mut self,arg: Args<'a>) -> bool;
}

pub struct SinkSystemInner{
    system: UnsafeCell<Box<System>>,
    count: Cell<usize>,
    dep_count: AtomicUsize,
}

unsafe impl Sync for SinkSystemInner{}
unsafe impl Send for SinkSystemInner{}

pub struct SinkSystem(Arc<SinkSystemInner>);

impl SinkSystem{
    fn new<S: System>(s: S) -> Self{
        SinkSystem(Arc::new(
                SinkSystemInner{
                    system: UnsafeCell::new(Box::new(s)),
                    count: Cell::new(1),
                    dep_count: AtomicUsize::new(1),
                }
                ))
    }
}

impl Clone for SinkSystem{
    fn clone(&self) -> Self{
        let cur = self.0.count.get();
        self.0.count.set(cur + 1);
        self.0.dep_count.store(self.0.count.get(),Ordering::Release);
        SinkSystem(self.0.clone())
    }
}

impl System for SinkSystem{
    fn execute<'a>(&'a mut self,arg: Args<'a>) -> bool{
        if self.0.dep_count.load(Ordering::Acquire) == 0{
            let res = unsafe{ (*self.0.system.get()).execute(arg) };
            if res {
                self.0.dep_count.store(self.0.count.get(),Ordering::Release);
            }
            res
        }else if self.0.dep_count.fetch_sub(1,Ordering::AcqRel) == 1{
            // each parent has completed and executed there childs.
            let res = unsafe{ (*self.0.system.get()).execute(arg) };
            if res {
                self.0.dep_count.store(self.0.count.get(),Ordering::Release);
            }
            res
        }else{
            true
        }
    }
}



#[derive(Debug,Copy,Clone,Eq,PartialEq,Ord,PartialOrd)]
pub struct SystemId(usize);

pub trait IntoSystemData{
    fn into_system_data(self) -> SystemData;
}

impl<S: System> IntoSystemData for S{
    fn into_system_data(self) -> SystemData{
        SystemData{
            system: UnsafeCell::new(Box::new(self)),
            childs: None,
        }
    }
}

impl IntoSystemData for SystemBuilder{
    fn into_system_data(self) -> SystemData{
        self.0
    }
}

pub struct SystemData{
    system: UnsafeCell<Box<System>>,
    childs: Option<Vec<SystemData>>,
}

unsafe impl Sync for SystemData{}

pub struct SystemBuilder(SystemData);

impl SystemBuilder{
    pub fn new<S: System>(system: S) -> Self{
        SystemBuilder(
            SystemData{
                system: UnsafeCell::new(Box::new(system)),
                childs: None,
            })
    }

    pub fn child<S: IntoSystemData>(&mut self,s: S){
        if let None = self.0.childs{
            self.0.childs = Some(Vec::new());
        }
        self.0.childs.as_mut().unwrap().push(s.into_system_data());
    }
}

struct Systems{
    systems: Vec<SystemData>,
}

impl Systems{
    pub fn new() -> Self{
        Systems{
            systems: Vec::new(),
        }
    }

    pub fn add<S: IntoSystemData>(&mut self,system: S){
        self.systems.push(system.into_system_data());
    }

    pub fn execute(&mut self,components: &Components){
        schedule(&self.systems,Args{
            components: components
        });
    }
}

fn schedule<'a>(systems: &[SystemData],arg: Args<'a>){
    if systems.len() == 1{
        unsafe{
            while !(*systems[0].system.get()).execute(arg.clone()){}
        }
        if let Some(x) = systems[0].childs.as_ref(){
            schedule(x,arg);
        }

    }else{
        let mid = systems.len() / 2;
        let (fir,sec) = systems.split_at(mid);
        let arg_c = arg.clone();
        task::join(||{
            schedule(fir,arg_c);
        }
        ,||{
            schedule(sec,arg);
        });

    }
}

