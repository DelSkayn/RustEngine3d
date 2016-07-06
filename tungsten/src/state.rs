
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

    pub fn running() -> bool{
        STATE.running.load(Ordering::Acquire)
    }

    pub fn quit(){
        STATE.running.store(false,Ordering::Release);
    }
}
