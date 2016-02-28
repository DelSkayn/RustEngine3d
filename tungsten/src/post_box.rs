use std::sync::atomic::AtomicUsize;
use std::cell::UnsafeCell;

use std::mem;

const NOTHING: usize = 0;
const DATA: usize = 1;
const END: usize = 2;
const YEILD: usize = 3;

/*
const NOTHING: usize = 0;
const NOTHING: usize = 0;
*/

pub struct PostBox<T:Sized>{
    state: AtomicUsize,
    value: UnsafeCell<T>,
}

impl<T:Sized> PostBox<T>{
    fn new() -> Self{
        let value = UnsafeCell::new(mem::uninitialized());
        PostBox{
            state: AtomicUsize::new(NOTHING),
            value: value,
        }
    }

    fn is_loaded(&self) -> bool{
        state.load(Ordering::Relaxed) == DATA
    }

    fn is_free(&self) -> bool{
        state.load(Ordering::Relaxed) == NOTHING
    }

    fn get(&self) -> Option<T>{
        let state = state.load(Ordering::Relaxed);
        if state != DATA{
            return None;
        }
        unsafe{
            let val = self.value.into_inner();
        }
        self.state.store(NOTHING,Ordering::Relaxed);
        val
    }

    fn set(&self,value: T){
        if self.state.load(Ordering::Relaxed) != NOTHING{
            panic!("Would overwrite data");
        }
        self.value.get() = value;
        self.state.store(DATA,Ordering::Relaxed);
    }
}
