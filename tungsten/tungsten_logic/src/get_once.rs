
use std::sync::atomic::{AtomicBool,Ordering};
use std::cell::UnsafeCell;
use std::mem;

pub struct GetOnce<T>
{
    func: UnsafeCell<T>,
    called: AtomicBool,
}

unsafe impl<T> Send for GetOnce<T>{}
unsafe impl<T> Sync for GetOnce<T>{}

impl<T> GetOnce<T>{
    pub fn new(value: T) -> Self{
        GetOnce{
            func: UnsafeCell::new(value),
            called: AtomicBool::new(false),
        }
    }

    pub fn get(&self) -> T{
        if self.called.compare_and_swap(false,true,Ordering::AcqRel){
            panic!("get once called before");
        }
        unsafe{
            mem::replace(&mut *self.func.get(),mem::uninitialized())
        }
    }
}

impl<T> Drop for GetOnce<T>{
    fn drop(&mut self){
        if self.called.load(Ordering::Acquire){
            mem::forget(self);
        }
    }
}
