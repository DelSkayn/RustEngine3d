use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;

use std::ptr;

use super::Game;
use super::Platform;
use super::Settings;

use super::render::RenderRoot;

///
///Root is a static data structure used throughout the engine
///for communication. Root can only be referenced as constant.
///So referies need to ensure that data can be accessed as 
///internaly mutable.
///
pub struct Root{
    /// Information about the platform the engine is running on.
    pub platform: Platform,
    /// Object of the game the engine is running.
    pub game: Box<Game>,
    /// An object used to determin if the object should continue running.
    pub running: Running, 
    /// Settings of versious things in the engine
    pub settings: Settings,
    /// Data used by rendering engine and everyone who needs to submit renderdata.
    pub render: RenderRoot,
}

///
///A structure for managing data on root. 
///Can be used to take ownership of data asyncronisly
pub struct AtomicOption<T>
    where T: Sized{
    inner: AtomicPtr<T>,
}

impl<T: Sized> AtomicOption<T>{
    ///Creates a new AtomicOption
    pub fn new() -> Self{
        AtomicOption{
            inner: AtomicPtr::new(ptr::null_mut()),
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


pub struct Running{
    interal: AtomicBool,
}

impl Running{
    fn new() -> Self{
        Running{
            interal: AtomicBool::new(true),
        }
    }

    pub fn quit(&self){
        self.interal.store(false,Ordering::Relaxed);
    }

    pub fn should(&self) -> bool{
        self.interal.load(Ordering::Relaxed)
    }
}

impl Root{
    /// creates a new root.
    pub fn new<G: Game + Sized + 'static>(game: G) -> Self{
        info!("Root created.");
        Root{
            running: Running::new(),
            game: Box::new(game),
            platform: Platform::new(),
            settings: Settings::new(),
            render: RenderRoot::new(),
        }
    }
}
