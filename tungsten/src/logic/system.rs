
use super::component::{
    Components,
    ComponentStorage,
    ComponentStorageBorrowReadGuard,
    ComponentStorageBorrowWriteGuard,
};

use task::{self,DynTaskImpl,ArcLatch,Latch};

use std::mem;
use std::cell::UnsafeCell;
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
            task::push_unchecked(temp.as_ref().unwrap());
            temp
        };
        res
    }

    fn w1r0run<A,F>(&mut self,func: F) 
        where F: FnOnce(&mut A) + Send + 'a,
              A: ComponentStorage 
    {
        let func_opt = UnsafeCell::new(Some(func));
        self.execute(move |args|{
            let mut a = if let Some(x) = args.borrow_mut::<A>(){ x }else{return true;};
            // TODO: Find a better way.
            unsafe{
                (*func_opt.get()).take().unwrap()(&mut *a);
            }
            true
        });
    }

}

