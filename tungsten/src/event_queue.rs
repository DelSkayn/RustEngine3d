use std::sync::atomic::{
    AtomicUsize,
    Ordering
};

use std::mem;
use std::ops::Index;

use std::cell::UnsafeCell;

use super::Event;

static MAX_EVENTS:usize = 1024;


pub struct EventQueue{
    events: UnsafeCell<[Event; 1024]>,
    size: AtomicUsize,
    first: AtomicUsize,
}

impl EventQueue{
    pub fn new() -> Self{
        let event;
        unsafe{
            event = mem::uninitialized();
        }
        EventQueue{
            events: UnsafeCell::new(event),
            size: AtomicUsize::new(0),
            first: AtomicUsize::new(0),
        }
    }

    ///
    /// Pushes an new event onto the queue.
    /// The event wil be placed at the front of the queue.
    /// When the queue is already fill it will warn and override an event.
    ///
    pub fn push(&self,event: Event){
        loop{
            let size = self.size.load(Ordering::AcqRel);
            if size == MAX_EVENTS{
                warn!("Tried to push to full event queue");
                let index = self.first.fetch_add(1,Ordering::AcqRel) + size;
                unsafe{
                    (*self.events.get())[index] = event;
                }
                break;
            }else{
                if self.size.compare_and_swap(size,size+1,Ordering::AcqRel) != size{
                    let index = self.first.load(Ordering::AcqRel) + size;
                    unsafe{
                        (*self.events.get())[index] = event;
                    }
                    break;
                }
            }
        }
    }
}

impl Index<usize> for EventQueue{
    type Output = Event;

    //The problems:
    //  How do we know were to start the event queue?
    fn index(&self,index: usize) -> &Event{
        let first = self.first.load(Ordering::Relaxed);
        let last = first + self.size.load(Ordering::Relaxed);
        if index < first || index >= last {
            panic!("Incorrect index EventQueue.");
        }
        unsafe{
            &(*self.events.get())[index % MAX_EVENTS]
        }
    }
}

//Old comments for documentation

// # about the modulo
//
//dont need to loop this one. if changes where made to first the other thread
//will also try to modulo the index. So eventually the index will be correct.
//However this does mean that a thread can load an incorrect first.
//In order to solve this each load must also try to correct the first.
//Than why not just have first keep increasing and loading handle the modulo?
//Hmmm..
//Good point, prob better.
