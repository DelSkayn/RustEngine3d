
use super::component::{
    Components,
    ComponentStorage,
    ComponentStorageBorrowReadGuard,
    ComponentStorageBorrowWriteGuard,
};
use super::get_once::GetOnce;

use task::{self,DynTaskImpl,ArcLatch,Latch};

use std::mem;
use std::marker::PhantomData;

pub type ArgReadGaurd<'a,T> = ComponentStorageBorrowReadGuard<'a,T>;
pub type ArgWriteGaurd<'a,T> = ComponentStorageBorrowWriteGuard<'a,T>;

/// Struct for handeling system data needs.
/// TODO: Test wether this allows for unsafe code.
#[derive(Clone)]
pub struct Args{
    components: *const Components,
}

unsafe impl Send for Args{}

impl Args{
    pub fn borrow<'a,T: ComponentStorage>(&'a self) -> Option<ArgReadGaurd<'a,T>>{
        unsafe{
            (*self.components).borrow::<T>()
        }
    }

    pub fn borrow_mut<'a,T: ComponentStorage>(&'a self) -> Option<ArgWriteGaurd<'a,T>>{
        unsafe{
            (*self.components).borrow_mut::<T>()
        }
    }
}

pub struct Schedular<'a>{
    components: &'a Components,
}


pub struct SchedularGaurd<'a>
{
    func: Option<DynTaskImpl<ArcLatch,()>>,
    latch: ArcLatch,
    _marker: PhantomData<&'a ()>
}

impl<'a> SchedularGaurd<'a>
{
    pub fn wait(self){
        mem::drop(self);
    }

    pub fn is_done(&self) -> bool{
        self.latch.done()
    }
}

impl<'a> Drop for SchedularGaurd<'a>
{
    fn drop(&mut self) {
        while !self.latch.done(){
            task::steal();
        }
        // Propagate posible panic.
        unsafe{
            self.func.take().unwrap().into_result();
        }
    }
}

impl<'a> Schedular<'a>{
    pub fn execute<F>(&mut self,func: F) -> SchedularGaurd<'a>
        where F: FnMut(Args) -> bool + Send + 'a,
    {
        let mut res = SchedularGaurd{
            func: None,
            latch: ArcLatch::new(),
            _marker: PhantomData,
        };
        let arg = Args{ components: self.components};
        res.func = unsafe{
            let temp = Some(DynTaskImpl::new(res.latch.clone(),move || {
                    let mut func = func;
                    while !func(arg.clone()){
                        task::steal();
                    }
            }));
            task::push_global(temp.as_ref().unwrap());
            temp
        };
        res
    }

    pub fn w0r1run<A,F>(&mut self,func: F) 
        where F: FnOnce(&A) + Send + 'a,
              A: ComponentStorage 
    {
        let func = GetOnce::new(func);
        self.execute(move |args|{
            let a = if let Some(x) = args.borrow::<A>(){ x }else{return false;};
            func.get()(&a);
            true
        });
    }

    pub fn w0r2run<A,B,F>(&mut self,func: F) 
        where F: FnOnce(&A,&B) + Send + 'a,
              A: ComponentStorage,
              B: ComponentStorage,
    {
        let func = GetOnce::new(func);
        self.execute(move |args|{
            let a = if let Some(x) = args.borrow::<A>(){ x }else{return false;};
            let b = if let Some(x) = args.borrow::<B>(){ x }else{return false;};
            func.get()(&a,&b);
            true
        });
    }

    pub fn w0r3run<A,B,C,F>(&mut self,func: F) 
        where F: FnOnce(&A,&B,&C) + Send + 'a,
              A: ComponentStorage,
              B: ComponentStorage,
              C: ComponentStorage,
    {
        let func = GetOnce::new(func);
        self.execute(move |args|{
            let a = if let Some(x) = args.borrow::<A>(){ x }else{return false;};
            let b = if let Some(x) = args.borrow::<B>(){ x }else{return false;};
            let c = if let Some(x) = args.borrow::<C>(){ x }else{return false;};
            func.get()(&a,&b,&c);
            true
        });
    }

    pub fn w0r4run<A,B,C,D,F>(&mut self,func: F) 
        where F: FnOnce(&A,&B,&C,&D) + Send + 'a,
              A: ComponentStorage,
              B: ComponentStorage,
              C: ComponentStorage,
              D: ComponentStorage,
    {
        let func = GetOnce::new(func);
        self.execute(move |args|{
            let a = if let Some(x) = args.borrow::<A>(){ x }else{return false;};
            let b = if let Some(x) = args.borrow::<B>(){ x }else{return false;};
            let c = if let Some(x) = args.borrow::<C>(){ x }else{return false;};
            let d = if let Some(x) = args.borrow::<D>(){ x }else{return false;};
            func.get()(&a,&b,&c,&d);
            true
        });
    }

    pub fn w1r0run<A,F>(&mut self,func: F) 
        where F: FnOnce(&mut A) + Send + 'a,
              A: ComponentStorage 
    {
        let func = GetOnce::new(func);
        self.execute(move |args|{
            let mut a = if let Some(x) = args.borrow_mut::<A>(){ x }else{return false;};
            // TODO: Find a better way.
            func.get()(&mut a);
            true
        });
    }

    pub fn w1r1run<A,B,F>(&mut self,func: F) 
        where F: FnOnce(&mut A,&B) + Send + 'a,
              A: ComponentStorage,
              B: ComponentStorage,
    {
        let func = GetOnce::new(func);
        self.execute(move |args|{
            let mut a = if let Some(x) = args.borrow_mut::<A>(){ x }else{return false;};
            let b = if let Some(x) = args.borrow::<B>(){ x }else{return false;};
            // TODO: Find a better way.
            func.get()(&mut a,&b);
            true
        });
    }

    pub fn w1r2run<A,B,C,F>(&mut self,func: F) 
        where F: FnOnce(&mut A,&B,&C) + Send + 'a,
              A: ComponentStorage,
              B: ComponentStorage,
              C: ComponentStorage,
    {
        let func = GetOnce::new(func);
        self.execute(move |args|{
            let mut a = if let Some(x) = args.borrow_mut::<A>(){ x }else{return false;};
            let b = if let Some(x) = args.borrow::<B>(){ x }else{return false;};
            let c = if let Some(x) = args.borrow::<C>(){ x }else{return false;};
            // TODO: Find a better way.
            func.get()(&mut a,&b,&c);
            true
        });
    }

    pub fn w1r3run<A,B,C,D,F>(&mut self,func: F) 
        where F: FnOnce(&mut A,&B,&C,&D) + Send + 'a,
              A: ComponentStorage,
              B: ComponentStorage,
              C: ComponentStorage,
              D: ComponentStorage,
    {
        let func = GetOnce::new(func);
        self.execute(move |args|{
            let mut a = if let Some(x) = args.borrow_mut::<A>(){ x }else{return false;};
            let b = if let Some(x) = args.borrow::<B>(){ x }else{return false;};
            let c = if let Some(x) = args.borrow::<C>(){ x }else{return false;};
            let d = if let Some(x) = args.borrow::<D>(){ x }else{return false;};
            // TODO: Find a better way.
            func.get()(&mut a,&b,&c,&d);
            true
        });
    }
}

