// 
// SCRAPED!!!!!!!!
//
// Consider this deleted.
// It is here for a reminder of how not to design 
// 
//

/*
use std::sync::atomic::{
    AtomicUsize,
    Ordering
};

use std::mem;
use std::ops::Index;

use std::cell::UnsafeCell;

use std::iter::Iterator;
use std::iter::IntoIterator;

use super::Event;



static MAX_EVENTS:usize = 1024;


pub struct EventQueue<T>{
    events: UnsafeCell<[T; 1024]>,
    size: AtomicUsize,
    first: AtomicUsize,
}


impl<T> EventQueue<T>{
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
    pub fn push(&self,event: T){
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
                    /// yeah no
                    /// This isnt gonna work
                    /// I dont know what you where thinking but....
                    /// This leads to all the reads of uninitialized mem.
                    let index = self.first.load(Ordering::AcqRel) + size;
                    unsafe{
                        (*self.events.get())[index] = event;
                    }
                    break;
                }
            }
        }
    }

    /// Clears the event queue.
    /// Can unfortunatly only be used when when there are 
    /// no borrows of the queu so it is unsafe.
    pub unsafe fn clear(&self){
        self.size.store(0,Ordering::Release);
    }
}

pub struct EventQueueIter<'a,T>{
    queue: &'a EventQueue,
    index: usize,
}

impl<'a,T> Iterator for EventQueue<'a,T>{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item>{
        let first = self.queue.first.load(Ordering::Acquire);
        if index < first {
            None
        }else{
            let size = self.queue.size.load(Ordering::Acquire);
            if index >= first+size{
                None
            }else{
                self.queue.
            }
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


#[cfg(test)]
mod test{
    use super::*;
    use std::sync::Arc;

    use std::thread::*;

    #[test]
    fn test_queue_cache(){
        let event_queue = Arc::new(EventQueue::new());
        let clone = event_queue.clone();
        let thread = spawn(move ||{
        });
    }
}
*/
