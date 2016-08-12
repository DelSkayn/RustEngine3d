/// This modules 
///
use std::sync::atomic::{AtomicBool,Ordering};

lazy_static!{static ref STATE: State = State::new();}

pub struct State{
    running: AtomicBool,
}

impl State{
    fn new() -> Self{
        State{
            running: AtomicBool::new(true),
        }
    }

    /// Returns wether the engine should run.
    pub fn running() -> bool{
        STATE.running.load(Ordering::Acquire)
    }

    /// Set the engine state to should no longer run.
    pub fn quit(){
        STATE.running.store(false,Ordering::Release);
    }
}
