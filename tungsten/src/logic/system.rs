use task;

use super::Component;
use super::component::{
    Components,
    ComponentStorageBorrowReadGuard,
    ComponentStorageBorrowWriteGuard,
};

use std::cell::UnsafeCell;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize,Ordering};

pub type ArgReadGaurd<'a,T> = ComponentStorageBorrowReadGuard<'a,T>;
pub type ArgWriteGaurd<'a,T> = ComponentStorageBorrowWriteGuard<'a,T>;

/// Struct for handeling system data needs.
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

pub trait System: 'static{
    /// Executes an system returns wether the system was properly
    /// executed or needs to be executed again.
    /// If system was not properly executed the state of the world was not changed,
    fn execute<'a>(&'a mut self,arg: Args<'a>) -> bool;
}

#[derive(Copy,Clone,Eq,PartialEq,Ord,PartialOrd)]
pub struct SystemId(u64);

#[derive(Clone)]
pub struct DepCount(Arc<AtomicUsize>);

impl DepCount{
    fn new() -> Self{
        DepCount(Arc::new(AtomicUsize::new(0)))
    }

    fn done(&self) -> bool{
        self.0.load(Ordering::Acquire) == 0
    }

    fn finish(&self){
        self.0.fetch_sub(1,Ordering::AcqRel);
    }

    fn set(&mut self,count: usize){
        self.0.store(count,Ordering::Release);
    }
}

pub struct SystemData{
    system: UnsafeCell<Box<System>>,
    from: Vec<DepCount>,
    deps: Option<DepCount>,
    deps_amount: usize,
    id: SystemId,
}

impl SystemData{
    fn new(id: SystemId,system: Box<System>) -> Self{
        SystemData{
            system: UnsafeCell::new(system),
            from: Vec::new(),
            deps: None,
            deps_amount: 0,
            id: id
        }
    }

    fn ready(&self) -> bool{
        if let Some(ref d) = self.deps{
            d.done()
        }else{
            true
        }
    }

    unsafe fn execute<'a>(&'a self,arg: Args<'a>) -> bool{
        (*self.system.get()).execute(arg)
    }

    fn reset(&mut self){
        if let Some(ref mut deps) = self.deps{
            deps.set(self.deps_amount);
        }
    }

    fn add_dependency_to(&mut self) -> DepCount{
        if self.deps.is_none(){
            self.deps = Some(DepCount::new());
        }
        self.deps_amount += 1;
        self.deps.as_mut().unwrap().set(self.deps_amount);
        self.deps.as_ref().unwrap().clone()
    }

    fn add_dependency_from(&mut self,dc: DepCount){
        self.from.push(dc);
    }
}

pub struct Systems{
    systems: Vec<SystemData>,
    next: u64,
    sorted: bool,
}

#[derive(Debug)]
enum Error{
    IdNotFound(SystemId)
}

impl Systems{
    pub fn new() -> Self{
        Systems{
            systems: Vec::new(),
            next: 0,
            sorted: true,
        }
    }

    pub fn add<T: System>(&mut self,sys: T) -> SystemId{
        self.sorted = false;
        let res = self.next;
        self.next += 1;
        self.systems.push(SystemData::new(SystemId(res),Box::new(sys)));
        SystemId(res)
    }

    /// Create a dependency of one system on an other.
    /// where from repesents the system which needs to be executed before the to.
    pub fn dependency(&mut self,to: SystemId,from: SystemId) -> Result<(),Error>{
        if !self.sorted{
            self.systems.sort_by(|ref a,ref b| a.id.cmp(&b.id));
        }
        let to_index = try!(self.systems.binairy_search_by(|ref a| a.id.cmp(&to))
            .map_err(|_| Error::IdNotFound(to)));
        let from_index = try!(self.systems.binairy_search_by(|ref a| a.id.cmp(&from))
            .map_err(|_| Error::IdNotFound(from))); 
        let dep = self.systems[to_index].add_dependency_to();
        self.systems[from_index].add_dependency_from(dep);
    }

    pub fn execute(&mut self,componets: &Components){
        
        unimplemented!();
    }
}
