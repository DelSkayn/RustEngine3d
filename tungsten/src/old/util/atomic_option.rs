use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;

use std::ptr;

use std::marker::PhantomData;
///
///A structure for managing data on root. 
///Can be used to take ownership of data asyncronisly
pub struct AtomicOption<T>{
    inner: AtomicPtr<T>,
    _marker: PhantomData<T>,
}

impl<T> AtomicOption<T>{
    ///Creates a new AtomicOption
    pub fn new() -> Self{
        AtomicOption{
            inner: AtomicPtr::new(ptr::null_mut()),
            _marker: PhantomData,
        }
    }

    fn swap_inner(&self, ptr: *mut T,ord: Ordering) -> Option<Box<T>>{
        let val = self.inner.swap(ptr,ord);
        if val == ptr::null_mut(){
            None
        }else{
            Some(unsafe{Box::from_raw(val)})
        }
    }

    /// Returns wether there is no value in the option.
    pub fn is_none(&self,ord: Ordering) -> bool{
        self.inner.load(ord) == ptr::null_mut()
    }

    /// Returns wether there is some value in the option.
    pub fn is_some(&self,ord: Ordering) -> bool{
        self.inner.load(ord) != ptr::null_mut()
    }

    /// Returns the value (some or none) and places none in the option.
    pub fn take(&self,ord: Ordering) -> Option<T>{
        self.swap_inner(ptr::null_mut(),ord).map(|ptr| *ptr)
    }

    /// Returns the value (some or none) and places none in the option.
    /// Can reuse allocation.
    pub fn swap_box(&self,value: Box<T>,ord: Ordering) -> Option<Box<T>>{
        self.swap_inner(Box::into_raw(value),ord)
    }

    /// places value given into the option and returns the current.
    pub fn swap(&self,value: T,ord: Ordering) -> Option<T>{
        let b = Box::new(value);
        self.swap_inner(Box::into_raw(b),ord).map(|ptr| *ptr)
    }
}
