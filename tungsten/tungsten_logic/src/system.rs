
use task::promise::DynamicPromise;
use task::worker;

use super::component::{Components,ComponentStorage,ComponentStorageBorrowReadGuard,ComponentStorageBorrowWriteGuard};
use super::get_once::GetOnce;

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
    pub fn borrow<'a,T: ComponentStorage>(&'a self) -> ArgReadGaurd<'a,T>{
        unsafe{
            loop{
                if let Some(x) = (*self.components).borrow::<T>(){
                    return x
                }
                worker::work();
            }
        }
    }

    pub fn borrow_mut<'a,T: ComponentStorage>(&'a self) -> ArgWriteGaurd<'a,T>{
        unsafe{
            loop{
                if let Some(x) = (*self.components).borrow_mut::<T>(){
                    return x
                }
                worker::work();
            }
        }
    }
}

pub struct Schedular<'a>{
    components: &'a Components,
}


pub struct SchedularGaurd<'a>
{
    func: DynamicPromise<()>,
    _marker: PhantomData<&'a ()>,
}

impl<'a> SchedularGaurd<'a>
{
    pub fn wait(self){
        mem::drop(self);
    }

    pub fn is_done(&self) -> bool{
        self.func.done()
    }
}

impl<'a> Schedular<'a>{
    pub fn execute<F>(&mut self,func: F) -> SchedularGaurd<'a>
        where F: FnOnce(Args) + Send + 'a,
    {
        let arg = Args{ components: self.components};
        let res = unsafe{
             SchedularGaurd{
                func: DynamicPromise::new_non_static(move || func(arg)),
                _marker: PhantomData,
            }
        };
        res.func.run();
        res
    }

    pub fn w0r1run<A,F>(&mut self,func: F) 
        where F: FnOnce(&A) + Send + 'a,
              A: ComponentStorage 
    {
        let func = GetOnce::new(func);
        self.execute(move |args|{
            let a = args.borrow::<A>();
            func.get()(&a);
        });
    }

    pub fn w0r2run<A,B,F>(&mut self,func: F) 
        where F: FnOnce(&A,&B) + Send + 'a,
              A: ComponentStorage,
              B: ComponentStorage,
    {
        let func = GetOnce::new(func);
        self.execute(move |args|{
            let a = args.borrow::<A>();
            let b = args.borrow::<B>();
            func.get()(&a,&b);
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
            let a = args.borrow::<A>();
            let b = args.borrow::<B>();
            let c = args.borrow::<C>();
            func.get()(&a,&b,&c);
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
            let a = args.borrow::<A>();
            let b = args.borrow::<B>();
            let c = args.borrow::<C>();
            let d = args.borrow::<D>();
            func.get()(&a,&b,&c,&d);
        });
    }

    pub fn w1r0run<A,F>(&mut self,func: F) 
        where F: FnOnce(&mut A) + Send + 'a,
              A: ComponentStorage 
    {
        let func = GetOnce::new(func);
        self.execute(move |args|{
            let mut a = args.borrow_mut::<A>();
            // TODO: Find a better way.
            func.get()(&mut a);
        });
    }

    pub fn w1r1run<A,B,F>(&mut self,func: F) 
        where F: FnOnce(&mut A,&B) + Send + 'a,
              A: ComponentStorage,
              B: ComponentStorage,
    {
        let func = GetOnce::new(func);
        self.execute(move |args|{
            let mut a = args.borrow_mut::<A>();
            let b = args.borrow::<B>();
            // TODO: Find a better way.
            func.get()(&mut a,&b);
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
            let mut a = args.borrow_mut::<A>();
            let b = args.borrow::<B>();
            let c = args.borrow::<C>();
            func.get()(&mut a,&b,&c);
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
            let mut a = args.borrow_mut::<A>();
            let b = args.borrow::<B>();
            let c = args.borrow::<C>();
            let d = args.borrow::<D>();
            func.get()(&mut a,&b,&c,&d);
        });
    }
}

