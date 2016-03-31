use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

pub struct Running{
    interal: AtomicBool,
}

impl Running{
    pub fn new() -> Self{
        Running{
            interal: AtomicBool::new(true),
        }
    }

    pub fn quit(&self){
        self.interal.store(false,Ordering::Release);
    }

    pub fn should(&self) -> bool{
        self.interal.load(Ordering::Acquire)
    }
}
