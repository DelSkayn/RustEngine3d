use std::any::TypeId;

pub struct RunArgs{
}

enum SystemError{
    Retry,
}

trait System{
    fn execute(&mut self,comp: RunArgs) -> bool;
}

impl<T> UpdateSystem for FnMut(&T){
}


struct Systems{
    tick: Vec<System>,
}

impl Systems{
    fn run(&mut self){
    }
}
